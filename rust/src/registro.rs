//! Registro em texto (`ddraw.log`, ao lado do exe). Recriado a cada execucao.

use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

static ARQUIVO: Mutex<Option<File>> = Mutex::new(None);

pub fn linha(texto: &str) {
    let mut arq = ARQUIVO.lock().unwrap_or_else(|e| e.into_inner());
    if arq.is_none() {
        let caminho = std::env::current_exe()
            .map(|p| p.with_file_name("ddraw.log"))
            .unwrap_or_else(|_| "ddraw.log".into());
        *arq = File::create(caminho).ok();
    }
    if let Some(f) = arq.as_mut() {
        let _ = writeln!(f, "{}", texto);
    }
}
