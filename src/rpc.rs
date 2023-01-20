use substreams_ethereum::pb::eth::rpc::{RpcCall, RpcCalls};
use substreams_ethereum::rpc::eth_call;

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
