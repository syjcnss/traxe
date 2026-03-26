mod blockscout;
mod etherscan;
pub mod fourbyte;
mod sourcify;
pub mod well_known;

use std::collections::HashMap;

use alloy_dyn_abi::{DynSolType, DynSolValue};
use alloy_json_abi::{EventParam, JsonAbi, Param};

use crate::types::{DecodedArg, ResolvedAbi};

/// Try to resolve ABIs for all addresses. Returns a map of lowercase address -> ResolvedAbi.
pub async fn resolve_abis(
    http: &reqwest::Client,
    addresses: &[String],
    chain_id: u64,
) -> HashMap<String, ResolvedAbi> {
    let mut result = HashMap::new();

    for addr in addresses {
        let lower = addr.to_lowercase();

        // 1. Sourcify
        if let Ok(abi) = sourcify::fetch_abi(http, &lower, chain_id).await {
            result.insert(
                lower.clone(),
                ResolvedAbi {
                    abi,
                    contract_name: None,
                },
            );
            continue;
        }

        // 2. Etherscan
        if let Ok((abi, name)) = etherscan::fetch_abi(http, &lower, chain_id).await {
            result.insert(
                lower.clone(),
                ResolvedAbi {
                    abi,
                    contract_name: name,
                },
            );
            continue;
        }

        // 3. Blockscout (if URL configured via env)
        if let Ok(bs_url) = std::env::var("BLOCKSCOUT_URL") {
            if let Ok(abi) = blockscout::fetch_abi(http, &bs_url, &lower).await {
                result.insert(
                    lower.clone(),
                    ResolvedAbi {
                        abi,
                        contract_name: None,
                    },
                );
            }
        }
    }

    result
}

/// Decode calldata against the ABI. Returns (function_name, decoded_args) or None.
pub fn decode_input(abi: &JsonAbi, input: &[u8]) -> Option<(String, Vec<DecodedArg>)> {
    if input.len() < 4 {
        return None;
    }
    let selector: [u8; 4] = input[..4].try_into().ok()?;
    let calldata = &input[4..];

    for func in abi.functions() {
        let func_sel: [u8; 4] = func.selector().into();
        if func_sel == selector {
            let args = try_decode_params(&func.inputs, calldata).unwrap_or_else(|| {
                if calldata.is_empty() {
                    vec![]
                } else {
                    vec![DecodedArg {
                        name: String::new(),
                        ty: "bytes".to_string(),
                        value: hex::encode(calldata),
                    }]
                }
            });
            return Some((func.name.clone(), args));
        }
    }
    None
}

/// Decode return data for a named function. Returns decoded args or None.
pub fn decode_output(abi: &JsonAbi, fn_name: &str, output: &[u8]) -> Option<Vec<DecodedArg>> {
    for func in abi.functions().filter(|f| f.name == fn_name) {
        if let Some(args) = try_decode_params(&func.outputs, output) {
            return Some(args);
        }
    }
    None
}

/// Decode an event log. `topics` is the raw bytes of each topic (32 bytes each).
/// Returns (event_name, decoded_args) or None if no matching event is found.
pub fn decode_event(
    abi: &JsonAbi,
    topics: &[Vec<u8>],
    data: &[u8],
) -> Option<(String, Vec<DecodedArg>)> {
    if topics.is_empty() {
        return None;
    }

    for event in abi.events() {
        let selector = event.selector();
        if selector.as_slice() != topics[0].as_slice() {
            continue;
        }

        let mut args: Vec<DecodedArg> = Vec::new();
        let mut topic_idx = 1usize;

        for input in &event.inputs {
            if input.indexed {
                let val = match topics.get(topic_idx) {
                    Some(topic_bytes) => {
                        // For static types, the topic is ABI-encoded (padded to 32 bytes).
                        // For dynamic types (bytes, string, arrays), it's a keccak256 hash.
                        match resolve_event_param_type(input)
                            .and_then(|ty| ty.abi_decode(topic_bytes).ok())
                        {
                            Some(v) => format_dyn_value(&v),
                            None => format!("0x{}", hex::encode(topic_bytes)),
                        }
                    }
                    None => String::new(),
                };
                topic_idx += 1;
                args.push(DecodedArg { name: input.name.clone(), ty: input.ty.clone(), value: val });
            }
        }

        // Non-indexed params are ABI-encoded in data
        let non_indexed: Vec<Param> = event
            .inputs
            .iter()
            .filter(|i| !i.indexed)
            .map(event_param_to_param)
            .collect();

        if !non_indexed.is_empty() {
            if let Some(decoded) = try_decode_params(&non_indexed, data) {
                args.extend(decoded);
            }
        }

        return Some((event.name.clone(), args));
    }
    None
}

fn event_param_to_param(ep: &EventParam) -> Param {
    Param {
        ty: ep.ty.clone(),
        name: ep.name.clone(),
        components: ep.components.clone(),
        internal_type: ep.internal_type.clone(),
    }
}

fn resolve_event_param_type(ep: &EventParam) -> Option<DynSolType> {
    resolve_param_type(&event_param_to_param(ep))
}

fn try_decode_params(params: &[Param], data: &[u8]) -> Option<Vec<DecodedArg>> {
    if params.is_empty() {
        return Some(vec![]);
    }

    let types: Vec<DynSolType> = params
        .iter()
        .filter_map(|p| resolve_param_type(p))
        .collect();

    if types.len() != params.len() {
        return None;
    }

    let tuple_type = DynSolType::Tuple(types);
    let decoded = tuple_type.abi_decode_sequence(data).ok()?;

    match decoded {
        DynSolValue::Tuple(vals) => Some(
            params
                .iter()
                .zip(vals.iter())
                .map(|(p, v)| DecodedArg {
                    name: p.name.clone(),
                    ty: p.ty.clone(),
                    value: format_dyn_value(v),
                })
                .collect(),
        ),
        _ => None,
    }
}

fn resolve_param_type(param: &Param) -> Option<DynSolType> {
    let ty = param.ty.as_str();

    // Handle tuple and tuple arrays
    if ty == "tuple" || ty.starts_with("tuple[") {
        let inner: Vec<DynSolType> = param
            .components
            .iter()
            .filter_map(|c| resolve_param_type(c))
            .collect();
        if inner.len() != param.components.len() {
            return None;
        }
        let tuple = DynSolType::Tuple(inner);
        if ty == "tuple" {
            return Some(tuple);
        }
        // tuple[] or tuple[N]
        let suffix = &ty[5..];
        return if suffix == "[]" {
            Some(DynSolType::Array(Box::new(tuple)))
        } else {
            let n: usize = suffix[1..suffix.len() - 1].parse().ok()?;
            Some(DynSolType::FixedArray(Box::new(tuple), n))
        };
    }

    // All other types can be parsed directly
    ty.parse().ok()
}

fn format_dyn_value(v: &DynSolValue) -> String {
    match v {
        DynSolValue::Address(a) => format!("{a}"),
        DynSolValue::Bytes(b) => format!("0x{}", hex::encode(b)),
        DynSolValue::FixedBytes(b, _) => format!("0x{}", hex::encode(b.as_slice())),
        DynSolValue::Uint(n, _) => format!("{n}"),
        DynSolValue::Int(n, _) => format!("{n}"),
        DynSolValue::Bool(b) => format!("{b}"),
        DynSolValue::String(s) => s.clone(),
        DynSolValue::Array(arr) | DynSolValue::FixedArray(arr) => {
            let items: Vec<String> = arr.iter().map(format_dyn_value).collect();
            format!("[{}]", items.join(", "))
        }
        DynSolValue::Tuple(vals) => {
            let items: Vec<String> = vals.iter().map(format_dyn_value).collect();
            format!("({})", items.join(", "))
        }
        DynSolValue::Function(f) => format!("{f}"),
    }
}
