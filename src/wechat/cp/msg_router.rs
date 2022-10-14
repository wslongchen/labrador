use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use serde_json::Value;
use crate::{CpMessage, CpReply, LabradorResult, SimpleStorage, WechatCpClient, WechatCpTpClient};
use crate::redis_store::RedisStorage;

pub struct WechatCpMessageContext {
    pub msg: CpMessage,
    pub ctx: Value,
    pub client: WechatCpClientWrapper
}

#[derive(Clone)]
pub enum WechatCpClientWrapper {
    CpSimple(WechatCpClient<SimpleStorage>),
    CpRedis(WechatCpClient<RedisStorage>),
    CpTpSimple(WechatCpTpClient<SimpleStorage>),
    CpTpRedis(WechatCpTpClient<RedisStorage>),
}

impl WechatCpClientWrapper {
    pub fn client_with_rds(&self) -> Option<&WechatCpClient<RedisStorage>> {
        match self {
            WechatCpClientWrapper::CpRedis(v) => v.into(),
            _=> None,
        }
    }
    pub fn client(&self) -> Option<&WechatCpClient<SimpleStorage>> {
        match self {
            WechatCpClientWrapper::CpSimple(v) => v.into(),
            _=> None,
        }
    }
    pub fn tp_client_with_rds(&self) -> Option<&WechatCpTpClient<RedisStorage>> {
        match self {
            WechatCpClientWrapper::CpTpRedis(v) => v.into(),
            _=> None,
        }
    }
    pub fn tp_client(&self) -> Option<&WechatCpTpClient<SimpleStorage>> {
        match self {
            WechatCpClientWrapper::CpTpSimple(v) => v.into(),
            _=> None,
        }
    }
}

pub struct WechatCpMessageRouter {
    rules: Vec<WechatCpMessageRouterRule>,
    client: WechatCpClientWrapper
}

#[allow(unused)]
impl WechatCpMessageRouter {
    pub fn from_client(client: WechatCpClient<SimpleStorage>) -> Self {
        Self {
            rules: vec![],
            client: WechatCpClientWrapper::CpSimple(client)
        }
    }
    pub fn from_client_rds(client: WechatCpClient<RedisStorage>) -> Self {
        Self {
            rules: vec![],
            client: WechatCpClientWrapper::CpRedis(client)
        }
    }
    pub fn from_tp_client(client: WechatCpTpClient<SimpleStorage>) -> Self {
        Self {
            rules: vec![],
            client: WechatCpClientWrapper::CpTpSimple(client)
        }
    }
    pub fn from_tp_client_rds(client: WechatCpTpClient<RedisStorage>) -> Self {
        Self {
            rules: vec![],
            client: WechatCpClientWrapper::CpTpRedis(client)
        }
    }

    pub fn rule(&self) -> WechatCpMessageRouterRule {
        WechatCpMessageRouterRule::new()
    }

    pub fn push_rule(mut self, rule: WechatCpMessageRouterRule) -> Self {
        self.rules.push(rule);
        self
    }

    pub async fn route(&self, msg: CpMessage, mut ctx: Value) -> Option<CpReply> {
        let match_rules = self.rules.iter().filter(|r| r.test(&msg)).collect::<Vec<_>>();
        if match_rules.is_empty() {
            None
        } else {
            let mut res = None;
            for rule in match_rules {
                res = rule.service(&msg, &mut ctx, &self.client).await.unwrap_or(None);
                res = rule.handlers(&msg, &mut ctx, &self.client).unwrap_or(None);
            }
            res
        }
    }
}

/// 微信处理器
pub trait WechatCpMessageHandler where Self : Send + Sync {

    fn handle(&self, msg: &CpMessage, context: &Value, client: &WechatCpClientWrapper) -> Option<CpReply>;
}

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;


type Handler = Box<dyn Fn(Arc<WechatCpMessageContext>) -> BoxFuture<'static, Option<CpReply>> + Send + Sync>;


/// 微信处理路由规则
pub struct WechatCpMessageRouterRule {
    source: String,
    msg_type: String,
    event: String,
    event_key: String,
    content: String,
    agent_id: i64,
    handlers: Vec<Arc<Box<dyn WechatCpMessageHandler>>>,
    services: Vec<Handler>
}



impl WechatCpMessageRouterRule {
    pub fn new() -> Self {
        Self {
            source: "".to_string(),
            msg_type: "".to_string(),
            event: "".to_string(),
            event_key: "".to_string(),
            content: "".to_string(),
            agent_id: 0,
            handlers: vec![],
            services: vec![]
        }
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn event_key(mut self, event_key: String) -> Self {
        self.event_key = event_key;
        self
    }

    pub fn event(mut self, event: String) -> Self {
        self.event = event;
        self
    }

    pub fn msg_type(mut self, msg_type: String) -> Self {
        self.msg_type = msg_type;
        self
    }

    pub fn source(mut self, source: String) -> Self {
        self.source = source;
        self
    }

    pub fn route<H, Fut>(mut self, service: H) -> Self
        where
            H: Fn(Arc<WechatCpMessageContext>) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Option<CpReply>> + Send + 'static,
    {
        self.services.push(Box::new(move |ctx| Box::pin(service(ctx))));
        self
    }

    pub fn handlers(&self, msg: &CpMessage, ctx: &Value, client: &WechatCpClientWrapper) -> LabradorResult<Option<CpReply>> {
        let mut res = None;
        for handler in &self.handlers {
            res = handler.handle(msg, ctx, client);
        }
        Ok(res)
    }

    pub async fn service(&self, msg: &CpMessage, ctx: &Value, client: &WechatCpClientWrapper) -> LabradorResult<Option<CpReply>> {
        let mut res = None;
        for service in &self.services {
            res = service(Arc::new(WechatCpMessageContext { msg: msg.clone(), ctx: ctx.clone(), client: client.clone() })).await;
        }
        Ok(res)
    }


    pub fn test(&self, msg: &CpMessage) -> bool {
        (self.source.is_empty() || self.source.eq(&msg.get_source())) &&
            (self.agent_id == 0 || self.agent_id.eq(&msg.get_agent_id())) && (self.event_key.is_empty() || self.event_key.eq(&msg.get_event_key())) &&
            (self.event.is_empty() || self.event.eq(&msg.get_event()))
    }

    pub fn handler<T: WechatCpMessageHandler + 'static>(mut self, handler: T) -> Self {
        self.handlers.push( Arc::new(Box::new(handler)));
        self
    }


}


#[cfg(test)]
mod test {
    use std::sync::Arc;
    use crate::{CpReply, CpUnknownMessage, SimpleStorage, WechatCpClient, WechatCpMessageContext, WechatCpMessageRouter};
    use serde_json::Value;
    use crate::{CpMessage, WechatCpClientWrapper, WechatCpMessageHandler};

    pub struct A;

    impl WechatCpMessageHandler for A {

        fn handle(&self, msg: &CpMessage, context: &Value, client: &WechatCpClientWrapper) -> Option<CpReply> {
            println!("成功！");
            None
        }
    }

    pub struct B;

    impl WechatCpMessageHandler for B {

        fn handle(&self, msg: &CpMessage, context: &Value, client: &WechatCpClientWrapper) -> Option<CpReply> {
            println!("成功2！");
            None
        }
    }


    pub async fn test(msg: Arc<WechatCpMessageContext>) -> Option<CpReply> {
        println!("成功3！");
        None
    }


    #[test]
    fn test_route() {
        let mut router = WechatCpMessageRouter::from_client(WechatCpClient::<SimpleStorage>::new("",""));
        let rule = router.rule().handler(A).handler(B).route(test).route(test);
        router = router.push_rule(rule);
        let result = router.route(CpMessage::UnknownMessage(CpUnknownMessage {
            source: "".to_string(),
            target: "".to_string(),
            create_time: 0,
            id: None,
            agent_id: None,
            raw: None
        }), Value::Null);

    }
}
