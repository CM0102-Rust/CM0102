// ---------- %%LEVA%%a leva: as telas da partida (11): os valores das estatísticas da partida, das zonas de ação e das notas dos jogadores (match_screens.cpp) ----------

/// Guarda em `copia` (o parâmetro 0x1d) o estado da partida `estado` (0xd7a bytes), como as três
/// rotinas de valores fazem no fim (a cópia é a 0x71dd70 / 0x71df00 em linha): campos de 4, 2 e 1
/// bytes até 0x11f, os dois blocos de time de 0x625 bytes e o fim 0xd69..0xd7a. Sem cópia ainda:
/// aloca (0x944cf1), constrói os dois blocos (0x9463af com 0x453c30) e grava no parâmetro 0x1d.
unsafe fn save_match_snapshot(mut copia: usize, estado: usize) {
    if copia == 0 {
        crate::comparison::branch("screens create the match state copy");
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
        let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
        copia = novo(0xd7a);
        if copia != 0 {
            vetor(copia + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        set_screen_param(0x1d, copia);
    }
    copy_fields(copia, estado, 0, 8, 4);
    copy_fields(copia, estado, 8, 0x34, 2);
    copy_fields(copia, estado, 0x34, 0x4c, 4);
    copy_fields(copia, estado, 0x4c, 0x4d, 1);
    copy_fields(copia, estado, 0x4d, 0x53, 2);
    copy_fields(copia, estado, 0x53, 0x11f, 4);
    for t in [0x11f, 0x11f + 0x625] {
        let (d, s) = (copia + t, estado + t);
        copy_fields(d, s, 0, 0x11, 1);
        copy_fields(d, s, 0x11, 0x1f, 2);
        copy_fields(d, s, 0x1f, 0x35, 1);
        copy_fields(d, s, 0x35, 0x61, 2);
        copy_fields(d, s, 0x61, 0x69, 4);
        copy_fields(d, s, 0x69, 0x71, 2);
        for j in 0..20 {
            match_player_row_assign(d + 0x71 + j * 0x49, s + 0x71 + j * 0x49);
        }
    }
    copy_fields(copia, estado, 0xd69, 0xd75, 4);
    copy_fields(copia, estado, 0xd75, 0xd76, 1);
    copy_fields(copia, estado, 0xd76, 0xd7a, 4);
}

/// O estado da partida que as rotinas de valores mostram: na partida (parâmetro 0) [p0 + 0x5f], fora
/// dela o parâmetro 0x20 (lido depois dos outros, como a original).
unsafe fn displayed_match_state(p0: usize) -> usize {
    if p0 != 0 {
        core::ptr::read_unaligned((p0 + 0x5f) as *const usize)
    } else {
        get_screen_param(0x20)
    }
}

const EMPTY_TEXT: usize = 0xad9c64;
const FORMAT_D: usize = 0x98806c; // "%d"
const FORMAT_PCT: usize = 0x98e8d4; // "%d%%"

/// { 0x71e750 } (cdecl, match_screens.cpp): passa às células das estatísticas da partida (a partir do
/// item do parâmetro 9) os valores do estado que mudaram desde a cópia do parâmetro 0x1d (sem cópia,
/// todos): os bytes dos dois times ("%d"; +0x127.. e +0x74c..), as duas diferenças +0x127 − +0x128 e
/// +0x74c − +0x74d e seis percentuais (num × 100 / den, "%d%%", ou "-" com den 0; o último compara
/// +0x75d mas mostra 100 − o do +0x138, como a original). No fim guarda o estado na cópia.
unsafe extern "C" fn refresh_match_stats_values() {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let texto: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x801c70usize);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let p0 = get_screen_param(0);
    let item = get_screen_param(9) as u32;
    let copia = get_screen_param(0x1d);
    let e = displayed_match_state(p0);
    let u8_ = |b: usize, k: usize| *((b + k) as *const u8) as i32;
    let i16_ = |b: usize, k: usize| core::ptr::read_unaligned((b + k) as *const i16) as i32;
    let byte = |k: usize, d: u32| {
        if copia == 0 || u8_(e, k) != u8_(copia, k) {
            formatar(BUF, FORMAT_D, u8_(e, k));
            texto(sm, item.wrapping_add(d), BUF);
        }
    };
    let diferenca = |a: usize, b: usize, d: u32| {
        if copia == 0 || u8_(e, a) - u8_(e, b) != u8_(copia, a) - u8_(copia, b) {
            formatar(BUF, FORMAT_D, u8_(e, a) - u8_(e, b));
            texto(sm, item.wrapping_add(d), BUF);
        }
    };
    let write_percent_pair = |muda: usize, den: usize, num: usize, d: u32, inverter: bool| {
        if copia == 0 || i16_(e, muda) != i16_(copia, muda) {
            if i16_(e, den) == 0 {
                *(BUF as *mut [u8; 2]) = *b"-\0";
            } else {
                let x = i16_(e, num) * 100 / i16_(e, den);
                formatar(BUF, FORMAT_PCT, if inverter { 100 - x } else { x });
            }
            texto(sm, item.wrapping_add(d), BUF);
        }
    };

    read_i8(0x127 + 0);
    read_i8(0x74c + 2);
    read_i8(0x128 + 3);
    read_i8(0x74d + 5);
    diferenca(0x127, 0x128, 6);
    diferenca(0x74c, 0x74d, 8);
    for (k, d) in [(0x129, 9), (0x74e, 0xb), (0x12a, 0xc), (0x74f, 0xe), (0x12b, 0xf), (0x750, 0x11), (0x12c, 0x12), (0x751, 0x14), (0x12d, 0x15), (0x752, 0x17)] {
        read_i8(k + d);
    }
    write_percent_pair(0x130, 0x130, 0x132, 0x18, false);
    write_percent_pair(0x755, 0x755, 0x757, 0x1a, false);
    write_percent_pair(0x134, 0x134, 0x136, 0x1b, false);
    write_percent_pair(0x759, 0x759, 0x75b, 0x1d, false);
    write_percent_pair(0x138, 0x138, 0x13a, 0x1e, false);
    write_percent_pair(0x75d, 0x138, 0x13a, 0x20, true);
    for (k, d) in [(0x12e, 0x21), (0x753, 0x23), (0x12f, 0x24), (0x754, 0x26)] {
        read_i8(k + d);
    }
    save_match_snapshot(copia, e);
}

/// { 0x720320 } (cdecl, match_screens.cpp): passa às células das zonas de ação os valores que mudaram
/// desde a cópia do parâmetro 0x1d (sem cópia, todos). Os cinco marcadores (parâmetro 0x23 + 0..4;
/// 0x801350 1 e 0x801fc0 4) quando a soma dos bytes +0x767/+0x142, +0x763/+0x13e .. +0x766/+0x141 não é
/// zero (o primeiro só na partida). Pelo lado do parâmetro 0xa (i8): a posse em três (+0xc, +0x16,
/// +0x20, "%d%%" truncado a i8; o terceiro é 100 − os outros) no item do parâmetro 0xb; o tempo de bola
/// em jogo (+0x2a × 60 / 11 segundos, vazio no lado 4); as cores dos dois clubes (0x53b640 com o
/// uniforme +0x152 e +0x777); as três zonas do campo (+0x143, +0x148, +0x14d; 50 sem valor ou sem a
/// marca +0x768/+0x76d/+0x772) em barras entre os parâmetros 0x16 e 0x18 na altura do 0x19; e a barra
/// da posse (+0x13e/+0x763) de 0xd4 a 0x2a6 na altura do parâmetro 0xc. No fim guarda o estado.
unsafe extern "C" fn refresh_action_zones_values() {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzido: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let texto: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x801c70usize);
    let retangulo: unsafe extern "thiscall" fn(usize, u32, i32, i32, i32, i32) = core::mem::transmute(0x800890usize);
    let fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801040usize);
    let ligar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801350usize);
    let estilo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801fc0usize);
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u8, *mut u8) = core::mem::transmute(0x53b640usize);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let p0 = get_screen_param(0);
    let lado = get_screen_param(0xa) as u8 as i8 as isize;
    let b = get_screen_param(0xb) as u32;
    let altura = get_screen_param(0xc) as i32;
    let esq = get_screen_param(0x16) as i32;
    let dir = get_screen_param(0x18) as i32;
    let base = get_screen_param(0x19) as i32;
    let copia = get_screen_param(0x1d);
    let marcas = get_screen_param(0x23) as u32;
    let e = displayed_match_state(p0);
    let u8_ = |x: usize, k: usize| *((x + k) as *const u8) as u32;
    let it = |d: u32| b.wrapping_add(d);

    for (i, a, c) in [(0u32, 0x767, 0x142), (1, 0x763, 0x13e), (2, 0x764, 0x13f), (3, 0x765, 0x140), (4, 0x766, 0x141)] {
        if (i > 0 || p0 != 0) && u8_(e, a) + u8_(e, c) != 0 {
            ligar(sm, marcas.wrapping_add(i), 1);
            estilo(sm, marcas.wrapping_add(i), 4);
        }
    }

    let s = |x: usize, k: isize| core::ptr::read_unaligned((x as isize + lado * 2 + k) as *const i16) as i32;
    if copia == 0 || s(e, 0xc) != s(copia, 0xc) || s(e, 0x16) != s(copia, 0x16) || s(e, 0x20) != s(copia, 0x20) {
        let total = || s(e, 0xc) + s(e, 0x16) + s(e, 0x20);
        let a = if total() == 0 { 0 } else { (s(e, 0xc) * 100 / total()) as i8 as i32 };
        formatar(BUF, FORMAT_PCT, a);
        texto(sm, b, BUF);
        let c = if total() == 0 { 0 } else { (s(e, 0x20) * 100 / total()) as i8 as i32 };
        formatar(BUF, FORMAT_PCT, c);
        texto(sm, it(2), BUF);
        formatar(BUF, FORMAT_PCT, if total() == 0 { 0 } else { 100 - c - a });
        texto(sm, it(1), BUF);
    }
    if copia == 0 || s(e, 0x2a) != s(copia, 0x2a) {
        if lado == 4 {
            *(BUF as *mut u8) = 0;
        } else {
            let x = s(e, 0x2a) * 60 / 11;
            traduzido(BUF, 0xa168b4, x / 60, x % 60);
        }
        texto(sm, it(3), BUF);
    }

    // as cores dos clubes: a primeira saída (+0x3c na pilha da original) é a cor da barra; a segunda
    // saída de cada chamada não é usada. Só os 16 bits de baixo valem (0x801040 lê uma palavra).
    let clube = |k: usize| (*(0xae23b4 as *const usize)).wrapping_add((*((e + k) as *const usize)).wrapping_mul(0x245));
    let (mut cor1, mut cor2, mut resto1, mut resto2) = ([0u8; 2], [0u8; 2], [0u8; 2], [0u8; 2]);
    cores_do_clube(clube(0), u8_(e, 0x152), cor1.as_mut_ptr(), resto1.as_mut_ptr());
    cores_do_clube(clube(4), u8_(e, 0x777), cor2.as_mut_ptr(), resto2.as_mut_ptr());
    let cor = |pct: u32| u16::from_le_bytes(if pct < 0x32 { cor2 } else { cor1 }) as u32;
    let valor = |campo: isize| *((e as isize + lado + campo) as *const u8);
    let mudou = |campo: isize| copia == 0 || valor(campo) != *((copia as isize + lado + campo) as *const u8);
    let pct = |campo: isize, marca: isize| if valor(campo) == 0 || valor(marca) == 0 { 0x32 } else { valor(campo) as u32 };
    let zona = |campo: isize, marca: isize, ini: i32, fim: i32, d: u32| {
        let p = pct(campo, marca);
        let x = (fim - ini - 4) * p as i32 / 100 + ini + 2;
        retangulo(sm, it(d), ini + 2, base - 0x12, x, base - 0xc);
        retangulo(sm, it(d + 1), x, base - 0x12, fim - 2, base - 0xc);
        fundo(sm, it(d + 2), cor(p));
    };
    if mudou(0x143) {
        zona(0x143, 0x768, esq + 10, (dir - esq) / 3 + esq - 10, 6);
    }
    if mudou(0x148) {
        zona(0x148, 0x76d, (dir - esq) / 3 + esq + 10, (dir - esq) * 2 / 3 + esq - 10, 0xb);
    }
    if mudou(0x14d) {
        zona(0x14d, 0x772, (dir - esq) * 2 / 3 + esq + 10, dir - 10, 0x10);
    }
    if mudou(0x13e) {
        let p = pct(0x13e, 0x763);
        formatar(BUF, FORMAT_PCT, p);
        let w = (p * 0x1d2 / 100) as i32 + 0xd4;
        retangulo(sm, it(0x15), 0xd4, altura - 0x23, w, altura - 0x14);
        texto(sm, it(0x15), BUF);
        formatar(BUF, FORMAT_PCT, 100u32.wrapping_sub(p));
        retangulo(sm, it(0x16), w, altura - 0x23, 0x2a6, altura - 0x14);
        texto(sm, it(0x16), BUF);
        fundo(sm, it(0x17), cor(p));
    }
    save_match_snapshot(copia, e);
}

/// { 0x723350 } (cdecl, match_screens.cpp): passa às linhas das notas dos jogadores (7 itens por linha a
/// partir do parâmetro 0xd; 20 linhas por time, a linha só se o item + 6 < 0x4b0, senão pula sem andar)
/// os valores que mudaram desde a cópia do parâmetro 0x1d (sem cópia, todos). Por linha (73 bytes em
/// +0x190): vazia (0xfb em +0x1d) limpa os itens; senão as margens/fundos dos dois cartões (+0x2c e
/// +0x2d), o minuto de lesão, expulsão, saída ou entrada (+0x33, +0x34, +0x30, +0x32, "inj/off/sub/on
/// <%d - minute>", com cor), a nota (+0x35) e os gols (+0x21; vazios para o reserva que não entrou) e a
/// cor do nome (o jogador em +4 igual ao [estado + 0x3c], quem saiu, o reserva que não entrou). No fim
/// guarda o estado.
unsafe extern "C" fn refresh_player_ratings_rows() {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzido: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let texto: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x801c70usize);
    let fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801040usize);
    let margem: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x800d30usize);
    let cor_do_texto: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801960usize);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let p0 = get_screen_param(0);
    let mut item = get_screen_param(0xd) as u32;
    let copia = get_screen_param(0x1d);
    let e = displayed_match_state(p0);
    let atual = core::ptr::read_unaligned((e + 0x3c) as *const u32);
    for t in 0..2usize {
        for j in 0..20usize {
            if item as i16 as i32 + 6 > 0x4af {
                continue;
            }
            let lin = j * 0x49;
            let i = item.wrapping_add(1);
            let r = e + t * 0x625 + lin + 0x190;
            let c = if copia != 0 { copia + t * 0x625 + lin + 0x190 } else { 0 };
            let er = |k: usize| *((r + k) as *const u8);
            let cr = |k: usize| *((c + k) as *const u8);
            let difere = |ks: &[usize]| c == 0 || ks.iter().any(|&k| er(k) != cr(k));
            if er(0x1d) == 0xfb {
                if c == 0 {
                    margem(sm, i, 1);
                    margem(sm, i + 1, 1);
                    texto(sm, i + 3, EMPTY_TEXT);
                    texto(sm, i + 4, EMPTY_TEXT);
                    texto(sm, i + 5, EMPTY_TEXT);
                }
            } else {
                if difere(&[0x2c, 0x2d]) {
                    let (a, d) = (er(0x2c), er(0x2d));
                    if a != 0 && d != 0 {
                        margem(sm, i, 0x20030);
                        fundo(sm, i, colour_word(0xaebdec) as u32);
                        margem(sm, i + 1, 0x40030);
                        fundo(sm, i + 1, colour_word(0xae31a8) as u32);
                    } else if a == 0 && d != 0 {
                        margem(sm, i, 0x20030);
                        fundo(sm, i, colour_word(0xae31a8) as u32);
                        margem(sm, i + 1, 1);
                    } else if a != 0 {
                        margem(sm, i, 0x20030);
                        fundo(sm, i, colour_word(0xaebdec) as u32);
                        margem(sm, i + 1, 1);
                    } else {
                        margem(sm, i, 1);
                        margem(sm, i + 1, 1);
                    }
                }
                let m = |k: usize| er(k) as i8;
                if difere(&[0x33, 0x34, 0x30, 0x32]) {
                    let minuto = if m(0x33) >= 0 {
                        Some((0xa16a68usize, m(0x33), 0xae3154usize)) // "inj <%d - minute>"
                    } else if m(0x34) >= 0 {
                        Some((0xa16a54, m(0x34), 0xae3154)) // "off <%d - minute>"
                    } else if m(0x30) >= 0 {
                        Some((0xa16a40, m(0x30), 0xae3154)) // "sub <%d - minute>"
                    } else if m(0x32) >= 0 {
                        Some((0xa16a2c, m(0x32), 0xaebdd4)) // "on <%d - minute>"
                    } else {
                        None
                    };
                    match minuto {
                        Some((f, v, cor)) => {
                            traduzido(BUF, f, v as i32 + 1);
                            cor_do_texto(sm, i + 3, colour_word(cor) as u32);
                            texto(sm, i + 3, BUF);
                        }
                        None => texto(sm, i + 3, EMPTY_TEXT),
                    }
                }
                if lin >= 0x323 && m(0x32) < 0 {
                    // o reserva que não entrou: só limpa, e só sem cópia
                    if c == 0 {
                        texto(sm, i + 4, EMPTY_TEXT);
                        texto(sm, i + 5, EMPTY_TEXT);
                    }
                } else {
                    if difere(&[0x35, 0x32]) {
                        formatar(BUF, FORMAT_D, er(0x35) as i32);
                        texto(sm, i + 4, BUF);
                    }
                    if difere(&[0x21, 0x32]) {
                        if er(0x21) != 0 {
                            formatar(BUF, FORMAT_D, er(0x21) as i32);
                            texto(sm, i + 5, BUF);
                        } else {
                            texto(sm, i + 5, EMPTY_TEXT);
                        }
                    }
                }
                let cor = if core::ptr::read_unaligned((r + 4) as *const u32) == atual {
                    0xaebde8
                } else if m(0x33) >= 0 || m(0x34) >= 0 || m(0x30) >= 0 || (lin > 0x2da && m(0x32) == -1) {
                    0xae317e
                } else {
                    0xae3184
                };
                if difere(&[0x33, 0x34, 0x30, 0x32]) || atual != core::ptr::read_unaligned((copia + 0x3c) as *const u32) {
                    cor_do_texto(sm, i + 2, colour_word(cor) as u32);
                }
            }
            item = item.wrapping_add(7);
        }
    }
    save_match_snapshot(copia, e);
}

/// As três rotinas são comparadas de verdade quando já existe a cópia do estado (parâmetro 0x1d): as
/// duas atualizam a partir do mesmo estado e o gerente de telas (com a posição do buffer de mensagens
/// em +0x3802, que cada atualização de item zera e preenche), o `Display` da tela atual, o bloco de
/// parâmetros, a cópia do estado (0xd7a bytes) e o texto 0xde1f64 têm de sair iguais. O conteúdo do
/// buffer de mensagens ([gerente + 0x37fe]) não entra (como na leva das listas).
unsafe fn compare_valores_da_partida(nome: &str, original: impl FnOnce(), rust: impl FnOnce()) {
    let copia = get_screen_param(0x1d);
    if copia == 0 {
        // ponytail: sem cópia a rotina aloca a cópia (0x944cf1) e a grava no parâmetro 0x1d; não dá
        // para rodar duas vezes. Só a original; a cópia Rust prova esse ramo no modo Rust.
        return original();
    }
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((copia, 0xd7a));
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, original, rust);
    SCOREBOARD.record(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("{}: diferencas em {:?}", nome, dif)
    });
}

unsafe extern "C" fn compare_refresh_match_stats_values() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    compare_valores_da_partida("refresh_match_stats_values", || o(), || refresh_match_stats_values());
}

unsafe extern "C" fn compare_refresh_action_zones_values() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_valores_da_partida("refresh_action_zones_values", || o(), || refresh_action_zones_values());
}

unsafe extern "C" fn compare_refresh_player_ratings_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    compare_valores_da_partida("refresh_player_ratings_rows", || o(), || refresh_player_ratings_rows());
}
