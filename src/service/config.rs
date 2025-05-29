use ::serde::{Serialize, de::DeserializeOwned};
use ::serde_json::{Map, Value};
use ::std::{cell::RefCell, env, path::PathBuf};

pub struct Config {
    path: PathBuf,
    inner: RefCell<Map<String, Value>>,
}

impl Config {
    pub fn init() -> Self {
        let path = PathBuf::from(env::var("LOCALAPPDATA").expect("env var LOCALAPPDATA not found"))
            .join("ShowFlow")
            .join("config.json");
        let inner = RefCell::new(Self::load(path.clone()));

        Self { path, inner }
    }

    fn load(path: PathBuf) -> Map<String, Value> {
        let config_str = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&config_str).unwrap_or_default()
    }

    fn save(&self) {
        let config_str = serde_json::to_string_pretty(&*self.inner.borrow()).unwrap_or_default();
        let _ = std::fs::write(&self.path, config_str);
    }

    pub fn get<T: DeserializeOwned + 'static>(&self, key: &str) -> Option<T> {
        let value = self.inner.borrow().get(key).cloned()?;
        serde_json::from_value(value).ok()
    }

    pub fn set<T: Serialize + 'static>(&self, key: &str, value: T) {
        let value = serde_json::to_value(value).unwrap_or_default();
        {
            self.inner.borrow_mut().insert(key.to_string(), value);
        }
        self.save();
    }
}
