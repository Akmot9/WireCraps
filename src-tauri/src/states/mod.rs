use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tauri::AppHandle;

pub struct SonarState {
    matrice: Arc<Mutex<HashMap<String, u32>>>,
    filter: Arc<Mutex<bool>>,
}

impl SonarState {
    pub fn new(app: AppHandle) -> Self {
        SonarState {
            matrice: Arc::new(Mutex::new(HashMap::new())),
            filter: Arc::new(Mutex::new(true)),
        }
    }
}
