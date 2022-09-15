use serde::{Deserialize, Serialize};


// 菜单事件
// 成员点击自定义菜单后，企业微信会把点击事件推送给应用。
// 点击菜单弹出子菜单，不会产生上报。
// 企业微信iPhone1.2.2/Android1.2.2版本开始支持菜单事件，旧版本企业微信成员点击后将没有回应，应用不能正常接收到事件推送。
// 自定义菜单可以在管理后台的应用设置界面配置。

/// 点击菜单拉取消息的事件推送
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpMenuClickEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 事件类型：click
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 事件KEY值，与自定义菜单接口中KEY值对应
    #[serde(rename="EventKey")]
    pub event_key: String,
    /// 企业应用的id，整型。可在应用的设置页面查看
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}

/// 点击菜单跳转链接的事件推送
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpMenuViewEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 事件类型：click
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 事件KEY值，与自定义菜单接口中KEY值对应
    #[serde(rename="EventKey")]
    pub event_key: String,
    /// 企业应用的id，整型。可在应用的设置页面查看
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}

/// 扫码推事件的事件推送
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpMenuScanCodePushEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 事件类型：click
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 事件KEY值，与自定义菜单接口中KEY值对应
    #[serde(rename="EventKey")]
    pub event_key: String,
    /// 企业应用的id，整型。可在应用的设置页面查看
    #[serde(rename="AgentID")]
    pub agent_id: i64,
    /// 扫描信息
    #[serde(rename="ScanCodeInfo")]
    pub scan_code_info: ScanCodeInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanCodeInfo {
    /// 扫描类型，一般是qrcode
    #[serde(rename = "ScanType")]
    pub scan_type: String,
    /// 扫描结果，即二维码对应的字符串信息
    #[serde(rename = "ScanResult")]
    pub scan_result: String,
}



/// 扫码推事件且弹出“消息接收中”提示框的事件推送
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpMenuScanCodeWaitMsgEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 事件类型：click
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 事件KEY值，与自定义菜单接口中KEY值对应
    #[serde(rename="EventKey")]
    pub event_key: String,
    /// 企业应用的id，整型。可在应用的设置页面查看
    #[serde(rename="AgentID")]
    pub agent_id: i64,
    /// 扫描信息
    #[serde(rename="ScanCodeInfo")]
    pub scan_code_info: ScanCodeInfo,
}


/// 弹出系统拍照发图的事件推送
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpMenuPicSysPhotoEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 事件类型：click
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 事件KEY值，与自定义菜单接口中KEY值对应
    #[serde(rename="EventKey")]
    pub event_key: String,
    /// 企业应用的id，整型。可在应用的设置页面查看
    #[serde(rename="AgentID")]
    pub agent_id: i64,
    /// 发送的图片信息
    #[serde(rename="SendPicsInfo")]
    pub send_pics_info: SendPicsInfo,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendPicsInfo {
    /// 发送的图片数量
    #[serde(rename = "Count")]
    pub count: i64,
    /// 扫描结果，即二维码对应的字符串信息
    #[serde(rename = "PicList")]
    pub pic_list: Option<String>,
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct PicList {
    #[serde(rename = "item")]
    items: Vec<PicItem>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct PicItem {
    #[serde(rename = "PicMd5Sum")]
    pic_md5_sum: String,
}


/// 弹出拍照或者相册发图的事件推送
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpMenuPicPhotoOrAlbumEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 事件类型：click
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 事件KEY值，与自定义菜单接口中KEY值对应
    #[serde(rename="EventKey")]
    pub event_key: String,
    /// 企业应用的id，整型。可在应用的设置页面查看
    #[serde(rename="AgentID")]
    pub agent_id: i64,
    /// 发送的图片信息
    #[serde(rename="SendPicsInfo")]
    pub send_pics_info: SendPicsInfo,
}



/// 弹出微信相册发图器的事件推送
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpMenuPicWeixinEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 事件类型：click
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 事件KEY值，与自定义菜单接口中KEY值对应
    #[serde(rename="EventKey")]
    pub event_key: String,
    /// 企业应用的id，整型。可在应用的设置页面查看
    #[serde(rename="AgentID")]
    pub agent_id: i64,
    /// 发送的图片信息
    #[serde(rename="SendPicsInfo")]
    pub send_pics_info: SendPicsInfo,
}



/// 弹出地理位置选择器的事件推送
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpMenuLocationSelectEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    /// 事件类型：click
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 事件KEY值，与自定义菜单接口中KEY值对应
    #[serde(rename="EventKey")]
    pub event_key: String,
    /// 企业应用的id，整型。可在应用的设置页面查看
    #[serde(rename="AgentID")]
    pub agent_id: i64,
    /// 发送的图片信息
    #[serde(rename="SendLocationInfo")]
    pub send_location_info: SendLocationInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendLocationInfo {
    #[serde(rename="Location_X")]
    pub location_x: f64,
    #[serde(rename="Location_Y")]
    pub location_y: f64,
    #[serde(rename="Scale")]
    pub scale: usize,
    #[serde(rename="Label")]
    pub label: String,
    /// POI的名字，可能为空
    #[serde(rename="Poiname")]
    pub poiname: Option<String>,
}
