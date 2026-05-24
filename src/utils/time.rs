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

use chrono::{DateTime, Duration, Utc};
use std::time::SystemTime;

/// 获取当前时间戳（秒）
pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}

/// 获取当前时间戳（毫秒）
pub fn timestamp_millis() -> i64 {
    Utc::now().timestamp_millis()
}

/// 获取当前时间戳（微秒）
pub fn timestamp_micros() -> i64 {
    Utc::now().timestamp_nanos() / 1000
}

/// 格式化时间为字符串
pub fn format_datetime(dt: DateTime<Utc>, format: &str) -> String {
    dt.format(format).to_string()
}

/// 解析时间字符串
pub fn parse_datetime(s: &str, format: &str) -> Option<DateTime<Utc>> {
    chrono::DateTime::parse_from_str(s, format)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

/// 检查时间是否过期
pub fn is_expired(timestamp: i64, ttl: Duration) -> bool {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    now > timestamp + ttl.num_seconds()
}

/// 计算过期时间
pub fn expiry_time(ttl: Duration) -> DateTime<Utc> {
    Utc::now() + ttl
}