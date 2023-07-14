use redis::{Commands, RedisResult};
use serde::Serialize;
use serde_json::Value;

pub fn connect_to_db() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    Ok(con)
}

pub fn store_struct<T: Serialize>(
    prefix: &str,
    key: &str,
    value: &T,
    conn: &mut redis::Connection,
) -> redis::RedisResult<()> {
    let serialized = serde_json::to_string(value).unwrap();
    redis::cmd("SET")
        .arg(format!("{}:{}", prefix, key))
        .arg(serialized)
        .query(conn)?;
    Ok(())
}

pub fn get_struct<T: serde::de::DeserializeOwned>(
    key: &str,
    conn: &mut redis::Connection,
) -> redis::RedisResult<T> {
    let serialized: String = redis::cmd("GET").arg(key).query(conn)?;
    let deserialized: T = serde_json::from_str(&serialized).unwrap();
    Ok(deserialized)
}

pub fn get_struct_by_property<T, V>(
    prefix: &str,
    property: &str,
    value: V,
    conn: &mut redis::Connection,
) -> redis::RedisResult<Option<T>>
where
    T: serde::de::DeserializeOwned,
    V: serde::Serialize,
{
    let keys: RedisResult<Vec<String>> = conn.keys(format!("{}{}", prefix, "*"));

    let mut entity: Option<T> = None;

    for key in keys.unwrap() {
        let json: String = redis::cmd("GET").arg(key).query(conn).unwrap();
        let dynamic_json: Value = serde_json::from_str(&json).unwrap();

        if dynamic_json[property] == serde_json::to_value(&value).unwrap() {
            entity = serde_json::from_str(&json).unwrap();
            break;
        }
    }

    match entity {
        Some(entity) => Ok(Some(entity)),
        None => Ok(None),
    }
}
