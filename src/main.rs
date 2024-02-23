use uuid::Uuid;
pub mod walk_dirs;

fn main() {
    let vid_path = "/media/pipi/0123-4567/PicCD2".to_string();
    let vid_out_path = "/media/pipi/0123-4567/AV/".to_string();
    let vid_list = walk_dirs::walk_dir(vid_path.clone());
    for vid in vid_list {
        let uuid = Uuid::new_v4();
        let fname = vid.split("/").collect::<Vec<&str>>();
        let filename = fname.last().unwrap().to_string();
        let ext_split = filename.split(".").collect::<Vec<&str>>();
        let ext = ext_split.last().unwrap().to_string();
        
        let new_out_path2 = format!("{}{:?}.{}", vid_out_path, uuid, ext);
        println!("{} ->\n {}", vid, new_out_path2);
        
        
        // Move vid to new_out_path2
        std::fs::rename(&vid, &new_out_path2).unwrap();
    }
    print!("{:?}", "Done!")
}
