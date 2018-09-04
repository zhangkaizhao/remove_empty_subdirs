use std::fs;
use std::io;
use std::path::Path;

/// Remove empty directories under a directory.
/// ```rust,ignore
/// extern crate remove_empty_subdirs;
///
/// use std::path::Path;
///
/// use remove_empty_subdirs::remove_empty_subdirs;
///
/// fn main() {
///     let path = Path::new("test_dir");
///     remove_empty_subdirs(path).unwrap();
/// }
/// ```
pub fn remove_empty_subdirs(dir: &Path) -> io::Result<()> {
    _remove_empty_subdirs(dir, dir.clone())
}

/// Remove empty directories under current directory.
fn _remove_empty_subdirs(dir: &Path, top_dir: &Path) -> io::Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(dirs) => dirs,
        Err(err) => {
            println!(
                "Failed to read directory `{}` due to `{}`.",
                dir.display(),
                err.to_string()
            );
            return Ok(());
        }
    };
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            // Ignore hidden directories which start with ".", e.g. ".git".
            let is_hidden = path.file_name().unwrap().to_str().unwrap().starts_with(".");
            if !is_hidden {
                let can_stop = _try_to_remove_empty_dir(&path, &top_dir);
                if !can_stop {
                    // Continue to remove sub-directories.
                    _remove_empty_subdirs(&path, &top_dir)?;
                }
            }
        }
    }
    Ok(())
}

/// Try to recursively remove empty directories upwards.
fn _try_to_remove_empty_dir(dir: &Path, top_dir: &Path) -> bool {
    if dir == top_dir {
        // No need to remove top directory.
        return true;
    }
    // Try to remove empty directory.
    match fs::remove_dir(&dir) {
        Ok(_) => {
            println!("Empty directory `{}` is removed.", dir.display());
            // Then try to remove parent directory.
            let parent_dir = dir.parent().unwrap();
            _try_to_remove_empty_dir(&parent_dir, &top_dir);
            true
        }
        Err(ref err) if err.kind() == io::ErrorKind::PermissionDenied => {
            // Permission denied. Then every sub-direcotry can't not be removed too.
            println!(
                "Failed to remove directory `{}` due to Permission denied.",
                dir.display()
            );
            true
        }
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
            // Already removed?
            println!(
                "Failed to remove directory `{}` due to Not found.",
                dir.display()
            );
            true
        }
        Err(_err) => {
            // Not empty directory or other os error while removing directory.
            false
        }
    }
}
