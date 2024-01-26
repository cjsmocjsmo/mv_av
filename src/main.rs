use md5::compute;
pub mod walk_dirs;

fn main() {
    let vid_path = "/media/pi/taz/hpics_copy".to_string();
    let vid_out_path = "/media/pi/taz/AV/".to_string();
    let mut ext_list = Vec::new();
    let vid_list = walk_dirs::walk_dir(vid_path.clone());
    for vid in vid_list {
        let ext_split = vid.split(".").collect::<Vec<&str>>();
        let ext = ext_split.last().unwrap().to_string();
        if ext_list.contains(&ext) {
            continue;
        } else {
            ext_list.push(ext.to_string());
        };
    



        // let digest = compute(&vid);
        // let new_out_path = format!("{}{:?}.{}", vid_out_path, digest, ext);
        // println!("{} -> {}", vid, new_out_path);
        // std::fs::copy(vid, new_out_path).unwrap();
    }
    print!("{:?}", ext_list)
}
