use crate::{model::*, service::*};
use ::dioxus::prelude::*;
use ::regex::Regex;
use ::std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

pub fn use_init_state() -> Rc<UseState> {
    let cfg = Rc::new(Config::init());
    let path = Rc::new(RefCell::new(cfg.get("path").unwrap_or_default()));
    let files = GlobalSignal::new(Vec::new);
    let queue = GlobalSignal::new(Vec::new);
    
    let app_state = Rc::new(UseState {
        cfg,
        path,
        files,
        queue,
    });
    
    use_context_provider(|| app_state)
}

pub fn use_state() -> Rc<UseState> {
    consume_context::<Rc<UseState>>()
}

pub struct UseState {
    cfg: Rc<Config>,
    path: Rc<RefCell<String>>,
    files: GlobalSignal<Vec<Entry>>,
    queue: GlobalSignal<Vec<String>>,
}

impl UseState {
    pub fn path(&self) -> String {
        self.path.borrow().clone()
    }

    pub fn cfg(&self) -> Rc<Config> {
        self.cfg.clone()
    }

    pub fn queue(&self) -> Signal<Vec<String>> {
        self.queue.resolve()
    }

    pub fn files(&self) -> Signal<Vec<Entry>> {
        self.files.resolve()
    }

    pub fn pick_folder(&self) -> bool {
        let Some(folder) = rfd::FileDialog::new().set_directory("/").pick_folder() else {
            return false;
        };
        let folder_str = folder.to_str().unwrap_or_default().to_string();

        self.cfg.set("path", folder_str.clone());
        *self.path.borrow_mut() = folder_str;

        true
    }

    pub fn refresh_files(&self) {
        let Ok(entries) = get_files(&self.path.borrow()) else {
            return;
        };
        *self.files.write() = entries;
        self.load_playlist()
    }

    pub fn clear_queue(&self) {
        self.queue.write().clear();
    }

    pub fn add_to_queue(&self, name: &str) {
        self.queue.write().push(name.to_string());
    }

    pub fn remove_from_queue(&self, idx: usize) {
        if idx >= self.queue.len() {
            return;
        }
        self.queue.write().remove(idx);
    }

    pub fn queue_contains(&self, value: &str) -> bool {
        self.queue.read().iter().any(|e| *e == value)
    }

    pub fn open(&self, idx: usize) {
        if idx >= self.queue.len() {
            return;
        }
        let name = self.queue.peek()[idx].clone();
        if let Some(path) = self
            .files
            .iter()
            .find(|e| e.name == name)
            .map(|e| e.path.clone())
        {
            open(path)
        }
        self.remove_from_queue(idx);
    }

    pub fn current(&self) -> Current {
        if self.queue.is_empty() {
            return Current::default();
        }
        let name = self.queue.peek()[0].clone();
        let Some(path) = self
            .files
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
        let mut path = PathBuf::from(&*self.path.borrow());
        if !path.exists() {
            return;
        }
        path.push(".playlist");
        let playlist = serde_json::to_string_pretty(&*self.queue.peek()).unwrap_or_default();
        let _ = fs::write(path, playlist);
    }

    pub fn load_playlist(&self) {
        let path = PathBuf::from(&*self.path.borrow()).join(".playlist");

        if !path.exists() {
            self.queue.write().clear();
            return;
        }

        let data = fs::read_to_string(path).unwrap_or_default();
        let playlist_origin = serde_json::from_str::<Vec<String>>(&data).unwrap_or_default();
        let playlist = playlist_origin
            .into_iter()
            .filter(|e| self.files.iter().any(|f| f.name == *e))
            .collect::<Vec<String>>();

        *self.queue.write() = playlist;
    }
}

fn get_files(path: &str) -> Result<Vec<Entry>, ()> {
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
