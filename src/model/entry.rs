use ::std::path::PathBuf;

#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
}
