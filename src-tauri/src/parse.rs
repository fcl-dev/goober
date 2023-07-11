use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use audiotags::Tag;
use jwalk::WalkDir;
use lofty::{AudioFile, Probe};
use sanitize_filename::sanitize;
use tauri::AppHandle;

#[derive(Clone, serde::Serialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TrackKey {
    disc_number: u16,
    track_number: u16,
}

#[derive(Clone, serde::Serialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct Track {
    #[serde(skip_serializing)]
    key: TrackKey,
    track: String,
    title: String,
    path: PathBuf,
    artist: String,
    track_album: TrackAlbum,
    duration: u64,
}

#[derive(Clone, serde::Serialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct Album {
    name: String,
    artist: String,
    cover: String,
    year: i32,
    tracks: Vec<Track>,
}

#[derive(Clone, serde::Serialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
/// Basic information required just for the proper displaying of tracks.
struct TrackAlbum {
    cover: String,
}

fn extract_filename(file_path: &str) -> Option<&str> {
    Path::new(file_path)
        .file_stem()
        .and_then(|name| name.to_str())
}

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    library: Vec<Album>,
}

// shoutout to u#5987669 in stackoverflow, this is way easier and more intuitive to use!
pub trait FileExtension {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool;
}

impl<P: AsRef<Path>> FileExtension for P {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool {
        if let Some(ref extension) = self.as_ref().extension().and_then(OsStr::to_str) {
            return extensions
                .iter()
                .any(|x| x.as_ref().eq_ignore_ascii_case(extension));
        }

        false
    }
}

pub fn parse_folder(p: PathBuf, app: AppHandle) -> Payload {
    let mut albums: Vec<Album> = vec![];

    let path_resolver = app.path_resolver();
    let cover_dir = path_resolver.app_data_dir().unwrap().join("covers");
    fs::create_dir_all(&cover_dir).unwrap();

    for entry in WalkDir::new(&p).into_iter().filter_map(Result::ok) {
        let file_name = entry.file_name().to_str().unwrap();
        let entry_path = entry.path();

        if !entry_path.is_file()
            || !entry_path
                .as_path()
                .has_extension(&["mp3", "wav", "ogg", "flac"])
        {
            continue;
        }

        let mut tag = Tag::new().read_from_path(entry_path);
        let name = extract_filename(file_name).unwrap_or(file_name);

        if let Ok(t) = tag {
            let title = t.title().unwrap_or(name).to_string();
            let artist = t.artist().unwrap_or("Unknown Artist").to_string();
            let album_name = t.album_title().unwrap_or(name).to_string();
            let year = t.year().unwrap_or(0);
            let cover = t.album_cover();
            let cover_path = cover_dir.join(format!("{}{}", sanitize(&album_name), ".jpg"));

            let disc_number = t.disc_number().unwrap_or(1);
            let track_number = t.track_number().unwrap_or(1);

            let mut duration = t.duration().unwrap_or(0.0) as u64;

            if !cover_path.is_file() {
                if let Some(c) = cover {
                    let mut file = BufWriter::new(File::create(&cover_path).unwrap());
                    file.write_all(c.data).unwrap();
                }
            }

            let path = if cover_path.is_file() {
                cover_path.to_string_lossy().into_owned()
            } else {
                String::new()
            };

            let album = match albums.iter_mut().find(|x| x.name == album_name) {
                Some(a) => a,
                None => {
                    let album = Album {
                        name: album_name,
                        artist: artist.clone(),
                        cover: path,
                        year,
                        tracks: vec![],
                    };

                    albums.push(album.clone());

                    albums.iter_mut().find(|a| a.name == album.name).unwrap()
                }
            };

            let track_album = TrackAlbum {
                cover: album.cover.clone(),
            };

            let mut track_key = TrackKey {
                disc_number,
                track_number,
            };

            if let Some(_) = album.tracks.iter().find(|x| x.key == track_key) {
                track_key.track_number = album.tracks.len() as u16;
            }

            let track = if track_key.disc_number == 0 || track_key.disc_number == 1 {
                format!("{:02}", track_key.track_number)
            } else {
                format!("{}-{:02}", track_key.disc_number, track_key.track_number)
            };

            if duration == 0 {
                if let Ok(d) = Probe::open(entry.path().clone()).unwrap().read() {
                    duration = d.properties().duration().as_secs();
                }
            }

            let track = Track {
                key: track_key,
                track,
                title,
                path: entry.path().to_path_buf(),
                artist,
                track_album,
                duration: duration as u64,
            };

            album.tracks.push(track);

            continue;
        }

        let mut duration: u64 = 0;

        if let Ok(d) = Probe::open(entry.path().clone()).unwrap().read() {
            duration = d.properties().duration().as_secs();
        }

        let track = Track {
            key: TrackKey {
                disc_number: 1,
                track_number: 1,
            },
            track: "".to_string(),
            title: name.to_string(),
            path: entry.path().to_path_buf(),
            artist: "Unknown Artist".to_string(),
            track_album: TrackAlbum {
                cover: "".to_string(),
            },
            duration,
        };

        let single = Album {
            name: name.to_string(),
            artist: "Unknown Artist".to_string(),
            cover: "".to_string(),
            year: 0,
            tracks: vec![track],
        };

        albums.push(single);
    }

    for album in &mut albums {
        album.tracks.sort_by(|a, b| a.key.cmp(&b.key));

        // hack: this works, because album.artist is the first track the algo finds
        // so it's technically ensuring that *all* artists are equal.
        if !album.tracks.iter().all(|x| x.artist == album.artist) {
            album.artist = "Various Artists".to_string();
        }
    }

    Payload { library: albums }
}
