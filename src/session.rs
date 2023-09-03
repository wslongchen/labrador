use std::{collections::BTreeMap, any::type_name, fmt, error};
use dashmap::DashMap;
use once_cell::sync::Lazy;

use redis::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use crate::{get_timestamp, LabradorResult};

pub trait SessionStore: Clone {
    fn get<'a, K: AsRef<str>, T: FromStore>(&self, key: K, default: Option<T>) -> LabradorResult<Option<T>>;
    fn set<'a, K: AsRef<str>, T: ToStore>(&self, key: K, value: T, ttl: Option<usize>) -> LabradorResult<()>;
}

pub trait ToStore {
    fn to_store(&self) -> Store;
}

pub trait FromStore: Sized {
    fn from_store(v: &Store) -> Self {
        match Self::from_store_opt(v) {
            Ok(x) => x,
            Err(_err) => panic!(
                "Couldn't from {:?} to type {}. (see FromStore documentation)",
                v,
                type_name::<Self>(),
            ),
        }
    }

    fn from_store_opt(v: &Store) -> Result<Self, StoreError>;
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Store {
    Json(serde_json::Value),
    Null, // no value
    Bool(bool),

    Number(Number),

    String(String),

    Object(BTreeMap<String, Store>),

    Array(Vec<Store>),
}


impl ToRedisArgs for Store {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap_or_default();
        // let encoded = serde_json::to_string(&self).unwrap_or_default();
        out.write_arg(&encoded[..])
    }
}

impl FromRedisValue for Store {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::Data(ref bytes) => {
                let data = bincode::deserialize::<Store>(bytes).unwrap_or(Store::Null);
                Ok(data)
            },
            redis::Value::Okay => Ok(Store::Null),
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Response was of incompatible type",
                format!("(response was {:?})", v),
            ))),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct Number {
    n: N,
}

#[derive(Copy, Clone, PartialEq, Deserialize, Serialize, Debug)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}

impl Eq for N {}

#[allow(unused)]
impl Number {
    #[inline]
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::PosInt(v) => v <= i64::max_value() as u64,
            N::NegInt(_) => true,
            N::Float(_) => false,
        }
    }

    #[inline]
    pub fn is_u64(&self) -> bool {
        match self.n {
            N::PosInt(_) => true,
            N::NegInt(_) | N::Float(_) => false,
        }
    }

    #[inline]
    pub fn is_f64(&self) -> bool {
        match self.n {
            N::Float(_) => true,
            N::PosInt(_) | N::NegInt(_) => false,
        }
    }

    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::PosInt(n) => {
                if n <= i64::max_value() as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::NegInt(n) => Some(n),
            N::Float(_) => None,
        }
    }

    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::PosInt(n) => Some(n),
            N::NegInt(_) | N::Float(_) => None,
        }
    }

    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::PosInt(n) => Some(n as f64),
            N::NegInt(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
    }

    #[inline]
    pub fn from_f64(f: f64) -> Option<Number> {
        if f.is_finite() {
            let n = {
                {
                    N::Float(f)
                }
            };
            Some(Number { n })
        } else {
            None
        }
    }
}


macro_rules! impl_to_store {
    ($ty:ty, $variant:ident) => {
        impl ToStore for $ty {
            fn to_store(&self) -> Store {
                Store::$variant(self.to_owned())
            }
        }
    };
}


macro_rules! impl_to_store_number {
    ($ty:ty, $variant:ident, $t_ty:ty) => {
        impl ToStore for $ty {
            fn to_store(&self) -> Store {
                let n = {
                    {
                        N::$variant(*self as $t_ty)
                    }
                };
                Store::Number(Number { n })
            }
        }
    };
}

impl_to_store_number!(i32, NegInt, i64);
impl_to_store_number!(f32, Float, f64);
impl_to_store_number!(f64, Float, f64);
impl_to_store_number!(i64, NegInt, i64);
impl_to_store_number!(i128, NegInt, i64);
impl_to_store_number!(u32, PosInt, u64);
impl_to_store_number!(u64, PosInt, u64);
impl_to_store_number!(u128, PosInt, u64);

impl_to_store!(String, String);
impl_to_store!(bool, Bool);
impl_to_store!(BTreeMap<String, Store>, Object);
impl_to_store!(serde_json::Value, Json);


impl <T> ToStore for Vec<T> 
where T: ToStore {
    fn to_store(&self) -> Store {
        Store::Array(self.iter().map(T::to_store).collect())
    }
}

impl <T> ToStore for &T 
where T: ToStore {
    fn to_store(&self) -> Store {
        T::to_store(&self)
    }
}
impl ToStore for &str {
    fn to_store(&self) -> Store {
        Store::String(self.to_string())
    }
}

impl <T> ToStore for Option<T> 
where T: ToStore {
    fn to_store(&self) -> Store {
        self.as_ref().map(|t| t.to_store()).unwrap_or(Store::Null)
    }
}

impl ToStore for Store {
    fn to_store(&self) -> Store {
        self.to_owned()
    }
}

macro_rules! impl_from_store_number {
    ($ty: ty) => {
        impl FromStore for $ty {
            fn from_store_opt(v: &Store) -> Result<Self, StoreError> {
                match v {
                    Store::Number(v) => {
                        Ok(match v.n {
                            N::PosInt(n) => n as $ty,
                            N::NegInt(n) => n as $ty,
                            N::Float(n) => n as $ty,
                        })
                    },
                    _ => Err(StoreError::NotSupported(format!("{:?}",v)))
                }
            }
        }
    }
}

impl_from_store_number!(u8);
impl_from_store_number!(u16);
impl_from_store_number!(u32);
impl_from_store_number!(u64);
impl_from_store_number!(i8);
impl_from_store_number!(i16);
impl_from_store_number!(i32);
impl_from_store_number!(i64);
impl_from_store_number!(isize);
impl_from_store_number!(usize);
impl_from_store_number!(f64);
impl_from_store_number!(f32);

macro_rules! impl_from_store {
    ($ty:ty, $variant:ident) => {
        impl FromStore for $ty {
            fn from_store_opt(v: &Store) -> Result<Self, StoreError> {
                match v {
                    Store::$variant(v) => Ok(v.to_owned()),
                    _ => Err(StoreError::NotSupported(format!("{:?}",v)))
                }
            }
        }
    };
}

impl_from_store!(String, String);
impl_from_store!(bool, Bool);
impl_from_store!(BTreeMap<String, Store>, Object);
impl_from_store!(serde_json::Value, Json);

impl <T> FromStore for Vec<T> 
where T: FromStore {
    fn from_store_opt(v: &Store) -> Result<Self, StoreError> {
        match v {
            Store::Array(v) => Ok(v.iter().map(T::from_store).collect()),
            _ => Err(StoreError::NotSupported(format!("{:?}",v)))
        }
    }
}

impl <T> FromStore for Option<T> 
where T: FromStore {
    fn from_store_opt(v: &Store) -> Result<Self, StoreError> {
        match *v {
            Store::Null => Ok(None),
            _ => FromStore::from_store_opt(v).map(Some),
        }
    }
}

impl FromStore for Store {
    fn from_store_opt(v: &Store) -> Result<Self, StoreError> {
        Ok(v.to_owned())
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum StoreError {
    NotSupported(String),
    Unknown,
}


impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StoreError::NotSupported(ref err) => write!(f, "NotSupported Store Error message: {}", err),
            StoreError::Unknown => write!(f, "Unknown Error"),
        }
    }
}

impl error::Error for StoreError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        match self {
            StoreError::NotSupported(ref err) => err,
            StoreError::Unknown => "Unknown Error",
        }
    }
}

pub static SIMPLE_STORAGE: Lazy<DashMap<String, (Option<usize>, Store)>> = Lazy::new(|| {
    DashMap::new()
});

#[derive(Debug, Clone)]
pub struct SimpleStorage {
}

impl SimpleStorage {
    pub fn new() -> SimpleStorage {
        SimpleStorage {  }
    }
}

impl SessionStore for SimpleStorage {
    fn get<'a, K: AsRef<str>, T: FromStore>(&self, key: K, default: Option<T>) -> LabradorResult<Option<T>> {
        let mut is_expire = false;
        let key = key.as_ref();
        let v = if let Some(v) = SIMPLE_STORAGE.get(&key.to_string()) {
            let (ttl, value) = v.value();
            if let Some(ttl) =  ttl {
                let current_stamp = get_timestamp() as usize;
                let exipre_at = current_stamp + *ttl;
                if current_stamp >= exipre_at {
                    // SIMPLE_STORAGE.remove(key);
                    is_expire = true;
                    None
                } else {
                    Some(T::from_store(&value))
                }
            } else {
                Some(T::from_store(&value))
            }
        } else {
            default
        };
        if is_expire {
            SIMPLE_STORAGE.remove(key);
        }
        Ok(v)
    }

    fn set<'a, K: AsRef<str>, T: ToStore>(&self, key: K, value: T, ttl: Option<usize>) -> LabradorResult<()> {
        let key = key.as_ref();
        let ttl = if let Some(ttl) = ttl {
            Some(ttl)
        } else {
            None
        };
        SIMPLE_STORAGE.insert(key.to_string(), (ttl, T::to_store(&value)));
        Ok(())
    }
}


pub mod redis_store {

    pub type RedisPool = Pool<redis::Client>;
    use r2d2::{Pool};
    use redis::{self, ToRedisArgs, ConnectionLike, Commands, FromRedisValue, streams};
    use crate::{LabradorResult, LabraError};

    use super::{SessionStore, ToStore, FromStore, Store};

    #[derive(Debug, Clone)]
    pub struct RedisStorage {
        client_pool: RedisPool
    }


    #[allow(unused)]
    impl RedisStorage {
        pub fn new(client: redis::Client) -> RedisStorage {
            let pool = Pool::builder().max_size(4).build(client).expect("can not get the redis client");
            RedisStorage {
                client_pool: pool,
            }
        }

        pub fn from_pool(client: Pool<redis::Client>) -> RedisStorage {
            RedisStorage {
                client_pool: client,
            }
        }

        pub fn from_url<U: AsRef<str>>(url: U) -> RedisStorage {
            let client = redis::Client::open(url.as_ref()).expect("can not get the redis pool");
            let pool = Pool::builder().max_size(4).build(client).expect("can not get the redis pool");
            RedisStorage {
                client_pool: pool,
            }
        }

        fn get_connect(&self) -> RedisPool {
            let pool = self.client_pool.to_owned();
            pool
        }

       

        pub fn del<K: AsRef<str>>(&self, key: K) -> LabradorResult<()> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            let s = client.del(key.as_ref())?;
            Ok(())
        }

        pub fn zlcount<K: AsRef<str>, M: ToRedisArgs, MM: ToRedisArgs, RV: FromRedisValue>(&self, key: K, min: M, max: MM) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zcount(key.as_ref(), min, max).map_err(LabraError::from)
        }

        /// 在zsetname集合中增加序号为n的value
        pub fn zadd<K: AsRef<str>, S: ToRedisArgs, M: ToRedisArgs, RV: FromRedisValue>(&self, key: K, member: M, score: S) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zadd(key.as_ref(), member, score).map_err(LabraError::from)
        }

        /// 排序指定的rank(排名)范围内的元素并输出
        pub fn zrange<K: AsRef<str>, RV: FromRedisValue>(&self, key: K, start: isize, stop: isize) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zrange(key.as_ref(), start, stop).map_err(LabraError::from)
        }

        pub fn zadd_multiple<K: AsRef<str>, S: ToRedisArgs, M: ToRedisArgs, RV: FromRedisValue>(&self, key: K, items: &[(S, M)]) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zadd_multiple(key.as_ref(), items).map_err(LabraError::from)
        }

        /// 反向排序
        pub fn zrevrange<K: AsRef<str>, RV: FromRedisValue>(&self, key: K, start: isize, stop: isize) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zrevrange(key.as_ref(), start, stop).map_err(LabraError::from)
        }

        /// 获取指定的score范围内的元素
        pub fn zrangebyscore<K: AsRef<str>, M: ToRedisArgs, MM: ToRedisArgs, RV: FromRedisValue>(&self, key: K, min: M, max: MM) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zrangebyscore(key.as_ref(), min, max).map_err(LabraError::from)
        }

        /// 获取a<=x<=b范围的数据，如果分页加上limit
        pub fn zrangebylex<K: AsRef<str>, M: ToRedisArgs, MM: ToRedisArgs, RV: FromRedisValue>(&self, key: K, min: M, max: MM) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zrangebylex(key.as_ref(), min, max).map_err(LabraError::from)
        }

        /// 为score累加n，新元素score基数为0
        pub fn zincr<K: AsRef<str>, M: ToRedisArgs, D: ToRedisArgs, RV: FromRedisValue>(&self, key: K, member: M, delta: D) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zincr(key.as_ref(), member, delta).map_err(LabraError::from)
        }

        /// 删除zsetname集合中指定的元素
        pub fn zrem<K: AsRef<str>, M: ToRedisArgs, RV: FromRedisValue>(&self, key: K, members: M) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zrem(key.as_ref(), members).map_err(LabraError::from)
        }

        /// 获取zsetname集合的元素个数
        pub fn zcard<K: AsRef<str>, RV: FromRedisValue>(&self, key: K) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zcard(key.as_ref()).map_err(LabraError::from)
        }

        /// 删除下标在start end 范围内的元素
        pub fn zremrangebyrank<K: AsRef<str>, RV: FromRedisValue>(&self, key: K, start: isize, stop: isize) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zremrangebyrank(key.as_ref(), start, stop).map_err(LabraError::from)
        }

        /// 命令判断成员元素是否是集合的成员
        pub fn sismember<K: AsRef<str>,M: ToRedisArgs, RV: FromRedisValue>(&self, key: K, member: M) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.sismember(key.as_ref(), member).map_err(LabraError::from)
        }

        /// 成员元素添加到集合中
        pub fn sadd<K: AsRef<str>,M: ToRedisArgs, RV: FromRedisValue>(&self, key: K, member: M) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.sadd(key.as_ref(), member).map_err(LabraError::from)
        }

        /// 删除score在[min [max 范围内的元素
        pub fn zrembyscore<K: AsRef<str>, M: ToRedisArgs, MM: ToRedisArgs, RV: FromRedisValue>(&self, key: K, min: M, max: MM) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zrembyscore(key.as_ref(), min, max).map_err(LabraError::from)
        }

        pub fn zrembylex<K: AsRef<str>, M: ToRedisArgs, MM: ToRedisArgs, RV: FromRedisValue>(&self, key: K, min: M, max: MM) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zrembylex(key.as_ref(), min, max).map_err(LabraError::from)
        }

        /// 查询指定value的排名，注意不是score
        pub fn zrank<K: AsRef<str>, M: ToRedisArgs, RV: FromRedisValue>(&self, key: K, member: M) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.zrank(key.as_ref(), member).map_err(LabraError::from)
        }

        pub fn xadd<K: AsRef<str>,  F: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(&self, key: K, items: &[(F, V)]) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xadd(key.as_ref(), "*", items).map_err(LabraError::from)
        }

        pub fn xadd_map<K: AsRef<str>,  BTM: ToRedisArgs, RV: FromRedisValue>(&self, key: K, items: BTM) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xadd_map(key.as_ref(), "*", items).map_err(LabraError::from)
        }

        pub fn xread<'a, K: ToRedisArgs,  ID: ToRedisArgs, RV: FromRedisValue>(&self, keys: &'a [K], ids: &'a [ID]) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xread(keys, ids).map_err(LabraError::from)
        }

        pub fn xinfo_consumers<'a, K: ToRedisArgs,  G: ToRedisArgs, RV: FromRedisValue>(&self, key: K, group: G) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xinfo_consumers(key, group).map_err(LabraError::from)
        }

        pub fn xinfo_groups<'a, K: ToRedisArgs, RV: FromRedisValue>(&self, key: K) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xinfo_groups(key).map_err(LabraError::from)
        }

        pub fn xinfo_stream<'a, K: ToRedisArgs, RV: FromRedisValue>(&self, key: K) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xinfo_stream(key).map_err(LabraError::from)
        }

        pub fn xread_options<'a, K: ToRedisArgs,  ID: ToRedisArgs, RV: FromRedisValue>(&self, keys: &'a [K], ids: &'a [ID], options: &'a streams::StreamReadOptions) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xread_options(keys, ids, options).map_err(LabraError::from)
        }

        pub fn xgroup_create<'a, K: ToRedisArgs, G: ToRedisArgs,  ID: ToRedisArgs, RV: FromRedisValue>(&self, key: K, group: G, id: ID) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xgroup_create(key, group, id).map_err(LabraError::from)
        }

        pub fn xgroup_delconsumer<'a, K: ToRedisArgs, G: ToRedisArgs,  C: ToRedisArgs, RV: FromRedisValue>(&self, key: K, group: G, consumer: C) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xgroup_delconsumer(key, group, consumer).map_err(LabraError::from)
        }

        pub fn xack<'a, K: ToRedisArgs, G: ToRedisArgs,  I: ToRedisArgs, RV: FromRedisValue>(&self, key: K, group: G, ids: &'a [I]) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xack(key, group, ids).map_err(LabraError::from)
        }

        pub fn xgroup_create_mkstream<'a, K: ToRedisArgs, G: ToRedisArgs, ID: ToRedisArgs, RV: FromRedisValue>(&self, key: K, group: G, id: ID) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xgroup_create_mkstream(key, group, id).map_err(LabraError::from)
        }

        pub fn xgroup_destroy<'a, K: ToRedisArgs, G: ToRedisArgs, RV: FromRedisValue>(&self, key: K, group: G) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xgroup_destroy(key, group).map_err(LabraError::from)
        }

        pub fn xdel<'a, K: ToRedisArgs, ID: ToRedisArgs, RV: FromRedisValue>(&self, key: K, ids: &'a [ID]) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xdel(key, ids).map_err(LabraError::from)
        }

        pub fn xpending<'a, K: ToRedisArgs, G: ToRedisArgs, RV: FromRedisValue>(&self, key: K, group: G) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xpending(key, group).map_err(LabraError::from)
        }

        pub fn xpending_count<'a, K: ToRedisArgs, G: ToRedisArgs, S: ToRedisArgs, E: ToRedisArgs, C: ToRedisArgs, RV: FromRedisValue>(&self, key: K, group: G, start: S, end: E, count: C) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xpending_count(key, group, start, end, count).map_err(LabraError::from)
        }

        pub fn xpending_consumer_count<'a, K: ToRedisArgs, G: ToRedisArgs, S: ToRedisArgs, E: ToRedisArgs, C: ToRedisArgs, CN: ToRedisArgs, RV: FromRedisValue>(&self, key: K, group: G, start: S, end: E, count: C, consumer: CN) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xpending_consumer_count(key, group, start, end, count, consumer).map_err(LabraError::from)
        }

        pub fn xrevrange<'a, K: ToRedisArgs, E: ToRedisArgs, S: ToRedisArgs, RV: FromRedisValue>(&self, key: K, start: S, end: E) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xrevrange(key, end, start).map_err(LabraError::from)
        }

        pub fn xrevrange_all<'a, K: ToRedisArgs, RV: FromRedisValue>(&self, key: K) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xrevrange_all(key).map_err(LabraError::from)
        }

        pub fn xrevrange_count<'a, K: ToRedisArgs, E: ToRedisArgs, S: ToRedisArgs, C: ToRedisArgs,RV: FromRedisValue>(&self, key: K, start: S, end: E, count: C) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.xrevrange_count(key, end, start, count).map_err(LabraError::from)
        }

        pub fn exists<'a, K: ToRedisArgs,RV: FromRedisValue>(&self, key: K) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.exists(key).map_err(LabraError::from)
        }

        pub fn expire<'a, K: ToRedisArgs,RV: FromRedisValue>(&self, key: K, seconds: usize) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.expire(key, seconds).map_err(LabraError::from)
        }

        pub fn expire_at<'a, K: ToRedisArgs,RV: FromRedisValue>(&self, key: K, ts: usize) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.expire_at(key, ts).map_err(LabraError::from)
        }

        pub fn lpush<'a, K: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(&self, key: K, value: V) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.lpush(key, value).map_err(LabraError::from)
        }

        pub fn lpush_exists<'a, K: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(&self, key: K, value: V) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.lpush_exists(key, value).map_err(LabraError::from)
        }

        /// 移出并获取列表的第一个元素， 如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止。
        pub fn blpop<'a, K: ToRedisArgs, RV: FromRedisValue>(&self, key: K, timeout: usize) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.blpop(key, timeout).map_err(LabraError::from)
        }

        /// 移出并获取列表的最后一个元素， 如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止。
        pub fn brpop<'a, K: ToRedisArgs, RV: FromRedisValue>(&self, key: K, timeout: usize) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.brpop(key, timeout).map_err(LabraError::from)
        }

        /// 移出并获取列表的第一个元素
        pub fn lpop<'a, K: ToRedisArgs, RV: FromRedisValue>(&self, key: K, count: Option<core::num::NonZeroUsize>) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.lpop(key, count).map_err(LabraError::from)
        }

        /// 通过索引获取列表中的元素
        pub fn lindex<'a, K: ToRedisArgs, RV: FromRedisValue>(&self, key: K, index: isize) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.lindex(key, index).map_err(LabraError::from)
        }

        /// 获取列表长度
        pub fn llen<'a, K: ToRedisArgs, RV: FromRedisValue>(&self, key: K) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.llen(key).map_err(LabraError::from)
        }

        /// 获取列表指定范围内的元素
        pub fn lrange<'a, K: ToRedisArgs, RV: FromRedisValue>(&self, key: K, start: isize, stop: isize) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.lrange(key, start, stop).map_err(LabraError::from)
        }

        /// 在列表中添加一个或多个值
        pub fn rpush<'a, K: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(&self, key: K, value: V) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.rpush(key, value).map_err(LabraError::from)
        }

        /// 在列表中添加一个或多个值
        pub fn rpush_exists<'a, K: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(&self, key: K, value: V) -> LabradorResult<RV> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            client.rpush_exists(key, value).map_err(LabraError::from)
        }
    }


    impl SessionStore for RedisStorage {
        
        fn get<'a, K: AsRef<str>, T: FromStore>(&self, key: K, default: Option<T>) -> LabradorResult<Option<T>> {
            let mut client = self.client_pool.get()?;
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            let data = client.get::<_, Store>(key.as_ref());
            if data.is_err() {
                return Ok(default);
            }
            let v = if let Ok(value) = data {
                match T::from_store_opt(&value) {
                    Ok(store) =>Some(store),
                    Err(_err) => None
                }
            } else {
                default
            };
            Ok(v)
        }

        fn set<'a, K: AsRef<str>, T: ToStore>(&self, key: K, value: T, ttl: Option<usize>) -> LabradorResult<()> {
            let mut client = self.client_pool.get()?;
            let key = key.as_ref();
            if !client.check_connection() {
                return Err(LabraError::ApiError("error to get redis connection".to_string()))
            }
            if let Some(seconds) = ttl {
                let _ = client.set_ex(key, value.to_store(), seconds)?;
            } else {
                let _ = client.set(key, value.to_store())?;
            }

            Ok(())
        }
    }
}


#[test]
fn test_simple() {
    println!("ssssssss");
    let encoded: Vec<u8> = bincode::serialize(&Store::String("234".to_string())).unwrap();
    let decode = bincode::deserialize::<Store>(&encoded).unwrap();
    println!("decode:{:?}", decode);
    // let session = SimpleStorage::new();
    // println!("000000");
    // let s  = session.set("a", "n", Some(0)).unwrap();
    // println!("1111");
    // let v = session.get::<&str, String>("a", None).unwrap();
    //
    // println!("v:{}" , v.unwrap_or_default());
}