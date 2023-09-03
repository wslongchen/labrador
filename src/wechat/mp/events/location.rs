use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationEvent {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: Option<i64>,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="Latitude")]
    pub latitude: f64,
    #[serde(rename="Longitude")]
    pub longitude: f64,
    #[serde(rename="Precision")]
    pub precision: f64,
}
