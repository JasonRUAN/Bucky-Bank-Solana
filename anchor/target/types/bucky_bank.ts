/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/bucky_bank.json`.
 */
export type BuckyBank = {
  "address": "552NAsVPEGfW2pZ8wPdg5Apk77E7YCKeVH6rj8C46ctN",
  "metadata": {
    "name": "buckyBank",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "approveWithdrawal",
      "discriminator": [
        75,
        48,
        146,
        122,
        201,
        158,
        210,
        123
      ],
      "accounts": [
        {
          "name": "buckyBank",
          "writable": true
        },
        {
          "name": "withdrawalRequest",
          "writable": true
        },
        {
          "name": "parent",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "approve",
          "type": "bool"
        },
        {
          "name": "reason",
          "type": "string"
        }
      ]
    },
    {
      "name": "createBuckyBank",
      "discriminator": [
        253,
        239,
        86,
        153,
        43,
        27,
        85,
        218
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "bankGlobalStats",
          "writable": true
        },
        {
          "name": "buckyBank",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "bank_global_stats.total_bucky_banks",
                "account": "bankGlobalStatsInfo"
              }
            ]
          }
        },
        {
          "name": "userBuckyBanks",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  98,
                  117,
                  99,
                  107,
                  121,
                  95,
                  98,
                  97,
                  110,
                  107,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "targetAmount",
          "type": "u64"
        },
        {
          "name": "durationDays",
          "type": "u64"
        },
        {
          "name": "childAddress",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "deposit",
      "discriminator": [
        242,
        35,
        198,
        137,
        82,
        225,
        242,
        182
      ],
      "accounts": [
        {
          "name": "bankGlobalStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              }
            ]
          }
        },
        {
          "name": "buckyBank",
          "writable": true
        },
        {
          "name": "depositor",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "depositBalance",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initializeBankGlobalStats",
      "discriminator": [
        143,
        138,
        143,
        246,
        111,
        29,
        108,
        54
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "bankGlobalStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "requestWithdrawal",
      "discriminator": [
        251,
        85,
        121,
        205,
        56,
        201,
        12,
        177
      ],
      "accounts": [
        {
          "name": "buckyBank",
          "writable": true
        },
        {
          "name": "withdrawalRequest",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  119,
                  105,
                  116,
                  104,
                  100,
                  114,
                  97,
                  119,
                  97,
                  108,
                  95,
                  114,
                  101,
                  113,
                  117,
                  101,
                  115,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "buckyBank"
              },
              {
                "kind": "account",
                "path": "requester"
              }
            ]
          }
        },
        {
          "name": "requester",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "reason",
          "type": "string"
        }
      ]
    },
    {
      "name": "withdraw",
      "discriminator": [
        183,
        18,
        70,
        156,
        148,
        109,
        161,
        34
      ],
      "accounts": [
        {
          "name": "buckyBank",
          "writable": true
        },
        {
          "name": "withdrawalRequest",
          "writable": true
        },
        {
          "name": "child",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "bankGlobalStatsInfo",
      "discriminator": [
        99,
        201,
        148,
        111,
        113,
        116,
        70,
        181
      ]
    },
    {
      "name": "buckyBankInfo",
      "discriminator": [
        30,
        156,
        155,
        19,
        76,
        154,
        184,
        226
      ]
    },
    {
      "name": "userBuckyBanksInfo",
      "discriminator": [
        81,
        225,
        128,
        17,
        109,
        63,
        48,
        203
      ]
    },
    {
      "name": "withdrawalRequestInfo",
      "discriminator": [
        147,
        251,
        173,
        62,
        167,
        179,
        42,
        159
      ]
    }
  ],
  "events": [
    {
      "name": "buckyBankCreated",
      "discriminator": [
        94,
        162,
        71,
        209,
        203,
        17,
        36,
        29
      ]
    },
    {
      "name": "depositMade",
      "discriminator": [
        210,
        201,
        130,
        183,
        244,
        203,
        155,
        199
      ]
    },
    {
      "name": "eventWithdrawalApproved",
      "discriminator": [
        110,
        116,
        106,
        81,
        22,
        92,
        18,
        109
      ]
    },
    {
      "name": "eventWithdrawalCompleted",
      "discriminator": [
        73,
        44,
        220,
        56,
        116,
        7,
        126,
        222
      ]
    },
    {
      "name": "eventWithdrawalRejected",
      "discriminator": [
        86,
        247,
        83,
        201,
        215,
        46,
        197,
        129
      ]
    },
    {
      "name": "eventWithdrawalRequested",
      "discriminator": [
        127,
        108,
        117,
        193,
        156,
        83,
        208,
        230
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "depositTooSmall",
      "msg": "Deposit amount must be at least 0.01 SOL"
    },
    {
      "code": 6001,
      "name": "insufficientFunds",
      "msg": "Insufficient funds for withdrawal"
    },
    {
      "code": 6002,
      "name": "invalidName",
      "msg": ""
    },
    {
      "code": 6003,
      "name": "invalidAmount",
      "msg": ""
    },
    {
      "code": 6004,
      "name": "invalidDeadline",
      "msg": ""
    },
    {
      "code": 6005,
      "name": "overflow",
      "msg": ""
    },
    {
      "code": 6006,
      "name": "bankNotActive",
      "msg": ""
    },
    {
      "code": 6007,
      "name": "notChild",
      "msg": ""
    },
    {
      "code": 6008,
      "name": "invalidDepositAmount",
      "msg": ""
    },
    {
      "code": 6009,
      "name": "notChildForWithdrawal",
      "msg": ""
    },
    {
      "code": 6010,
      "name": "invalidWithdrawalAmount",
      "msg": ""
    },
    {
      "code": 6011,
      "name": "reasonTooLong",
      "msg": ""
    },
    {
      "code": 6012,
      "name": "notParent",
      "msg": ""
    },
    {
      "code": 6013,
      "name": "invalidRequestStatus",
      "msg": ""
    },
    {
      "code": 6014,
      "name": "requestNotFound",
      "msg": ""
    }
  ],
  "types": [
    {
      "name": "bankGlobalStatsInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalBuckyBanks",
            "type": "u64"
          },
          {
            "name": "totalDeposits",
            "type": "u64"
          },
          {
            "name": "totalWithdrawals",
            "type": "u64"
          },
          {
            "name": "admin",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "buckyBankCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "buckyBankId",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "parent",
            "type": "pubkey"
          },
          {
            "name": "child",
            "type": "pubkey"
          },
          {
            "name": "targetAmount",
            "type": "u64"
          },
          {
            "name": "createdAtMs",
            "type": "u64"
          },
          {
            "name": "deadlineMs",
            "type": "u64"
          },
          {
            "name": "durationDays",
            "type": "u64"
          },
          {
            "name": "currentBalance",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "buckyBankInfo",
      "docs": [
        "存钱罐对象"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "parent",
            "docs": [
              "存钱罐创建方地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "config",
            "docs": [
              "存钱罐配置"
            ],
            "type": {
              "defined": {
                "name": "config"
              }
            }
          },
          {
            "name": "currentBalance",
            "docs": [
              "当前存款余额（单位：lamports）"
            ],
            "type": "u64"
          },
          {
            "name": "status",
            "docs": [
              "存钱罐状态"
            ],
            "type": {
              "defined": {
                "name": "buckyBankStatus"
              }
            }
          },
          {
            "name": "depositCount",
            "docs": [
              "存款次数"
            ],
            "type": "u64"
          },
          {
            "name": "createdAtMs",
            "docs": [
              "存钱罐创建时间（毫秒）"
            ],
            "type": "u64"
          },
          {
            "name": "lastDepositMs",
            "docs": [
              "最近一次存款时间（毫秒）"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "buckyBankStatus",
      "docs": [
        "存钱罐状态枚举"
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "active"
          },
          {
            "name": "completed"
          },
          {
            "name": "failed"
          }
        ]
      }
    },
    {
      "name": "config",
      "docs": [
        "存钱罐配置"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "docs": [
              "存钱罐名称"
            ],
            "type": "string"
          },
          {
            "name": "targetAmount",
            "docs": [
              "目标存款金额（单位：lamports）"
            ],
            "type": "u64"
          },
          {
            "name": "deadlineMs",
            "docs": [
              "存款截止时间戳（毫秒）"
            ],
            "type": "u64"
          },
          {
            "name": "childAddress",
            "docs": [
              "存钱罐使用方地址"
            ],
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "depositMade",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "buckyBankId",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "depositor",
            "type": "pubkey"
          },
          {
            "name": "createdAtMs",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "eventWithdrawalApproved",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "requestId",
            "type": "pubkey"
          },
          {
            "name": "buckyBankId",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "approvedBy",
            "type": "pubkey"
          },
          {
            "name": "requester",
            "type": "pubkey"
          },
          {
            "name": "reason",
            "type": "string"
          },
          {
            "name": "createdAtMs",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "eventWithdrawalCompleted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "requestId",
            "type": "pubkey"
          },
          {
            "name": "buckyBankId",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "leftBalance",
            "type": "u64"
          },
          {
            "name": "withdrawer",
            "type": "pubkey"
          },
          {
            "name": "createdAtMs",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "eventWithdrawalRejected",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "requestId",
            "type": "pubkey"
          },
          {
            "name": "buckyBankId",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "requester",
            "type": "pubkey"
          },
          {
            "name": "rejectedBy",
            "type": "pubkey"
          },
          {
            "name": "reason",
            "type": "string"
          },
          {
            "name": "createdAtMs",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "eventWithdrawalRequested",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "requestId",
            "type": "pubkey"
          },
          {
            "name": "buckyBankId",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "requester",
            "type": "pubkey"
          },
          {
            "name": "reason",
            "type": "string"
          },
          {
            "name": "status",
            "type": "u8"
          },
          {
            "name": "approvedBy",
            "type": "pubkey"
          },
          {
            "name": "createdAtMs",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "userBuckyBanksInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "buckyBankIds",
            "type": {
              "vec": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "withdrawalRequestInfo",
      "docs": [
        "取款请求对象"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "buckyBankId",
            "docs": [
              "存钱罐 ID"
            ],
            "type": "pubkey"
          },
          {
            "name": "requester",
            "docs": [
              "请求者地址（孩子）"
            ],
            "type": "pubkey"
          },
          {
            "name": "amount",
            "docs": [
              "取款金额（单位：lamports）"
            ],
            "type": "u64"
          },
          {
            "name": "reason",
            "docs": [
              "取款原因"
            ],
            "type": "string"
          },
          {
            "name": "status",
            "docs": [
              "取款请求状态"
            ],
            "type": {
              "defined": {
                "name": "withdrawalStatus"
              }
            }
          },
          {
            "name": "approvedBy",
            "docs": [
              "审批者地址（家长）"
            ],
            "type": "pubkey"
          },
          {
            "name": "createdAtMs",
            "docs": [
              "创建时间（毫秒）"
            ],
            "type": "u64"
          },
          {
            "name": "approvedAtMs",
            "docs": [
              "审批时间（毫秒）"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "withdrawalStatus",
      "docs": [
        "取款请求状态枚举"
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "pending"
          },
          {
            "name": "approved"
          },
          {
            "name": "rejected"
          },
          {
            "name": "completed"
          }
        ]
      }
    }
  ]
};
