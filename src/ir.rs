use serde::{Deserialize, Serialize};

use crate::types::{CallFrame, CallType, DecodedArg, Log};

/// The IR node tree. Every observable event during a transaction is one of these.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Node {
    Call(CallNode),
    Event(EventNode),
    // Future: Sstore(SstoreNode), Sload(SloadNode)
}


/// A call/create frame in the execution trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallNode {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_input: Option<Vec<DecodedArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_output: Option<Vec<DecodedArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_label: Option<String>,

    /// Ordered child nodes: sub-calls followed by emitted events.
    pub children: Vec<Node>,
}

/// An event log emitted during a call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNode {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_args: Option<Vec<DecodedArg>>,
}

// --- Conversions from the raw CallFrame / Log types ---

impl From<CallFrame> for Node {
    fn from(frame: CallFrame) -> Self {
        let mut children: Vec<Node> = frame.calls.into_iter().map(Node::from).collect();
        children.extend(frame.logs.into_iter().map(Node::from));

        Node::Call(CallNode {
            call_type: frame.call_type,
            from: frame.from,
            to: frame.to,
            value: frame.value,
            gas: frame.gas,
            gas_used: frame.gas_used,
            input: frame.input,
            output: frame.output,
            error: frame.error,
            revert_reason: frame.revert_reason,
            function_name: frame.function_name,
            decoded_input: frame.decoded_input,
            decoded_output: frame.decoded_output,
            contract_label: frame.contract_label,
            children,
        })
    }
}

impl From<Log> for Node {
    fn from(log: Log) -> Self {
        Node::Event(EventNode {
            address: log.address,
            topics: log.topics,
            data: log.data,
            event_name: log.event_name,
            decoded_args: log.decoded_args,
        })
    }
}
