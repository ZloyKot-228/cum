use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

pub struct FilesystemManager {
    root: PathBuf,
}

impl FilesystemManager {
    pub fn new(root: PathBuf) -> Result<Self, std::io::Error> {
        Ok(Self {
            root: root.canonicalize()?,
        })
    }

    /// All pathes are relative to root
    pub fn find_all_with_extension(&self, ext: &str, dir: &Path) -> Vec<PathBuf> {
        if !dir.exists() {
            return vec![];
        }
        WalkDir::new(self.root.join(dir))
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|s| s.to_str().unwrap_or(""))
                    .map(|s| s == ext)
                    .unwrap_or(false)
            })
            .filter_map(|e| e.path().strip_prefix(&self.root).ok().map(PathBuf::from))
            .collect()
    }

    #[inline]
    pub fn find_newest(list: &[PathBuf]) -> Option<PathBuf> {
        list.iter()
            .reduce(|a, b| {
                if Self::is_newer(a, b).unwrap_or(false) {
                    a
                } else {
                    b
                }
            })
            .cloned()
    }

    /// Path is relative to root
    #[inline]
    pub fn mkdir(&self, path: &Path) -> std::io::Result<()> {
        fs::create_dir_all(self.root.join(path))
    }

    /// Path is relative to root
    #[inline]
    pub fn mkfile(&self, path: &Path) -> std::io::Result<()> {
        let path = self.root.join(path);
        if !path.exists() {
            path.parent().map(fs::create_dir_all).unwrap_or(Ok(()))?;
            let _ = File::create(path)?;
        }
        Ok(())
    }

    /// Path is relative to root
    #[inline]
    pub fn delete(&self, path: &Path) -> std::io::Result<()> {
        let path = self.root.join(path);
        if path.exists() && path.is_file() {
            fs::remove_file(path)
        } else if path.exists() && path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            Ok(())
        }
    }

    /// Path is relative to root
    #[inline]
    pub fn clear_dir(&self, path: &Path) -> std::io::Result<()> {
        let path = self.root.join(path);
        if path.exists() && path.is_dir() {
            fs::remove_dir_all(&path)?;
            fs::create_dir_all(path)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn is_older(first: &Path, second: &Path) -> Option<bool> {
        let first_time = fs::metadata(first).ok()?.modified().ok()?;
        let second_time = fs::metadata(second).ok()?.modified().ok()?;
        Some(first_time < second_time)
    }

    #[inline]
    pub fn is_newer(first: &Path, second: &Path) -> Option<bool> {
        Some(!Self::is_older(first, second)?)
    }

    #[inline]
    pub fn to_full(&self, path: &Path) -> PathBuf {
        self.root.join(path)
    }

    /// src/deps/dep1.cpp turns into target/obj/src.deps.dep1.o
    pub fn src_to_obj(path: &PathBuf) -> PathBuf {
        let dotted: String = path
            .with_extension("o")
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(".");
        PathBuf::from(format!("target/obj/{dotted}"))
    }

    /// src/deps/dep1.cpp turns into target/obj/src.deps.dep1.o
    pub fn src_to_obj_path(path: &Path) -> PathBuf {
        let dotted: String = path
            .with_extension("o")
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(".");
        PathBuf::from(format!("target/obj/{dotted}"))
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }
}

impl Default for FilesystemManager {
    fn default() -> Self {
        Self {
            root: env::current_dir().unwrap_or_default(),
        }
    }
}
