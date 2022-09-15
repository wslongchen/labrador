use serde::{Deserialize, Serialize};


// 通讯录变更事件
// 当企业通过通讯录助手开通通讯录权限后，成员的变更会通知给企业。变更的事件，将推送到企业微信管理端通讯录助手中的‘接收事件服务器’。
// 由通讯录同步助手调用接口触发的变更事件不回调通讯录同步助手本身。管理员在管理端更改组织架构或者成员信息以及企业微信的成员在客户端变更自己的个人信息将推送给通讯录同步助手

/// 新增成员事件
/// 该事件会回调给通讯录同步助手，代开发自建应用以及上游企业共享的应用
///
/// 【重要】对于2022年8月15号后通讯录助手新配置或修改的回调url，成员属性只回调UserId/Department两个字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpContactCreateUserEvent {
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="FromUserName")]
    pub source: String,
    /// 成员名称;代开发自建应用需要管理员授权才返回
    #[serde(rename="Name")]
    pub name: Option<String>,
    /// 座机;代开发自建应用需要管理员授权才返回。上游共享的应用不返回该字段
    #[serde(rename="Telephone")]
    pub telephone: Option<String>,
    /// 地址。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Address")]
    pub address: Option<String>,
    /// 成员别名。上游共享的应用不返回该字段
    #[serde(rename="Alias")]
    pub alias: Option<String>,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    /// 成员UserID
    #[serde(rename="UserID")]
    pub user_id: String,
    /// 职位信息。长度为0~64个字节;代开发自建应用需要管理员授权才返回。上游共享的应用不返回该字段
    #[serde(rename="Position")]
    pub position: Option<String>,
    /// 头像url。 注：如果要获取小图将url最后的”/0”改成”/100”即可。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Avatar")]
    pub avatar: Option<String>,
    /// 主部门
    #[serde(rename="MainDepartment")]
    pub main_department: i64,
    /// 激活状态：1=已激活 2=已禁用 4=未激活 已激活代表已激活企业微信或已关注微信插件（原企业号）5=成员退出
    #[serde(rename="Status")]
    pub status: i64,
    /// 性别。0表示未定义，1表示男性，2表示女性。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段。注：不可获取指返回值0
    #[serde(rename="Gender")]
    pub gender: Option<i64>,
    /// 成员部门列表，仅返回该应用有查看权限的部门id
    #[serde(rename="Department")]
    pub department: String,
    /// 表示所在部门是否为部门负责人，0-否，1-是，顺序与Department字段的部门逐一对应。上游共享的应用不返回该字段
    #[serde(rename="IsLeaderInDept")]
    pub is_leader_in_dept: Option<String>,
    /// 直属上级UserID，最多5个。代开发的自建应用和上游共享的应用不返回该字段
    #[serde(rename="DirectLeader")]
    pub direct_leader: Option<Vec<String>>,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    /// 手机号码，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Mobile")]
    pub mobile: Option<String>,
    /// 邮箱，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Email")]
    pub email: String,
    /// 企业邮箱，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="BizMail")]
    pub biz_mail: Option<String>,
    #[serde(rename="ExtAttr")]
    pub ext_attrs: Option<ExtAttrs>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpExtAttrItemText {
    #[serde(rename = "Value")]
    pub value: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpExtAttrItemWeb {
    #[serde(rename="Title")]
    pub title: String,
    #[serde(rename="Url")]
    pub url: String,
}





/// 更新成员事件
/// 该事件会回调给通讯录同步助手，代开发自建应用以及上游企业共享的应用
///
/// 【重要】对于2022年8月15号后通讯录助手新配置或修改的回调url，该事件只会在成员所属部门变更或UserId变更的情况下触发，并且成员属性只回调UserId/Department/NewUserId三个字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpContactUpdateUserEvent {
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 成员UserID
    #[serde(rename="UserID")]
    pub user_id: String,
    /// 新的UserID，变更时推送（userid由系统生成时可更改一次）
    #[serde(rename="NewUserID")]
    pub new_user_id: String,
    /// 成员名称;代开发自建应用需要管理员授权才返回
    #[serde(rename="Name")]
    pub name: Option<String>,
    /// 成员部门列表，仅返回该应用有查看权限的部门id
    #[serde(rename="Department")]
    pub department: String,
    /// 主部门
    #[serde(rename="MainDepartment")]
    pub main_department: i64,
    /// 表示所在部门是否为部门负责人，0-否，1-是，顺序与Department字段的部门逐一对应。上游共享的应用不返回该字段
    #[serde(rename="IsLeaderInDept")]
    pub is_leader_in_dept: Option<String>,
    /// 职位信息。长度为0~64个字节;代开发自建应用需要管理员授权才返回。上游共享的应用不返回该字段
    #[serde(rename="Position")]
    pub position: Option<String>,
    /// 手机号码，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Mobile")]
    pub mobile: Option<String>,
    /// 性别。0表示未定义，1表示男性，2表示女性。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段。注：不可获取指返回值0
    #[serde(rename="Gender")]
    pub gender: Option<i64>,
    /// 邮箱，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Email")]
    pub email: String,
    /// 激活状态：1=已激活 2=已禁用 4=未激活 已激活代表已激活企业微信或已关注微信插件（原企业号）5=成员退出
    #[serde(rename="Status")]
    pub status: i64,
    /// 头像url。 注：如果要获取小图将url最后的”/0”改成”/100”即可。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Avatar")]
    pub avatar: Option<String>,
    /// 成员别名。上游共享的应用不返回该字段
    #[serde(rename="Alias")]
    pub alias: Option<String>,
    /// 座机;代开发自建应用需要管理员授权才返回。上游共享的应用不返回该字段
    #[serde(rename="Telephone")]
    pub telephone: Option<String>,
    /// 地址。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Address")]
    pub address: Option<String>,
    /// 直属上级UserID，最多5个。代开发的自建应用和上游共享的应用不返回该字段
    #[serde(rename="DirectLeader")]
    pub direct_leader: Option<String>,
    /// 企业邮箱，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="BizMail")]
    pub biz_mail: Option<String>,
    #[serde(rename="ExtAttr")]
    pub ext_attrs: Option<ExtAttrs>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ExtAttrs {
     #[serde(rename = "Item")]
     items: Vec<ExtAttrItem>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ExtAttrItem {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Type")]
    attr_type: u32,
    #[serde(rename="Text")]
    pub text: Option<CpExtAttrItemText>,
    #[serde(rename="Web")]
    pub web: Option<CpExtAttrItemWeb>,
}



/// 删除成员事件
/// 该事件会回调给通讯录同步助手，代开发自建应用以及上游企业共享的应用。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpContactDeleteUserEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 成员UserID
    #[serde(rename="UserID")]
    pub user_id: String,
}




/// 新增部门事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpContactCreatePartyEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 部门Id
    #[serde(rename="Id")]
    pub id: i32,
    /// 父部门id
    #[serde(rename="ParentId")]
    pub parent_id: i32,
    /// 部门名称
    #[serde(rename="Name")]
    pub name: String,
    /// 部门排序
    #[serde(rename="Order")]
    pub order: i32,
}


/// 更新部门事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpContactUpdatePartyEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 部门Id
    #[serde(rename="Id")]
    pub id: i32,
    #[serde(rename="ParentId")]
    pub parent_id: i32,
    /// 部门名称
    #[serde(rename="Name")]
    pub name: String,
}



/// 删除部门事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpContactDeletePartyEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 部门Id
    #[serde(rename="Id")]
    pub id: i32,
}

/// 标签成员变更事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpContactUpdateTagEvent {
    #[serde(rename = "ToUserName")]
    pub target: String,
    #[serde(rename = "FromUserName")]
    pub source: String,
    # [serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename="Event")]
    pub event: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    #[serde(rename="MsgType")]
    pub msg_type: String,
    /// 标签Id
    #[serde(rename="TagId")]
    pub tag_id: i32,
    /// 标签中新增的成员userid列表，用逗号分隔
    #[serde(rename="AddUserItems")]
    pub add_users: Option<String>,
    /// 标签中删除的成员userid列表，用逗号分隔
    #[serde(rename="DelUserItems")]
    pub delete_users: Option<String>,
    /// 标签中新增的部门id列表，用逗号分隔
    #[serde(rename="AddPartyItems")]
    pub add_party: Option<String>,
    /// 标签中删除的部门id列表，用逗号分隔
    #[serde(rename="DelPartyItems")]
    pub delete_party: Option<String>,
}


#[cfg(test)]
mod tests {
    use crate::{CpContactUpdateUserEvent, XmlMessageParser};
    use super::CpContactCreateUserEvent;

    #[test]
    fn test_create_user_from_xml() {
        let xml = "<xml>
	<ToUserName><![CDATA[toUser]]></ToUserName>
	<FromUserName><![CDATA[sys]]></FromUserName>
	<CreateTime>1403610513</CreateTime>
	<MsgType><![CDATA[event]]></MsgType>
	<Event><![CDATA[change_contact]]></Event>
	<ChangeType>create_user</ChangeType>
	<UserID><![CDATA[zhangsan]]></UserID>
	<Name><![CDATA[张三]]></Name>
	<Department><![CDATA[1,2,3]]></Department>
	<MainDepartment>1</MainDepartment>
	<IsLeaderInDept><![CDATA[1,0,0]]></IsLeaderInDept>
	<DirectLeader><![CDATA[lisi,wangwu]]></DirectLeader>
	<Position><![CDATA[产品经理]]></Position>
	<Mobile>13800000000</Mobile>
	<Gender>1</Gender>
	<Email><![CDATA[zhangsan@gzdev.com]]></Email>
	<BizMail><![CDATA[zhangsan@qyycs2.wecom.work]]></BizMail>
	<Status>1</Status>
	<Avatar><![CDATA[http://wx.qlogo.cn/mmopen/ajNVdqHZLLA3WJ6DSZUfiakYe37PKnQhBIeOQBO4czqrnZDS79FH5Wm5m4X69TBicnHFlhiafvDwklOpZeXYQQ2icg/0]]></Avatar>
	<Alias><![CDATA[zhangsan]]></Alias>
	<Telephone><![CDATA[020-123456]]></Telephone>
	<Address><![CDATA[广州市]]></Address>

</xml>";
        let item: CpContactCreateUserEvent = CpContactCreateUserEvent::from_xml(xml).unwrap();
        println!("{}", serde_json::to_string(&item).unwrap_or_default());
    }

    #[test]
    fn test_update_user_from_xml() {
        let xml = "<xml>
	<ToUserName><![CDATA[toUser]]></ToUserName>
	<FromUserName><![CDATA[sys]]></FromUserName>
	<CreateTime>1403610513</CreateTime>
	<MsgType><![CDATA[event]]></MsgType>
	<Event><![CDATA[change_contact]]></Event>
	<ChangeType>update_user</ChangeType>
	<UserID><![CDATA[zhangsan]]></UserID>
	<NewUserID><![CDATA[zhangsan001]]></NewUserID>
	<Name><![CDATA[张三]]></Name>
	<Department><![CDATA[1,2,3]]></Department>
	<MainDepartment>1</MainDepartment>
	<IsLeaderInDept><![CDATA[1,0,0]]></IsLeaderInDept>
	<Position><![CDATA[产品经理]]></Position>
	<Mobile>13800000000</Mobile>
	<Gender>1</Gender>
	<Email><![CDATA[zhangsan@gzdev.com]]></Email>
	<Status>1</Status>
	<Avatar><![CDATA[http://wx.qlogo.cn/mmopen/ajNVdqHZLLA3WJ6DSZUfiakYe37PKnQhBIeOQBO4czqrnZDS79FH5Wm5m4X69TBicnHFlhiafvDwklOpZeXYQQ2icg/0]]></Avatar>
	<Alias><![CDATA[zhangsan]]></Alias>
	<Telephone><![CDATA[020-123456]]></Telephone>
	<Address><![CDATA[广州市]]></Address>
	<ExtAttr>
		<Item>
		<Name><![CDATA[爱好]]></Name>
		<Type>0</Type>
		<Text>
			<Value><![CDATA[旅游]]></Value>
		</Text>
		</Item>
		<Item>
		<Name><![CDATA[卡号]]></Name>
		<Type>1</Type>
		<Web>
			<Title><![CDATA[企业微信]]></Title>
			<Url><![CDATA[https://work.weixin.qq.com]]></Url>
		</Web>
		</Item>
	</ExtAttr>
</xml>";
        let item: CpContactUpdateUserEvent = CpContactUpdateUserEvent::from_xml(xml).unwrap();
        println!("{}", serde_json::to_string(&item).unwrap_or_default());
    }
}