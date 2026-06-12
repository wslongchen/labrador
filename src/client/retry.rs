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
//! 重试机制实现

use crate::response::Response;
use std::time::Duration;

/// 重试策略
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RetryStrategy {
    /// 固定间隔重试
    Fixed(Duration),
    /// 指数退避重试
    Exponential {
        /// 初始延迟
        initial_delay: Duration,
        /// 退避乘数
        multiplier: f32,
        /// 最大延迟
        max_delay: Duration,
    },
    /// 随机退避重试
    Random {
        /// 最小延迟
        min_delay: Duration,
        /// 最大延迟
        max_delay: Duration,
    },
    /// 线性退避重试
    Linear {
        /// 初始延迟
        initial_delay: Duration,
        /// 每次增加的延迟
        increment: Duration,
        /// 最大延迟
        max_delay: Duration,
    },
    /// 立即重试
    Immediate,
}

impl Default for RetryStrategy {
    fn default() -> Self {
        RetryStrategy::Exponential {
            initial_delay: Duration::from_millis(100),
            multiplier: 2.0,
            max_delay: Duration::from_secs(10),
        }
    }
}

impl RetryStrategy {
    /// 计算第N次重试的延迟时间
    pub fn delay_for_attempt(&self, attempt: u64) -> Duration {
        match self {
            RetryStrategy::Fixed(delay) => *delay,
            RetryStrategy::Exponential {
                initial_delay,
                multiplier,
                max_delay,
            } => {
                let delay_ms =
                    initial_delay.as_millis() as f32 * multiplier.powi(attempt as i32 - 1);
                Duration::from_millis(delay_ms.min(max_delay.as_millis() as f32) as u64)
            }
            RetryStrategy::Random {
                min_delay,
                max_delay,
            } => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let min_ms = min_delay.as_millis() as u64;
                let max_ms = max_delay.as_millis() as u64;
                let inclusive: u64 = rng.gen_range(min_ms..max_ms);
                Duration::from_millis(inclusive)
            }
            RetryStrategy::Linear {
                initial_delay,
                increment,
                max_delay,
            } => {
                let total_increment = increment.as_millis() as u64 * (attempt - 1);
                let delay = *initial_delay + Duration::from_millis(total_increment);
                delay.min(*max_delay)
            }
            RetryStrategy::Immediate => Duration::from_millis(0),
        }
    }

    /// 获取策略描述
    pub fn description(&self) -> String {
        match self {
            RetryStrategy::Fixed(delay) => format!("固定间隔重试: {:?}", delay),
            RetryStrategy::Exponential {
                initial_delay,
                multiplier,
                max_delay,
            } => {
                format!(
                    "指数退避重试: 初始{:?}, 乘数{}, 最大{:?}",
                    initial_delay, multiplier, max_delay
                )
            }
            RetryStrategy::Random {
                min_delay,
                max_delay,
            } => {
                format!("随机退避重试: 范围{:?}-{:?}", min_delay, max_delay)
            }
            RetryStrategy::Linear {
                initial_delay,
                increment,
                max_delay,
            } => {
                format!(
                    "线性退避重试: 初始{:?}, 增量{:?}, 最大{:?}",
                    initial_delay, increment, max_delay
                )
            }
            RetryStrategy::Immediate => "立即重试".to_string(),
        }
    }
}

/// 重试条件
#[derive(Debug, Clone)]
pub struct RetryCondition {
    /// 可重试的状态码
    pub statuses: Vec<reqwest::StatusCode>,
    /// 可重试的错误类型
    pub error_patterns: Vec<String>,
    /// 最大重试次数
    pub max_attempts: u64,
    /// 是否启用抖动
    pub jitter: bool,
    /// 抖动因子 (0.0-1.0)
    pub jitter_factor: f32,
}

impl Default for RetryCondition {
    fn default() -> Self {
        Self {
            statuses: vec![
                reqwest::StatusCode::TOO_MANY_REQUESTS,
                reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                reqwest::StatusCode::BAD_GATEWAY,
                reqwest::StatusCode::SERVICE_UNAVAILABLE,
                reqwest::StatusCode::GATEWAY_TIMEOUT,
                reqwest::StatusCode::REQUEST_TIMEOUT,
            ],
            error_patterns: vec![
                "timeout".to_string(),
                "connection".to_string(),
                "network".to_string(),
                "temporary".to_string(),
            ],
            max_attempts: 3,
            jitter: true,
            jitter_factor: 0.1,
        }
    }
}

impl RetryCondition {
    /// 创建新的重试条件
    pub fn new() -> Self {
        Self::default()
    }

    /// 检查响应是否应该重试
    pub fn should_retry_response(&self, response: &Response) -> bool {
        self.statuses.contains(&response.status())
    }

    /// 检查错误是否应该重试
    pub fn should_retry_error(&self, error: &crate::errors::LabraError) -> bool {
        let error_str = error.to_string().to_lowercase();
        self.error_patterns
            .iter()
            .any(|pattern| error_str.contains(pattern))
    }

    /// 添加可重试的状态码
    pub fn add_status(mut self, status: reqwest::StatusCode) -> Self {
        self.statuses.push(status);
        self
    }

    /// 添加可重试的错误模式
    pub fn add_error_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.error_patterns.push(pattern.into());
        self
    }

    /// 设置最大重试次数
    pub fn max_attempts(mut self, max_attempts: u64) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// 启用/禁用抖动
    pub fn jitter(mut self, enabled: bool) -> Self {
        self.jitter = enabled;
        self
    }

    /// 设置抖动因子
    pub fn jitter_factor(mut self, factor: f32) -> Self {
        self.jitter_factor = factor.clamp(0.0, 1.0);
        self
    }

    /// 应用抖动到延迟时间
    pub fn apply_jitter(&self, delay: Duration) -> Duration {
        if !self.jitter || self.jitter_factor <= 0.0 {
            return delay;
        }

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let jitter_range = self.jitter_factor * delay.as_millis() as f32;
        let jitter = rng.gen_range(-jitter_range..jitter_range);
        let new_delay_ms = ((delay.as_millis() as f32) + jitter).max(0.0) as u64;

        Duration::from_millis(new_delay_ms)
    }
}

/// 智能重试器
pub struct SmartRetrier {
    /// 重试策略
    strategy: RetryStrategy,
    /// 重试条件
    condition: RetryCondition,
    /// 当前尝试次数
    current_attempt: u64,
    /// 最后延迟时间
    last_delay: Option<Duration>,
    /// 启用日志
    log_enabled: bool,
}

impl SmartRetrier {
    /// 创建新的智能重试器
    pub fn new(strategy: RetryStrategy, condition: RetryCondition) -> Self {
        Self {
            strategy,
            condition,
            current_attempt: 0,
            last_delay: None,
            log_enabled: true,
        }
    }

    /// 创建默认重试器
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self::new(RetryStrategy::default(), RetryCondition::default())
    }

    /// 启用/禁用日志
    pub fn log_enabled(mut self, enabled: bool) -> Self {
        self.log_enabled = enabled;
        self
    }

    /// 重置重试器状态
    pub fn reset(&mut self) {
        self.current_attempt = 0;
        self.last_delay = None;
    }

    /// 获取下次重试的延迟时间
    pub fn next_delay(&mut self) -> Duration {
        self.current_attempt += 1;
        let mut delay = self.strategy.delay_for_attempt(self.current_attempt);

        // 应用抖动
        delay = self.condition.apply_jitter(delay);

        self.last_delay = Some(delay);
        delay
    }

    /// 检查是否应该继续重试
    pub fn should_continue(&self) -> bool {
        self.current_attempt < self.condition.max_attempts
    }

    /// 获取当前尝试次数
    pub fn current_attempt(&self) -> u64 {
        self.current_attempt
    }

    /// 获取最大尝试次数
    pub fn max_attempts(&self) -> u64 {
        self.condition.max_attempts
    }

    /// 获取最后延迟时间
    pub fn last_delay(&self) -> Option<Duration> {
        self.last_delay
    }

    /// 执行带重试的操作
    pub async fn retry<F, T, E>(&mut self, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
        E: std::fmt::Display + std::fmt::Debug,
    {
        loop {
            match operation() {
                Ok(result) => {
                    if self.log_enabled && self.current_attempt > 0 {
                        tracing::info!("请求在第{}次尝试后成功", self.current_attempt);
                    }
                    return Ok(result);
                }
                Err(error) => {
                    let error_str = error.to_string();

                    // 检查是否应该重试
                    let should_retry = self
                        .condition
                        .error_patterns
                        .iter()
                        .any(|pattern| error_str.to_lowercase().contains(pattern));

                    if !should_retry || !self.should_continue() {
                        if self.log_enabled {
                            tracing::error!(
                                "请求失败且不重试: {} (尝试次数: {})",
                                error_str,
                                self.current_attempt
                            );
                        }
                        return Err(error);
                    }

                    // 计算延迟并等待
                    let delay = self.next_delay();

                    if self.log_enabled {
                        tracing::warn!(
                            "请求失败，将在{:?}后重试 (尝试次数: {}/{}, 错误: {})",
                            delay,
                            self.current_attempt,
                            self.condition.max_attempts,
                            error_str
                        );
                    }

                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    /// 执行带响应检查的重试操作
    pub async fn retry_with_response<F, T>(
        &mut self,
        mut operation: F,
    ) -> Result<T, crate::errors::LabraError>
    where
        F: FnMut() -> crate::errors::LabradorResult<Result<T, crate::errors::LabraError>>,
    {
        loop {
            match operation()? {
                Ok(result) => {
                    if self.log_enabled && self.current_attempt > 0 {
                        tracing::info!("请求在第{}次尝试后成功", self.current_attempt);
                    }
                    return Ok(result);
                }
                Err(error) => {
                    // 检查是否应该重试
                    if !self.condition.should_retry_error(&error) || !self.should_continue() {
                        if self.log_enabled {
                            tracing::error!(
                                "请求失败且不重试: {} (尝试次数: {})",
                                error,
                                self.current_attempt
                            );
                        }
                        return Err(error);
                    }

                    // 计算延迟并等待
                    let delay = self.next_delay();

                    if self.log_enabled {
                        tracing::warn!(
                            "请求失败，将在{:?}后重试 (尝试次数: {}/{}, 错误: {})",
                            delay,
                            self.current_attempt,
                            self.condition.max_attempts,
                            error
                        );
                    }

                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}

/// 重试管理器
pub struct RetryManager {
    /// 重试器集合
    retriers: std::collections::HashMap<String, SmartRetrier>,
}

impl RetryManager {
    /// 创建新的重试管理器
    pub fn new() -> Self {
        Self {
            retriers: std::collections::HashMap::new(),
        }
    }

    /// 获取或创建重试器
    pub fn get_or_create(&mut self, name: &str) -> &mut SmartRetrier {
        self.retriers
            .entry(name.to_string())
            .or_insert_with(SmartRetrier::default)
    }

    /// 获取重试器
    pub fn get(&mut self, name: &str) -> Option<&mut SmartRetrier> {
        self.retriers.get_mut(name)
    }

    /// 移除重试器
    pub fn remove(&mut self, name: &str) {
        self.retriers.remove(name);
    }

    /// 重置所有重试器
    pub fn reset_all(&mut self) {
        for retrier in self.retriers.values_mut() {
            retrier.reset();
        }
    }

    /// 重置指定重试器
    pub fn reset(&mut self, name: &str) {
        if let Some(retrier) = self.retriers.get_mut(name) {
            retrier.reset();
        }
    }
}

impl Default for RetryManager {
    fn default() -> Self {
        Self::new()
    }
}
