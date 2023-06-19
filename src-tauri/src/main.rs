// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parse;

use declarative_discord_rich_presence::activity::Activity;
use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;
use parse::parse_folder;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize)]
struct Presence {
    state: String,
    details: String,
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

#[tauri::command]
async fn set_presence(
    presence: Presence,
    discord_ipc_client: State<'_, DeclarativeDiscordIpcClient>,
) -> Result<(), ()> {
    discord_ipc_client
        .set_activity(
            Activity::new()
                .state(&presence.state)
                .details(&presence.details),
        )
        .unwrap();

    Ok(())
}

fn main() {
    let open_folder = CustomMenuItem::new("open_folder".to_string(), "Open Folder");
    let preferences = CustomMenuItem::new("preferences".to_string(), "Preferences");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");

    let submenu = Submenu::new(
        "File",
        Menu::new()
            .add_item(open_folder)
            .add_item(preferences)
            .add_item(exit),
    );
    let menu = Menu::new().add_submenu(submenu);

    let stream = OutputStream::try_default().unwrap();

    tauri::Builder::default()
        .menu(menu)
        .setup(|app| {
            let discord_ipc_client = DeclarativeDiscordIpcClient::new("1117401973247975506");

            discord_ipc_client.enable();
            discord_ipc_client
                .set_activity(Activity::new().state("Browsing").details("Yes"))
                .unwrap();
            app.manage(discord_ipc_client);

            Ok(())
        })
        .on_menu_event(|event| match event.menu_item_id() {
            "exit" => event.window().close().unwrap(),
            "preferences" => {
                event.window().get_window("preferences").unwrap().show();
            }
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
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if event.window().label() != "main" {
                    event.window().hide().unwrap();
                    api.prevent_close();

                    return;
                }

                // L + ratio, tauri
                #[cfg(target_os = "windows")]
                event.window().app_handle().exit(0);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            play,
            stop,
            pause,
            resume,
            volume,
            set_presence
        ])
        .manage(Mutex::new(PlaybackState::new(stream.1)))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
