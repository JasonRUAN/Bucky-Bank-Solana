use crate::database::{
    Database,
    models::{NewBuckyBankCreatedEvent, NewDepositMadeEvent, NewWithdrawalRequestEvent, NewEventWithdrawedEvent, NewCursor, WithdrawalStatus},
};
use anyhow::Result;
use std::sync::Arc;
use solana_client::rpc_client::{RpcClient, GetConfirmedSignaturesForAddress2Config};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use tracing::{debug, error, info};
use base64::Engine;
use borsh::BorshDeserialize;

// Anchor 事件结构体定义（用于 Borsh 反序列化）
#[derive(Debug, BorshDeserialize)]
pub struct BuckyBankCreatedEvent {
    pub bucky_bank_id: Pubkey,
    pub name: String,
    pub parent: Pubkey,
    pub child: Pubkey,
    pub target_amount: u64,
    pub created_at_ms: u64,
    pub deadline_ms: u64,
    pub duration_days: u64,
    pub current_balance: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct DepositMadeEvent {
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub depositor: Pubkey,
    pub created_at_ms: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct EventWithdrawalRequestedEvent {
    pub request_id: Pubkey,
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub requester: Pubkey,
    pub reason: String,
    pub status: u8,
    pub approved_by: Pubkey,
    pub created_at_ms: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct EventWithdrawalApprovedEvent {
    pub request_id: Pubkey,
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub approved_by: Pubkey,
    pub requester: Pubkey,
    pub reason: String,
    pub created_at_ms: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct EventWithdrawalRejectedEvent {
    pub request_id: Pubkey,
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub rejected_by: Pubkey,
    pub requester: Pubkey,
    pub reason: String,
    pub created_at_ms: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct EventWithdrawalCompletedEvent {
    pub request_id: Pubkey,
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub requester: Pubkey,
    pub created_at_ms: u64,
}

#[derive(Debug)]
pub struct EventProcessingResult {
    pub total_processed: usize,
    pub has_next_page: bool,
}

#[derive(Debug, Clone)]
pub enum EventType {
    BuckyBankCreated,
    DepositMade,
    WithdrawalRequested,
    WithdrawalApproved,
    WithdrawalRejected,
    EventWithdrawed,
}

impl EventType {
    pub fn name(&self) -> &'static str {
        match self {
            EventType::BuckyBankCreated => "BuckyBankCreated",
            EventType::DepositMade => "DepositMade",
            EventType::WithdrawalRequested => "EventWithdrawalRequested",
            EventType::WithdrawalApproved => "EventWithdrawalApproved",
            EventType::WithdrawalRejected => "EventWithdrawalRejected",
            EventType::EventWithdrawed => "EventWithdrawalCompleted",
        }
    }

    pub fn all_event_types() -> Vec<EventType> {
        vec![
            EventType::BuckyBankCreated, 
            EventType::DepositMade, 
            EventType::WithdrawalRequested,
            EventType::WithdrawalApproved,
            EventType::WithdrawalRejected,
            EventType::EventWithdrawed,
        ]
    }
}

pub struct BuckyBankIndexer {
    client: Arc<RpcClient>,
    program_id: Pubkey,
    db: Arc<Database>,
}

impl BuckyBankIndexer {
    pub fn new(
        client: Arc<RpcClient>,
        program_id: String,
        db: Arc<Database>,
    ) -> Result<Self> {
        info!("Attempting to parse program_id: '{}'", program_id);
        let program_id = match program_id.parse::<Pubkey>() {
            Ok(pk) => {
                info!("Successfully parsed program_id: {}", pk);
                pk
            }
            Err(e) => {
                error!("Failed to parse program_id '{}': {}", program_id, e);
                return Err(anyhow::anyhow!("Invalid program_id '{}': {}", program_id, e));
            }
        };
        Ok(Self {
            client,
            program_id,
            db,
        })
    }

    pub async fn query_and_process_events(&self) -> Result<EventProcessingResult> {
        info!("\n\n###################################################################\n\nQuerying all BuckyBank events...");

        let mut total_processed = 0;
        let mut has_next_page = false;

        for event_type in EventType::all_event_types() {
            info!("Processing {} events...", event_type.name());

            match self.query_and_process_events_for_type(&event_type).await {
                Ok((count, next_page)) => {
                    total_processed += count;
                    has_next_page = has_next_page || next_page;
                    info!("Processed {} {} events", count, event_type.name());
                }
                Err(e) => {
                    error!("Failed to process {} events: {}", event_type.name(), e);
                }
            }
        }

        Ok(EventProcessingResult {
            total_processed,
            has_next_page,
        })
    }

    pub async fn query_and_process_events_for_type(&self, event_type: &EventType) -> Result<(usize, bool)> {
        info!(">>> Querying {} events with cursor...", event_type.name());

        // 获取最新的游标
        let cursor_signature = self.get_latest_cursor(event_type.name()).await?;
        
        // ✅ 修复：使用 until 参数而不是 before
        // until: 返回在该签名之后的所有签名（不包含该签名本身）
        // before: 返回在该签名之前的所有签名（这是错误的用法）
        let until_sig = if let Some(cursor_sig_str) = cursor_signature.as_ref() {
            match cursor_sig_str.parse::<Signature>() {
                Ok(sig) => {
                    info!("Fetching signatures after cursor: {}", cursor_sig_str);
                    Some(sig)
                }
                Err(e) => {
                    error!("Failed to parse cursor signature '{}': {}, fetching latest signatures", cursor_sig_str, e);
                    None
                }
            }
        } else {
            info!("No cursor found, fetching latest signatures");
            None
        };
        
        let signatures = self.client.get_signatures_for_address_with_config(
            &self.program_id,
            GetConfirmedSignaturesForAddress2Config {
                before: None,
                until: until_sig,
                limit: Some(100),
                commitment: None,
            },
        )?;
        
        info!("Found {} new signatures for program", signatures.len());

        let mut processed_count = 0;
        let mut latest_signature = None;
        let mut latest_slot: Option<u64> = None;

        for sig_info in signatures {

            // 获取交易详情
            info!("[DEBUG] Processing signature: '{}' (slot: {})", sig_info.signature, sig_info.slot);
            let signature: Signature = match sig_info.signature.parse() {
                Ok(sig) => {
                    debug!("Successfully parsed signature");
                    sig
                }
                Err(e) => {
                    error!("Failed to parse signature '{}': {}", sig_info.signature, e);
                    continue;
                }
            };
            match self.client.get_transaction(&signature, UiTransactionEncoding::Json) {
                Ok(transaction) => {
                    info!("[DEBUG] Got transaction: {}", sig_info.signature);

                    // 解析交易中的事件
                    if let Some(meta) = transaction.transaction.meta.as_ref() {
                        let empty_vec = vec![];
                        let log_messages = meta.log_messages.as_ref().unwrap_or(&empty_vec);
                        info!("[DEBUG] Transaction has {} log messages", log_messages.len());
                        
                        // 处理交易中的所有事件（可能有多个）
                        if let Ok(events_processed) = self.process_transaction_events(log_messages, event_type).await {
                            if events_processed > 0 {
                                processed_count += events_processed;
                                info!(
                                    "Successfully processed {} {} events from tx: {}",
                                    events_processed, event_type.name(), sig_info.signature
                                );
                            }
                        }
                    } else {
                        info!("[DEBUG] Transaction has no meta information");
                    }
                    
                    // 始终更新latest_signature和latest_slot，即使没有处理事件
                    // 这样可以防止重复查询同一个签名
                    latest_signature = Some(sig_info.signature.to_string());
                    latest_slot = Some(sig_info.slot);
                }
                Err(e) => {
                    error!("Failed to get transaction {}: {}", sig_info.signature, e);
                }
            }
        }

        // 更新游标：无论是否处理了事件，都要更新游标到最新的签名，防止重复查询
        if let (Some(signature), Some(slot)) = (latest_signature, latest_slot) {
            self.update_cursor_with_metadata(
                event_type.name(),
                &signature,
                slot as i64,
                processed_count as i64,
            )
            .await?;
            info!(
                "Updated cursor for {} to signature={}, slot={} after processing {} events",
                event_type.name(), signature, slot, processed_count
            );
        }

        info!("Processed {} {} events", processed_count, event_type.name());
        Ok((processed_count, false)) // Solana 不需要分页
    }

    pub async fn run_continuous_polling(&self) -> Result<()> {
        loop {
            info!("next loop...");

            let ctrl_c = tokio::signal::ctrl_c();
            tokio::pin!(ctrl_c);

            tokio::select! {
                poll_result = self.query_and_process_events() => {
                    match poll_result {
                        Ok(result) => {
                            if result.total_processed > 0 {
                                info!("Processed events, waiting 1 second...");
                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            } else {
                                info!("No new events, waiting for next poll cycle...");
                                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                info!("wake up...");
                            }
                        }
                        Err(e) => {
                            error!("Error during polling: {}", e);
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                }

                _ = &mut ctrl_c => {
                    info!("Received shutdown signal, stopping polling");
                    break;
                }
            }

            info!("finish...");
        }
        Ok(())
    }

    async fn get_latest_cursor(&self, event_type: &str) -> Result<Option<String>> {
        match self.db.get_cursor(event_type).await? {
            Some(cursor) => Ok(cursor.last_processed_signature),
            None => Ok(None),
        }
    }

    async fn update_cursor_with_metadata(
        &self,
        event_type: &str,
        signature: &str,
        slot: i64,
        total_events: i64,
    ) -> Result<()> {
        let cursor = NewCursor {
            id: event_type.to_string(),
            last_processed_signature: Some(signature.to_string()),
            last_processed_slot: Some(slot),
            total_events_processed: Some(total_events),
        };

        self.db.save_cursor(&cursor).await?;
        Ok(())
    }

    async fn process_transaction_events(&self, log_messages: &[String], event_type: &EventType) -> Result<usize> {
        let mut events_processed = 0;
        let mut i = 0;
        
        info!("[DEBUG] === Processing {} log messages for {} ===", log_messages.len(), event_type.name());
        for (idx, msg) in log_messages.iter().enumerate() {
            info!("[DEBUG] Log[{}]: {}", idx, msg);
        }
        
        while i < log_messages.len() {
            let log = &log_messages[i];
            
            // 检查是否是指令日志
            if self.matches_event_type(log, event_type) {
                info!("[DEBUG] ✓ Matched {} event in log: {}", event_type.name(), log);
                
                // 查找后续的 "Program data:" 行
                let mut j = i + 1;
                let mut found_data = false;
                while j < log_messages.len() {
                    let next_log = &log_messages[j];
                    info!("[DEBUG] Checking log at index {}: {}", j, next_log);
                    
                    if next_log.starts_with("Program data:") {
                        info!("[DEBUG] Found Program data at index {}: {}", j, next_log);
                        found_data = true;
                        
                        // 提取 base64 数据
                        if let Some(data_str) = next_log.strip_prefix("Program data: ") {
                            match self.process_event_from_base64(data_str, event_type).await {
                                Ok(_) => {
                                    events_processed += 1;
                                    info!("Successfully processed {} event from base64 data", event_type.name());
                                }
                                Err(e) => {
                                    error!("Failed to process {} event from base64: {}", event_type.name(), e);
                                }
                            }
                        }
                        break;
                    } else if next_log.starts_with("Program") && !next_log.starts_with("Program data:") {
                        // 遇到其他程序日志，停止查找
                        info!("[DEBUG] Encountered other program log, stopping search: {}", next_log);
                        // 这个地方导致没有入库，不是搞啥呀
                        // break;
                    }
                    
                    j += 1;
                }
                
                if !found_data {
                    info!("[DEBUG] ⚠ No Program data found after matched event");
                }
            }
            
            i += 1;
        }
        
        info!("[DEBUG] === Finished processing, events_processed: {} ===\n\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n", events_processed);
        Ok(events_processed)
    }

    async fn process_event_from_base64(&self, base64_data: &str, event_type: &EventType) -> Result<()> {
        // 解码 base64 数据
        let decoded = base64::engine::general_purpose::STANDARD.decode(base64_data)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64: {}", e))?;
        
        info!("[DEBUG] Decoded {} bytes from base64", decoded.len());
        
        // 根据事件类型反序列化
        match event_type {
            EventType::BuckyBankCreated => self.process_bucky_bank_created_from_bytes(&decoded).await,
            EventType::DepositMade => self.process_deposit_made_from_bytes(&decoded).await,
            EventType::WithdrawalRequested => self.process_withdrawal_requested_from_bytes(&decoded).await,
            EventType::WithdrawalApproved => self.process_withdrawal_approved_from_bytes(&decoded).await,
            EventType::WithdrawalRejected => self.process_withdrawal_rejected_from_bytes(&decoded).await,
            EventType::EventWithdrawed => self.process_event_withdrawed_from_bytes(&decoded).await,
        }
    }

    fn matches_event_type(&self, log: &str, event_type: &EventType) -> bool {
        // Solana 事件通过日志中的特定前缀来识别
        // 格式通常是: "Program log: Instruction: <instruction_name>" 或 "Program log: <event_name>: <data>"
        let patterns = match event_type {
            EventType::BuckyBankCreated => vec!["Instruction: CreateBuckyBank", "BuckyBankCreated"],
            EventType::DepositMade => vec!["Instruction: Deposit", "DepositMade"],
            EventType::WithdrawalRequested => vec!["Instruction: RequestWithdrawal", "WithdrawalRequested", "EventWithdrawalRequested"],
            EventType::WithdrawalApproved => vec!["Instruction: ApproveWithdrawal", "WithdrawalApproved", "EventWithdrawalApproved"],
            EventType::WithdrawalRejected => vec!["Instruction: RejectWithdrawal", "WithdrawalRejected", "EventWithdrawalRejected"],
            EventType::EventWithdrawed => vec!["Instruction: Withdraw", "EventWithdrawalCompleted", "EventWithdrawed"],
        };
        
        let matches = patterns.iter().any(|pattern| log.contains(pattern));
        
        if !matches {
            debug!("[DEBUG] Log does NOT match {:?} - checking patterns: {:?} in log: {}", 
                event_type, patterns, log);
        } else {
            info!("[DEBUG] ✓ Matched {:?} with log: {}", event_type, log);
        }
        
        matches
    }

    async fn process_event_from_log(&self, log: &str, event_type: &EventType) -> Result<()> {
        // 从日志中提取事件数据
        // 这里需要根据实际的日志格式进行解析
        match event_type {
            EventType::BuckyBankCreated => self.process_bucky_bank_created_event(log).await,
            EventType::DepositMade => self.process_deposit_made_event(log).await,
            EventType::WithdrawalRequested => self.process_withdrawal_requested_event(log).await,
            EventType::WithdrawalApproved => self.process_withdrawal_approved_event(log).await,
            EventType::WithdrawalRejected => self.process_withdrawal_rejected_event(log).await,
            EventType::EventWithdrawed => self.process_event_withdrawed_event(log).await,
        }
    }

    async fn process_bucky_bank_created_event(&self, log: &str) -> Result<()> {
        debug!("Processing BuckyBankCreated event from log");

        // 解析日志中的事件数据
        // 格式: "Program log: BuckyBankCreated: <json_data>"
        let event_data = self.extract_event_data(log, "BuckyBankCreated")?;

        let bucky_bank_id = event_data
            .get("bucky_bank_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing bucky_bank_id"))?;

        let name = event_data
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing name"))?;

        let parent_address = event_data
            .get("parent")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing parent"))?;

        let child_address = event_data
            .get("child")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing child"))?;

        let target_amount = event_data
            .get("target_amount")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing target_amount"))?;

        let created_at_ms = event_data
            .get("created_at_ms")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing created_at_ms"))?;

        let deadline_ms = event_data
            .get("deadline_ms")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing deadline_ms"))?;

        let duration_days = event_data
            .get("duration_days")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing duration_days"))?;

        let current_balance = event_data
            .get("current_balance")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing current_balance"))?;

        let new_event = NewBuckyBankCreatedEvent {
            bucky_bank_id: bucky_bank_id.to_string(),
            name: name.to_string(),
            parent_address: parent_address.to_string(),
            child_address: child_address.to_string(),
            target_amount: target_amount as i64,
            created_at_ms: created_at_ms as i64,
            deadline_ms: deadline_ms as i64,
            duration_days: duration_days as i64,
            current_balance: current_balance as i64,
        };

        match self.db.save_bucky_bank_created_event(&new_event).await {
            Ok(saved_event) => {
                info!("Saved BuckyBankCreated event: {}", saved_event.id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to save BuckyBankCreated event: {}", e);
                Err(e)
            }
        }
    }

    async fn process_deposit_made_event(&self, log: &str) -> Result<()> {
        debug!("Processing DepositMade event from log");

        let event_data = self.extract_event_data(log, "DepositMade")?;

        let bucky_bank_id = event_data
            .get("bucky_bank_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing bucky_bank_id"))?;

        let amount = event_data
            .get("amount")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing amount"))?;

        let depositor = event_data
            .get("depositor")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing depositor"))?;

        let created_at_ms = event_data
            .get("created_at_ms")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing created_at_ms"))?;

        let new_event = NewDepositMadeEvent {
            bucky_bank_id: bucky_bank_id.to_string(),
            amount: amount as i64,
            depositor: depositor.to_string(),
            created_at_ms: created_at_ms as i64,
        };

        match self.db.save_deposit_made_event(&new_event).await {
            Ok(saved_event) => {
                info!("Saved DepositMade event: {}", saved_event.id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to save DepositMade event: {}", e);
                Err(e)
            }
        }
    }

    async fn process_withdrawal_requested_event(&self, log: &str) -> Result<()> {
        debug!("Processing EventWithdrawalRequested event from log");

        let event_data = self.extract_event_data(log, "EventWithdrawalRequested")?;

        let request_id = event_data
            .get("request_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing request_id"))?;

        let bucky_bank_id = event_data
            .get("bucky_bank_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing bucky_bank_id"))?;

        let amount = event_data
            .get("amount")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing amount"))?;

        let requester = event_data
            .get("requester")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing requester"))?;

        let reason = event_data
            .get("reason")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing reason"))?;

        let status_num = event_data
            .get("status")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing status"))?;

        let status = match status_num {
            0 => WithdrawalStatus::Pending,
            1 => WithdrawalStatus::Approved,
            2 => WithdrawalStatus::Rejected,
            3 => WithdrawalStatus::Withdrawed,
            _ => WithdrawalStatus::Pending,
        };

        let approved_by = event_data
            .get("approved_by")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let created_at_ms = event_data
            .get("created_at_ms")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing created_at_ms"))?;

        let new_event = NewWithdrawalRequestEvent {
            request_id: request_id.to_string(),
            bucky_bank_id: bucky_bank_id.to_string(),
            amount: amount as i64,
            requester: requester.to_string(),
            reason: reason.to_string(),
            status: status.to_string(),
            approved_by: approved_by.unwrap_or_default(),
            created_at_ms: created_at_ms as i64,
            audit_at_ms: None,
            tx_digest: String::new(),
            event_seq: 0,
            timestamp_ms: 0,
        };

        match self.db.save_withdrawal_request_event(&new_event).await {
            Ok(saved_event) => {
                info!("Saved EventWithdrawalRequested event: {}", saved_event.id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to save EventWithdrawalRequested event: {}", e);
                Err(e)
            }
        }
    }

    async fn process_withdrawal_approved_event(&self, _log: &str) -> Result<()> {
        debug!("Processing EventWithdrawalApproved event from log");
        // 实现类似的逻辑
        Ok(())
    }

    async fn process_withdrawal_rejected_event(&self, _log: &str) -> Result<()> {
        debug!("Processing EventWithdrawalRejected event from log");
        // 实现类似的逻辑
        Ok(())
    }

    async fn process_event_withdrawed_event(&self, log: &str) -> Result<()> {
        debug!("Processing EventWithdrawalCompleted event from log");

        let event_data = self.extract_event_data(log, "EventWithdrawalCompleted")?;

        let request_id = event_data
            .get("request_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing request_id"))?;

        let bucky_bank_id = event_data
            .get("bucky_bank_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing bucky_bank_id"))?;

        let amount = event_data
            .get("amount")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing amount"))?;

        let left_balance = event_data
            .get("left_balance")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing left_balance"))?;

        let withdrawer = event_data
            .get("withdrawer")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing withdrawer"))?;

        let created_at_ms = event_data
            .get("created_at_ms")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing created_at_ms"))?;

        let new_event = NewEventWithdrawedEvent {
            request_id: request_id.to_string(),
            bucky_bank_id: bucky_bank_id.to_string(),
            amount: amount as i64,
            left_balance: left_balance as i64,
            withdrawer: withdrawer.to_string(),
            created_at_ms: created_at_ms as i64,
        };

        match self.db.save_event_withdrawed_event(&new_event).await {
            Ok(saved_event) => {
                info!("Saved EventWithdrawalCompleted event: {}", saved_event.id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to save EventWithdrawalCompleted event: {}", e);
                Err(e)
            }
        }
    }

    async fn process_bucky_bank_created_from_bytes(&self, bytes: &[u8]) -> Result<()> {
        // 跳过 Anchor 事件头（8 字节）
        if bytes.len() < 8 {
            return Err(anyhow::anyhow!("Event data too short"));
        }
        
        let event: BuckyBankCreatedEvent = BorshDeserialize::deserialize(&mut &bytes[8..])?;
        info!("[DEBUG] Deserialized BuckyBankCreated: {:?}", event);
        
        let new_event = NewBuckyBankCreatedEvent {
            bucky_bank_id: event.bucky_bank_id.to_string(),
            name: event.name,
            parent_address: event.parent.to_string(),
            child_address: event.child.to_string(),
            target_amount: event.target_amount as i64,
            created_at_ms: event.created_at_ms as i64,
            deadline_ms: event.deadline_ms as i64,
            duration_days: event.duration_days as i64,
            current_balance: event.current_balance as i64,
        };
        
        self.db.save_bucky_bank_created_event(&new_event).await?;
        info!("Saved BuckyBankCreated event to database");
        Ok(())
    }

    async fn process_deposit_made_from_bytes(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() < 8 {
            return Err(anyhow::anyhow!("Event data too short"));
        }
        
        let event: DepositMadeEvent = BorshDeserialize::deserialize(&mut &bytes[8..])?;
        info!("[DEBUG] Deserialized DepositMade: {:?}", event);
        
        let new_event = NewDepositMadeEvent {
            bucky_bank_id: event.bucky_bank_id.to_string(),
            amount: event.amount as i64,
            depositor: event.depositor.to_string(),
            created_at_ms: event.created_at_ms as i64,
        };
        
        self.db.save_deposit_made_event(&new_event).await?;
        info!("Saved DepositMade event to database");
        Ok(())
    }

    async fn process_withdrawal_requested_from_bytes(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() < 8 {
            return Err(anyhow::anyhow!("Event data too short"));
        }
        
        let event: EventWithdrawalRequestedEvent = BorshDeserialize::deserialize(&mut &bytes[8..])?;
        info!("[DEBUG] Deserialized EventWithdrawalRequested: {:?}", event);
        
        let status_str = match event.status {
            0 => "Pending",
            1 => "Approved",
            2 => "Rejected",
            3 => "Withdrawed",
            _ => "Pending",
        };
        
        let new_event = NewWithdrawalRequestEvent {
            request_id: event.request_id.to_string(),
            bucky_bank_id: event.bucky_bank_id.to_string(),
            amount: event.amount as i64,
            requester: event.requester.to_string(),
            reason: event.reason,
            status: status_str.to_string(),
            approved_by: event.approved_by.to_string(),
            created_at_ms: event.created_at_ms as i64,
            audit_at_ms: None,
            tx_digest: String::new(),
            event_seq: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };
        
        self.db.save_withdrawal_request_event(&new_event).await?;
        info!("Saved EventWithdrawalRequested event to database");
        Ok(())
    }

    async fn process_withdrawal_approved_from_bytes(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() < 8 {
            return Err(anyhow::anyhow!("Event data too short"));
        }
        
        let event: EventWithdrawalApprovedEvent = BorshDeserialize::deserialize(&mut &bytes[8..])?;
        info!("[DEBUG] Deserialized EventWithdrawalApproved: {:?}", event);
        
        // 更新数据库中的提现请求状态
        self.db.update_withdrawal_request_status(
            &event.request_id.to_string(),
            &WithdrawalStatus::Approved,
            Some(&event.approved_by.to_string()),
            Some(chrono::Utc::now().timestamp_millis()),
        ).await?;
        
        info!("Updated withdrawal request status to Approved");
        Ok(())
    }

    async fn process_withdrawal_rejected_from_bytes(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() < 8 {
            return Err(anyhow::anyhow!("Event data too short"));
        }
        
        let event: EventWithdrawalRejectedEvent = BorshDeserialize::deserialize(&mut &bytes[8..])?;
        info!("[DEBUG] Deserialized EventWithdrawalRejected: {:?}", event);
        
        // 更新数据库中的提现请求状态
        self.db.update_withdrawal_request_status(
            &event.request_id.to_string(),
            &WithdrawalStatus::Rejected,
            Some(&event.rejected_by.to_string()),
            Some(chrono::Utc::now().timestamp_millis()),
        ).await?;
        
        info!("Updated withdrawal request status to Rejected");
        Ok(())
    }

    async fn process_event_withdrawed_from_bytes(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() < 8 {
            return Err(anyhow::anyhow!("Event data too short"));
        }
        
        let event: EventWithdrawalCompletedEvent = BorshDeserialize::deserialize(&mut &bytes[8..])?;
        info!("[DEBUG] Deserialized EventWithdrawalCompleted: {:?}", event);
        
        let new_event = NewEventWithdrawedEvent {
            request_id: event.request_id.to_string(),
            bucky_bank_id: event.bucky_bank_id.to_string(),
            amount: event.amount as i64,
            left_balance: 0, // 需要从合约获取
            withdrawer: event.requester.to_string(),
            created_at_ms: event.created_at_ms as i64,
        };
        
        self.db.save_event_withdrawed_event(&new_event).await?;
        info!("Saved EventWithdrawalCompleted event to database");
        Ok(())
    }

    fn extract_event_data(&self, log: &str, event_name: &str) -> Result<serde_json::Value> {
        // 从日志中提取 JSON 数据
        // 格式: "Program log: <event_name>: <json_data>"
        if let Some(start) = log.find(&format!("{}: ", event_name)) {
            let data_str = &log[start + event_name.len() + 2..];
            if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(data_str) {
                return Ok(json_data);
            }
        }
        
        // 如果找不到标准格式，尝试解析整个日志
        Err(anyhow::anyhow!("Failed to extract event data from log: {}", log))
    }
}
