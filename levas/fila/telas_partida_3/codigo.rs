// ---------- %%LEVA%%a leva: as telas da partida (3): a tela de detalhes da partida (match_screens.cpp) ----------

/// Lê e grava os parâmetros da tela atual do gerente de telas [0xb74450] (0x7ff010, 0x7ff270).
unsafe fn get_screen_param(k: u32) -> usize {
    let ler: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7ff010usize);
    ler(0xb74450, k)
}
unsafe fn set_screen_param(k: u32, v: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x7ff270usize);
    gravar(0xb74450, k, v, 0);
}

/// { 0x71b4a0 } (cdecl, match_screens.cpp): abre a tela de detalhes da partida `partida` (0x7fe600 com a
/// montagem 0x71b630, a resposta 0x71bdc0 e o fechamento 0x71c170) e grava os parâmetros dela (a
/// partida em 0x1e; a aba 2 começa em 6).
unsafe extern "C" fn open_match_details(partida: usize) {
    crate::comparison::branch("telas abrir os detalhes");
    let abrir: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(0x7fe600usize);
    if abrir(0xb74450, 0x71b630, 0x71bdc0, 0, 0x71c170, 0) == 0 {
        return;
    }
    const MENOS1: usize = usize::MAX;
    for (k, v) in [
        (0u32, 0usize), (2, 6), (9, MENOS1), (0xa, 3), (0xb, MENOS1), (0xc, 0), (0xd, MENOS1), (0x16, 0), (0x17, 0), (0x18, 0), (0x19, 0),
        (0x1a, MENOS1), (0x1b, 0), (0x1c, 1), (0x1d, 0), (0x1e, partida), (0x1f, 0), (0x20, 0), (0x21, MENOS1), (0x22, 0), (0x23, MENOS1), (0x2d, 0),
    ] {
        set_screen_param(k, v);
    }
}

/// { 0x71bdc0 } (cdecl, match_screens.cpp): a resposta da tela de detalhes ao item `item` (−1 nenhum).
/// Sem [0xb77c84]: zera 0xb6b7a8/ac/b0; um item de id 1 (ou [0xdd6bde]) troca a aba (parâmetro 2) pelo
/// dado do item (+0x48 do item no `Display` do humano da vez) e pede −11; de id 2 abre a tática do clube
/// do item (0x899a50) e devolve −4. Depois, pela aba: 1 posse (0x71d9e0), 2 estatísticas (0x71e750), 3
/// zonas de ação (0x720320; e um item de id 4 troca a ação do parâmetro 0xa), 4 relatório, 5 e 7 as
/// estatísticas do time (0x722dc0), 6 as notas (0x723350); os redesenhos só com a marca 4 do menu da
/// partida ([[parâmetro 0]+0x5b]) e [0xb6b7ac]. Por fim −11, o redesenho (0x802d00) e a barra de menu.
unsafe extern "C" fn match_details_input(item: u32) -> i32 {
    crate::comparison::branch("telas respond aos detalhes");
    let g = |a: usize| *(a as *const u32);
    let item16 = item as u16 as i16;
    let current_human_display = || *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
    let id_ou_padrao = |w: u16| if w == 0 { *(0xdd6bde as *const u16) } else { w };
    if g(0xb77c84) == 0 {
        let aba = get_screen_param(2) as u8 as i8;
        let v1e = get_screen_param(0x1e);
        let p1f = get_screen_param(0x1f);
        *(0xb6b7ac as *mut u32) = 0;
        *(0xb6b7b0 as *mut u32) = 0;
        *(0xb6b7a8 as *mut u32) = 0;
        if item16 != -1 {
            let deslocamento = item16 as isize * 0x18c;
            let registro = |d: usize| (d as isize + deslocamento) as usize;
            let e = registro(current_human_display());
            if id_ou_padrao(core::ptr::read_unaligned((e + 0xba966) as *const u16)) == 1 {
                let dado = core::ptr::read_unaligned((e + 0xba9a6) as *const usize);
                if dado != get_screen_param(2) {
                    set_screen_param(2, dado);
                    *(0xb6b7b0 as *mut u32) = 1;
                }
            }
            let e = registro(current_human_display());
            if id_ou_padrao(core::ptr::read_unaligned((e + 0xba966) as *const u16)) == 2 {
                let clube = core::ptr::read_unaligned((e + 0xba9a6) as *const usize);
                let marca = if clube == *((p1f + 0x1c) as *const usize) { *((p1f + 0x40) as *const u8) } else { *((p1f + 0x41) as *const u8) };
                let tatica: unsafe extern "C" fn(usize, u32, u32, u32, usize, u32) = core::mem::transmute(0x899a50usize);
                tatica(clube, (marca == 0) as u32, 0, 0, v1e, 0);
                return -4;
            }
        }
        let menu_da_partida = |bit: u8| {
            let p = get_screen_param(0);
            g(0xb6b7ac) != 0 && *(*((p + 0x5b) as *const usize) as *const u8) & bit != 0
        };
        let refazer = |f: usize| -> bool {
            // os redesenhos de aba: com a marca 4; com [0xb6b7b0] ligado a resposta é −11
            if g(0xb6b7b0) != 0 {
                return true;
            }
            let r: unsafe extern "C" fn() = core::mem::transmute(f);
            r();
            *(0xb6b7a8 as *mut u32) = 1;
            false
        };
        let estatisticas_do_time: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x722dc0usize);
        match aba {
            1 => {
                let p = get_screen_param(0);
                if g(0xb6b7ac) != 0 {
                    let m = *(*((p + 0x5b) as *const usize) as *const u8);
                    if m & 0x20 != 0 || m & 0x10 != 0 {
                        *(0xb6b7b0 as *mut u32) = 1;
                    }
                    if m & 4 != 0 && refazer(0x71d9e0) {
                        return -11;
                    }
                }
            }
            2 => {
                if menu_da_partida(4) && refazer(0x71e750) {
                    return -11;
                }
            }
            3 => {
                if menu_da_partida(4) && g(0xb6b7b0) == 0 {
                    refazer(0x720320);
                }
                if item16 != -1 {
                    let d = current_human_display();
                    let id: unsafe extern "thiscall" fn(usize, u32) -> u16 = core::mem::transmute(0x468b10usize);
                    if id_ou_padrao(id(d, item)) == 4 {
                        let dado: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x4739a0usize);
                        if dado(d, item) != get_screen_param(0xa) {
                            let acao: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x468b90usize);
                            set_screen_param(0xa, acao(0xb74450, item));
                            *(0xb6b7b0 as *mut u32) = 1;
                            return -11;
                        }
                    }
                }
            }
            4 => {
                if menu_da_partida(0x20) {
                    *(0xb6b7b0 as *mut u32) = 1;
                    return -11;
                }
            }
            5 | 7 => {
                let r = estatisticas_do_time(item);
                if r != 0 {
                    return r;
                }
            }
            6 => {
                if menu_da_partida(4) && refazer(0x723350) {
                    return -11;
                }
            }
            _ => {}
        }
        if g(0xb6b7b0) != 0 {
            return -11;
        }
        if g(0xb6b7a8) != 0 {
            let redesenhar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x802d00usize);
            redesenhar(0xb74450, 0, 0);
        }
    }
    let comando: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7615d0usize);
    let r = comando(item);
    if r != 0 {
        return r;
    }
    let ligacao: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7643d0usize);
    ligacao(item)
}

/// { 0x71c170 } (cdecl, match_screens.cpp): fecha a tela de detalhes: os parâmetros 0x2d (0x5a8f60),
/// 0x1d e 0x20 (0x453bc0) são destruídos e liberados, o 0x1f só liberado (0x944c94), e todos zerados.
unsafe extern "C" fn close_match_details() {
    crate::comparison::branch("telas fechar os detalhes");
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    for (k, destrutor) in [(0x2du32, Some(0x5a8f60usize)), (0x1d, Some(0x453bc0)), (0x1f, None), (0x20, Some(0x453bc0))] {
        if get_screen_param(k) == 0 {
            continue;
        }
        let p = get_screen_param(k);
        match destrutor {
            Some(d) => {
                if p != 0 {
                    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(d);
                    destruir(p);
                    liberar(p);
                }
            }
            None => liberar(p),
        }
        set_screen_param(k, 0);
    }
}

// ponytail: as três mexem em telas, menus e memória (não rodam duas vezes): no modo comparação só a
// original roda; as cópias se provam no jogo do Lucas em modo Rust (contadores "telas ..." no ddraw.log)
unsafe extern "C" fn comparar_open_match_details(partida: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(partida)
}

unsafe extern "C" fn comparar_match_details_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%)(item)
}

unsafe extern "C" fn comparar_close_match_details() {
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}
