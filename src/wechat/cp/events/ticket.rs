
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpTicketEvent {
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    #[serde(rename="SuiteTicket")]
    pub suite_ticket: String,
}