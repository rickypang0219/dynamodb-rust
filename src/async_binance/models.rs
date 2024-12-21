use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenKey {
    #[allow(non_snake_case)]
    pub listenKey: String,
}
