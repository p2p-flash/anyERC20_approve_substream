mod pb;
#[path = "kv_out.rs"]
mod kv;
use substreams::errors::Error;
use hex_literal::hex;
use pb::eth::tx::v1 as tx;
use substreams::prelude::*;
use substreams::{log, store::StoreAddInt64, Hex};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth::v2::{self as eth, Block};
use substreams_entity_change::tables::Tables;
substreams_ethereum::init!();
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use hex::encode as hex_encode;
use substreams::proto;
use crate::pb::eth::tx::v1::EthTransactions;



const TARGET_METHOD_ID: &[u8] = &hex!("095ea7b3");  // Method ID for the function you're interested in

#[substreams::handlers::map]
fn map_transactions(blk: eth::Block) -> Result<tx::EthTransactions, substreams::errors::Error> {
    Ok(tx::EthTransactions {
        transactions: blk
            .transactions()
            .filter(|transaction|  transaction.input.starts_with(TARGET_METHOD_ID) && hex[31:63]=="b49d5f96157039474a394dcf433a4ee9ca3bd601")
            .map(|transaction| {
                substreams::log::info!("Transaction seen");

                tx::EthTransaction {
                    trx_hash: transaction.hash.clone(),
                    nonce: transaction.nonce.to_string(),
                    gas_limit: transaction.gas_limit.to_string(),
                    data: transaction.input.clone(),
                    v: transaction.v.clone(),
                    to: transaction.to.to_ascii_lowercase(),
                    r: transaction.r.clone(),
                    s: transaction.s.clone(),
                }
            })
            .collect(),
    })
}

