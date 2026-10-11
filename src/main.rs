use askama::Template;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
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

fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| {
            IMAGE_EXTENSIONS
                .iter()
                .any(|known| known.eq_ignore_ascii_case(ext))
        })
}

fn collect_images(cwd: &Path) -> io::Result<Vec<ImageEntry>> {
    let mut images = Vec::new();
    for entry in fs::read_dir(cwd)? {
        let entry = entry?;
        let path = entry.path();
        if !is_image(&path) {
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        if !meta.is_file() {
            continue;
        }
        images.push(ImageEntry {
            relative: entry.file_name().to_string_lossy().into_owned(),
            absolute: path.to_string_lossy().into_owned(),
            size_str: format_size(meta.len()),
        });
    }
    images.sort_unstable_by(|a, b| natord::compare_ignore_case(&a.relative, &b.relative));
    Ok(images)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    if args.next().as_deref() == Some("--cleanup-after") {
        let target = args.next().ok_or("--cleanup-after requires a path")?;
        thread::sleep(Duration::from_secs(45));
        let _ = fs::remove_file(target);
        return Ok(());
    }

    // Canonicalized once; image paths are joined from it.
    let cwd = resolve_cwd().canonicalize()?;

    let images = collect_images(&cwd).map_err(|e| format!("Error scanning directory: {e}"))?;
    if images.is_empty() {
        return Err("No images found in the current directory.".into());
    }

    let relative_names: Vec<&str> = images.iter().map(|i| i.relative.as_str()).collect();
    let images_json = serde_json::to_string(&relative_names)?;

    let template = ScrollerTemplate {
        dir_name: cwd
            .file_name()
            .unwrap_or(cwd.as_os_str())
            .to_string_lossy()
            .into_owned(),
        dir_path: cwd.to_string_lossy().into_owned(),
        images,
        images_json,
    };

    let html = template
        .render()
        .map_err(|e| format!("Error rendering viewer template: {e}"))?;

    let viewer_path = cwd.join(".scroller_view.html");
    fs::write(&viewer_path, html).map_err(|e| format!("Error creating viewer file: {e}"))?;

    open_in_browser(&viewer_path);

    // Delete the viewer file after 45s via a detached copy of this binary,
    // so the shell prompt is not blocked.
    if let Ok(exe) = env::current_exe() {
        let _ = Command::new(exe)
            .arg("--cleanup-after")
            .arg(&viewer_path)
            .spawn();
    }
    Ok(())
}
