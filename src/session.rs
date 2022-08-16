use std::{collections::BTreeMap, any::type_name, fmt, error, time::{SystemTime, UNIX_EPOCH}};

use redis::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

pub trait SessionStore: Clone {
    fn get<'a, K: AsRef<str>, T: FromStore>(&mut self, key: K, default: Option<T>) -> Option<T>;
    fn set<'a, K: AsRef<str>, T: ToStore>(&mut self, key: K, value: T, ttl: Option<usize>);
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
        // let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        let encoded = serde_json::to_string(&self).unwrap_or_default();
        out.write_arg(encoded.as_bytes())
    }
}

impl FromRedisValue for Store {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::Data(ref bytes) => {
                let data = String::from_utf8(bytes.to_vec()).unwrap_or_default();
                match serde_json::from_str::<Store>(&data) {
                    Ok(decoded) => Ok(decoded),
                    Err(_err) => {
                        // 出错了则直接返回该字符串
                        Ok(Store::String(data))
                    }
                }
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

#[derive(Debug, Clone)]
pub struct SimpleStorage {
    data: BTreeMap<String, (Option<usize>, Store)>
}

impl SimpleStorage {
    pub fn new() -> SimpleStorage {
        SimpleStorage { data: BTreeMap::new() }
    }
}

impl SessionStore for SimpleStorage {
    fn get<'a, K: AsRef<str>, T: FromStore>(&mut self, key: K, default: Option<T>) -> Option<T> {
        let key = key.as_ref();
        if let Some((ttl, value)) = self.data.get(&key.to_string()) {
            if let Some(ttl) =  ttl.to_owned() {
                let current_stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
                if current_stamp > ttl {
                    self.data.remove(key);
                    None
                } else {
                    Some(T::from_store(&value))
                }
            } else {
                Some(T::from_store(&value))
            }
        } else {
            default
        }
    }

    fn set<'a, K: AsRef<str>, T: ToStore>(&mut self, key: K, value: T, ttl: Option<usize>) {
        let key = key.as_ref();
        let ttl = if let Some(ttl) = ttl {
            let current_stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let ttl = current_stamp as usize + ttl;
            Some(ttl)
        } else {
            None
        };
        self.data.insert(key.to_string(), (ttl, T::to_store(&value)));
    }
}


pub mod redis_store {

    pub type RedisPool = Pool<redis::Client>;
    use std::ops::DerefMut;

    use r2d2::{Pool};
    use redis::{self, ToRedisArgs, ConnectionLike, Commands};

    use super::{SessionStore, ToStore, FromStore, Store};

    #[derive(Debug, Clone)]
    pub struct RedisStorage {
        client_pool: RedisPool
    }


    #[allow(unused)]
    impl RedisStorage {
        pub fn new(client: redis::Client) -> RedisStorage {
            let pool = Pool::builder().max_size(4).build(client).unwrap();
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
            let client = redis::Client::open(url.as_ref()).unwrap();
            let pool = Pool::builder().max_size(4).build(client).unwrap();
            RedisStorage {
                client_pool: pool,
            }
        }

        fn get_connect(&self) -> RedisPool {
            let pool = self.client_pool.to_owned();
            pool
        }

       

        fn del<K: AsRef<str>>(&self, key: K) {
            let mut client = self.client_pool.get().unwrap();
            if !client.check_connection() {
                return;
            }
            let conn = client.deref_mut();
            let _: () = redis::pipe().del(key.as_ref()).ignore().query(conn).unwrap_or(());
        }

        fn zlcount<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, min: T, max: T) -> Option<u32> {
            let mut client = self.client_pool.get().unwrap();
            if !client.check_connection() {
                return 0.into();
            }
            let conn = client.deref_mut();
            redis::pipe().zlexcount(key.as_ref(), min, max).ignore().query::<u32>(conn).unwrap_or(0).into()
        }

        fn zadd<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, member: T, score: T) -> Option<u32> {
            let mut client = self.client_pool.get().unwrap();
            if !client.check_connection() {
                return 0.into();
            }
            let conn = client.deref_mut();
            redis::pipe().zadd(key.as_ref(), member, score).ignore().query::<u32>(conn).unwrap_or(0).into()
        }
    }


    impl SessionStore for RedisStorage {
        
        fn get<'a, K: AsRef<str>, T: FromStore>(&mut self, key: K, default: Option<T>) -> Option<T> {
            let mut client = self.client_pool.get().unwrap();
            if !client.check_connection() {
                return default;
            }
            let data = client.get::<_, Store>(key.as_ref());
            if data.is_err() {
                return default;
            }
            if let Ok(value) = data {
                
                Some(T::from_store(&value))
            } else {
                default
            }
        }

        fn set<'a, K: AsRef<str>, T: ToStore>(&mut self, key: K, value: T, ttl: Option<usize>) {
            let mut client = self.client_pool.get().unwrap();
            let key = key.as_ref();
            if !client.check_connection() {
                return;
            }
            let conn = client.deref_mut();
            if let Some(seconds) = ttl {
                let _: () = redis::pipe().set_ex(key, value.to_store(), seconds).ignore().query(conn).unwrap();
            } else {
                let _: () = redis::pipe().set(key, value.to_store()).ignore().query(conn).unwrap();
            }
        }
    }
}
