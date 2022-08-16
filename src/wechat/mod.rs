use serde::{Serialize, Deserialize};

mod mp;
mod pay;
mod cryptos;

pub use mp::*;
pub use pay::*;
pub use cryptos::*;

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCommonResponse<T> {
    pub errcode: Option<i64>,
    pub errmsg: Option<String>,
    pub result: Option<T>,
}

impl <T> WechatCommonResponse<T> {
    pub fn is_success(&self) -> bool {
        self.errcode.unwrap_or(0) == 0
    }
}