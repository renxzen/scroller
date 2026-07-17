use askama::Template;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

const IMAGE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "webp", "gif", "svg", "bmp", "tiff", "ico",
];

struct ImageEntry {
    relative: String,
    absolute: String,
    size_str: String,
}

#[derive(Template)]
#[template(path = "scroller.html")]
struct ScrollerTemplate {
    dir_name: String,
    dir_path: String,
    images: Vec<ImageEntry>,
    images_json: String,
}

fn natural_sort_key(name: &str) -> Vec<(bool, String, u64)> {
    let mut key = Vec::new();
    let mut chars = name.chars().peekable();
    while chars.peek().is_some() {
        if chars.peek().unwrap().is_ascii_digit() {
            let mut num = String::new();
            while let Some(c) = chars.peek() {
                if c.is_ascii_digit() {
                    num.push(*c);
                    chars.next();
                } else {
                    break;
                }
            }
            key.push((false, String::new(), num.parse().unwrap_or(0)));
        } else {
            let mut text = String::new();
            while let Some(c) = chars.peek() {
                if !c.is_ascii_digit() {
                    text.push(*c);
                    chars.next();
                } else {
                    break;
                }
            }
            key.push((true, text.to_lowercase(), 0));
        }
    }
    key
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn resolve_cwd() -> PathBuf {
    if let Ok(paths) = env::var("NAUTILUS_SCRIPT_SELECTED_FILE_PATHS")
        && let Some(first) = paths.lines().find(|l| !l.trim().is_empty())
    {
        let path = PathBuf::from(first);
        if path.is_dir() {
            return path;
        }
    }
    env::current_dir().expect("failed to read current directory")
}

fn open_in_browser(path: &Path) {
    let result = if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(path).spawn()
    } else {
        eprintln!("Unsupported platform for opening a browser.");
        return;
    };

    if let Err(e) = result {
        eprintln!("Error opening browser: {e}");
    }
}

fn main() {
    let mut args = env::args();
    args.next(); // skip argv[0]
    let rest: Vec<String> = args.collect();

    if rest.first().map(String::as_str) == Some("--cleanup-after") {
        let Some(target) = rest.get(1) else {
            std::process::exit(1);
        };
        thread::sleep(Duration::from_secs(45));
        let _ = fs::remove_file(target);
        return;
    }

    let cwd = resolve_cwd();

    let mut image_files: Vec<PathBuf> = match fs::read_dir(&cwd) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file())
            .filter(|p| {
                p.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
                    .unwrap_or(false)
            })
            .collect(),
        Err(e) => {
            eprintln!("Error scanning directory: {e}");
            std::process::exit(1);
        }
    };

    if image_files.is_empty() {
        eprintln!("No images found in the current directory.");
        std::process::exit(1);
    }

    image_files.sort_by(|a, b| {
        let a_name = a.file_name().unwrap().to_string_lossy();
        let b_name = b.file_name().unwrap().to_string_lossy();
        natural_sort_key(&a_name).cmp(&natural_sort_key(&b_name))
    });

    let mut images = Vec::with_capacity(image_files.len());
    let mut relative_names = Vec::with_capacity(image_files.len());

    for img in &image_files {
        let relative = img.file_name().unwrap().to_string_lossy().to_string();
        let absolute = fs::canonicalize(img)
            .unwrap_or_else(|_| img.clone())
            .to_string_lossy()
            .to_string();
        let size_str = fs::metadata(img)
            .map(|m| format_size(m.len()))
            .unwrap_or_default();

        relative_names.push(relative.clone());
        images.push(ImageEntry {
            relative,
            absolute,
            size_str,
        });
    }

    let dir_name = cwd
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| cwd.to_string_lossy().to_string());
    let dir_path = cwd.to_string_lossy().to_string();
    let images_json = serde_json::to_string(&relative_names).unwrap();

    let template = ScrollerTemplate {
        dir_name,
        dir_path,
        images,
        images_json,
    };

    let html = match template.render() {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Error rendering viewer template: {e}");
            std::process::exit(1);
        }
    };

    let viewer_path = cwd.join(".scroller_view.html");
    if let Err(e) = fs::write(&viewer_path, html) {
        eprintln!("Error creating viewer file: {e}");
        std::process::exit(1);
    }

    open_in_browser(&viewer_path);

    // The viewer file needs to survive long enough for the browser to load
    // it (and tolerate a manual reload) before it's cleaned up so it doesn't
    // litter the directory. That wait can't happen in this process without
    // blocking the shell prompt for 45s, so it's handed off to a detached
    // copy of this same binary running in --cleanup-after mode.
    if let Ok(exe) = env::current_exe() {
        let _ = Command::new(exe)
            .arg("--cleanup-after")
            .arg(&viewer_path)
            .spawn();
    }
}
