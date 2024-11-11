use std::env;
use rusty_ytdl::Video;
use rusty_ytdl::search::{Playlist, PlaylistSearchOptions};
use rusty_ytdl;
use url::Url;
use std::error::Error;
use rusty_ytdl::VideoSearchOptions::Video as OtherVideo;
use serde::Serialize;
use serde_json::json;

#[derive(Debug,Serialize)]
struct VidInf {
    title: String,
    url: String,
}

#[derive (Debug,Serialize)]
struct Sources {
    audio_source: String,
    video_source: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <YouTube URL> <command>", args[0]);
        eprintln!("Commands:");
        eprintln!("  --get-sources        Print audio and video source URLs for a single video.");
        eprintln!("  --dump-videos        Print array of video titles and URLs for a playlist.");
        return Ok(());
    }

    let url_str = &args[1];
    let command = &args[2];

    let url = Url::parse(url_str);
    if url.is_err() {
        eprintln!("Invalid URL provided.");
        return Ok(());
    }

    match command.as_str() {
        "--get-sources" => get_sources(url_str).await?,
        "--dump-videos" => dump_videos(url_str).await?,
        _ => eprintln!("Invalid command."),
    }

    Ok(())
}

async fn get_sources(url_str: &str) -> Result<(), Box<dyn Error>> {
    let video = Video::new(url_str)?;
    let video_info = video.get_info().await?;

    let audio_source = video_info
        .formats
        .iter()
        .find(|f| f.mime_type.audio_codec.is_some())
        .map(|f| f.url.clone())
        .unwrap_or("No audio source found".to_string());

    let video_source = video_info
        .formats
        .iter()
        .find(|f| f.mime_type.video_codec.is_some())
        .map(|f| f.url.clone())
        .unwrap_or("No video source found".to_string());

    let sources = Sources {
        audio_source,
        video_source,
    };

    println!("{}", serde_json::to_string(&sources)?);
    Ok(())
}

async fn dump_videos(url_str: &str) -> Result<(), Box<dyn Error>> {
    let url = Url::parse(url_str)?;
    let is_playlist = url.path().contains("playlist");

    //If a single video provided for arguments --dump-videos return a video object with a title and an url
    if !is_playlist {
        //eprintln!("The provided URL is not a playlist.");
        let video = Video::new(url_str)?;
        let detail = video.get_basic_info().await?;
        let data = VidInf {
            title: detail.video_details.title,
            url: detail.video_details.video_url
        };
        let videos = vec![data];
        println!("{}",serde_json::to_string(&videos)?);
        return Ok(());
    }

    let playlist_options = Some(PlaylistSearchOptions {
        limit: 10000,
        request_options: None,
        fetch_all: false,
    });

    let playlist = Playlist::get(url_str, playlist_options.as_ref()).await?;
    let videos: Vec<VidInf> = playlist
        .videos
        .into_iter()
        .map(|video| VidInf {
            title: video.title,
            url: build_youtube_url(&video.url),
        })
        .collect();

    println!("{}", serde_json::to_string(&videos)?);
    Ok(())
}

fn build_youtube_url(video_id: &str) -> String {
    let base_url = "https://www.youtube.com/watch?v=";
    format!("{}{}", base_url, video_id)
}