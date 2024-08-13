use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    offset: usize,

    limit: usize,
}
