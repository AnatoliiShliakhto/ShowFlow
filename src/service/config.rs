use ::serde::{Serialize, de::DeserializeOwned};
use ::serde_json::{Map, Value};
use ::std::path::PathBuf;
use ::std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct Config {
    path: PathBuf,
    inner: Arc<Mutex<Map<String, Value>>>,
}

impl Config {
    pub fn init() -> Self {
        let path = std::env::current_dir().unwrap().join("assets").join("config.json");
        Self {
            path: path.clone(),
            inner: Arc::new(Mutex::new(Self::load(path))),
        }
    }

    fn lock(&self) -> Map<String, Value> {
        self.inner.lock().unwrap().clone()
    }

    fn load(path: PathBuf) -> Map<String, Value> {
        let config_str = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&config_str).unwrap_or_default()
    }

    fn save(&self) {
        let config_str = serde_json::to_string_pretty(&self.lock()).unwrap_or_default();
        let _ = std::fs::write(&self.path, config_str);
    }

    pub fn get<T: DeserializeOwned + 'static>(&self, key: &str) -> Option<T> {
        let Some(value) = self.lock().get(key).map(<Value>::clone) else {
            return None;
        };
        serde_json::from_value(value).ok()
    }

    pub fn set<T: Serialize + 'static>(&self, key: &str, value: T) {
        let value = serde_json::to_value(value).unwrap_or_default();
        {
            self.inner.lock().unwrap().insert(key.to_string(), value);
        }
        self.save();
    }
}
