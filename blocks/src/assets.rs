use notify;
use notify::Watcher;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub struct Assets {
    root: PathBuf,
    path_to_modified: Arc<Mutex<HashMap<PathBuf, Arc<Mutex<bool>>>>>,
    #[allow(unused)]
    watcher: notify::RecommendedWatcher,
}

impl Assets {
    pub fn new(root: PathBuf) -> Assets {
        let path_to_modified = Arc::new(Mutex::new(HashMap::new()));
        let path_to_modified_clone = path_to_modified.clone();

        let (tx, rx) = mpsc::channel();

        let mut watcher = notify::watcher(tx, Duration::from_millis(100)).unwrap();

        watcher
            .watch(&root, notify::RecursiveMode::Recursive)
            .unwrap();

        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(event) => {
                        use notify::DebouncedEvent;
                        match event {
                            DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                                path_to_modified_clone
                                    .lock()
                                    .unwrap()
                                    .entry(path)
                                    .and_modify(|m: &mut Arc<Mutex<bool>>| {
                                        *m.lock().unwrap() = true;
                                    });
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {
                        // Only error is disconnect.
                        break;
                    }
                }
            }
        });

        Assets {
            root,
            path_to_modified,
            watcher,
        }
    }

    /// path must be an absolute path.
    pub fn get_modified<P: AsRef<Path> + Into<PathBuf>>(&mut self, path: P) -> Arc<Mutex<bool>> {
        let mut path_to_modified = self.path_to_modified.lock().unwrap();
        let (value, insert) = match path_to_modified.get(path.as_ref()) {
            Some(v) => (v.clone(), false),
            None => (Arc::new(Mutex::new(true)), true),
        };
        if insert {
            path_to_modified.insert(PathBuf::from(path.into()), value.clone());
        }
        value
    }

    pub fn get_path<P: AsRef<Path>>(&self, sub_path: P) -> PathBuf {
        let mut p = ::std::fs::canonicalize(&self.root).unwrap();
        p.push(sub_path);
        p
    }
}

pub fn file_to_string<P: AsRef<Path>>(path: P) -> ::std::io::Result<String> {
    use std::io::Read;
    let mut file = ::std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
