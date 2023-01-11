mod abi;
mod pb;
mod rpc;

use crate::abi::{dispatcher, erc20};
use crate::pb::vaults;
use crate::rpc::*;
use hex_literal::hex;
use substreams::prelude::*;
use substreams::Hex;
use substreams_ethereum::pb::eth;
use substreams_ethereum::pb::eth::rpc::RpcCall;
use substreams_ethereum::Event;

substreams_ethereum::init!();

const DISPATCHER_CONTRACT: [u8; 20] = hex!("c3dc853dd716bd5754f421ef94fdcbac3902ab32");

#[substreams::handlers::map]
fn map_vault_creations(blk: eth::v2::Block) -> Result<vaults::Vaults, substreams::errors::Error> {
    let vaults = blk
        .events::<dispatcher::events::VaultProxyDeployed>(&[&DISPATCHER_CONTRACT])
        .map(|(event, log)| {
            let vault = event.vault_proxy;
            let calls = vec![
                RpcCall {
                    data: erc20::functions::Name {}.encode(),
                    to_addr: vault.to_owned(),
                },
                RpcCall {
                    data: erc20::functions::Symbol {}.encode(),
                    to_addr: vault.to_owned(),
                },
            ];

            let results = fetch_many(calls);
            let name = read_string(&results[0]).expect("failed to read name");
            let symbol = read_string(&results[1]).expect("failed to read symbol");

            Ok(vaults::Vault {
                ordinal: log.ordinal(),
                vault,
                name,
                symbol,
            })
        })
        .collect::<Result<Vec<vaults::Vault>, _>>()?;

    Ok(vaults::Vaults { vaults })
}

#[substreams::handlers::store]
fn store_vault_proxies(vaults: vaults::Vaults, output: StoreSetString) {
    for vault in vaults.vaults {
        output.set(vault.ordinal, Hex::encode(vault.vault), &"");
    }
}

#[substreams::handlers::map]
fn map_vault_share_transfers(
    blk: eth::v2::Block,
    store: StoreGetString,
) -> Result<vaults::VaultShareTransfers, substreams::errors::Error> {
    let transfers = blk
        .logs()
        .filter_map(|log| {
            store
                .get_at(log.ordinal(), Hex::encode(log.address()))
                .and_then(|_| {
                    erc20::events::Transfer::match_and_decode(log).and_then(|event| {
                        Some(vaults::VaultShareTransfer {
                            ordinal: log.ordinal(),
                            vault: log.address().to_owned(),
                            from: event.from,
                            to: event.to,
                            shares: event.value.to_string(),
                        })
                    })
                })
        })
        .collect();

    Ok(vaults::VaultShareTransfers { transfers })
}
