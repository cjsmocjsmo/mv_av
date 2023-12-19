use md5::compute;
pub mod walk_dirs;

fn main() {
    let vid_path = "/home/teresa/Pictures/AllPics".to_string();
    let vid_out_path = "/home/teresa/Videos/".to_string();
    let vid_list = walk_dirs::walk_dir(vid_path);
    for vid in vid_list {
        let ext_split = vid.split(".").collect::<Vec<&str>>();
        let ext = ext_split.last().unwrap();
        let digest = compute(&vid);
        let new_out_path = format!("{}{:?}.{}", vid_out_path, digest, ext);
        println!("{} -> {}", vid, new_out_path);
        std::fs::copy(vid, new_out_path).unwrap();
    }
}
