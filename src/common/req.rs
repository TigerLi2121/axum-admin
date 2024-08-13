use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    pub offset: usize,

    pub limit: usize,
}
