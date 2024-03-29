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

const DISPATCHER_CONTRACT: [u8; 20] = hex!("c3dc853dd716bd5754f421ef94fdcbac3902ab32");

#[substreams::handlers::map]
fn map_vault_creations(blk: eth::v2::Block) -> Result<vaults::Vaults, substreams::errors::Error> {
    let vaults = blk
        .events_at::<dispatcher::events::VaultProxyDeployed>(&[&DISPATCHER_CONTRACT])
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
            let name = erc20::functions::Name::output(&results[0]).expect("failed to read name");
            let symbol = erc20::functions::Symbol::output(&results[1]).expect("failed to read symbol");

            Ok(vaults::Vault {
                ordinal: log.ordinal(),
                block: blk.number,
                timestamp: blk.timestamp_seconds(),
                transaction: Hex::encode(log.receipt.transaction.to_owned().hash),
                vault: Hex::encode(vault),
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
        output.set(vault.ordinal, vault.vault, &"");
    }
}

#[substreams::handlers::map]
fn map_vault_share_transfers(
    blk: eth::v2::Block,
    store: StoreGetString,
) -> Result<vaults::VaultShareTransfers, substreams::errors::Error> {
    let transfers = blk
        .events::<erc20::events::Transfer>()
        .filter_map(|(event, log)| {
            // Look for any transfers of a vault's shares.
            let filter = store.get_at(log.ordinal(), Hex::encode(log.address()));

            filter.map(|_| vaults::VaultShareTransfer {
                block: blk.number,
                timestamp: blk.timestamp_seconds(),
                transaction: Hex::encode(log.receipt.transaction.to_owned().hash),
                vault: Hex::encode(log.address()),
                from: Hex::encode(event.from),
                to: Hex::encode(event.to),
                amount: event.value.to_string(),
            })
        })
        .collect();

    Ok(vaults::VaultShareTransfers { transfers })
}

#[substreams::handlers::map]
fn map_vault_asset_transfers(
    blk: eth::v2::Block,
    store: StoreGetString,
) -> Result<vaults::VaultAssetTransfers, substreams::errors::Error> {
    let transfers = blk
        .events::<erc20::events::Transfer>()
        .filter_map(|(event, log)| {
            // Look for any transfers of any asset to or from a vault's portfolio.
            let filter = store
                .get_at(log.ordinal(), Hex::encode(event.to.to_owned()))
                .or_else(|| store.get_at(log.ordinal(), Hex::encode(event.from.to_owned())));

            filter.map(|_| vaults::VaultAssetTransfer {
                block: blk.number,
                timestamp: blk.timestamp_seconds(),
                transaction: Hex::encode(log.receipt.transaction.to_owned().hash),
                asset: Hex::encode(log.address()),
                from: Hex::encode(event.from.to_owned()),
                to: Hex::encode(event.to.to_owned()),
                amount: event.value.to_string(),
            })
        })
        .collect();

    Ok(vaults::VaultAssetTransfers { transfers })
}
