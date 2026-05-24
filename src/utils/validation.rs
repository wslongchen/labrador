/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, SnackCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.snackcloud.cn developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: SnackCloud
 *  *
 *  
 */

/// 验证电子邮件
pub fn is_email(s: &str) -> bool {
    let email_regex = regex::Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    email_regex.is_match(s)
}

/// 验证手机号码（中国）
pub fn is_chinese_phone(s: &str) -> bool {
    let phone_regex = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
    phone_regex.is_match(s)
}

/// 验证身份证号码（中国）
pub fn is_chinese_id_card(s: &str) -> bool {
    let id_regex = regex::Regex::new(
        r"^[1-9]\d{5}(18|19|20)\d{2}(0[1-9]|1[0-2])(0[1-9]|[1-2]\d|3[0-1])\d{3}(\d|X|x)$"
    ).unwrap();
    id_regex.is_match(s)
}

/// 验证URL
pub fn is_url(s: &str) -> bool {
    let url_regex = regex::Regex::new(
        r"^https?://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(/.*)?$"
    ).unwrap();
    url_regex.is_match(s)
}

/// 验证IP地址
pub fn is_ip_address(s: &str) -> bool {
    let ipv4_regex = regex::Regex::new(
        r"^(\d{1,3}\.){3}\d{1,3}$"
    ).unwrap();

    if ipv4_regex.is_match(s) {
        s.split('.')
            .all(|part| part.parse::<u8>().is_ok())
    } else {
        false
    }
}

/// 验证是否为空或空白
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

/// 验证长度范围
pub fn length_in_range(s: &str, min: usize, max: usize) -> bool {
    let len = s.chars().count();
    len >= min && len <= max
}

/// 验证数字范围
pub fn number_in_range(num: i64, min: i64, max: i64) -> bool {
    num >= min && num <= max
}