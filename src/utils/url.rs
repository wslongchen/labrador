/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, WoofCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.woofcloud.com developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: WoofCloud
 *  *
 *
 */

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

/// URL编码
pub fn encode(s: &str) -> String {
    utf8_percent_encode(s, NON_ALPHANUMERIC).to_string()
}

/// 构建查询字符串
pub fn build_query(params: &[(String, String)]) -> String {
    let mut query = String::new();

    for (i, (key, value)) in params.iter().enumerate() {
        if i > 0 {
            query.push('&');
        }
        query.push_str(&format!("{}={}", encode(key), encode(value)));
    }

    query
}

/// 解析查询字符串
pub fn parse_query(query: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();

    for pair in query.split('&') {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            params.push((
                percent_encoding::percent_decode_str(key)
                    .decode_utf8_lossy()
                    .into_owned(),
                percent_encoding::percent_decode_str(value)
                    .decode_utf8_lossy()
                    .into_owned(),
            ));
        }
    }

    params
}

/// 合并URL和路径
pub fn join_url(base_url: &str, path: &str) -> String {
    let base = base_url.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{}/{}", base, path)
}
