mod soundcloud;


pub fn main(){
    let client = soundcloud::SoundCloudClient::new();

    let q = "Blessings odd mob remix";

    let r = client.search_tracks(q, 5);

    println!("req: {:?}", r);
}