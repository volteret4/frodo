use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use serde::{Deserialize, Serialize};

const SIDEBAR_WIDTH: i32 = 64;
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub url: String,
    pub color: String,
    pub preset: Option<String>,
}

pub struct AppState {
    pub services: Mutex<Vec<Service>>,
    pub active_id: Mutex<Option<String>>,
}

fn data_path(app: &AppHandle) -> std::path::PathBuf {
    app.path().app_data_dir().unwrap().join("services.json")
}

fn load_services(app: &AppHandle) -> Vec<Service> {
    let path = data_path(app);
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_services(app: &AppHandle, services: &[Service]) {
    let path = data_path(app);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    if let Ok(content) = serde_json::to_string_pretty(services) {
        std::fs::write(path, content).ok();
    }
}

// ─── Linux GTK multi-view ────────────────────────────────────────────────────

#[cfg(target_os = "linux")]
mod linux {
    use super::SIDEBAR_WIDTH;
    use super::USER_AGENT;
    use gtk::prelude::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use webkit2gtk::{SettingsExt, WebViewExt};

    thread_local! {
        static VBOX: RefCell<Option<gtk::Box>> = RefCell::new(None);
        static VIEWS: RefCell<HashMap<String, webkit2gtk::WebView>> = RefCell::new(HashMap::new());
    }

    fn make_webview(url: &str) -> webkit2gtk::WebView {
        let settings = webkit2gtk::Settings::new();
        settings.set_user_agent(Some(USER_AGENT));
        settings.set_enable_javascript(true);
        settings.set_javascript_can_open_windows_automatically(true);
        settings.set_enable_media_stream(true);
        settings.set_media_playback_requires_user_gesture(false);
        settings.set_enable_encrypted_media(true);
        let wv = webkit2gtk::WebView::with_settings(&settings);
        wv.load_uri(url);
        wv
    }

    pub fn setup(vbox: gtk::Box) {
        use gtk::PackType;
        vbox.set_orientation(gtk::Orientation::Horizontal);
        // Constrain the main webview (sidebar) to SIDEBAR_WIDTH px
        if let Some(child) = vbox.children().into_iter().next() {
            child.set_size_request(SIDEBAR_WIDTH, -1);
            vbox.set_child_packing(&child, false, false, 0, PackType::Start);
        }
        VBOX.with(|v| *v.borrow_mut() = Some(vbox));
    }

    pub fn activate(service_id: Option<&str>, url: Option<&str>) {
        VIEWS.with(|views| {
            let mut views = views.borrow_mut();
            // Hide all
            for wv in views.values() {
                wv.set_visible(false);
            }
            let (Some(sid), Some(url)) = (service_id, url) else {
                // No service — let main webview fill the window
                VBOX.with(|v| {
                    if let Some(ref vbox) = *v.borrow() {
                        if let Some(child) = vbox.children().into_iter().next() {
                            child.set_size_request(-1, -1);
                            vbox.set_child_packing(&child, true, true, 0, gtk::PackType::Start);
                        }
                    }
                });
                return;
            };
            // Constrain main webview back to sidebar width
            VBOX.with(|v| {
                if let Some(ref vbox) = *v.borrow() {
                    if let Some(child) = vbox.children().into_iter().next() {
                        child.set_size_request(SIDEBAR_WIDTH, -1);
                        vbox.set_child_packing(&child, false, false, 0, gtk::PackType::Start);
                    }
                }
            });
            // Show or create the service webview
            if let Some(wv) = views.get(sid) {
                wv.set_visible(true);
            } else {
                VBOX.with(|v| {
                    if let Some(ref vbox) = *v.borrow() {
                        let wv = make_webview(url);
                        vbox.pack_start(&wv, true, true, 0);
                        wv.show();
                        views.insert(sid.to_string(), wv);
                    }
                });
            }
        });
    }

    pub fn remove(service_id: &str) {
        VIEWS.with(|views| {
            let mut views = views.borrow_mut();
            if let Some(wv) = views.remove(service_id) {
                VBOX.with(|v| {
                    if let Some(ref vbox) = *v.borrow() {
                        vbox.remove(&wv);
                    }
                });
            }
        });
    }
}

// ─── Tauri commands ──────────────────────────────────────────────────────────

#[tauri::command]
fn get_services(state: State<AppState>) -> Vec<Service> {
    state.services.lock().unwrap().clone()
}

#[tauri::command]
fn add_service(app: AppHandle, state: State<AppState>, service: Service) {
    let mut services = state.services.lock().unwrap();
    services.push(service);
    save_services(&app, &services);
}

#[tauri::command]
fn remove_service(app: AppHandle, state: State<AppState>, id: String) {
    {
        let mut services = state.services.lock().unwrap();
        services.retain(|s| s.id != id);
        save_services(&app, &services);
    }
    let mut active = state.active_id.lock().unwrap();
    if active.as_deref() == Some(&id) {
        *active = None;
    }

    #[cfg(target_os = "linux")]
    {
        let id_clone = id.clone();
        app.run_on_main_thread(move || linux::remove(&id_clone)).ok();
    }
    #[cfg(not(target_os = "linux"))]
    let _ = app;
}

#[tauri::command]
fn activate_service(
    app: AppHandle,
    state: State<AppState>,
    id: Option<String>,
) -> Result<(), String> {
    let services = state.services.lock().unwrap().clone();

    let service_url: Option<String> = id.as_ref().and_then(|sid| {
        services.iter().find(|s| &s.id == sid).map(|s| s.url.clone())
    });

    if id.is_some() && service_url.is_none() {
        return Err(format!("service {:?} not found", id));
    }

    #[cfg(target_os = "linux")]
    {
        let id_clone = id.clone();
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        app.run_on_main_thread(move || {
            linux::activate(id_clone.as_deref(), service_url.as_deref());
            tx.send(()).ok();
        })
        .map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?;
    }

    *state.active_id.lock().unwrap() = id;
    Ok(())
}

// No-op on Linux since webviews are embedded; kept for API compatibility.
#[tauri::command]
fn update_webview_bounds(_app: AppHandle) {}

// ─── App setup ───────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let services = load_services(app.handle());
            app.manage(AppState {
                services: Mutex::new(services),
                active_id: Mutex::new(None),
            });

            #[cfg(target_os = "linux")]
            {
                let main_ww = app.get_webview_window("main").unwrap();
                let vbox = main_ww.default_vbox().unwrap();
                linux::setup(vbox);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_services,
            add_service,
            remove_service,
            activate_service,
            update_webview_bounds,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
