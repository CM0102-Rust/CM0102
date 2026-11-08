// ---------- %%LEVA%%a leva: as telas da partida (6): a tela dos jogos do dia e o capitão padrão (match_screens.cpp) ----------

/// { 0x717cc0 } (cdecl, match_screens.cpp): abre a tela dos jogos do dia na vaga `vaga` (0x7fe4a0 com a
/// montagem 0x717db0 e a resposta 0x718930, sem match_wrap_up) e grava os parâmetros dela (0x7ff130): 1 o
/// grupo de jogos, 2 e 3 zero, 4 um. Grupo nulo: a original (caixa de erro e [0xb67a34] = 0).
unsafe extern "C" fn open_day_fixtures_screen(vaga: u32, grupo: usize) {
    if grupo == 0 {
        return original_fn::<unsafe extern "C" fn(u32, usize)>(%%I0%%)(vaga, grupo);
    }
    crate::comparacao::ramo("telas abrir os jogos do dia");
    let abrir: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(0x7fe4a0usize);
    if abrir(GERENTE_DE_TELAS, vaga, 0x717db0, 0x718930, 0, 0, 0) == 0 {
        return;
    }
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32) = core::mem::transmute(0x7ff130usize);
    for (k, v) in [(1u32, grupo), (2, 0), (3, 0), (4, 1)] {
        gravar(GERENTE_DE_TELAS, vaga, k, v, 0);
    }
}

/// { 0x718930 } (cdecl, match_screens.cpp): a resposta da tela dos jogos do dia ao item `item` (−1 =
/// nenhum). Com item: o parâmetro 2 recebe 0x803320 do parâmetro 3. Pelo id do item (0 vira o padrão
/// [0xdd6bde]): 1 abre outro grupo de jogos (0x717cc0 na vaga [0xb74c26] do humano da vez com o dado
/// +0xba9a6 do item) e 2 o andamento da competição (0x49b830 com o dado), ambos −11; senão a barra de
/// menu (0x7615d0, senão 0x7643d0). O item vai inteiro (32 bits) para a barra, como na original.
unsafe extern "C" fn handle_day_fixtures_input(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond aos jogos do dia");
    let i = item as i16;
    let id = |i: i16| if i == -1 { *(0xdd6bde as *const u16) } else { display_item_id(i) };
    let dado = |i: i16| {
        if i == -1 {
            return 0usize;
        }
        let display = *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
        core::ptr::read_unaligned((display as isize + i as isize * 0x18c + 0xba9a6) as *const usize)
    };
    if i != -1 {
        let converter: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x803320usize);
        set_screen_param(2, converter(GERENTE_DE_TELAS, get_screen_param(3)));
    }
    match id(i) {
        1 => {
            let vaga = *((0xb74c26 + *(0xb77c7a as *const i16) as isize * 0x300) as *const u32);
            let abrir: unsafe extern "C" fn(u32, usize) = core::mem::transmute(0x717cc0usize);
            abrir(vaga, dado(i));
            -11
        }
        2 => {
            let andamento: unsafe extern "C" fn(usize) = core::mem::transmute(0x49b830usize);
            andamento(dado(i));
            -11
        }
        _ => {
            let comando: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7615d0usize);
            let r = comando(item);
            if r != 0 {
                return r;
            }
            let ligacao: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7643d0usize);
            ligacao(item)
        }
    }
}

/// { 0x727e20 } (cdecl, match_screens.cpp): a vaga (0..10) dos onze da tática `tatica` com a maior nota
/// da comissão (0x8968f0 em [0xde1d08] com `contexto`, nota 0x6e1cf0(pessoa, 0, tatica), comparação i16;
/// no empate fica a primeira), o capitão padrão; −1 (0xff) sem ninguém. Tática nula: a original (caixa
/// de erro). Quem chama só usa `al`.
unsafe extern "C" fn match_screens_find_best_rated_starter_slot(tatica: usize, contexto: i32) -> u8 {
    if tatica == 0 {
        return original_fn::<unsafe extern "C" fn(usize, i32) -> u8>(%%I2%%)(tatica, contexto);
    }
    let da_tatica: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> usize = core::mem::transmute(0x8968f0usize);
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(0x6e1cf0usize);
    let mut vaga = 0xffu8;
    let mut melhor = 0i16;
    for k in 0..11u32 {
        let p = da_tatica(*(0xde1d08 as *const usize), tatica, k, contexto);
        if p != 0 {
            let n = avaliar(p, 0, tatica);
            if vaga == 0xff || n > melhor {
                vaga = k as u8;
                melhor = n;
            }
        }
    }
    vaga
}

// ponytail: abrir e respond mexem em telas e menu (não rodam duas vezes): no modo comparação só a
// original roda; as cópias se provam no jogo do Lucas em modo Rust (contadores "telas ..." no ddraw.log)
unsafe extern "C" fn comparar_open_day_fixtures_screen(vaga: u32, grupo: usize) {
    original_fn::<unsafe extern "C" fn(u32, usize)>(%%I0%%)(vaga, grupo)
}

unsafe extern "C" fn comparar_handle_day_fixtures_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%)(item)
}

/// Só lê: compara a vaga devolvida (tática nula: só a original, pela caixa de erro).
unsafe extern "C" fn comparar_match_screens_find_best_rated_starter_slot(tatica: usize, contexto: i32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, i32) -> u8>(%%I2%%)(tatica, contexto);
    if tatica == 0 {
        return r;
    }
    let n = match_screens_find_best_rated_starter_slot(tatica, contexto);
    PLACAR.registrar(r == n, || format!("match_screens_find_best_rated_starter_slot({:#x}, {}) {} x {}", tatica, contexto, r as i8, n as i8));
    r
}
