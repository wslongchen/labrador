use serde::{Deserialize, Serialize};


/// 新增成员事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpTpContactCreateUserEvent {
    /// 第三方应用ID
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    /// 授权企业的CorpID
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    /// 成员UserID
    #[serde(rename="UserID")]
    pub user_id: String,
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
    /// 直属上级UserID，最多5个。代开发的自建应用和上游共享的应用不返回该字段
    #[serde(rename="DirectLeader")]
    pub direct_leader: Option<Vec<String>>,
    /// 手机号码，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Mobile")]
    pub mobile: Option<String>,
    /// 性别。0表示未定义，1表示男性，2表示女性。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段。注：不可获取指返回值0
    #[serde(rename="Gender")]
    pub gender: Option<i64>,
    /// 职位信息。长度为0~64个字节;代开发自建应用需要管理员授权才返回。上游共享的应用不返回该字段
    #[serde(rename="Position")]
    pub position: Option<String>,
    /// 邮箱，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Email")]
    pub email: String,
    /// 企业邮箱，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="BizMail")]
    pub biz_mail: Option<String>,
    /// 头像url。 注：如果要获取小图将url最后的”/0”改成”/100”即可。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Avatar")]
    pub avatar: Option<String>,
    /// 座机;代开发自建应用需要管理员授权才返回。上游共享的应用不返回该字段
    #[serde(rename="Telephone")]
    pub telephone: Option<String>,
    /// 地址。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    #[serde(rename="Address")]
    pub address: Option<String>,
    /// 成员别名。上游共享的应用不返回该字段
    #[serde(rename="Alias")]
    pub alias: Option<String>,
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpTpContactUpdateUserEvent {
    /// 第三方应用ID
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    /// 授权企业的CorpID
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    /// 成员UserID
    #[serde(rename="UserID")]
    pub user_id: String,
    /// 全局唯一。对于同一个服务商，不同应用获取到企业内同一个成员的OpenUserID是相同的，最多64个字节。
    #[serde(rename="OpenUserID")]
    pub open_user_id: String,
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpTpContactDeleteUserEvent {
    /// 第三方应用ID
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    /// 授权企业的CorpID
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    /// 全局唯一。对于同一个服务商，不同应用获取到企业内同一个成员的OpenUserID是相同的，最多64个字节。
    #[serde(rename="OpenUserID")]
    pub open_user_id: String,
    /// 成员UserID
    #[serde(rename="UserID")]
    pub user_id: String,
}




/// 新增部门事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpTpContactCreatePartyEvent {
    /// 第三方应用ID
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    /// 授权企业的CorpID
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
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
pub struct CpTpContactUpdatePartyEvent {
    /// 第三方应用ID
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    /// 授权企业的CorpID
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
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
pub struct CpTpContactDeletePartyEvent {
    /// 第三方应用ID
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    /// 授权企业的CorpID
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
    /// 部门Id
    #[serde(rename="Id")]
    pub id: i32,
}

/// 标签成员变更事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpTpContactUpdateTagEvent {
    /// 第三方应用ID
    #[serde(rename="SuiteId")]
    pub suite_id: String,
    #[serde(rename="InfoType")]
    pub info_type: String,
    #[serde(rename="TimeStamp")]
    pub time: i64,
    /// 授权企业的CorpID
    #[serde(rename="AuthCorpId")]
    pub auth_corp_id: String,
    #[serde(rename="ChangeType")]
    pub change_type: String,
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

