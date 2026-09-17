use std::{path::PathBuf, vec};

use rfd::FileDialog;
use walkdir::WalkDir;

use crate::soundcloud::SoundCloudClient;

mod soundcloud;
mod mp3;

pub fn main(){
    let sc = SoundCloudClient::new();

    let file = FileDialog::new().pick_file().unwrap(); 

    let mut track = mp3::Mp3::new(file);

    let q = track.construct_query();

    let res = sc.search_tracks(&q, 5);

    println!("{:?}", res);

    track.set_title().unwrap();
}