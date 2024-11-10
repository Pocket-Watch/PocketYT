use std::env;
use rusty_ytdl::Video;
use rusty_ytdl::search::{Playlist, PlaylistSearchOptions};
use rusty_ytdl;
use url::Url;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <YouTube URL>", args[0]);
        return Ok(());
    }
    let url_str = &args[1];
    let url = Url::parse(url_str);
    if url.is_err() {
        eprintln!("Invalid URL provided.");
        return Ok(());
    }
    let url = Url::parse(url_str);
    let is_playlist = url.unwrap().path().contains("playlist");
    let playlist_options: Option<PlaylistSearchOptions> = Some(PlaylistSearchOptions {
        limit: 10000,
        request_options: None,
        fetch_all: false,
    });
    if is_playlist {
        let playlist = Playlist::get(url_str, Option::from(playlist_options).as_ref()).await?;
        for video in &playlist.videos {
            let full_url = build_youtube_url(&video.url);
            println!(r#"{{"title": "{}", "url": "{}"}}"#, video.title, full_url);
        }
    } else {
        let video = Video::new(url_str).unwrap();
        let video_info = video.get_info().await.unwrap();
        let vid_title = video_info.video_details.title;
        println!(r#"{{"title": "{}", "url": "{}"}}"#, vid_title, Url::parse(url_str).unwrap());
    }
    return Ok(());
}
fn build_youtube_url(video_id: &str) -> String {
    let base_url = "https://www.youtube.com/watch?v=";
    format!("{}{}", base_url, video_id)
}