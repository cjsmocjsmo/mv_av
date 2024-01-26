use md5::compute;
use uuid::Uuid;
pub mod walk_dirs;

fn main() {
    let uuid = Uuid::new_v4();
    let vid_path = "/media/pi/taz/hpics_copy".to_string();
    let vid_out_path = "/media/pi/taz/AV/".to_string();
    let vid_list = walk_dirs::walk_dir(vid_path.clone());
    for vid in vid_list {
        let fname = vid.split("/").collect::<Vec<&str>>();
        let filename = fname.last().unwrap().to_string();
        let ext_split = filename.split(".").collect::<Vec<&str>>();
        let ext = ext_split.last().unwrap().to_string();
        // let new_file_path = format!("{}{}", vid_out_path, filename);
        // println!("{} ->\n {}", vid, new_file_path);
        
        
    



        // let digest = compute(&vid);
        // let new_out_path = format!("{}{:?}.{}", vid_out_path, digest, ext);
        let new_out_path2 = format!("{}{:?}.{}", vid_out_path, uuid, ext);
        println!("{} ->\n {}", vid, new_out_path2);
        // std::fs::copy(vid, new_out_path).unwrap();
    }
    print!("{:?}", "Done!")
}
