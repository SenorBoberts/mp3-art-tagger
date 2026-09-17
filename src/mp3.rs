use std::path::PathBuf;

use id3::{Tag, TagLike, v1v2::read_from_path, Version};

pub struct Mp3{
    track: Tag,
    path: PathBuf
}

impl Mp3{
    pub fn new(path: PathBuf) -> Self{
        Self{
            track: read_from_path(&path).unwrap(),
            path: path
        }
    }

    pub fn construct_query(&self) -> String{
        let artist = self.track.artist().unwrap();
        let title = self.track.title().unwrap();

        format!("{} {}", artist, title)
    }

    pub fn set_title(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        self.track.set_title("TEST TITLE");
        self.track.write_to_path(&self.path,Version::Id3v23)?;

        Ok(())
    }
}