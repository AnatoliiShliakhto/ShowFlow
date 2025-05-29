use crate::model::*;
use ::dioxus::prelude::*;
use ::std::{collections::HashMap, rc::Rc};

pub fn use_init_i18n() {
    use_context_provider(|| Rc::new(UseI18n::init()));
}

pub fn use_i18n() -> Rc<UseI18n> {
    consume_context::<Rc<UseI18n>>()
}

#[macro_export]
macro_rules! t {
    ($id:expr) => {{ $crate::service::use_i18n().translate($id) }};
}

pub struct UseI18n {
    langs: HashMap<String, HashMap<String, String>>,
    active: Signal<String>,
}

impl UseI18n {
    pub fn init() -> Self {
        let mut langs = HashMap::new();
        // let path = PathBuf::from(env::current_dir().unwrap()).join("/assets/i18n/");
        //
        // let Ok(mut folder) = fs::read_dir(path) else {
        //     return Self::default();
        // };
        //
        // while let Some(Ok(child)) = folder.next() {
        //     let Ok(meta) = child.metadata() else {
        //         continue;
        //     };
        //     if !meta.is_file() {
        //         continue;
        //     }
        //     let Ok(filename) = child.file_name().into_string() else {
        //         continue;
        //     };
        //
        //     let Some(lang) = filename.split(".ftl").next() else {
        //         continue;
        //     };
        //
        //     let data = fs::read_to_string(child.path()).unwrap_or_default();
        //     let parsed = parse_ftl(&data);
        //     langs.insert(lang.to_string(), parsed);
        // }

        let en = parse_ftl(include_str!("../../assets/i18n/en-US.ftl"));
        let ua = parse_ftl(include_str!("../../assets/i18n/uk-UA.ftl"));

        langs.insert("en-US".to_string(), en);
        langs.insert("uk-UA".to_string(), ua);

        let active = langs
            .keys()
            .next()
            .cloned()
            .unwrap_or_default();

        Self {
            langs,
            active: Signal::new(active),
        }
    }

    pub fn active(&self) -> Signal<String> {
        self.active
    }

    pub fn langs(&self) -> Vec<String> {
        self.langs.keys().cloned().collect()
    }

    pub fn change(&self, lang: &str) {
        let mut active = self.active;
        active.set(lang.to_string());
    }

    pub fn translate(&self, key: &str) -> String {
        let active = self.active;
        let Some(map) = self.langs.get(&active()).cloned() else {
            return key.to_string();
        };
        map.get(key).unwrap_or(&key.to_string()).to_string()
    }
}

fn parse_ftl(data: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'=')
        .has_headers(false)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(data.as_bytes());

    for record in reader.deserialize::<Pair>().flatten() {
        map.insert(record.key, record.value);
    }

    map
}
