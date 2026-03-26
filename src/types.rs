use serde::{Deserialize, Serialize};

/// A single call frame in the trace tree.
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

    // Resolved metadata (filled in by resolvers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_input: Option<Vec<DecodedArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_output: Option<Vec<DecodedArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_label: Option<String>,
}

/// A single event log emitted during a call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,

    // Resolved metadata (filled in by resolvers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_args: Option<Vec<DecodedArg>>,
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
}
