// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parse;

use declarative_discord_rich_presence::activity::{Activity, Assets};
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
    sink: Sink,
}

impl PlaybackState {
    pub fn new(stream: OutputStreamHandle) -> Self {
        Self {
            stream,
            sink: Sink::new_idle().0,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Presence {
    state: String,
    details: String,
    large_text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PackageJson {
    version: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn play(
    path: &str,
    volume: f32,
    playback_mutex: State<'_, Mutex<PlaybackState>>,
) -> Result<(), ()> {
    let mut state = playback_mutex.lock().unwrap();

    let sink = Sink::try_new(&state.stream).unwrap();
    state.sink.clear();

    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    sink.set_volume(volume);

    state.sink = sink;

    Ok(())
}

#[tauri::command]
async fn stop(playback_mutex: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    let state = playback_mutex.lock().unwrap();

    state.sink.clear();

    Ok(())
}

#[tauri::command]
async fn pause(playback_mutex: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    let state = playback_mutex.lock().unwrap();

    state.sink.pause();

    Ok(())
}

#[tauri::command]
async fn resume(playback_mutex: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    let state = playback_mutex.lock().unwrap();

    state.sink.play();

    Ok(())
}

#[tauri::command]
async fn volume(playback_mutex: State<'_, Mutex<PlaybackState>>, volume: f32) -> Result<(), ()> {
    let state = playback_mutex.lock().unwrap();

    state.sink.set_volume(volume);

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
                .assets(
                    Assets::new()
                        .large_image("goober-icon")
                        .large_text(&presence.large_text),
                )
                .state(&presence.state)
                .details(&presence.details),
        )
        .unwrap();

    Ok(())
}

fn main() {
    let open_library_folder = CustomMenuItem::new("open_folder".to_string(), "Open library folder");
    let preferences = CustomMenuItem::new("preferences".to_string(), "Preferences");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");

    let file = Submenu::new(
        "File",
        Menu::new()
            .add_item(open_library_folder)
            .add_item(preferences)
            .add_item(exit),
    );

    let about = CustomMenuItem::new("about".to_string(), "About");

    let help = Submenu::new("Help", Menu::new().add_item(about));

    let menu = Menu::new().add_submenu(file).add_submenu(help);

    let stream = OutputStream::try_default().unwrap();

    tauri::Builder::default()
        .menu(menu)
        .setup(|app| {
            let package_json_path = app
                .path_resolver()
                .resolve_resource("../package.json")
                .expect("failed to resolve resource");

            let file = std::fs::File::open(&package_json_path).unwrap();

            let package_json: PackageJson = serde_json::from_reader(file).unwrap();

            let discord_ipc_client = DeclarativeDiscordIpcClient::new("1117401973247975506");

            discord_ipc_client.enable();
            discord_ipc_client
                .set_activity(
                    Activity::new()
                        .state("Browsing")
                        .assets(
                            Assets::new()
                                .large_image("goober-icon")
                                .large_text(&format!("v{}", &package_json.version)),
                        )
                        .details("Just launched"),
                )
                .unwrap();

            app.manage(discord_ipc_client);

            Ok(())
        })
        .on_menu_event(|event| match event.menu_item_id() {
            "exit" => event.window().close().unwrap(),
            "preferences" => {
                let _ = event.window().get_window("preferences").unwrap().show();
            }
            "open_folder" => {
                dialog::FileDialogBuilder::default().pick_folder(move |path_buf| match path_buf {
                    Some(p) => {
                        let app_handle = event.window().app_handle();
                        let payload = parse_folder(p, app_handle);

                        event.window().emit_all("library", payload).unwrap();
                    }
                    _ => {}
                })
            }
            "about" => {
                let _ = event.window().get_window("about").unwrap().show();
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
