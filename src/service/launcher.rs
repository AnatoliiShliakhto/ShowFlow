use ::std::{
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
};

static POWERPOINT_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    if let Some(path) = find_powerpoint_exe() {
        path
    } else {
        PathBuf::new()
    }
});

fn find_powerpoint_exe() -> Option<PathBuf> {
    let candidates = [
        r"C:\Program Files\Microsoft Office\root\Office16\POWERPNT.EXE",
        r"C:\Program Files (x86)\Microsoft Office\root\Office16\POWERPNT.EXE",
        r"C:\Program Files\Microsoft Office\Office16\POWERPNT.EXE",
        r"C:\Program Files (x86)\Microsoft Office\Office16\POWERPNT.EXE",
        r"C:\Program Files\Microsoft Office\Office15\POWERPNT.EXE",
        r"C:\Program Files (x86)\Microsoft Office\Office15\POWERPNT.EXE",
        // Add more versions if needed
    ];
    candidates
        .iter()
        .find(|path| Path::new(path).exists())
        .map(PathBuf::from)
}

pub fn open(path: PathBuf) {
    tokio::spawn(async move {
        let path_str = path.to_str().unwrap_or_default();
        if (path_str.ends_with(".pptx") || path_str.ends_with(".ppt"))
            && Command::new(POWERPOINT_PATH.as_os_str())
                .arg("/s")
                .arg(path_str)
                .spawn()
                .is_ok()
        {
            return;
        }

        let _ = opener::open(path);
    });
}
