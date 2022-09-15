use serde::{Serialize, Deserialize};

/// 审批状态通知事件
/// 本事件触发时机为：
/// 1.自建/第三方应用调用审批流程引擎发起申请之后，审批状态发生变化时
/// 2.自建/第三方应用调用审批流程引擎发起申请之后，在“审批中”状态，有任意审批人进行审批操作时
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpOpenApprovalChangeEvent {
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
    #[serde(rename="ApprovalInfo")]
    pub approval_info: Option<ApprovalInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApprovalInfo {
    /// 审批单编号，由开发者在发起申请时自定义
    #[serde(rename="ThirdNo")]
    pub third_no: String,
    /// 审批模板名称
    #[serde(rename="OpenSpName")]
    pub open_sp_name: String,
    /// 审批模板id
    #[serde(rename="OpenTemplateId")]
    pub open_template_id: i64,
    /// 申请单当前审批状态：1-审批中；2-已通过；3-已驳回；4-已取消
    #[serde(rename="OpenSpStatus")]
    pub open_sp_status: i32,
    #[serde(rename="ApplyTime")]
    /// 提交申请时间
    pub apply_time: i64,
    /// 提交者姓名
    #[serde(rename="ApplyUserName")]
    pub apply_user_name: String,
    /// 提交者userid
    #[serde(rename="ApplyUserId")]
    pub apply_user_id: String,
    /// 提交者所在部门
    #[serde(rename="ApplyUserParty")]
    pub apply_user_party: String,
    /// 提交者头像
    #[serde(rename="ApplyUserImage")]
    pub apply_user_image: String,
    /// 审批流程信息，可以有多个审批节点
    #[serde(rename="ApprovalNodes")]
    pub approval_nodes: ApprovalNodes,
    /// 抄送信息，可能有多个抄送人
    #[serde(rename="NotifyNodes")]
    pub notify_nodes: NotifyNodes,
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ApprovalNodes {
    #[serde(rename = "ApprovalNode")]
    items: Vec<ApprovalNode>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ApprovalNode {
    #[serde(rename = "NodeStatus")]
    name: u32,
    #[serde(rename = "NodeAttr")]
    node_attr: u32,
    #[serde(rename = "NodeType")]
    node_type: u32,
    #[serde(rename="Items")]
    pub items: Option<ApprovalNodeItems>,
}

/// 抄送信息，可能有多个抄送人
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct NotifyNodes {
    #[serde(rename = "NotifyNode")]
    items: Vec<NotifyNode>,
    /// 当前审批节点：0-第一个审批节点；1-第二个审批节点…以此类推
    #[serde(rename = "approverstep")]
    approver_step: Vec<NotifyNode>
}

/// 抄送人信息
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct NotifyNode {
    /// 抄送人姓名
    #[serde(rename = "ItemName")]
    name: String,
    /// 抄送人userid
    #[serde(rename = "ItemUserId")]
    item_user_id: u32,
    /// 抄送人头像
    #[serde(rename = "ItemImage")]
    item_image: String,
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ApprovalNodeItems {
    #[serde(rename = "Item")]
    items: Vec<ApprovalNodeItem>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ApprovalNodeItem {
    /// 分支审批人姓名
    #[serde(rename = "ItemName")]
    name: String,
    /// 分支审批人userid
    #[serde(rename = "ItemUserId")]
    item_user_id: u32,
    /// 分支审批人头像
    #[serde(rename = "ItemImage")]
    item_image: String,
    /// 分支审批审批操作状态：1-审批中；2-已同意；3-已驳回；4-已转审
    #[serde(rename = "ItemStatus")]
    item_status: u32,
    /// 分支审批人审批意见
    #[serde(rename = "ItemSpeech")]
    item_speech: u32,
    /// 分支审批人操作时间
    #[serde(rename = "ItemOpTime")]
    item_op_time: u32,
}

#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
    use super::CpSubscribeEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>
 <ToUserName><![CDATA[wwddddccc7775555aaa]]></ToUserName>
  <FromUserName><![CDATA[sys]]></FromUserName>
  <CreateTime>1527838022</CreateTime>
  <MsgType><![CDATA[event]]></MsgType>
  <Event><![CDATA[open_approval_change]]></Event>
  <AgentID>1</AgentID>
  <ApprovalInfo>
    <ThirdNo><![CDATA[201806010001]]></ThirdNo>
    <OpenSpName><![CDATA[付款]]></OpenSpName>
    <OpenTemplateId><![CDATA[1234567890]]></OpenTemplateId>
    <OpenSpStatus>1</OpenSpStatus>
    <ApplyTime>1527837645</ApplyTime>
    <ApplyUserName><![CDATA[xiaoming]]></ApplyUserName>
    <ApplyUserId><![CDATA[1]]></ApplyUserId>
    <ApplyUserParty><![CDATA[产品部]]></ApplyUserParty>
    <ApplyUserImage><![CDATA[http://www.qq.com/xxx.png]]></ApplyUserImage>
    <ApprovalNodes>
      <ApprovalNode>
        <NodeStatus>1</NodeStatus>
        <NodeAttr>1</NodeAttr>
        <NodeType>1</NodeType>
        <Items>
          <Item>
            <ItemName><![CDATA[xiaohong]]></ItemName>
            <ItemUserId><![CDATA[2]]></ItemUserId>
            <ItemImage><![CDATA[http://www.qq.com/xxx.png]]></ItemImage>
            <ItemStatus>1</ItemStatus>
            <ItemSpeech><![CDATA[]]></ItemSpeech>
            <ItemOpTime>0</ItemOpTime>
          </Item>
        </Items>
      </ApprovalNode>
    </ApprovalNodes>
    <NotifyNodes>
      <NotifyNode>
        <ItemName><![CDATA[xiaogang]]></ItemName>
        <ItemUserId><![CDATA[3]]></ItemUserId>
        <ItemImage><![CDATA[http://www.qq.com/xxx.png]]></ItemImage>
      </NotifyNode>
    </NotifyNodes>
    <approverstep>0</approverstep>
  </ApprovalInfo>
</xml>";
        let msg = serde_xml_rs::from_str::<serde_json::Value>(xml).unwrap();
        println!("{}", msg.to_string());
    }
}