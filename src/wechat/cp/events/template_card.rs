use serde::{Serialize, Deserialize};

/// 模板卡片事件推送
/// 应用下发的模板卡片消息，用户点击按钮之后触发此事件
/// 应用收到该事件之后，可以响应回复模板卡片更新消息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpTemplateCardEvent {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
    #[serde(rename="TaskId")]
    pub task_id: Option<String>,
    #[serde(rename="CardType")]
    pub card_type: Option<String>,
    /// 用于调用更新卡片接口的ResponseCode，24小时内有效，且只能使用一次
    #[serde(rename="ResponseCode")]
    pub response_code: Option<String>,
    #[serde(rename="SelectedItems")]
    pub selected_items: Option<SelectedItems>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct SelectedItems {
    #[serde(rename = "SelectedItem")]
    items: Vec<SelectedItem>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct SelectedItem {
    /// 问题的key值
    #[serde(rename = "QuestionKey")]
    question_key: Option<String>,
    /// 对应问题的选项列表
    #[serde(rename = "OptionIds")]
    option_ids: Option<OptionIds>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct OptionIds {
    #[serde(rename = "OptionId")]
    items: Option<String>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct OptionId {
    #[serde(rename = "QuestionKey")]
    question_key: String,
    #[serde(rename = "OptionIds")]
    option_ids: OptionIds,
}



/// 通用模板卡片右上角菜单事件推送
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpTemplateCardMenuEvent {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
    #[serde(rename="TaskId")]
    pub task_id: Option<String>,
    #[serde(rename="CardType")]
    pub card_type: Option<String>,
    /// 用于调用更新卡片接口的ResponseCode，24小时内有效，且只能使用一次
    #[serde(rename="ResponseCode")]
    pub response_code: Option<String>,
}