use std::{env, path::PathBuf};

pub fn get_path(path: &str) -> PathBuf {
    let current_path = env::current_dir()
        .unwrap();
    
   let joined = current_path.join(path);
   joined
}
