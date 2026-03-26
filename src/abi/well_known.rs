use alloy_json_abi::JsonAbi;

/// ABI for an Ethereum precompile contract, keyed by address.
/// Returns None for unknown addresses.
pub fn precompile_abi(addr: &str) -> Option<JsonAbi> {
    let json: &str = match addr {
        // 0x01 – ecrecover: 128-byte input (4 × 32-byte words), 32-byte output (address)
        "0x0000000000000000000000000000000000000001" => r#"[{
            "type": "function", "name": "ecrecover",
            "inputs": [
                {"name": "hash", "type": "bytes32"},
                {"name": "v",    "type": "uint256"},
                {"name": "r",    "type": "bytes32"},
                {"name": "s",    "type": "bytes32"}
            ],
            "outputs": [{"name": "signer", "type": "address"}],
            "stateMutability": "pure"
        }]"#,
        // 0x02 – sha256: arbitrary raw bytes in, bytes32 out
        "0x0000000000000000000000000000000000000002" => r#"[{
            "type": "function", "name": "sha256",
            "inputs":  [{"name": "data", "type": "bytes"}],
            "outputs": [{"name": "hash", "type": "bytes32"}],
            "stateMutability": "pure"
        }]"#,
        // 0x03 – ripemd160: arbitrary raw bytes in, bytes32 out (20-byte hash, zero-padded left)
        "0x0000000000000000000000000000000000000003" => r#"[{
            "type": "function", "name": "ripemd160",
            "inputs":  [{"name": "data", "type": "bytes"}],
            "outputs": [{"name": "hash", "type": "bytes32"}],
            "stateMutability": "pure"
        }]"#,
        // 0x04 – identity: pass-through, arbitrary raw bytes
        "0x0000000000000000000000000000000000000004" => r#"[{
            "type": "function", "name": "identity",
            "inputs":  [{"name": "data", "type": "bytes"}],
            "outputs": [{"name": "data", "type": "bytes"}],
            "stateMutability": "pure"
        }]"#,
        // 0x05 – modexp (EIP-198): complex format, exposed as raw bytes
        "0x0000000000000000000000000000000000000005" => r#"[{
            "type": "function", "name": "modexp",
            "inputs":  [{"name": "input",  "type": "bytes"}],
            "outputs": [{"name": "result", "type": "bytes"}],
            "stateMutability": "pure"
        }]"#,
        // 0x06 – ecadd (BN-254): 128-byte input (4 × uint256), 64-byte output (2 × uint256)
        "0x0000000000000000000000000000000000000006" => r#"[{
            "type": "function", "name": "ecadd",
            "inputs": [
                {"name": "x1", "type": "uint256"},
                {"name": "y1", "type": "uint256"},
                {"name": "x2", "type": "uint256"},
                {"name": "y2", "type": "uint256"}
            ],
            "outputs": [
                {"name": "x", "type": "uint256"},
                {"name": "y", "type": "uint256"}
            ],
            "stateMutability": "pure"
        }]"#,
        // 0x07 – ecmul (BN-254): 96-byte input (3 × uint256), 64-byte output (2 × uint256)
        "0x0000000000000000000000000000000000000007" => r#"[{
            "type": "function", "name": "ecmul",
            "inputs": [
                {"name": "x",  "type": "uint256"},
                {"name": "y",  "type": "uint256"},
                {"name": "s",  "type": "uint256"}
            ],
            "outputs": [
                {"name": "rx", "type": "uint256"},
                {"name": "ry", "type": "uint256"}
            ],
            "stateMutability": "pure"
        }]"#,
        // 0x08 – ecpairing (BN-254): N × 192-byte pairs (raw bytes), uint256 success out
        "0x0000000000000000000000000000000000000008" => r#"[{
            "type": "function", "name": "ecpairing",
            "inputs":  [{"name": "points",  "type": "bytes"}],
            "outputs": [{"name": "success", "type": "uint256"}],
            "stateMutability": "pure"
        }]"#,
        // 0x09 – blake2f (EIP-152): 213-byte raw input, 64-byte raw output
        "0x0000000000000000000000000000000000000009" => r#"[{
            "type": "function", "name": "blake2f",
            "inputs":  [{"name": "input",  "type": "bytes"}],
            "outputs": [{"name": "output", "type": "bytes"}],
            "stateMutability": "pure"
        }]"#,
        // 0x0a – point evaluation (EIP-4844): 192-byte raw input,
        //        64-byte output (fieldElementsPerBlob ++ blsModulus as uint256 pair)
        "0x000000000000000000000000000000000000000a" => r#"[{
            "type": "function", "name": "pointEvaluation",
            "inputs":  [{"name": "input", "type": "bytes"}],
            "outputs": [
                {"name": "fieldElementsPerBlob", "type": "uint256"},
                {"name": "blsModulus",           "type": "uint256"}
            ],
            "stateMutability": "pure"
        }]"#,
        _ => return None,
    };
    serde_json::from_str(json).ok()
}

/// Full ERC-20 ABI: standard functions + Transfer/Approval events.
pub fn erc20_abi() -> JsonAbi {
    serde_json::from_str(r#"[
        {
            "type": "function", "name": "transfer",
            "inputs":  [{ "name": "to",     "type": "address" }, { "name": "value",   "type": "uint256" }],
            "outputs": [{ "name": "",        "type": "bool"    }],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function", "name": "transferFrom",
            "inputs":  [{ "name": "from",    "type": "address" }, { "name": "to",      "type": "address" }, { "name": "value",   "type": "uint256" }],
            "outputs": [{ "name": "",        "type": "bool"    }],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function", "name": "approve",
            "inputs":  [{ "name": "spender", "type": "address" }, { "name": "value",   "type": "uint256" }],
            "outputs": [{ "name": "",        "type": "bool"    }],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function", "name": "allowance",
            "inputs":  [{ "name": "owner",   "type": "address" }, { "name": "spender", "type": "address" }],
            "outputs": [{ "name": "",        "type": "uint256" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "balanceOf",
            "inputs":  [{ "name": "account", "type": "address" }],
            "outputs": [{ "name": "",        "type": "uint256" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "totalSupply",
            "inputs":  [],
            "outputs": [{ "name": "", "type": "uint256" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "name",
            "inputs":  [],
            "outputs": [{ "name": "", "type": "string" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "symbol",
            "inputs":  [],
            "outputs": [{ "name": "", "type": "string" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "decimals",
            "inputs":  [],
            "outputs": [{ "name": "", "type": "uint8" }],
            "stateMutability": "view"
        },
        {
            "type": "event", "name": "Transfer", "anonymous": false,
            "inputs": [
                { "name": "from",  "type": "address", "indexed": true  },
                { "name": "to",    "type": "address", "indexed": true  },
                { "name": "value", "type": "uint256", "indexed": false }
            ]
        },
        {
            "type": "event", "name": "Approval", "anonymous": false,
            "inputs": [
                { "name": "owner",   "type": "address", "indexed": true  },
                { "name": "spender", "type": "address", "indexed": true  },
                { "name": "value",   "type": "uint256", "indexed": false }
            ]
        }
    ]"#)
    .expect("valid ERC-20 ABI")
}

/// Full ERC-721 ABI: standard functions + Transfer/Approval/ApprovalForAll events.
pub fn erc721_abi() -> JsonAbi {
    serde_json::from_str(r#"[
        {
            "type": "function", "name": "transferFrom",
            "inputs":  [{ "name": "from", "type": "address" }, { "name": "to", "type": "address" }, { "name": "tokenId", "type": "uint256" }],
            "outputs": [],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function", "name": "safeTransferFrom",
            "inputs":  [{ "name": "from", "type": "address" }, { "name": "to", "type": "address" }, { "name": "tokenId", "type": "uint256" }],
            "outputs": [],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function", "name": "safeTransferFrom",
            "inputs":  [{ "name": "from", "type": "address" }, { "name": "to", "type": "address" }, { "name": "tokenId", "type": "uint256" }, { "name": "data", "type": "bytes" }],
            "outputs": [],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function", "name": "approve",
            "inputs":  [{ "name": "to",       "type": "address" }, { "name": "tokenId",  "type": "uint256" }],
            "outputs": [],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function", "name": "setApprovalForAll",
            "inputs":  [{ "name": "operator", "type": "address" }, { "name": "approved", "type": "bool" }],
            "outputs": [],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function", "name": "getApproved",
            "inputs":  [{ "name": "tokenId", "type": "uint256" }],
            "outputs": [{ "name": "", "type": "address" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "isApprovedForAll",
            "inputs":  [{ "name": "owner", "type": "address" }, { "name": "operator", "type": "address" }],
            "outputs": [{ "name": "", "type": "bool" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "ownerOf",
            "inputs":  [{ "name": "tokenId", "type": "uint256" }],
            "outputs": [{ "name": "", "type": "address" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "balanceOf",
            "inputs":  [{ "name": "owner", "type": "address" }],
            "outputs": [{ "name": "", "type": "uint256" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "name",
            "inputs":  [],
            "outputs": [{ "name": "", "type": "string" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "symbol",
            "inputs":  [],
            "outputs": [{ "name": "", "type": "string" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "tokenURI",
            "inputs":  [{ "name": "tokenId", "type": "uint256" }],
            "outputs": [{ "name": "", "type": "string" }],
            "stateMutability": "view"
        },
        {
            "type": "function", "name": "supportsInterface",
            "inputs":  [{ "name": "interfaceId", "type": "bytes4" }],
            "outputs": [{ "name": "", "type": "bool" }],
            "stateMutability": "view"
        },
        {
            "type": "event", "name": "Transfer", "anonymous": false,
            "inputs": [
                { "name": "from",    "type": "address", "indexed": true },
                { "name": "to",      "type": "address", "indexed": true },
                { "name": "tokenId", "type": "uint256", "indexed": true }
            ]
        },
        {
            "type": "event", "name": "Approval", "anonymous": false,
            "inputs": [
                { "name": "owner",    "type": "address", "indexed": true },
                { "name": "approved", "type": "address", "indexed": true },
                { "name": "tokenId",  "type": "uint256", "indexed": true }
            ]
        },
        {
            "type": "event", "name": "ApprovalForAll", "anonymous": false,
            "inputs": [
                { "name": "owner",    "type": "address", "indexed": true  },
                { "name": "operator", "type": "address", "indexed": true  },
                { "name": "approved", "type": "bool",    "indexed": false }
            ]
        }
    ]"#)
    .expect("valid ERC-721 ABI")
}
