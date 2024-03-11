use std::collections::HashMap;
use std::fmt;

use chrono::NaiveDateTime;

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
        let first = parts.next().ok_or("Key cannot be empty")?;
        if let Some(val) = parts.next() {
            let nested_env = self.map.entry(first.to_string()).or_insert_with(|| EnvironmentValue::Nested(Box::new(StandardEnvironment::new())));
            match nested_env {
                EnvironmentValue::Nested(env) => env.set(val, value),
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
        match self.map.get(first).ok_or("Key not found")? {
            EnvironmentValue::Nested(env) => {
                let remaining_key = parts.collect::<Vec<_>>().join(".");
                env.get(&remaining_key)
            }
            value => Ok(value),
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

pub fn run() {
    let mut env = StandardEnvironment::new();
    env.set("A.B.C", EnvironmentValue::String("ValueABC".to_string())).unwrap();
    env.set("A.B.D", EnvironmentValue::Int64(123)).unwrap();
    env.set("X.Y.Z", EnvironmentValue::Float64(3.14)).unwrap();
    env.set("X.Y.Time", EnvironmentValue::DateTime(NaiveDateTime::parse_from_str("2022-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap())).unwrap();

    match env.get("A.B.C") {
        Ok(value) => println!("A.B.C: {:?}", value),
        Err(err) => println!("Error getting A.B.C: {}", err),
    }

    match env.get("A.B.D") {
        Ok(value) => println!("A.B.D: {:?}", value),
        Err(err) => println!("Error getting A.B.D: {}", err),
    }

    match env.get("X.Y.Z") {
        Ok(value) => println!("X.Y.Z: {:?}", value),
        Err(err) => println!("Error getting X.Y.Z: {}", err),
    }

    match env.get("X.Y.Time") {
        Ok(value) => println!("X.Y.Time: {:?}", value),
        Err(err) => println!("Error getting X.Y.Time: {}", err),
    }
}
