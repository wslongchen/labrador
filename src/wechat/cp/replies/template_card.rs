use crate::{current_timestamp, QuoteArea, ReplyRenderer};

/// 更新点击用户的整张卡片
#[derive(Debug, Clone)]
pub struct CpTemplateCardTextReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    /// 模板卡片类型，文本通知型填写 "text_notice"
    pub card_type: String,
    /// 卡片来源样式信息，不需要来源样式可不填写
    pub card_source: TemplateCardSource,
    pub main_title: TemplateCardContent,
    /// 二级普通文本
    pub sub_title: Option<String>,
    /// 二级标题+文本列表，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过6
    pub horizontal_content: Vec<HorizontalContentList>,
    /// 跳转指引样式的列表，该字段可为空数组，但有数据的话需确认对应字段是否必填，列表长度不超过3
    pub jump_list: JumpList,
    /// 整体卡片的点击跳转事件，必填
    pub card_action: CardAction,
    /// 关键数据样式的数据内容
    pub emphasis_content: TemplateCardContent,
    /// 卡片右上角更多操作按钮容
    pub action_menu: ActionMenu,
    /// 引用文献样式
    pub quote_area: QuoteArea,
}

#[derive(Debug, Clone)]
pub struct ActionMenu {
    /// 更多操作界面的描述
    pub desc: Option<String>,
    /// 操作列表，列表长度取值范围为 [1, 10]
    pub action_list: Vec<ActionListItem>,
}


#[derive(Debug, Clone)]
pub struct ActionListItem {
    /// 操作的描述文案
    pub text: Option<String>,
    /// 操作key值，用户点击后，会产生回调事件将本参数作为EventKey回调，最长支持1024字节，不可重复，必填
    pub key: Option<String>,
}



#[derive(Debug, Clone)]
pub struct HorizontalContentList {
    /// 二级标题，必填
    pub key_name: String,
    /// 二级文本，如果HorizontalContentList.Type是2，该字段代表文件名称（要包含文件类型）
    pub value: Option<String>,
    /// 链接跳转的url，HorizontalContentList.Type是1时必填
    pub url: Option<String>,
    /// 附件的media_id，HorizontalContentList.Type是2时必填
    pub media_id: Option<String>,
    /// 成员详情的userid，HorizontalContentList.Type是3时必填
    pub user_id: Option<String>,
    /// 链接类型，0或不填或错填代表不是链接，1 代表跳转url，2 代表下载附件，3 代表点击跳转成员详情
    pub url_type: Option<u8>,
}



#[derive(Debug, Clone)]
pub struct CardAction {
    /// 跳转链接样式的文案内容，必填
    pub title: Option<String>,
    /// 跳转事件类型，0或不填或错填代表不是链接，1 代表跳转url，2 代表下载附件
    pub action_type: Option<u8>,
    /// 跳转事件的url，CardAction.Type是1时必填
    pub url: Option<String>,
    /// 跳转事件的小程序的pagepath，CardAction.Type是2时选填
    pub page_path: Option<String>,
    /// 跳转事件的小程序的appid，CardAction.Type是2时必填
    pub app_id: Option<String>,
}





#[derive(Debug, Clone)]
pub struct JumpList {
    /// 跳转链接样式的文案内容，必填
    pub title: String,
    /// 跳转链接类型，0或不填或错填代表不是链接，1 代表跳转url，2 代表跳转小程序
    pub jump_type: Option<u8>,
    /// 链接跳转的url，HorizontalContentList.Type是1时必填
    pub url: Option<String>,
    /// 跳转链接的小程序的pagepath，JumpList.Type是2时选填
    pub page_path: Option<String>,
    /// 跳转链接的小程序的appid，JumpList.Type是2时必填
    pub app_id: Option<String>,
}




#[derive(Debug, Clone)]
pub struct TemplateCardContent {
    pub title: Option<String>,
    pub desc: Option<String>,
}


#[derive(Debug, Clone)]
pub struct TemplateCardSource {
    /// 来源图片的url
    pub icon_url: String,
    /// 来源图片的描述
    pub desc: String,
    /// 来源文字的颜色，目前支持：0(默认) 灰色，1 黑色，2 红色，3 绿色
    pub desc_color: i64,
}

#[allow(unused)]
impl CpTemplateCardTextReply {
    #[inline]
    pub fn new<S: Into<String>>(source: S, target: S) -> CpTemplateCardTextReply {
        CpTemplateCardTextReply {
            source: source.into(),
            target: target.into(),
            time: current_timestamp(),
            card_type: "".to_string(),
            card_source: TemplateCardSource {
                icon_url: "".to_string(),
                desc: "".to_string(),
                desc_color: 0
            },
            main_title: TemplateCardContent { title: None, desc: None },
            sub_title: None,
            horizontal_content: vec![],
            jump_list: JumpList{
                title: "".to_string(),
                jump_type: None,
                url: None,
                page_path: None,
                app_id: None
            },
            card_action: CardAction {
                title: None,
                action_type: None,
                url: None,
                page_path: None,
                app_id: None
            },
            emphasis_content: TemplateCardContent{ title: None, desc: None },
            action_menu: ActionMenu { desc: None, action_list: vec![] },
            quote_area: QuoteArea {
                r#type: None,
                url: None,
                appid: None,
                pagepath: None,
                title: None,
                quote_text: None
            }
        }
    }
}

impl ReplyRenderer for CpTemplateCardTextReply {
    #[inline]
    fn render(&self) -> String {

        let mut horizontal_content = String::default();
        for content in &self.horizontal_content {
            let mut xml = format!("<HorizontalContentList>
                <KeyName><![CDATA[{}]]></KeyName>
                <Type>{}</Type>
                <Value><![CDATA[{}]]></Value>", content.key_name, content.url_type.to_owned().unwrap_or_default(),content.value.to_owned().unwrap_or_default());
            if let Some(url) = &content.url {
                xml.push_str(&format!("<Url><![CDATA[{}]]></Url>", url))
            }
            if let Some(user_id) = &content.user_id {
                xml.push_str(&format!("<UserId><![CDATA[{}]]></UserId>", user_id))
            }
            if let Some(media_id) = &content.media_id {
                xml.push_str(&format!("<MediaId><![CDATA[{}]]></MediaId>", media_id))
            }
            xml.push_str("</HorizontalContentList>\n");
            horizontal_content.push_str(&xml);
        }

        let mut action_list = String::default();
        for item in &self.action_menu.action_list {
            let xml = format!("<ActionList>
                <Text><![CDATA[{}]]></Text>
                <Key>{}</Key></ActionList>", item.text.to_owned().unwrap_or_default(),item.key.to_owned().unwrap_or_default());
            action_list.push_str(&xml);
        }
        format!("<xml>\n\
            <ToUserName><![CDATA[{target}]]></ToUserName>\n\
            <FromUserName><![CDATA[{source}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[update_template_card]]></MsgType>\n\
            <TemplateCard>\n\
                <CardType><![CDATA[{card_type}]]></CardType>\n\
                <Source>\n\
                    <IconUrl><![CDATA[{icon_url}]]></IconUrl>\n\
                    <Desc><![CDATA[{source_desc}]]></Desc>\n\
                    <DescColor>{source_desc_color}</DescColor>\n\
                </Source>\n\
                <MainTitle>
                    <Title><![CDATA[{main_title}]]></Title>
                    <Desc><![CDATA[{main_title_desc}]]></Desc>
                </MainTitle>
                <SubTitleText><![CDATA[{sub_title}]]></SubTitleText>
                {horizontal_content}
                <JumpList>
                    <Title><![CDATA[{jump_title}]]></Title>
                    <Type>{jump_type}</Type>
                    <Url><![CDATA[{jump_url}]]></Url>
			    </JumpList>
                <CardAction>
                    <Title><![CDATA[{action_title}]]></Title>
                    <Type>{action_type}</Type>
                    <Url><![CDATA[{action_url}]]></Url>
			    </CardAction>
                <EmphasisContent>
                    <Title><![CDATA[{emphasis_title}]]></Title>
                    <Desc><![CDATA[{emphasis_desc}]]></Desc>
                </EmphasisContent>
                <ActionMenu>
                    <Desc><![CDATA[{action_menu_desc}]]></Desc>
                    {action_list}
                </ActionMenu>
                <QuoteArea>
                    <Type><![CDATA[{quote_type}]]></Type>
                    <Url><![CDATA[{quote_url}]]></Url>
                    <Title><![CDATA[{quote_title}]]></Title>
                    <QuoteText><![CDATA[{quote_text}]]></QuoteText>
                </QuoteArea>
            </TemplateCard>\n\
            </xml>",
            target=self.target,
            source=self.source,
            time=self.time,
            card_type=self.card_type,
            icon_url=self.card_source.icon_url,
            source_desc=self.card_source.desc,
            source_desc_color=self.card_source.desc_color,
            main_title=self.main_title.title.to_owned().unwrap_or_default(),
            main_title_desc=self.main_title.desc.to_owned().unwrap_or_default(),
            sub_title=self.sub_title.to_owned().unwrap_or_default(),
            horizontal_content=horizontal_content,
                action_list=action_list,
            jump_title=self.jump_list.title,
            jump_type=self.jump_list.jump_type.to_owned().unwrap_or_default(),
            jump_url=self.jump_list.url.to_owned().unwrap_or_default(),
            action_title=self.card_action.title.to_owned().unwrap_or_default(),
            action_type=self.card_action.action_type.to_owned().unwrap_or_default(),
            action_url=self.card_action.url.to_owned().unwrap_or_default(),
            emphasis_title=self.emphasis_content.title.to_owned().unwrap_or_default(),
            emphasis_desc=self.emphasis_content.desc.to_owned().unwrap_or_default(),
            quote_type=self.quote_area.r#type.to_owned().unwrap_or_default(),
            quote_url=self.quote_area.url.to_owned().unwrap_or_default(),
            quote_title=self.quote_area.title.to_owned().unwrap_or_default(),
            quote_text=self.quote_area.quote_text.to_owned().unwrap_or_default(),
            action_menu_desc=self.action_menu.desc.to_owned().unwrap_or_default(),
        )
    }
}
