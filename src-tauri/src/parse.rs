use audiotags::{Picture, Tag};
use jwalk::WalkDir;
use lofty::{AudioFile, Probe};
use sanitize_filename::sanitize;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use tauri::AppHandle;

#[derive(Clone, serde::Serialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TrackKey {
    disc_number: u16,
    track_number: u16,
}

#[derive(Clone, serde::Serialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct Track {
    track: String,
    title: String,
    path: PathBuf,
    artist: String,
    album: Album,
    duration: u64,
}

#[derive(Clone, serde::Serialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct Album {
    name: String,
    artist: String,
    cover: String,
    tracks: Vec<Track>,
}

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    albums: Vec<Album>,
}

struct SortedAlbum {
    artist: String,
    cover: String,
    tracks: BTreeMap<TrackKey, Track>,
}

fn extract_filename(file_path: &str) -> Option<&str> {
    Path::new(file_path)
        .file_stem()
        .and_then(|name| name.to_str())
}

// TODO: use HashMaps instead of BTreeMaps to ditch useless iteration
// TODO: make albums that have various artists say "Various Artists" instead of the first artist it finds
pub fn parse_folder(p: PathBuf, app: AppHandle) -> Payload {
    let mut sorted_albums: BTreeMap<String, SortedAlbum> = BTreeMap::new();

    let path_resolver = app.path_resolver();
    let dir = path_resolver.app_data_dir().unwrap();
    fs::create_dir_all(&dir).unwrap();

    for entry in WalkDir::new(&p).into_iter().filter_map(Result::ok) {
        let file_name = entry.file_name().to_str().unwrap();

        let md = match entry.metadata() {
            Ok(md) => md,
            Err(_) => continue,
        };

        // scan only files
        if !md.is_file() {
            continue;
        }

        let probe = match Probe::open(entry.path().clone()).unwrap().read() {
            Ok(x) => x,
            Err(_) => continue,
        };

        let tag = Tag::default().read_from_path(entry.path());

        // reading through tags...
        if let Ok(x) = tag {
            let name = extract_filename(file_name).unwrap_or(file_name);

            let title = x.title().unwrap_or(name).to_string();
            let artist = x.artist().unwrap_or("").to_string();
            let album_name = x.album_title().unwrap_or(name).to_string();

            let album_cover = x.album_cover();
            let album_cover_path = dir.join(format!("{}{}", sanitize(&album_name), ".png"));

            let duration = probe.properties().duration().as_secs();

            if let Some(x) = album_cover {
                if !album_cover_path.is_file() {
                    let mut file = BufWriter::new(File::create(&album_cover_path).unwrap());
                    file.write_all(x.data).unwrap();
                }
            }

            let cover = if album_cover_path.is_file() {
                album_cover_path.to_string_lossy().into_owned()
            } else {
                String::new()
            };

            let sorted_album =
                sorted_albums
                    .entry(album_name.clone())
                    .or_insert_with(|| SortedAlbum {
                        artist: artist.clone(),
                        cover: cover.clone(),
                        tracks: BTreeMap::new(),
                    });

            let disc_number = x.disc_number().unwrap_or(1);
            let mut track_number = x
                .track_number()
                .unwrap_or((sorted_album.tracks.len() + 1) as u16);

            for track in &sorted_album.tracks {
                if track.0.disc_number == disc_number && track.0.track_number == track_number {
                    track_number = sorted_album.tracks.len() as u16;
                }
            }

            let track_key = TrackKey {
                disc_number,
                track_number,
            };

            let key = if disc_number > 1 {
                format!("{}-{:02}", disc_number, track_number)
            } else {
                format!("{:02}", track_number)
            };

            let album = Album {
                name: album_name.clone(),
                artist: artist.clone(),
                cover,
                tracks: Vec::new(),
            };

            let track = Track {
                track: key,
                title,
                artist,
                path: entry.path().to_path_buf(),
                duration,
                album,
            };

            sorted_album.tracks.insert(track_key, track);

            continue;
        }

        // without id3
        // way more basic information, fallback method
        let name = extract_filename(file_name).unwrap_or(file_name);

        let title = name.to_string();
        let artist = "Unknown Artist".to_string();
        let album_name = name.to_string();

        let duration = probe.properties().duration().as_secs();

        let album = Album {
            name: album_name.clone(),
            artist: artist.clone(),
            cover: String::new(),
            tracks: Vec::new(),
        };

        let sorted_album = sorted_albums
            .entry(album_name.clone())
            .or_insert_with(|| SortedAlbum {
                artist: artist.clone(),
                cover: String::new(),
                tracks: BTreeMap::new(),
            });

        let track = Track {
            track: "".to_string(),
            title,
            artist,
            path: entry.path().to_path_buf(),
            duration,
            album,
        };

        let track_key = TrackKey {
            disc_number: 1,
            track_number: 1,
        };

        sorted_album.tracks.insert(track_key, track);
    }

    let payload_albums: Vec<Album> = sorted_albums
        .into_iter()
        .map(|(album_name, album_data)| Album {
            name: album_name,
            artist: album_data.artist,
            cover: album_data.cover,
            tracks: album_data
                .tracks
                .into_iter()
                .map(|(_, track)| track)
                .collect(),
        })
        .collect();

    Payload {
        albums: payload_albums,
    }
}
