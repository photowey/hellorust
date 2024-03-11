use std::{fmt, fs};
use std::collections::HashMap;

use chrono::NaiveDateTime;
use toml::Value;

mod v1;

enum EnvironmentValue {
    Nested(Box<dyn Environment>),
    String(String),
    Int64(i64),
    Int32(i32),
    Float64(f64),
    Float32(f32),
    DateTime(NaiveDateTime),
}

impl fmt::Debug for EnvironmentValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvironmentValue::Nested(_) => write!(f, "Nested"),
            EnvironmentValue::String(s) => write!(f, "{:?}", s),
            EnvironmentValue::Int64(i) => write!(f, "{:?}", i),
            EnvironmentValue::Int32(i) => write!(f, "{:?}", i),
            EnvironmentValue::Float64(fl) => write!(f, "{:?}", fl),
            EnvironmentValue::Float32(fl) => write!(f, "{:?}", fl),
            EnvironmentValue::DateTime(dt) => write!(f, "{:?}", dt),
        }
    }
}

trait Environment {
    fn set(&mut self, key: &str, value: EnvironmentValue) -> Result<(), &'static str>;
    fn get(&self, key: &str) -> Result<&EnvironmentValue, &'static str>;
}

struct StandardEnvironment {
    map: HashMap<String, EnvironmentValue>,
}

impl Environment for StandardEnvironment {
    fn set(&mut self, key: &str, value: EnvironmentValue) -> Result<(), &'static str> {
        let mut parts = key.split('.');
        let first = parts.next().ok_or("Empty key")?;

        if let Some(part) = parts.next() {
            let nested_env = self.map
                .entry(first.to_string())
                .or_insert_with(|| EnvironmentValue::Nested(Box::new(StandardEnvironment::new())));
            match nested_env {
                EnvironmentValue::Nested(env) => env.set(part, value),
                _ => Err("Attempted to set a nested value on a non-nested environment"),
            }
        } else {
            self.map.insert(first.to_string(), value);
            Ok(())
        }
    }

    fn get(&self, key: &str) -> Result<&EnvironmentValue, &'static str> {
        let mut parts = key.split('.');
        let first = parts.next().ok_or("Empty key")?;

        match self.map.get(first) {
            Some(EnvironmentValue::Nested(env)) => {
                let remaining_key = parts.collect::<Vec<_>>().join(".");
                env.get(&remaining_key)
            }
            Some(value) => Ok(value),
            None => Err("Key not found"),
        }
    }
}

impl StandardEnvironment {
    fn new() -> StandardEnvironment {
        StandardEnvironment {
            map: HashMap::new(),
        }
    }
}

impl From<String> for EnvironmentValue {
    fn from(value: String) -> Self {
        EnvironmentValue::String(value)
    }
}

impl From<&str> for EnvironmentValue {
    fn from(value: &str) -> Self {
        EnvironmentValue::String(value.to_string())
    }
}

impl From<i64> for EnvironmentValue {
    fn from(value: i64) -> Self {
        EnvironmentValue::Int64(value)
    }
}

impl From<i32> for EnvironmentValue {
    fn from(value: i32) -> Self {
        EnvironmentValue::Int32(value)
    }
}

impl From<f64> for EnvironmentValue {
    fn from(value: f64) -> Self {
        EnvironmentValue::Float64(value)
    }
}

impl From<f32> for EnvironmentValue {
    fn from(value: f32) -> Self {
        EnvironmentValue::Float32(value)
    }
}

impl From<NaiveDateTime> for EnvironmentValue {
    fn from(value: NaiveDateTime) -> Self {
        EnvironmentValue::DateTime(value)
    }
}

fn load_config(file_path: &str) -> HashMap<String, EnvironmentValue> {
    let config_str = fs::read_to_string(file_path).expect("Unable to read config file");
    let config: Value = config_str.parse().expect("Invalid TOML in config file");

    let mut config_map = HashMap::new();
    parse_toml_value(&config, &mut config_map);

    config_map
}

fn parse_toml_value(value: &Value, map: &mut HashMap<String, EnvironmentValue>) {
    match value {
        Value::Table(table) => {
            for (key, val) in table {
                parse_toml_value(val, map);
                map.insert(key.to_string(), EnvironmentValue::String(val.to_string()));
            }
        }
        _ => {
            todo!()
        }
    }
}

pub fn run() {
    let mut env = StandardEnvironment::new();
    env.set("A.B.C", "ValueABC".into()).unwrap();
    env.set("A.B.H", 123_i32.into()).unwrap();
    env.set("A.B.D", 123_i64.into()).unwrap();
    env.set("X.Y.U", 3.14_f32.into()).unwrap();
    env.set("X.Y.Z", 3.14_f64.into()).unwrap();
    env.set("X.Y.Time", NaiveDateTime::parse_from_str("2022-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().into()).unwrap();

    println!("A.B.H: {:?}", env.get("A.B.H"));
    println!("A.B.C: {:?}", env.get("A.B.C"));
    println!("A.B.D: {:?}", env.get("A.B.D"));
    println!("X.Y.U: {:?}", env.get("X.Y.U"));
    println!("X.Y.Z: {:?}", env.get("X.Y.Z"));
    println!("X.Y.Time: {:?}", env.get("X.Y.Time"));
}