use substreams::Hex;
use substreams_ethereum::pb::eth::rpc::{RpcCall, RpcCalls};
use substreams_ethereum::rpc::eth_call;

pub fn read_uint32(input: &[u8]) -> Result<u32, String> {
    if input.len() != 32 {
        return Err(format!("uint32 invalid length: {}", input.len()));
    }
    let as_array: [u8; 4] = input[28..32].try_into().unwrap();
    Ok(u32::from_be_bytes(as_array))
}

pub fn read_string(input: &[u8]) -> Result<String, String> {
    if input.len() < 96 {
        return Err(format!("string invalid length: {}", input.len()));
    }

    let next = read_uint32(&input[0..32])?;
    if next != 32 {
        return Err(format!("invalid string uint32 value: {}", next));
    };

    let size = read_uint32(&input[32..64])?;
    let end: usize = (size as usize) + 64;

    if end > input.len() {
        return Err(format!(
            "invalid input: end {:?}, length: {:?}, next: {:?}, size: {:?}, whole: {:?}",
            end,
            input.len(),
            next,
            size,
            Hex::encode(&input[32..64])
        ));
    }

    Ok(String::from_utf8_lossy(&input[64..end]).to_string())
}

pub fn fetch_many(calls: Vec<RpcCall>) -> Vec<Vec<u8>> {
    return eth_call(&RpcCalls { calls })
        .responses
        .iter()
        .map(|response| {
            if response.failed {
                panic!("eth call failed")
            }

            response.raw.to_owned()
        })
        .collect();
}
