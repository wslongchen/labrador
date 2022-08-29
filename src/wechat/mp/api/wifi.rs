use std::vec;

use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{session::SessionStore, request::{RequestType}, WechatCommonResponse, WeChatMpClient, LabradorResult};
use crate::wechat::mp::method::{MpWifiMethod, WechatMpMethod};

/// 微信连接WI-FI接口.
#[derive(Debug, Clone)]
pub struct WeChatMpWifi<'a, T: SessionStore> {
    client: &'a WeChatMpClient<T>,
}

#[allow(unused)]
impl<'a, T: SessionStore> WeChatMpWifi<'a, T> {

    #[inline]
    pub fn new(client: &WeChatMpClient<T>) -> WeChatMpWifi<T> {
        WeChatMpWifi {
            client,
        }
    }

    /// <pre>
    /// 获取Wi-Fi门店列表.
    /// 通过此接口获取WiFi的门店列表，该列表包括公众平台的门店信息、以及添加设备后的WiFi相关信息。创建门店方法请参考“微信门店接口”。
    /// 注：微信连Wi-Fi下的所有接口中的shop_id，必需先通过此接口获取。
    ///
    /// http请求方式: POST
    /// 请求URL：<a href="https://api.weixin.qq.com/bizwifi/shop/list?access_token=ACCESS_TOKEN">地址</a>
    /// </pre>
    pub async fn list_shop(&self, page_index: u32, page_size: u32) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
           "pageindex": page_index,
            "pagesize": page_size
        });
        self.client.post(WechatMpMethod::Wifi(MpWifiMethod::ShopList), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

    /// <pre>
    /// 查询门店Wi-Fi信息
    /// 通过此接口查询某一门店的详细Wi-Fi信息，包括门店内的设备类型、ssid、密码、设备数量、商家主页URL、顶部常驻入口文案。
    ///
    /// http请求方式: POST
    /// 请求URL：<a href="https://api.weixin.qq.com/bizwifi/shop/get?access_token=ACCESS_TOKEN">地址</a>
    /// POST数据格式：JSON
    /// </pre>
    pub async fn get_shop(&self, shop_id: u64) -> LabradorResult<WechatMpWifiShopDataResponse> {
        let req = json!({
           "shop_id": shop_id,
        });
        let v = self.client.post(WechatMpMethod::Wifi(MpWifiMethod::ShopList), vec![], req, RequestType::Json).await?.json::<Value>()?;
        WechatCommonResponse::parse::<WechatMpWifiShopDataResponse>(v)

    }

    /// <pre>
    /// 修改门店网络信息.
    /// 通过此接口修改门店的网络信息，包括网络名称（ssid）或密码。需注意：
    /// 只有门店下已添加Wi-Fi网络信息，才能调用此接口修改网络信息；添加方式请参考“添加密码型设备”和"添加portal型设备”接口文档。
    /// 网络信息修改后，密码型设备需同步修改所有设备的ssid或密码；portal型设备需修改所有设备的ssid，并按照《硬件鉴权协议接口》修改“第二步：改造移动端portal页面”中的ssid参数，否则将无法正常连网。
    /// 文档地址：<a href="https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1457435413">地址</a>
    /// </pre>
    pub async fn update_shop_wifi(&self, shop_id: u64, old_ssid: &str, ssid: &str, password: Option<&str>) -> LabradorResult<WechatCommonResponse> {
        let req = json!({
            "shop_id": shop_id,
            "old_ssid": old_ssid,
            "ssid": ssid,
            "password": password,
        });
        self.client.post(WechatMpMethod::Wifi(MpWifiMethod::UpdateShop), vec![], req, RequestType::Json).await?.json::<WechatCommonResponse>()
    }

}

//----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMpWifiShopDataResponse {
    /// 门店名称
    pub shop_name: Option<String>,
    /// 无线网络设备的ssid，未添加设备为空，多个ssid时显示第一个.
    pub ssid: Option<String>,
    /// 无线网络设备的ssid列表，返回数组格式.
    pub ssid_list: Option<Vec<String>>,
    /// ssid和密码的列表，数组格式。当为密码型设备时，密码才有值.
    pub ssid_password_list: Option<Vec<SsidPassword>>,
    /// 设备密码，当设备类型为密码型时返回.
    pub password: Option<String>,
    /// 门店内设备的设备类型，0-未添加设备，4-密码型设备，31-portal型设备.
    pub protocol_type: Option<u8>,
    /// 门店内设备总数
    pub ap_count: Option<i32>,
    /// 商家主页模板类型
    pub template_id: Option<i32>,
    /// 商家主页链接
    pub homepage_url: Option<String>,
    /// 顶部常驻入口上显示的文本内容：0--欢迎光临+公众号名称；1--欢迎光临+门店名称；2--已连接+公众号名称+WiFi；3--已连接+门店名称+Wi-Fi.
    pub bar_type: Option<u8>,
    /// 连网完成页链接
    pub finishpage_url: Option<String>,
    /// 商户自己的id，与门店poi_id对应关系，建议在添加门店时候建立关联关系，具体请参考“微信门店接口”.
    pub sid: Option<String>,
    /// 门店ID（适用于微信卡券、微信门店业务），具体定义参考微信门店，与shop_id一一对应.
    pub poi_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsidPassword {
    /// 无线网络设备的ssid
    pub ssid: Option<String>,
    /// 无线网络设备的password
    pub password: Option<String>,
}