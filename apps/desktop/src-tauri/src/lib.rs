use std::sync::Mutex;
use std::{thread, time::Duration};
use tauri::Emitter;

struct CounterState {
    count: Mutex<i32>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]

//for normal greet fn
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

//for returning the addition of 2 no
#[tauri::command]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tauri::command]
fn increment_counter(state: tauri::State<CounterState>) -> i32 {
    let mut count = state.count.lock().unwrap();

    *count += 1;
    *count
}

//for market tick
#[tauri::command]
fn start_ticks(app: tauri::AppHandle) {
    thread::spawn(move || {
        let mut price = 100.0;

        loop {
            price += 0.25;

            app.emit("tick", price).unwrap();

            thread::sleep(Duration::from_millis(1000));
        }
    });
}

#[tauri::command]
async fn run_backtest() -> String {
    tokio::time::sleep(
        Duration::from_secs(3)
    ).await;
    "Backtest complete".to_string()
}
pub fn run() {
    tauri::Builder::default()
        .manage(CounterState {
            count: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            add,
            start_ticks,
            increment_counter,
            run_backtest
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
