//! Hooks a DLL may set at load time; unset, they do nothing.
//! - `ramo`: branch counters (the project's comparison mode counts them).
//! - `visivel`: allocations the game keeps (the project's simulation puts them in a fixed region).
//!
//! [pt] Ganchos que uma DLL pode ligar na carga; desligados, não fazem nada.
//! - `ramo`: contadores de ramo (o modo comparação do projeto os conta).
//! - `visivel`: alocações que o jogo guarda (a simulação do projeto as põe numa região fixa).

use core::sync::atomic::{AtomicUsize, Ordering};

static RAMO: AtomicUsize = AtomicUsize::new(0);
static ENTRAR: AtomicUsize = AtomicUsize::new(0);
static SAIR: AtomicUsize = AtomicUsize::new(0);

/// Sets the branch counter.
pub fn definir_ramo(f: fn(&'static str)) {
    RAMO.store(f as usize, Ordering::Relaxed);
}

/// Sets the pair that wraps allocations the game keeps: `entrar` returns the previous state, `sair` restores it.
pub fn definir_visivel(entrar: fn() -> bool, sair: fn(bool)) {
    ENTRAR.store(entrar as usize, Ordering::Relaxed);
    SAIR.store(sair as usize, Ordering::Relaxed);
}

pub fn ramo(nome: &'static str) {
    let f = RAMO.load(Ordering::Relaxed);
    if f != 0 {
        unsafe { core::mem::transmute::<usize, fn(&'static str)>(f)(nome) }
    }
}

pub fn visivel<R>(f: impl FnOnce() -> R) -> R {
    let (e, s) = (ENTRAR.load(Ordering::Relaxed), SAIR.load(Ordering::Relaxed));
    if e == 0 {
        return f();
    }
    let antes = unsafe { core::mem::transmute::<usize, fn() -> bool>(e)() };
    let r = f();
    unsafe { core::mem::transmute::<usize, fn(bool)>(s)(antes) };
    r
}
