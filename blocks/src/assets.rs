use std::path::Path;
use std::path::PathBuf;

pub struct Assets {
    pub root: PathBuf,
    pub post_renderer_vert: PathBuf,
    pub post_renderer_frag: PathBuf,
    pub chunk_renderer_vert: PathBuf,
    pub chunk_renderer_frag: PathBuf,
    pub text_renderer_vert: PathBuf,
    pub text_renderer_frag: PathBuf,
    pub dirt_xyz_png: PathBuf,
    pub stone_xyz_png: PathBuf,
    pub font_padded_sdf_png: PathBuf,
}

impl Assets {
    pub fn new(root: PathBuf) -> Assets {
        let root = ::std::fs::canonicalize(root).expect("Failed to canonicalize asset root path.");

        let post_renderer_vert = [root.as_path(), Path::new("post_renderer.vert")]
            .iter()
            .collect();
        let post_renderer_frag = [root.as_path(), Path::new("post_renderer.frag")]
            .iter()
            .collect();
        let chunk_renderer_vert = [root.as_path(), Path::new("chunk_renderer.vert")]
            .iter()
            .collect();
        let chunk_renderer_frag = [root.as_path(), Path::new("chunk_renderer.frag")]
            .iter()
            .collect();
        let text_renderer_vert = [root.as_path(), Path::new("text_renderer.vert")]
            .iter()
            .collect();
        let text_renderer_frag = [root.as_path(), Path::new("text_renderer.frag")]
            .iter()
            .collect();
        let dirt_xyz_png = [root.as_path(), Path::new("dirt_xyz.png")].iter().collect();
        let stone_xyz_png = [root.as_path(), Path::new("stone_xyz.png")]
            .iter()
            .collect();
        let font_padded_sdf_png = [root.as_path(), Path::new("font-padded-sdf.png")]
            .iter()
            .collect();

        Assets {
            root,
            post_renderer_vert,
            post_renderer_frag,
            chunk_renderer_vert,
            chunk_renderer_frag,
            text_renderer_vert,
            text_renderer_frag,
            dirt_xyz_png,
            stone_xyz_png,
            font_padded_sdf_png,
        }
    }
}

pub fn file_to_string<P: AsRef<Path>>(path: P) -> ::std::io::Result<String> {
    use std::io::Read;
    let mut file = ::std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
