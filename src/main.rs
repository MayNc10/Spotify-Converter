use std::{char::MAX, fs::{self, read_to_string}, path::PathBuf};

use clap::Parser;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct InputInfo {
    ts: String,
    ms_played: u64,
    master_metadata_track_name: Option<String>,
    master_metadata_album_artist_name: Option<String>,
    master_metadata_album_album_name: Option<String>,
}

#[derive(Serialize)]
struct OutputInfo {
    #[serde(rename = "albumName")]
    album_name: String,

    #[serde(rename = "artistName")]
    artist_name: String,

    #[serde(rename = "trackName")]
    track_name: String,

    time: String,

    //duration: u64,
}

impl OutputInfo {
    pub fn from_input_info(input: InputInfo) -> Option<OutputInfo> {
        Some(OutputInfo { 
            album_name: input.master_metadata_album_album_name?,
            artist_name: input.master_metadata_album_artist_name?,
            track_name: input.master_metadata_track_name?,
            time: input.ts,
            //duration: input.ms_played,
        })
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name="INPUT FOLDER")]
    input_folder: String,
    #[arg(short, long, value_name="MAX DATE")]
    max_date: Option<String>,
}

const MAX_PER_JSON: usize = 2600;


fn main() {
    let cli = Cli::parse();
    let max_date = cli.max_date.and_then(|s| dateparser::parse(&s).ok());

    let json_chunks = fs::read_dir(cli.input_folder.clone()).unwrap()
        .filter_map(|p| p.map(|p| p.path() ).ok() )
        .filter(|p| p.extension().is_some_and(|ext| ext == "json") && p.to_str().is_some())
        .filter_map(|p| serde_json::from_str::<Vec<InputInfo>>(&fs::read_to_string(p).ok()?).ok() ) 
        .flatten()  
        .filter(|input| if let Some(max_date) = max_date { dateparser::parse(&input.ts).unwrap() <= max_date } else { true })
        .filter_map(|input| OutputInfo::from_input_info(input))
        .chunks(MAX_PER_JSON);
    
    for (i, chunk) in json_chunks.into_iter().enumerate() {
        let fname = format!("songs_{}.json", i+1);
        let path = PathBuf::from(cli.input_folder.clone()).join(fname);

        fs::write(path, serde_json::to_string(&chunk.collect::<Vec<_>>()).unwrap()).unwrap();
    }
}        
