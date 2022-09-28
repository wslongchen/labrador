use crate::{current_timestamp, ReplyRenderer};

/// 更新点击用户的按钮文案
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CpUpdateButtonReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    /// 点击卡片按钮后显示的按钮名称
    pub replace_name: String,
}

#[allow(unused)]
impl CpUpdateButtonReply {
    #[inline]
    pub fn new<S: Into<String>>(source: S, target: S) -> CpUpdateButtonReply {
        CpUpdateButtonReply {
            source: source.into(),
            target: target.into(),
            time: current_timestamp(),
            replace_name: "".to_string()
        }
    }

    pub fn set_replace_name<S: Into<String>>(&mut self, replace_name: S) -> &mut Self {
        self.replace_name = replace_name.into();
        self
    }
}

impl ReplyRenderer for CpUpdateButtonReply {
    #[inline]
    fn render(&self) -> String {
        format!("<xml>\n\
            <ToUserName><![CDATA[{target}]]></ToUserName>\n\
            <FromUserName><![CDATA[{source}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[update_button]]></MsgType>\n\
            <Button><ReplaceName><![CDATA[{replace_name}]]></ReplaceName></Button>\n\
            </xml>",
            target=self.target,
            source=self.source,
            time=self.time,
            replace_name=self.replace_name,
        )
    }
}