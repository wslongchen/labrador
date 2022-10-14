use serde::{Serialize, Deserialize};

/// 异步任务完成事件推送
/// 本事件是成员在使用异步任务接口时，用于接收任务执行完毕的结果通知。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpBatchJobResultEvent {
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
    #[serde(rename="AgentID")]
    pub agent_id: i64,
    #[serde(rename="BatchJob")]
    pub batch_job: Option<BatchJob>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchJob {
    #[serde(rename="JobId")]
    pub job_id: String,
    #[serde(rename="JobType")]
    pub job_type: String,
    #[serde(rename="ErrMsg")]
    pub err_msg: String,
    #[serde(rename="ErrCode")]
    pub err_code: i32,
}