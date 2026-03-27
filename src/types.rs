use serde::{Deserialize, Serialize};

/// A single call frame in the raw trace — pure provider output, no annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallFrame {
    pub call_type: CallType,
    pub from: String,
    pub to: Option<String>,
    pub value: Option<String>,
    pub gas: String,
    pub gas_used: String,
    pub input: String,
    pub output: Option<String>,
    pub error: Option<String>,
    pub revert_reason: Option<String>,
    pub calls: Vec<CallFrame>,
    pub logs: Vec<Log>,
}

/// A single event log emitted during a call — pure provider output, no annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CallType {
    Call,
    DelegateCall,
    StaticCall,
    CallCode,
    Create,
    Create2,
}

impl std::fmt::Display for CallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallType::Call => write!(f, "CALL"),
            CallType::DelegateCall => write!(f, "DELEGATECALL"),
            CallType::StaticCall => write!(f, "STATICCALL"),
            CallType::CallCode => write!(f, "CALLCODE"),
            CallType::Create => write!(f, "CREATE"),
            CallType::Create2 => write!(f, "CREATE2"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedArg {
    pub name: String,
    pub ty: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedAbi {
    pub abi: alloy_json_abi::JsonAbi,
    pub contract_name: Option<String>,
    /// True when the ABI has no 4-byte selector prefix (e.g. EVM precompiles).
    pub selector_free: bool,
}
