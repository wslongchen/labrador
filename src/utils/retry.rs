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

use std::time::Duration;
use tokio::time;

/// 执行带重试的操作
pub async fn with_retry<F, T, E, Fut>(
    mut operation: F,
    max_retries: u32,
    initial_delay: Duration,
    backoff_factor: f32,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut attempts = 0;
    let mut delay = initial_delay;

    loop {
        attempts += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempts >= max_retries {
                    return Err(e);
                }

                // 等待延迟时间
                time::sleep(delay).await;

                // 增加延迟时间（指数退避）
                delay = Duration::from_millis((delay.as_millis() as f32 * backoff_factor) as u64);
            }
        }
    }
}

/// 执行带条件的重试
pub async fn with_conditional_retry<F, T, E, Fut, C>(
    mut operation: F,
    mut condition: C,
    max_retries: u32,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    C: FnMut(&E, u32) -> Option<Duration>,
{
    let mut attempts = 0;

    loop {
        attempts += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempts >= max_retries {
                    return Err(e);
                }

                if let Some(delay) = condition(&e, attempts) {
                    time::sleep(delay).await;
                } else {
                    return Err(e);
                }
            }
        }
    }
}
