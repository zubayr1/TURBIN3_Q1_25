/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/one_cs.json`.
 */
export type OneCs = {
  "address": "coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF",
  "metadata": {
    "name": "oneCs",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addPermission",
      "docs": [
        "Add a new permission to the permission list"
      ],
      "discriminator": [
        144,
        66,
        124,
        76,
        232,
        97,
        99,
        77
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "encapsulatedData",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  101,
                  114,
                  109,
                  105,
                  115,
                  115,
                  105,
                  111,
                  110,
                  115
                ]
              },
              {
                "kind": "const",
                "value": [
                  9,
                  43,
                  225,
                  230,
                  237,
                  13,
                  216,
                  237,
                  44,
                  121,
                  170,
                  202,
                  171,
                  13,
                  123,
                  122,
                  17,
                  76,
                  28,
                  165,
                  22,
                  109,
                  84,
                  97,
                  252,
                  22,
                  20,
                  59,
                  1,
                  116,
                  88,
                  174
                ]
              },
              {
                "kind": "arg",
                "path": "label"
              }
            ]
          }
        },
        {
          "name": "permittedWallet"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "label",
          "type": "string"
        },
        {
          "name": "roleIndex",
          "type": "u64"
        },
        {
          "name": "startTime",
          "type": "u64"
        },
        {
          "name": "endTime",
          "type": "u64"
        }
      ]
    },
    {
      "name": "encapsulate",
      "docs": [
        "Encapsulate the data and create a new permission data account"
      ],
      "discriminator": [
        169,
        85,
        157,
        64,
        95,
        156,
        28,
        101
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "encapsulatedData",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  101,
                  114,
                  109,
                  105,
                  115,
                  115,
                  105,
                  111,
                  110,
                  115
                ]
              },
              {
                "kind": "const",
                "value": [
                  9,
                  43,
                  225,
                  230,
                  237,
                  13,
                  216,
                  237,
                  44,
                  121,
                  170,
                  202,
                  171,
                  13,
                  123,
                  122,
                  17,
                  76,
                  28,
                  165,
                  22,
                  109,
                  84,
                  97,
                  252,
                  22,
                  20,
                  59,
                  1,
                  116,
                  88,
                  174
                ]
              },
              {
                "kind": "arg",
                "path": "label"
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
          "name": "label",
          "type": "string"
        },
        {
          "name": "data",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "permissionData",
      "discriminator": [
        150,
        112,
        179,
        118,
        211,
        71,
        19,
        75
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unauthorized",
      "msg": "Unauthorized publickey"
    },
    {
      "code": 6001,
      "name": "badRole",
      "msg": "Bad role"
    },
    {
      "code": 6002,
      "name": "invalidTime",
      "msg": "Invalid time"
    }
  ],
  "types": [
    {
      "name": "permission",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "role",
            "type": {
              "defined": {
                "name": "role"
              }
            }
          },
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "startTime",
            "type": "u64"
          },
          {
            "name": "endTime",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "permissionData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "label",
            "type": "string"
          },
          {
            "name": "data",
            "type": "string"
          },
          {
            "name": "permissions",
            "type": {
              "vec": {
                "defined": {
                  "name": "permission"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "role",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "owner"
          },
          {
            "name": "admin"
          },
          {
            "name": "fullAccess"
          },
          {
            "name": "timeLimited"
          }
        ]
      }
    }
  ]
};
