use ::std::{
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
};
use ::tokio::spawn;

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
        .map(|s| PathBuf::from(s))
}

pub fn open(path: PathBuf) {
    spawn(async move {
        let path_str = path.to_str().unwrap_or_default();
        if path_str.ends_with(".pptx") || path_str.ends_with(".ppt") {
            if Command::new(POWERPOINT_PATH.as_os_str())
                .arg("/s")
                .arg(path.clone().to_str().unwrap())
                .spawn()
                .is_ok()
            {
                return;
            }
        }

        let _ = opener::open(path_str);
    });
    //let _ = opener::open(path_str).is_err();
}
