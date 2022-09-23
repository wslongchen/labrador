//! 事件格式
//! 开启接收消息模式后，可以配置接收事件消息。
//! 当企业成员通过企业微信APP或微信插件（原企业号）触发进入应用、上报地理位置、点击菜单等事件时，企业微信会将这些事件消息发送给企业后台。
//! 如何接收消息已经在使用接收消息说明，本小节是对事件消息结构体的说明。
//!
//! 注：以下出现的数据包仅是接收的消息包中的<a href="https://developer.work.weixin.qq.com/document/path/90240#12977/%E4%BD%BF%E7%94%A8%E6%8E%A5%E6%94%B6%E6%B6%88%E6%81%AF">Encrypt参数</a>解密后的内容说明
//!
mod subscribe;
mod unsubscribe;
mod enter_agent;
mod location;
mod batch_job_result;
mod contact_change;
mod menu;
mod approval;
mod share_agent;
mod share_chain;
mod template_card;
mod ticket;
mod auth;
mod permanent_code;
mod app_admin;
mod tp_contact_change;
mod unlicensed;
mod licecnse_pay_success;
mod licecnse_refund;
mod auto_activate;
mod customer;

pub use subscribe::CpSubscribeEvent;
pub use unsubscribe::CpUnsubscribeEvent;
pub use enter_agent::CpEnterAgentEvent;
pub use location::CpLocationEvent;
pub use batch_job_result::CpBatchJobResultEvent;
pub use contact_change::*;
pub use menu::*;
pub use approval::*;
pub use share_agent::*;
pub use share_chain::*;
pub use ticket::*;
pub use template_card::*;
pub use auth::*;
pub use permanent_code::*;
pub use app_admin::*;
pub use tp_contact_change::*;
pub use unlicensed::*;
pub use licecnse_pay_success::*;
pub use licecnse_refund::*;
pub use auto_activate::*;
pub use customer::*;