//! 常量类

pub static GRANT_TYPE: &str = "grant_type";
pub static JS_CODE: &str = "js_code";
pub static CODE: &str = "code";
pub static AGENTID: &str = "agentid";
pub static CLIENT_CREDENTIAL: &str = "client_credential";
pub static AUTHORIZATION_CODE: &str = "authorization_code";
pub static CORPID: &str = "corpid";
pub static CORPSECRET: &str = "corpsecret";
pub static OPENID: &str = "openid";
pub static LANG: &str = "lang";
pub static ZH_CN: &str = "zh_CN";
pub static SECRET: &str = "secret";
pub static ACCESS_TOKEN: &str = "access_token";
pub static PROVIDER_ACCESS_TOKEN: &str = "provider_access_token";
pub static REFRESH_TOKEN: &str = "refresh_token";
pub static EXTERNAL_USERID: &str = "external_userid";
pub static CURSOR: &str = "cursor";
pub static USERID: &str = "userid";
pub static MEDIA_TYPE: &str = "media_type";
pub static TYPE: &str = "type";
pub static ATTACHMENT_TYPE: &str = "attachment_type";
pub static AUTH_URL_INSTALL: &str = "https://open.work.weixin.qq.com/3rdapp/install";

pub static ACCESS_TOKEN_KEY: &str = ":accessTokenKey:";

/**
 * 不弹出授权页面，直接跳转，只能获取用户openid.
 */
pub static SNSAPI_BASE: &str = "snsapi_base";

/**
 * 弹出授权页面，可通过openid拿到昵称、性别、所在地。并且，即使在未关注的情况下，只要用户授权，也能获取其信息.
 */
pub static SNSAPI_USERINFO: &str = "snsapi_userinfo";

/**
 * 手动授权,可获取成员的详细信息,包含手机、邮箱。只适用于企业微信或企业号.
 */
pub static SNSAPI_PRIVATEINFO: &str = "snsapi_privateinfo";


/**
 * 图片消息.
 */
pub static WELCOME_MSG_TYPE_IMAGE: &str = "image";
/**
 * 图文消息.
 */
pub static WELCOME_MSG_TYPE_LINK: &str = "link";
/**
 * 视频消息.
 */
pub static WELCOME_MSG_TYPE_VIDEO: &str = "video";
/**
 * 小程序消息.
 */
pub static WELCOME_MSG_TYPE_MINIPROGRAM: &str = "miniprogram";

/**
 * 文件消息.
 */
pub static WELCOME_MSG_TYPE_FILE: &str = "file";


/**
 * 文本消息.
 */
pub static GROUP_ROBOT_MSG_TEXT: &str = "text";

/**
 * 图片消息.
 */
pub static GROUP_ROBOT_MSG_IMAGE: &str = "image";

/**
 * markdown消息.
 */
pub static GROUP_ROBOT_MSG_MARKDOWN: &str = "markdown";

/**
 * 图文消息（点击跳转到外链）.
 */
pub static GROUP_ROBOT_MSG_NEWS: &str = "news";

/**
 * 文件类型消息.
 */
pub static GROUP_ROBOT_MSG_FILE: &str = "file";