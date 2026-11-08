// ---------- %%LEVA%%a leva: o player_setup.cfg: a pessoa pelo nome (no clube ou em todos), o texto entre aspas da linha e o tipo da linha pelo nome (player_regen.cpp) ----------

unsafe fn cqh9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// strncmp (0x946fa0, cdecl).
unsafe fn cqh9_strncmp(a: usize, b: usize, n: u32) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x946fa0usize);
    f(a, b, n)
}

/// O nome `k` (+4 primeiro, +8 segundo, +0xc o comum; o registro do nome começa pelo texto) da pessoa bate com `texto`
/// (até 26 caracteres)? `texto` vazio sempre bate.
unsafe fn name_matches(p: usize, k: usize, texto: usize) -> bool {
    *(texto as *const u8) == 0 || cqh9_strncmp(cqh9_d(p + k), texto, 0x1a) == 0
}

/// { 0x7a50f0 } (cdecl, player_regen.cpp): `player_setup_find_staff(primeiro, comum, segundo, clube, no_elenco)`. Sem
/// `clube` (nulo ou vazio): a primeira das [0xae2364] pessoas ([0xae23bc], 0x6e bytes) cujos nomes não vazios batem
/// (cada nome conferido só se o texto e o nome da pessoa existem). Com `clube`: o clube com esse nome (0x545880, 51
/// caracteres); sem achar, 0. Com `no_elenco` 1: o primeiro das 50 vagas do elenco (+0xd7) com os nomes que batem;
/// senão (ou nenhum do elenco), o técnico do clube (+0xcf) se os nomes batem; senão 0.
unsafe extern "C" fn player_setup_find_staff(primeiro: usize, comum: usize, segundo: usize, clube: usize, no_elenco: u32) -> usize {
    if clube == 0 || *(clube as *const u8) == 0 {
        let mut p = cqh9_d(0xae23bc);
        for _ in 0..cqh9_i(0xae2364).max(0) {
            let falha = (*(primeiro as *const u8) != 0 && cqh9_d(p + 4) != 0 && cqh9_strncmp(cqh9_d(p + 4), primeiro, 0x1a) != 0)
                || (*(segundo as *const u8) != 0 && cqh9_d(p + 8) != 0 && cqh9_strncmp(cqh9_d(p + 8), segundo, 0x1a) != 0) // cqh9: o segundo nome
                || (*(comum as *const u8) != 0 && cqh9_d(p + 0xc) != 0 && cqh9_strncmp(cqh9_d(p + 0xc), comum, 0x1a) != 0);
            if !falha {
                crate::comparison::branch("cqh_9 pessoa pelo nome");
                return p;
            }
            p += 0x6e;
        }
        return 0;
    }
    let nome_do_clube: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x545880usize);
    let mut achado = 0usize;
    for k in 0..cqh9_i(0xae235c).max(0) {
        let c = cqh9_d(0xae23b4).wrapping_add(k as usize * 0x245);
        if cqh9_strncmp(nome_do_clube(c), clube, 0x33) == 0 {
            achado = c;
            break;
        }
    }
    if achado == 0 {
        return 0;
    }
    let c = achado;
    if no_elenco as u8 == 1 {
        for k in 0..0x32usize {
            let q = cqh9_d(c + 0xd7 + 4 * k);
            if q != 0 && name_matches(q, 4, primeiro) && name_matches(q, 8, segundo) && name_matches(q, 0xc, comum) {
                crate::comparison::branch("cqh_9 jogador do clube pelo nome");
                return q; // cqh9: o do elenco pelo nome
            }
        }
    }
    let t = cqh9_d(c + 0xcf);
    if t == 0 {
        return 0;
    }
    if name_matches(t, 4, primeiro) && name_matches(t, 8, segundo) && name_matches(t, 0xc, comum) {
        return t;
    }
    0
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_player_setup_find_staff(primeiro: usize, comum: usize, segundo: usize, clube: usize, no_elenco: u32) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32) -> usize>(%%I0%%)(primeiro, comum, segundo, clube, no_elenco);
    let n = player_setup_find_staff(primeiro, comum, segundo, clube, no_elenco);
    PLACAR.record(r == n, || format!("player_setup_find_staff(clube {:x}, elenco {}) original={:x} rust={:x}", clube, no_elenco as u8, r, n));
    r
}

/// { 0x7a5090 } (cdecl, player_regen.cpp): `player_setup_read_quoted_token(origem, destino)`: copia para `destino` o
/// texto entre as primeiras aspas de `origem`: espaços e '\r' no começo do texto e '\n' antes dele são pulados; termina
/// nas aspas de fecho ou no zero (o destino ganha o zero) e devolve o ponteiro dois depois do fim.
unsafe extern "C" fn player_setup_read_quoted_token(origem: usize, destino: usize) -> usize {
    let (mut i, mut n, mut aberto) = (0usize, 0usize, false);
    loop {
        let c = *((origem + i) as *const u8);
        i += 1;
        if c == b'"' {
            if aberto {
                break;
            }
            aberto = true;
            continue;
        }
        if (c == b' ' || c == b'\r') && n == 0 {
            continue;
        }
        if c == b'\n' {
            if n == 0 {
                continue;
            }
        } else if c == 0 {
            break;
        }
        if aberto {
            crate::comparison::branch("cqh_9 texto entre aspas");
            *((destino + n) as *mut u8) = c; // cqh9: o caractere do texto
            n += 1;
        }
    }
    *((destino + n) as *mut u8) = 0;
    origem + i + 1
}

/// Comparador: lado a lado no destino (até 0x100 bytes) e na devolução.
unsafe extern "C" fn comparar_player_setup_read_quoted_token(origem: usize, destino: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I1%%);
    if origem == 0 || destino == 0 {
        return o(origem, destino);
    }
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&[(destino, 0x100)], || o(origem, destino), || player_setup_read_quoted_token(origem, destino));
    PLACAR.record(igual && r == n, || format!("player_setup_read_quoted_token({:x}) original={:x} rust={:x}", origem, r, n));
    r
}

/// Os tipos de linha do player_setup.cfg (o texto no .rdata e o bit), na ordem da original (strncmp de 100); o último,
/// AWOL (0xa6b2f8), dá 0x40, e nenhum, 0.
const PLAYER_SETUP_LINE_TYPES: [(usize, u32); 14] = [
    (0xa6b3a0, 1),      // INJURY
    (0xa6b390, 2),      // MANAGER_CONF
    (0xa6b384, 0x200),  // RETIREMENT
    (0xa6b374, 4),      // INT_RETIREMENT
    (0xa6b364, 8),      // FUTURE_TRANSFER
    (0xa6b35c, 0x10),   // LOAN
    (0xa6b350, 0x100),  // NANDROLONE
    (0xa6b348, 0x100),  // DRUGS
    (0xa6b33c, 0x1000), // MISCONDUCT
    (0xa6b32c, 0x800),  // FALSE_PASSPORT
    (0xa6b324, 0x20),   // ARSE
    (0xa6b314, 0x2000), // PRESET_TACTICS
    (0xa6b308, 0x400),  // COLOURBLIND
    (0xa6b300, 0x80),   // GERMAN
];

/// { 0x7a3140 } (cdecl, player_regen.cpp): `player_setup_type_from_name(nome)`: o bit do tipo da linha (INJURY 1,
/// MANAGER_CONF 2, RETIREMENT 0x200, INT_RETIREMENT 4, FUTURE_TRANSFER 8, LOAN 0x10, NANDROLONE e DRUGS 0x100,
/// MISCONDUCT 0x1000, FALSE_PASSPORT 0x800, ARSE 0x20, PRESET_TACTICS 0x2000, COLOURBLIND 0x400, GERMAN 0x80, AWOL 0x40;
/// nenhum 0), pela aplicação das linhas (0x7a3a10) na criação do jogo.
unsafe extern "C" fn player_setup_type_from_name(nome: usize) -> u32 {
    for &(texto, bit) in PLAYER_SETUP_LINE_TYPES.iter() {
        if cqh9_strncmp(nome, texto, 100) == 0 {
            crate::comparison::branch("cqh_9 tipo da linha");
            return bit;
        }
    }
    if cqh9_strncmp(nome, 0xa6b2f8, 100) == 0 { 0x40 } else { 0 }
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_player_setup_type_from_name(nome: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(nome);
    let n = player_setup_type_from_name(nome);
    PLACAR.record(r == n, || format!("player_setup_type_from_name({:x}) original={:x} rust={:x}", nome, r, n));
    r
}
