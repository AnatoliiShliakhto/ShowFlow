use crate::{model::*, service::*};
use ::dioxus::prelude::*;
use ::regex::Regex;
use ::std::{fs, path::PathBuf};

#[derive(Default, Clone)]
pub struct UseState {
    cfg: Signal<Config>,
    path: Signal<PathBuf>,
    files: Signal<Vec<Entry>>,
    queue: Signal<Vec<String>>,
}

impl UseState {
    pub fn pick_folder(&self) -> bool {
        let Some(folder) = rfd::FileDialog::new().set_directory("/").pick_folder() else {
            return false;
        };
        let mut path = self.path;
        let cfg = self.cfg;

        cfg().set("path", folder.to_str().unwrap_or_default().to_string());
        path.set(folder);

        true
    }

    pub fn refresh_files(&self) {
        let path = self.path;
        let mut files = self.files;

        let Ok(entries) = get_files(path()) else {
            return;
        };
        files.set(entries);
        self.load_playlist()
    }

    pub fn cfg(&self) -> Signal<Config> {
        self.cfg
    }

    pub fn path(&self) -> Signal<PathBuf> {
        self.path
    }

    pub fn files(&self) -> Signal<Vec<Entry>> {
        self.files
    }

    pub fn queue(&self) -> Signal<Vec<String>> {
        self.queue
    }

    pub fn clear_queue(&self) {
        let mut queue = self.queue;
        queue.set(vec![]);
    }

    pub fn add_to_queue(&self, name: &str) {
        let mut queue = self.queue;
        let mut updated = queue();
        updated.push(name.to_string());
        queue.set(updated);
    }

    pub fn remove_from_queue(&self, idx: usize) {
        let mut queue = self.queue;
        let mut updated = queue();
        updated.remove(idx);
        queue.set(updated);
    }

    pub fn queue_contains(&self, value: &str) -> bool {
        let queue = self.queue;
        queue().iter().any(|e| e == value)
    }

    pub fn open(&self, idx: usize) {
        let files = self.files;
        let queue = self.queue;
        let name = queue()[idx].clone();
        if let Some(path) = files()
            .iter()
            .find(|e| e.name == name)
            .map(|e| e.path.clone())
        {
            open(path)
        }
        self.remove_from_queue(idx);
    }

    pub fn current(&self) -> Current {
        let queue = self.queue;
        let files = self.files;
        if queue().is_empty() {
            return Current::default();
        }
        let name = queue()[0].clone();
        let Some(path) = files()
            .iter()
            .find(|e| e.name == name)
            .map(|e| e.path.clone())
        else {
            return Current::default();
        };

        Current {
            name,
            thumbnail: thumbnail(path),
        }
    }

    pub fn save_playlist(&self) {
        let path = self.path;
        if path().exists() {
            let path = path().join(".playlist");
            let queue = self.queue();
            let playlist = serde_json::to_string_pretty(&queue()).unwrap_or_default();
            let _ = fs::write(path, playlist);
        }
    }

    pub fn load_playlist(&self) {
        let path = self.path;
        let path = path().join(".playlist");
        let mut queue = self.queue;
        let files = self.files;
        if !path.exists() {
            queue.set(vec![]);
            return;
        }

        let data = fs::read_to_string(path).unwrap_or_default();
        let playlist_origin = serde_json::from_str::<Vec<String>>(&data).unwrap_or_default();
        let playlist = playlist_origin
            .into_iter()
            .filter(|e| files().iter().any(|f| f.name.eq(e)))
            .collect::<Vec<String>>();

        queue.set(playlist);
    }
}

pub fn use_init_state() {
    let cfg = Config::init();
    let path = PathBuf::from(cfg.get::<String>("path").unwrap_or_default());

    use_context_provider(|| UseState {
        cfg: Signal::new(cfg),
        path: Signal::new(path),
        files: Default::default(),
        queue: Default::default(),
    });
}

pub fn use_state() -> UseState {
    consume_context::<UseState>()
}

fn get_files(path: PathBuf) -> Result<Vec<Entry>, ()> {
    let mut list = vec![];

    let Ok(mut folder) = fs::read_dir(path) else {
        return Ok(list);
    };

    while let Some(Ok(child)) = folder.next() {
        let Ok(meta) = child.metadata() else {
            continue;
        };
        if !meta.is_file() {
            continue;
        }
        let Ok(filename) = child.file_name().into_string() else {
            continue;
        };

        if filename.starts_with('.') || filename.contains('~') {
            continue;
        }

        let Some(title) = filename.split('.').next() else {
            continue;
        };

        let entry = Entry {
            name: split_before_special_or_digit(title).to_string(),
            path: child.path(),
        };

        list.push(entry);
    }

    Ok(list)
}

fn split_before_special_or_digit(s: &str) -> &str {
    let re = Regex::new(r"[^\p{L}\s]|\d").unwrap();
    if let Some(m) = re.find(s) {
        &s[..m.start()]
    } else {
        s
    }
}
