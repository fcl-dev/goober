// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parse;

use parse::parse_folder;
use serde::Serialize;
use tauri::State;
use tauri::{api::dialog, CustomMenuItem, Manager, Menu, Submenu};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

#[derive(Serialize)]
pub(crate) struct PlaybackState {
    #[serde(skip_serializing)]
    stream: OutputStreamHandle,
    #[serde(skip_serializing)]
    sinks: Vec<Sink>,
}

impl PlaybackState {
    pub fn new(stream: OutputStreamHandle) -> Self {
        Self {
            stream,
            sinks: vec![],
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn play(path: &str, playback_mutex: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    let mut state = playback_mutex.lock().unwrap();
    let sink = Sink::try_new(&state.stream).unwrap();

    if !sink.is_paused() {
        state.sinks.clear();
    }

    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    state.sinks.push(sink);

    Ok(())
}

#[tauri::command]
async fn stop(playback_mutex: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    let mut state = playback_mutex.lock().unwrap();
    state.sinks.clear();
    Ok(())
}

#[tauri::command]
async fn pause(playback_mutex: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    let state = playback_mutex.lock().unwrap();

    for sink in &state.sinks {
        sink.pause();
    }

    Ok(())
}

#[tauri::command]
async fn resume(playback_mutex: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    let state = playback_mutex.lock().unwrap();

    for sink in &state.sinks {
        sink.play();
    }

    Ok(())
}

#[tauri::command]
async fn volume(playback_mutex: State<'_, Mutex<PlaybackState>>, volume: f32) -> Result<(), ()> {
    let state = playback_mutex.lock().unwrap();

    for sink in &state.sinks {
        sink.set_volume(volume);
    }

    Ok(())
}

fn main() {
    let open_folder = CustomMenuItem::new("open_folder".to_string(), "Open Folder");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let submenu = Submenu::new("File", Menu::new().add_item(open_folder).add_item(exit));
    let menu = Menu::new().add_submenu(submenu);

    let stream = OutputStream::try_default().unwrap();

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "exit" => event.window().close().unwrap(),
            "open_folder" => {
                dialog::FileDialogBuilder::default().pick_folder(move |path_buf| match path_buf {
                    Some(p) => {
                        let app_handle = event.window().app_handle();
                        let payload = parse_folder(p, app_handle);

                        event.window().emit_all("music", payload).unwrap();
                    }
                    _ => {}
                })
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![play, stop, pause, resume])
        .manage(Mutex::new(PlaybackState::new(stream.1)))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
