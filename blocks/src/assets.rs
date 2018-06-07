use std::path::Path;
use std::path::PathBuf;
use std::env;

#[inline]
pub fn get_asset_path<P: AsRef<Path>>(sub_path: P) -> PathBuf {
    let mut p = env::var_os("BLOCKS_ASSET_DIR").map_or_else(
        || PathBuf::from("assets"),
        PathBuf::from,
    );
    p.push(sub_path);
    p
}
