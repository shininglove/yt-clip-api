pub mod youtube {
    use anyhow::Result;
    use std::path::PathBuf;
    use ytd_rs::{Arg, YoutubeDL, YoutubeDLResult};
    pub struct SoundRequest {
        pub name: String,
        pub url: String,
        pub duration: f32,
        pub start_time: f32,
    }
    #[allow(dead_code)]
    pub enum MediaFmt {
        WAV,
        MP3,
    }

    impl MediaFmt {
        pub fn extenstion(&self) -> &'static str {
            match self {
                MediaFmt::WAV => "wav",
                MediaFmt::MP3 => "mp3",
            }
        }
    }

    pub fn download(sound: &SoundRequest) -> Result<YoutubeDLResult> {
        let media_format = MediaFmt::WAV;
        let output_name = format!("{}.tmp.%(ext)s", sound.name);
        let args = vec![
            Arg::new("--quiet"),
            Arg::new("-x"),
            Arg::new_with_arg("--audio-format", media_format.extenstion()),
            Arg::new_with_arg("--output", output_name.as_str()),
        ];
        let link = sound.url.as_str();
        let path = PathBuf::from("./");
        let ytd = YoutubeDL::new(&path, args, link)?;

        // start download
        let download = ytd.download()?;
        return Ok(download);
    }
}

pub mod audio {
    use crate::youtube::{MediaFmt, SoundRequest};
    use anyhow::Result;
    use dotenvy::dotenv;
    use hound::{WavReader, WavWriter};
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use std::{env, fs};

    pub fn sound_splice(sound: SoundRequest, output_name: &str) -> Result<()> {
        // Open the input file
        dotenv()?;
        let sound_location = env::var("SOUNDS_DIR")?;
        fs::create_dir_all(&sound_location)?;
        let media_type = MediaFmt::WAV;
        let ext = media_type.extenstion();
        let media_name = format!("{}.tmp.{}", sound.name, ext);
        let media_location = Path::new(output_name).join(media_name);
        let file = File::open(&media_location)?;
        let mut reader = WavReader::new(BufReader::new(file))?;
        // Get the sample rate and number of channels from the input file
        let sample_rate = reader.spec().sample_rate;
        let channels = reader.spec().channels;

        let duration = (reader.len() as f32 / sample_rate as f32) / 2.0;

        println!("{}", duration);

        // Calculate the start time and end time (in seconds)
        let start_time = sound.start_time;
        let end_time = sound.duration * 2.0;

        // Calculate the start sample and end sample
        let start_sample = (sample_rate as f32 * start_time) as u32;
        let end_sample = (sample_rate as f32 * end_time) as u32;

        // Skip to the start time
        reader.seek(start_sample)?;

        // Take the audio data up to the end time (end time was to be double what you want in
        // duration)
        let samples = reader.samples::<i16>().take(end_sample as usize);

        // Open the output file
        // let output_file = File::create("output.wav")?;
        let spec = hound::WavSpec {
            channels: channels as u16,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let write_filename = format!("{}.{}", sound.name, ext);
        let write_location = Path::new(&sound_location).join(write_filename);
        let mut writer = WavWriter::create(write_location, spec)?;

        // Write the audio data to the output file
        for sample in samples {
            writer.write_sample(sample?)?;
        }

        writer.finalize()?;
        fs::remove_file(&media_location)?;
        Ok(())
    }
}
