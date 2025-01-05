use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pair<A, B> {
    pub(crate) first: A,
    pub(crate) second: B,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Triple<A, B, C> {
    first: A,
    second: B,
    third: C,
}
