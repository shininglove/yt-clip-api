use actix_files::NamedFile;
use actix_web::web::{Json, Query};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use soundapi::audio::sound_splice;
use soundapi::youtube::{download, SoundRequest};

#[derive(Deserialize)]
struct ClipRequest {
    name: String,
    url: String,
    start: String,
    end: String,
}

#[derive(Deserialize)]
struct SoundClip {
    name: String,
    sound_type: String,
}

#[get("/sounds")]
async fn get_sound_clip(sound: Query<SoundClip>) -> Result<NamedFile> {
    let sound_name = &sound.name;
    let sound_type = &sound.sound_type;
    dbg!(&sound_type);
    dbg!(&sound_name);
    Ok(NamedFile::open(sound_name)?)
}

#[post("/add_sound")]
async fn clip_request(req: Json<ClipRequest>) -> impl Responder {
    let name = req.name.clone();
    let url = req.url.clone();
    let start_time = parse_time_to_seconds(&req.start);
    let end_time = parse_time_to_seconds(&req.end);
    let duration = end_time - start_time;
    let sound = SoundRequest {
        name,
        url,
        duration,
        start_time,
    };
    let sound_download = download(&sound).unwrap();
    let download_location = sound_download.output_dir().to_string_lossy();
    println!("Your download: {}", download_location);
    sound_splice(sound, &download_location).unwrap();

    let message = format!("Name: {} has successfully been downloaded", req.name);
    HttpResponse::Ok().body(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(clip_request).service(get_sound_clip))
        .bind(("127.0.0.1", 9000))?
        .run()
        .await
}

fn parse_time_to_seconds(time: &String) -> f32 {
    let mins_secs: Vec<_> = time.split(":").collect();
    let mins = mins_secs[0].parse::<f32>().unwrap_or_default() * 60.0;
    let secs = mins_secs[1].parse::<f32>().unwrap_or_default();
    let total_time = mins + secs;
    return total_time;
}
