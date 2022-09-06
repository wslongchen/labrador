use serde::{Serialize, Deserialize};
use serde_json::{Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, LabradorResult, WechatCpClient, WechatCpNewArticle, WechatMpNewsArticle};
use crate::wechat::cp::method::{CpMessageMethod, WechatCpMethod};

/// 菜单管理相关接口
#[derive(Debug, Clone)]
pub struct WechatCpMessage<'a, T: SessionStore> {
    client: &'a WechatCpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WechatCpMessage<'a, T> {

    #[inline]
    pub fn new(client: &WechatCpClient<T>) -> WechatCpMessage<T> {
        WechatCpMessage {
            client,
        }
    }

    /// <pre>
    /// 发送消息
    /// 详情请见: <a href="https://work.weixin.qq.com/api/doc/90000/90135/90236">文档</a>
    /// </pre>
    pub async fn send(&self, mut req: WechatCpMessageRequest) -> LabradorResult<WechatCpMessageResponse> {
        let agent_id = req.agent_id.unwrap_or_default();
        if agent_id == 0 {
            req.agent_id = self.client.agent_id;
        }
       let v= self.client.post(WechatCpMethod::Message(CpMessageMethod::Send), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpMessageResponse>(v)
    }

    /// <pre>
    /// 互联企业的应用支持推送文本、图片、视频、文件、图文等类型。
    /// 详情请见: <a href="https://qyapi.weixin.qq.com/cgi-bin/linkedcorp/message/send?access_token=ACCESS_TOKEN">文档</a>
    /// </pre>
    pub async fn send_linked_corp_message(&self, mut req: WechatCpLinkedCorpMessage) -> LabradorResult<WechatCpLinkedCorpMessageResponse> {
        let agent_id = req.agent_id.unwrap_or_default();
        if agent_id == 0 {
            req.agent_id = self.client.agent_id;
        }
        let v = self.client.post(WechatCpMethod::Message(CpMessageMethod::LinkedCorpSend), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpLinkedCorpMessageResponse>(v)
    }

    /// <pre>
    /// 发送「学校通知」
    /// <p>
    /// 学校可以通过此接口来给家长发送不同类型的学校通知，来满足多种场景下的学校通知需求。目前支持的消息类型为文本、图片、语音、视频、文件、图文。
    /// <p>
    /// 详情请见: <a href="https://developer.work.weixin.qq.com/document/path/92321">文档</a>
    /// </pre>
    pub async fn send_school_contact_message(&self, mut req: WechatCpSchoolContactMessage) -> LabradorResult<WechatCpSchoolContactMessageResponse> {
        let agent_id = req.agentid.unwrap_or_default();
        if agent_id == 0 {
            req.agentid = self.client.agent_id;
        }
       let v = self.client.post(WechatCpMethod::Message(CpMessageMethod::LinkedCorpSend), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpSchoolContactMessageResponse>(v)
    }

    /// <pre>
    /// 查询应用消息发送统计
    /// 请求方式：POST（HTTPS）
    /// 请求地址： <a href="https://qyapi.weixin.qq.com/cgi-bin/message/get_statistics?access_token=ACCESS_TOKEN">文档</a>
    ///
    /// 详情请见:  <a href="https://work.weixin.qq.com/api/doc/90000/90135/92369">文档</a>
    /// </pre>
    pub async fn get_statistics(&self, req: WechatCpLinkedCorpMessage) -> LabradorResult<WechatCpMessageSendStatistics> {
       let v = self.client.post(WechatCpMethod::Message(CpMessageMethod::Statistics), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatCpMessageSendStatistics>(v)
    }

}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMessageRequest {
    pub to_user: String,
    pub to_party: Option<String>,
    pub to_tag: Option<String>,
    pub agent_id: Option<i32>,
    pub msg_type: String,
    pub content: String,
    pub media_id: Option<String>,
    pub thumb_media_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub music_url: Option<String>,
    pub hq_music_url: Option<String>,
    pub safe: Option<String>,
    pub url: Option<String>,
    pub btn_txt: Option<String>,
    pub app_id: Option<String>,
    pub page: Option<String>,
    /// 任务卡片特有的属性
    pub task_id: Option<String>,
    /// 模板卡片类型，文本通知型卡片填写 “text_notice”,
    /// 图文展示型卡片此处填写 “news_notice”,
    /// 按钮交互型卡片填写”button_interaction”,
    /// 投票选择型卡片填写”vote_interaction”,
    /// 多项选择型卡片填写 “multiple_interaction”
    pub card_type: Option<String>,
    /// 卡片来源样式信息，不需要来源样式可不填写
    /// 来源图片的url
    pub source_icon_url: Option<String>,
    /// 卡片来源样式信息，不需要来源样式可不填写
    /// 来源图片的描述，建议不超过20个字
    pub source_desc: Option<String>,
    /// 更多操作界面的描述
    pub action_menu_desc: Option<String>,
    /// 任务卡片特有的属性
    pub task_buttons: Option<Vec<TaskCardButton>>,
    pub emphasis_first_item: Option<u8>,
    /// 来源文字的颜色，目前支持：0(默认) 灰色，1 黑色，2 红色，3 绿色
    pub source_desc_color: Option<u8>,
    /// 表示是否开启id转译，0表示否，1表示是，默认0
    pub enable_id_trans: Option<u8>,
    /// 表示是否开启重复消息检查，0表示否，1表示是，默认0
    pub enable_duplicate_check: Option<u8>,
    /// 表示是否重复消息检查的时间间隔，默认1800s，最大不超过4小时
    pub duplicate_check_interval: Option<u8>,
    pub content_items: Option<Value>,
    pub articles: Option<Vec<WechatCpNewArticle>>,
    pub mpnews_articles: Option<Vec<WechatMpNewsArticle>>,
    pub action_menu_action_list: Option<Vec<ActionMenuItem>>,
    /// 一级标题，建议不超过36个字
    pub main_title: Option<String>,
    /// 标题辅助信息，建议不超过44个字
    pub main_title_desc: Option<String>,
    /// 图文展示型的卡片必须有图片字段。
    /// 图片的url.
    pub card_image_url: Option<String>,
    /// 关键数据样式
    /// 关键数据样式的数据内容，建议不超过14个字.
    pub emphasis_content_title: Option<String>,
    /// 关键数据样式的数据描述内容，建议不超过22个字
    pub emphasis_content_desc: Option<String>,
    /// 二级普通文本，建议不超过160个字
    pub sub_title_text: Option<String>,
    /// 卡片二级垂直内容，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过4
    pub vertical_contents: Option<Vec<VerticalContent>>,
    /// 二级标题+文本列表，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过6
    pub horizontal_contents: Option<Vec<HorizontalContent>>,
    /// 跳转指引样式的列表，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过3
    pub jumps: Option<Vec<TemplateCardJump>>,
    /// 整体卡片的点击跳转事件，text_notice必填本字段
    /// 跳转事件类型，1 代表跳转url，2 代表打开小程序。text_notice卡片模版中该字段取值范围为[1,2]
    pub card_action_type: Option<u8>,
    /// 跳转事件的url，card_action.type是1时必填
    pub card_action_url: Option<String>,
    /// 跳转事件的小程序的appid，必须是与当前应用关联的小程序，card_action.type是2时必填
    pub card_action_appid: Option<String>,
    /// 跳转事件的小程序的pagepath，card_action.type是2时选填
    pub card_action_pagepath: Option<String>,
    /// 按钮交互型卡片需指定
    /// 按钮列表，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过6
    pub buttons: Option<Vec<TemplateCardButton>>,
    /// 投票选择型卡片需要指定
    /// 选择题key值，用户提交选项后，会产生回调事件，回调事件会带上该key值表示该题，最长支持1024字节
    pub checkbox_question_key: Option<String>,
    /// 选择题模式，单选：0，多选：1，不填默认0
    pub checkbox_mode: Option<u8>,
    /// 选项list，选项个数不超过 20 个，最少1个
    pub options: Option<Vec<CheckboxOption>>,
    /// 按钮文案，建议不超过10个字，不填默认为提交
    pub submit_button_text: Option<String>,
    /// 提交按钮的key，会产生回调事件将本参数作为EventKey返回，最长支持1024字节
    pub submit_button_key: Option<String>,
    /// 下拉式的选择器列表，multiple_interaction类型的卡片该字段不可为空，一个消息最多支持 3 个选择器
    pub selects: Option<Vec<MultipleSelect>>,
    /// 引用文献样式
    pub quote_area: Option<QuoteArea>,
    /// 图片的url.
    pub card_image_aspect_ratio: Option<f64>,
}

/// 引用文献样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteArea {
    /// 非必填 引用文献样式区域点击事件，0或不填代表没有点击事件，1 代表跳转url，2 代表跳转小程序
    #[serde(rename="type")]
    pub r#type: Option<u8>,
    /// 点击跳转的url，quote_area.type是1时必填
    pub url: Option<String>,
    /// 点击跳转的小程序的appid，必须是与当前应用关联的小程序，quote_area.type是2时必填
    pub appid: Option<String>,
    /// 点击跳转的小程序的pagepath，quote_area.type是2时选填
    pub pagepath: Option<String>,
    /// 引用文献样式的标题
    pub title: Option<String>,
    /// 引用文献样式的引用文案
    pub quote_text: Option<String>,
}

/// 任务卡片按钮
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCardButton {
    pub key: Option<String>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub bold: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMenuItem {
    /// 操作的描述文案
    pub text: Option<String>,
    /// 按钮key值，用户点击后，会产生回调事件将本参数作为EventKey返回，回调事件会带上该key值，最长支持1024字节，不可重复
    pub key: Option<String>,
}

/// 卡片二级垂直内容，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerticalContent {
    /// 卡片二级标题，建议不超过38个字.必填字段
    pub title: Option<String>,
    /// 二级普通文本，建议不超过160个字
    pub desc: Option<String>,
}
/// 二级标题+文本列表，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过6
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalContent {
    /// 链接类型，0或不填代表不是链接，1 代表跳转url，2 代表下载附件
    #[serde(rename="type")]
    pub r#type: Option<u8>,
    /// 二级标题，建议不超过5个字
    pub keyname: Option<String>,
    /// 二级文本，如果horizontal_content_list.type是2，该字段代表文件名称（要包含文件类型），建议不超过30个字
    pub value: Option<String>,
    /// 链接跳转的url，horizontal_content_list.type是1时必填
    pub url: Option<String>,
    /// 附件的media_id，horizontal_content_list.type是2时必填
    pub media_id: Option<String>,
    /// 成员详情的userid，horizontal_content_list.type是3时必填
    pub userid: Option<String>,
}

/// 跳转指引样式的列表，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateCardJump {
    /// 跳转链接类型，0或不填代表不是链接，1 代表跳转url，2 代表跳转小程序
    #[serde(rename="type")]
    pub r#type: Option<u8>,
    /// 跳转链接样式的文案内容，建议不超过18个字
    pub title: Option<String>,
    /// 跳转链接的url，jump_list.type是1时必填
    pub url: Option<String>,
    /// 跳转链接的小程序的appid，必须是与当前应用关联的小程序，jump_list.type是2时必填
    pub appid: Option<String>,
    /// 跳转链接的小程序的pagepath，jump_list.type是2时选填
    pub pagepath: Option<String>,
}


/// 按钮列表，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过6
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateCardButton {
    /// 按钮文案，建议不超过10个字
    pub text: Option<String>,
    /// 按钮样式，目前可填1~4，不填或错填默认1
    pub style: Option<u8>,
    /// 按钮key值，用户点击后，会产生回调事件将本参数作为EventKey返回，回调事件会带上该key值，最长支持1024字节，不可重复
    pub key: Option<String>,
}


/// 按钮列表，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过6
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxOption {
    /// 选项id，用户提交选项后，会产生回调事件，回调事件会带上该id值表示该选项，最长支持128字节，不可重复
    pub id: String,
    /// 选项文案描述，建议不超过17个字.
    pub text: String,
    /// 该选项是否要默认选中
    pub is_checked: bool,
}


/// 下拉式的选择器列表，multiple_interaction类型的卡片该字段不可为空，一个消息最多支持 3 个选择器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleSelect {
    /// 下拉式的选择器题目的key，用户提交选项后，会产生回调事件，回调事件会带上该key值表示该题，最长支持1024字节，不可重复
    pub question_key: Option<String>,
    /// 下拉式的选择器上面的title
    pub title: Option<String>,
    /// 默认选定的id，不填或错填默认第一个
    pub selected_id: Option<String>,
    /// 选项列表，下拉选项不超过 10 个，最少1个
    pub options: Option<Vec<CheckboxOption>>,
}

/// 应用消息发送统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMessageSendStatistics {
    pub statistics: Option<Vec<StatisticItem>>,
}

/// 应用消息发送统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticItem {
    /// 应用名
    pub app_name: Option<String>,
    /// 应用id
    pub agentid: Option<i32>,
    /// 发消息成功人次
    pub count: Option<i32>,
}


/// 互联企业消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpLinkedCorpMessage {
    /// 1表示发送给应用可见范围内的所有人（包括互联企业的成员），默认为0
    pub is_to_all: Option<u8>,
    /// 成员ID列表（消息接收者，最多支持1000个）。每个元素的格式为： corpid/userid，其中，corpid为该互联成员所属的企业，userid为该互联成员所属企业中的帐号。如果是本企业的成员，则直接传userid即可
    pub to_users: Option<Vec<String>>,
    /// 部门ID列表，最多支持100个。partyid在互联圈子内唯一。每个元素都是字符串类型，格式为：linked_id/party_id，其中linked_id是互联id，party_id是在互联圈子中的部门id。如果是本企业的部门，则直接传party_id即可。
    pub to_parties: Option<Vec<String>>,
    /// 本企业的标签ID列表，最多支持100个。
    pub to_tags: Option<Vec<String>>,
    /// 企业应用的id，整型。可在应用的设置页面查看
    pub agent_id: Option<i32>,
    /// 企业应用的id，整型。可在应用的设置页面查看
    pub msg_type: String,
    /// 消息内容，最长不超过2048个字节
    pub content: String,
    /// 图片媒体文件id，可以调用上传临时素材接口获取
    pub media_id: Option<String>,
    pub thumb_media_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub appid: Option<String>,
    pub page: Option<String>,
    pub emphasis_first_item: Option<u8>,
    pub content_items: Option<Value>,
    pub btn_txt: Option<String>,
    pub articles: Option<Vec<WechatCpNewArticle>>,
    pub mp_news_articles: Option<Vec<WechatMpNewsArticle>>,
    /// 表示是否是保密消息，0表示否，1表示是，默认0
    pub is_safe: Option<u8>,
}



/// 发送「学校通知」
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpSchoolContactMessage {
    /// 指定发送对象，0表示发送给家长，1表示发送给学生，2表示发送给家长和学生，默认为0。
    pub recv_scope: Option<u8>,
    /// 家校通讯录家长列表，recv_scope为0或2表示发送给对应的家长，recv_scope为1忽略，（最多支持1000个）
    pub to_parent_userid: Option<Vec<String>>,
    /// 家校通讯录学生列表，recv_scope为0表示发送给学生的所有家长，recv_scope为1表示发送给学生，recv_scope为2表示发送给学生和学生的所有家长（最多支持1000个）
    pub to_student_userid: Option<Vec<String>>,
    /// 家校通讯录部门列表，recv_scope为0表示发送给班级的所有家长，recv_scope为1表示发送给班级的所有学生，recv_scope为2表示发送给班级的所有学生和家长（最多支持100个）
    pub to_party: Option<Vec<String>>,
    /// 1表示字段生效，0表示字段无效。recv_scope为0表示发送给学校的所有家长，recv_scope为1表示发送给学校的所有学生，recv_scope为2表示发送给学校的所有学生和家长，默认为0
    pub to_all: Option<u8>,
    /// 企业应用的id，整型。可在应用的设置页面查看
    pub agentid: Option<i32>,
    /// 消息类型
    pub msgtype: String,
    /// 消息内容，最长不超过2048个字节
    pub content: String,
    /// 表示是否开启id转译，0表示否，1表示是，默认0
    pub enable_id_trans: Option<u8>,
    /// 表示是否开启重复消息检查，0表示否，1表示是，默认0
    pub enable_duplicate_check: Option<u8>,
    /// 表示是否重复消息检查的时间间隔，默认1800s，最大不超过4小时
    pub duplicate_check_interval: Option<u8>,
    /// 图片媒体文件id，可以调用上传临时素材接口获取
    pub media_id: Option<String>,
    /// 小程序消息封面的mediaid，封面图建议尺寸为520*416
    pub thumb_media_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub appid: Option<String>,
    pub pagepath: Option<String>,
    pub articles: Option<Vec<WechatCpNewArticle>>,
    pub mp_news_articles: Option<Vec<WechatMpNewsArticle>>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpLinkedCorpMessageResponse {
    pub invaliduser: Option<Vec<String>>,
    pub invalidparty: Option<Vec<String>>,
    pub invalidtag: Option<Vec<String>>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpSchoolContactMessageResponse {
    pub invalid_parent_userid: Option<Vec<String>>,
    pub invalid_student_userid: Option<Vec<String>>,
    pub invalid_party: Option<Vec<String>>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatCpMessageResponse {
    pub invaliduser: Option<String>,
    pub invalidparty: Option<String>,
    pub invalidtag: Option<String>,
    pub msgid: Option<String>,
}
