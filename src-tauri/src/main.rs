#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use oath::{totp_generate, TotpSecret, Digits};
use std::{
  sync::{Arc, Mutex, MutexGuard},
  time::{SystemTime, UNIX_EPOCH},
};

const TIMESTEP_OFFSET: u32 = 60;
const DIGITS: Digits = Digits::Six;

struct State {
  secret: Arc<Mutex<TotpSecret>>,
}

fn main() {
  let secret_key = "\x30\x31\x32\x33".to_owned().into();
  let secret: Arc<Mutex<TotpSecret>> = Arc::new(Mutex::new(secret_key));
  let state = State { secret };
  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![
        set_key,
        get_key,
        get_totp,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn set_key(state: tauri::State<'_, State>, new_secret: String) -> Result<(), ()> {
  let arc: Arc<Mutex<TotpSecret>> = state.secret.clone();
  let mut secret: MutexGuard<TotpSecret> = arc.lock().unwrap();
  let new: TotpSecret = new_secret.into();
  *secret = new;
  Ok(())
}


#[tauri::command]
async fn get_key(state: tauri::State<'_, State>) -> Result<String, ()> {
  let arc: Arc<Mutex<TotpSecret>> = state.secret.clone();
  let mutex_guard: MutexGuard<TotpSecret> = arc.lock().unwrap();
  let secret: TotpSecret = mutex_guard.clone();
  let ret: String = secret.into();
  Ok(ret)
}

#[tauri::command]
async fn get_totp(state: tauri::State<'_, State>) -> Result<String, ()> {
  let current_time = SystemTime::now();
  let duration = current_time.duration_since(UNIX_EPOCH).unwrap();
  let now = duration.as_secs() as i64;
  let arc: Arc<Mutex<TotpSecret>> = state.secret.clone();
  let mutex_guard: MutexGuard<TotpSecret> = arc.lock().unwrap();
  let secret: &TotpSecret = &*mutex_guard;
  Ok(totp_generate(secret, now, TIMESTEP_OFFSET, 0, DIGITS))
}
