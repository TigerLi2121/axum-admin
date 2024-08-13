use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Pagination {
    page: usize,

    limit: usize,
}
