//! Registro em texto (`ddraw.log`, ao lado do exe). Recriado a cada execucao; o da execucao anterior
//! fica em `ddraw.anterior.log` (o Lucas fechava e reabria o jogo e perdia o relatorio da comparacao).

use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

static ARQUIVO: Mutex<Option<File>> = Mutex::new(None);
static NOME: Mutex<&'static str> = Mutex::new("ddraw.log");

/// Sets the log file name (next to the exe) before the first line; the default is `ddraw.log`.
pub fn definir_nome(nome: &'static str) {
    *NOME.lock().unwrap_or_else(|e| e.into_inner()) = nome;
}

pub fn linha(texto: &str) {
    let mut arq = ARQUIVO.lock().unwrap_or_else(|e| e.into_inner());
    if arq.is_none() {
        let nome = *NOME.lock().unwrap_or_else(|e| e.into_inner());
        let caminho = std::env::current_exe().map(|p| p.with_file_name(nome)).unwrap_or_else(|_| nome.into());
        let _ = std::fs::rename(&caminho, caminho.with_extension("anterior.log"));
        *arq = File::create(caminho).ok();
    }
    if let Some(f) = arq.as_mut() {
        let _ = writeln!(f, "{}", texto);
    }
}
