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

//! 缓存工具

use moka::future::Cache as MokaCache;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// 缓存项
#[derive(Debug, Clone)]
pub struct CacheItem<T> {
    /// 缓存值
    pub value: T,
    /// 过期时间
    pub expires_at: Option<Instant>,
    /// 创建时间
    pub created_at: Instant,
    /// 最后访问时间
    pub last_accessed: Instant,
    /// 访问次数
    pub access_count: u64,
}

impl<T> CacheItem<T> {
    /// 创建新的缓存项
    pub fn new(value: T, ttl: Option<Duration>) -> Self {
        let now = Instant::now();
        Self {
            value,
            expires_at: ttl.map(|ttl| now + ttl),
            created_at: now,
            last_accessed: now,
            access_count: 0,
        }
    }

    /// 检查是否过期
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Instant::now() > expires_at
        } else {
            false
        }
    }

    /// 获取剩余生存时间
    pub fn ttl(&self) -> Option<Duration> {
        self.expires_at.map(|expires_at| {
            let now = Instant::now();
            if expires_at > now {
                expires_at.duration_since(now)
            } else {
                Duration::from_secs(0)
            }
        })
    }

    /// 获取值并更新访问时间
    pub fn get(&mut self) -> &T {
        self.last_accessed = Instant::now();
        self.access_count += 1;
        &self.value
    }
}

/// 缓存特质
pub trait Cache<K, V> {
    /// 获取缓存值
    fn get(&self, key: &K) -> Option<V>;

    /// 设置缓存值
    fn set(&mut self, key: K, value: V, ttl: Option<Duration>);

    /// 删除缓存值
    fn delete(&mut self, key: &K);

    /// 清空缓存
    fn clear(&mut self);

    /// 检查是否包含键
    fn contains_key(&self, key: &K) -> bool;

    /// 获取缓存大小
    fn len(&self) -> usize;

    /// 检查缓存是否为空
    fn is_empty(&self) -> bool;

    /// 获取所有键
    fn keys(&self) -> Vec<K>;

    /// 获取所有值
    fn values(&self) -> Vec<V>;

    /// 获取所有条目
    fn entries(&self) -> Vec<(K, V)>;
}

/// 内存缓存
pub struct MemoryCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    cache: HashMap<K, CacheItem<V>>,
    max_size: Option<usize>,
    default_ttl: Option<Duration>,
    hits: u64,
    misses: u64,
}

impl<K, V> MemoryCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// 创建新的内存缓存
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_size: None,
            default_ttl: None,
            hits: 0,
            misses: 0,
        }
    }

    /// 设置最大大小
    pub fn with_max_size(mut self, max_size: usize) -> Self {
        self.max_size = Some(max_size);
        self
    }

    /// 设置默认TTL
    pub fn with_default_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

    /// 清理过期条目
    pub fn cleanup(&mut self) {
        self.cache.retain(|_, item| !item.is_expired());
    }

    /// 获取命中率
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// 获取缓存统计
    pub fn stats(&self) -> CacheStats {
        let mut expired_count = 0;
        let mut total_ttl = Duration::from_secs(0);
        let mut items_with_ttl = 0;

        for item in self.cache.values() {
            if item.is_expired() {
                expired_count += 1;
            }
            if let Some(ttl) = item.ttl() {
                total_ttl += ttl;
                items_with_ttl += 1;
            }
        }

        CacheStats {
            size: self.cache.len(),
            hits: self.hits,
            misses: self.misses,
            expired_count,
            average_ttl: if items_with_ttl > 0 {
                Some(total_ttl / items_with_ttl)
            } else {
                None
            },
        }
    }
}

impl<K, V> Cache<K, V> for MemoryCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        if let Some(item) = self.cache.get(key) {
            if item.is_expired() {
                return None;
            }
            Some(item.value.clone())
        } else {
            None
        }
    }

    fn set(&mut self, key: K, value: V, ttl: Option<Duration>) {
        let ttl = ttl.or(self.default_ttl);
        let item = CacheItem::new(value, ttl);

        // 检查是否达到最大大小
        if let Some(max_size) = self.max_size {
            if self.cache.len() >= max_size && !self.cache.contains_key(&key) {
                // 移除最旧的项目（基于最后访问时间）
                let oldest_key = self
                    .cache
                    .iter()
                    .min_by_key(|(_, item)| item.last_accessed)
                    .map(|(k, _)| k.clone());

                if let Some(oldest_key) = oldest_key {
                    self.cache.remove(&oldest_key);
                }
            }
        }

        self.cache.insert(key, item);
    }

    fn delete(&mut self, key: &K) {
        self.cache.remove(key);
    }

    fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    fn contains_key(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }

    fn len(&self) -> usize {
        self.cache.len()
    }

    fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    fn keys(&self) -> Vec<K> {
        self.cache.keys().cloned().collect()
    }

    fn values(&self) -> Vec<V> {
        self.cache.values().map(|item| item.value.clone()).collect()
    }

    fn entries(&self) -> Vec<(K, V)> {
        self.cache
            .iter()
            .map(|(k, item)| (k.clone(), item.value.clone()))
            .collect()
    }
}

impl<K, V> Default for MemoryCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// 线程安全的内存缓存
pub struct SyncMemoryCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    cache: Arc<RwLock<MemoryCache<K, V>>>,
}

impl<K, V> SyncMemoryCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    /// 创建新的线程安全内存缓存
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(MemoryCache::new())),
        }
    }

    /// 设置最大大小
    pub fn with_max_size(self, max_size: usize) -> Self {
        let arc = self.cache.clone();
        let mut cache = arc.write().unwrap();
        *cache = MemoryCache::new().with_max_size(max_size);
        self
    }

    /// 设置默认TTL
    pub fn with_default_ttl(self, ttl: Duration) -> Self {
        let arc = self.cache.clone();
        let mut cache = arc.write().unwrap();
        *cache = MemoryCache::new().with_default_ttl(ttl);
        self
    }

    /// 获取缓存值
    pub fn get(&self, key: &K) -> Option<V> {
        self.cache.read().unwrap().get(key)
    }

    /// 设置缓存值
    pub fn set(&self, key: K, value: V, ttl: Option<Duration>) {
        self.cache.write().unwrap().set(key, value, ttl);
    }

    /// 删除缓存值
    pub fn delete(&self, key: &K) {
        self.cache.write().unwrap().delete(key);
    }

    /// 清空缓存
    pub fn clear(&self) {
        self.cache.write().unwrap().clear();
    }

    /// 检查是否包含键
    pub fn contains_key(&self, key: &K) -> bool {
        self.cache.read().unwrap().contains_key(key)
    }

    /// 获取缓存大小
    pub fn len(&self) -> usize {
        self.cache.read().unwrap().len()
    }

    /// 检查缓存是否为空
    pub fn is_empty(&self) -> bool {
        self.cache.read().unwrap().is_empty()
    }

    /// 清理过期条目
    pub fn cleanup(&self) {
        self.cache.write().unwrap().cleanup();
    }

    /// 获取缓存统计
    pub fn stats(&self) -> CacheStats {
        self.cache.read().unwrap().stats()
    }
}

impl<K, V> Default for SyncMemoryCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

/// 异步缓存（使用Moka）
pub struct AsyncCache<K, V>
where
    K: Eq + Hash + Send + Sync + Clone + 'static,
    V: Clone + Send + Sync + 'static,
{
    cache: MokaCache<K, V>,
}

impl<K, V> AsyncCache<K, V>
where
    K: Eq + Hash + Send + Sync + Clone + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// 创建新的异步缓存
    pub fn new(max_capacity: u64) -> Self {
        let cache = MokaCache::builder().max_capacity(max_capacity).build();
        Self { cache }
    }

    /// 创建带有TTL的异步缓存
    pub fn with_ttl(max_capacity: u64, ttl: Duration) -> Self {
        let cache = MokaCache::builder()
            .max_capacity(max_capacity)
            .time_to_live(ttl)
            .build();
        Self { cache }
    }

    /// 创建带有TTL和空闲时间的异步缓存
    pub fn with_ttl_and_idle(max_capacity: u64, ttl: Duration, idle: Duration) -> Self {
        let cache = MokaCache::builder()
            .max_capacity(max_capacity)
            .time_to_live(ttl)
            .time_to_idle(idle)
            .build();
        Self { cache }
    }

    /// 获取缓存值
    pub async fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).await
    }

    /// 设置缓存值
    pub async fn set(&self, key: K, value: V) {
        self.cache.insert(key, value).await;
    }

    /// 删除缓存值
    pub async fn delete(&self, key: &K) {
        self.cache.invalidate(key).await;
    }

    /// 清空缓存
    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }

    /// 检查是否包含键
    pub async fn contains_key(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }

    /// 获取缓存大小
    pub async fn len(&self) -> u64 {
        self.cache.entry_count()
    }

    /// 检查缓存是否为空
    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}

/// 缓存统计
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// 缓存大小
    pub size: usize,
    /// 命中次数
    pub hits: u64,
    /// 未命中次数
    pub misses: u64,
    /// 过期项目数量
    pub expired_count: usize,
    /// 平均TTL
    pub average_ttl: Option<Duration>,
}

impl CacheStats {
    /// 获取命中率
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

/// 缓存管理器
pub struct CacheManager {
    caches: Arc<Mutex<HashMap<String, Arc<dyn AnyCache>>>>,
}

impl CacheManager {
    /// 创建新的缓存管理器
    pub fn new() -> Self {
        Self {
            caches: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 注册缓存
    pub async fn register<K, V>(&self, name: &str, cache: impl AnyCache + 'static)
    where
        K: Eq + Hash + Send + Sync + Clone + 'static,
        V: Clone + Send + Sync + 'static,
    {
        let mut caches = self.caches.lock().await;
        caches.insert(name.to_string(), Arc::new(cache));
    }

    /// 获取缓存
    pub async fn get<K, V>(&self, name: &str) -> Option<Arc<dyn AnyCache>>
    where
        K: Eq + Hash + Send + Sync + Clone + 'static,
        V: Clone + Send + Sync + 'static,
    {
        let caches = self.caches.lock().await;
        caches.get(name).cloned()
    }

    /// 移除缓存
    pub async fn remove(&self, name: &str) {
        let mut caches = self.caches.lock().await;
        caches.remove(name);
    }

    /// 清空所有缓存
    pub async fn clear_all(&self) {
        let mut caches = self.caches.lock().await;
        caches.clear();
    }
}

/// 缓存特质（简化版）
pub trait AnyCache: Send + Sync {
    /// 清空缓存
    fn clear(&self);
}

impl<K, V> AnyCache for SyncMemoryCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn clear(&self) {
        self.clear();
    }
}
