use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// contains query kinds that we expect to handle in `sudo_kv_query_result`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum QueryKind {
    // Balance query
    Balance,
    // You can add your handlers to understand what query to deserialize by query_id in sudo callback
}

pub const BALANCES_REPLY_ID: u64 = 1;

pub const KV_QUERY_ID_TO_CALLBACKS: Map<u64, QueryKind> = Map::new("kv_query_id_to_callbacks");
