// ---------- %%LEVA%%a leva: as telas da partida (1): o rótulo do controle, fechar a tela e a espera (match_screens.cpp) ----------

/// { 0x725a70 } (cdecl, match_screens.cpp): o rótulo do botão de controle da partida para o estado
/// `estado` (1 esperando, 2 começar, 3 recomeçar, 4 pausar/retomar, 5 continuar ou esperar os
/// jogadores) em `saida` pela 0x66f4e0 (o texto passa pelo idioma), com as marcas 0x1021 (mais 0x40
/// quando o botão fica desligado). Sem partida, sem saída, com menos de 100 bytes ou estado fora: a
/// original (caixa de erro). Devolve 1.
unsafe extern "C" fn match_control_label(estado: u8, m: usize, saida: *mut u8, tam: i32, marcas: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u8, usize, *mut u8, i32, *mut u32) -> u32>(%%I0%%);
    if m == 0 || saida.is_null() || tam < 100 || !(1..=5).contains(&estado) {
        return o(estado, m, saida, tam, marcas);
    }
    let texto: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    *saida = 0;
    *marcas = 0x1021;
    let humano = *(0xb77c7a as *const i16) as isize;
    let (formato, desligado) = match estado {
        1 => (0xa16d8c, false),
        2 => (0xa16d7c, false),
        3 => (0xa16d6c, false),
        4 => {
            if core::ptr::read_unaligned((m as isize + 0xc5 + 4 * humano) as *const u32) != 0 {
                (0xa16d5c, false)
            } else if *((m + 0x108) as *const u8) != 0 {
                (0xa16d54, true)
            } else {
                (0xa16d48, false)
            }
        }
        _ => {
            let esperando: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x7ffe40usize);
            let jogador = *((0xb74c26 + humano * 0x300) as *const u32);
            if esperando(0xb74450, jogador) != 0 { (0xa16d34, true) } else { (0xa16d24, false) }
        }
    };
    crate::comparison::branch("screens control label");
    texto(saida, formato);
    if desligado {
        *marcas |= 0x40;
    }
    1
}

/// { 0x71b120 } (cdecl, match_screens.cpp): fecha a tela da partida: os parâmetros 0x2d e 0x1d do
/// gerente de telas [0xb74450] (0x7ff010), se houver, são destruídos (0x5a8f60 e 0x453bc0), liberados
/// (0x944c94) e zerados (0x7ff270).
unsafe extern "C" fn close_match_screen() {
    crate::comparison::branch("screens close the match");
    let ler: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7ff010usize);
    let gravar: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x7ff270usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    for (k, destrutor) in [(0x2du32, 0x5a8f60usize), (0x1d, 0x453bc0)] {
        if ler(0xb74450, k) == 0 {
            continue;
        }
        let p = ler(0xb74450, k);
        if p != 0 {
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destrutor);
            destruir(p);
            liberar(p);
        }
        gravar(0xb74450, k, 0, 0);
    }
}

/// { 0x727f00 } (cdecl, match_screens.cpp): abre a tela de espera na vaga `vaga` do gerente de telas
/// (0x7fe4a0 com o desenho 0x727fa0 e a resposta 0x727fb0). Vaga 0: a original (caixa de erro).
unsafe extern "C" fn open_waiting_screen(vaga: u32) {
    if vaga == 0 {
        return original_fn::<unsafe extern "C" fn(u32)>(%%I2%%)(vaga);
    }
    crate::comparison::branch("screens open the wait");
    let abrir: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, u32, u32) = core::mem::transmute(0x7fe4a0usize);
    abrir(0xb74450, vaga, 0x727fa0, 0x727fb0, 0, 0, 0);
}

unsafe extern "C" fn compare_match_control_label(estado: u8, m: usize, saida: *mut u8, tam: i32, marcas: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u8, usize, *mut u8, i32, *mut u32) -> u32>(%%I0%%);
    if m == 0 || saida.is_null() || tam < 100 || marcas.is_null() {
        return o(estado, m, saida, tam, marcas);
    }
    let reg = [(saida as usize, tam.min(0x100) as usize), (marcas as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(estado, m, saida, tam, marcas), || match_control_label(estado, m, saida, tam, marcas));
    SCOREBOARD.record(igual && r == n, || format!("match_control_label(estado {}) original={} rust={} {:x?} x {:x?}", estado, r, n, mo, mr));
    r
}

// ponytail: fechar e abrir mexem no gerente de telas e na memória (não rodam duas vezes): no modo
// comparação só a original roda; as cópias se provam no jogo do Lucas em modo Rust (JOGAR.bat), com
// os contadores "telas ..." no ddraw.log
unsafe extern "C" fn compare_close_match_screen() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

unsafe extern "C" fn compare_open_waiting_screen(vaga: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(%%I2%%)(vaga)
}
