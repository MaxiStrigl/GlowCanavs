use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Serialize,Deserialize, Clone)]
enum Mode {
    Draw,
    Erase
}

#[derive(Serialize, Deserialize, Clone)]
struct Stroke {
    points: Vec<(f64, f64)>,
    color: i32,
    mode: Mode,
}

lazy_static! {
    static ref STROKES: Mutex<Vec<Stroke>> = Mutex::new(Vec::new());
    static ref CURRENT_STROKE: Mutex<Stroke> = Mutex::new(Stroke { points: Vec::new(), color: 0,  mode: Mode::Draw});
}

#[tauri::command]
fn start_stroke(point: (f64, f64)) -> Stroke {
    let mut stroke = CURRENT_STROKE.lock().unwrap();
    stroke.points = vec![point];
    stroke.clone()
}

#[tauri::command]
fn finish_stroke(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    points
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![start_stroke, finish_stroke])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
