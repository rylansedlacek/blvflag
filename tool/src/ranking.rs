use crate::diff;
use crate::buckets;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankedCycle {
    pub ranking: u64,

    //TODO more fields
    
}