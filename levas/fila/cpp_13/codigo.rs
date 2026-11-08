// ---------- %%LEVA%%a leva: as copas e a divisão dos clubes: a ordem dos cabeças de chave do sorteio (decrescente e crescente) e o teto da divisão dos clubes no fim da temporada (Cup.cpp, league.cpp) ----------

unsafe fn cpp13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O nível (+4, byte com sinal) e a reputação do clube (+0 → +0x80, palavra com sinal) de uma entrada de copa.
unsafe fn entry_level(e: usize) -> i32 {
    *((e + 4) as *const i8) as i32
}

unsafe fn entry_club_reputation(e: usize) -> i32 {
    *((cpp13_d(e) + 0x80) as *const i16) as i32
}

/// { 0x522190 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_desc(a, b)`: o qsort das entradas das copas (sorteios da
/// Copa da UEFA, Intertoto, copas do Japão e da Escócia) pelo nível, o maior primeiro; no empate pela reputação do
/// clube, a menor primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn cup_compare_entries_by_level_desc(a: usize, b: usize) -> i32 {
    let r = entry_level(b) - entry_level(a); // o maior nível primeiro
    if r != 0 {
        return r;
    }
    entry_club_reputation(a) - entry_club_reputation(b)
}

/// { 0x5221c0 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_asc(a, b)`: o qsort das entradas da copa da liga
/// inglesa (0x56ccc0) pelo nível, o menor primeiro; no empate pela reputação do clube, a menor primeiro. Quente
/// (qsort): sem contador.
unsafe extern "C" fn cup_compare_entries_by_level_asc(a: usize, b: usize) -> i32 {
    let r = entry_level(a) - entry_level(b); // o menor nível primeiro
    if r != 0 {
        return r;
    }
    entry_club_reputation(a) - entry_club_reputation(b)
}

/// Leva o byte +0x64 do clube (com sinal) até `valor`: para baixo (`baixar` ≠ 0) se está acima, para cima se está
/// abaixo.
unsafe fn cap_professional_status(clube: usize, valor: i8, baixar: u32) {
    let p = (clube + 0x64) as *mut i8;
    let muda = if baixar != 0 { *p > valor } else { *p < valor };
    if muda {
        crate::comparacao::ramo("cpp_13 teto da divisao");
        *p = valor; // o teto novo
    }
}

/// { 0x68a980 } (thiscall, ret 0x10, league.cpp): `League::limit_club_division_tier(valor, fora, dentro, baixar)`
/// (chamada pelo fim de temporada de cada liga: Argentina, Bélgica, Brasil, Croácia, Dinamarca, Inglaterra, França,
/// Alemanha, Holanda, Itália, Rússia, Escócia, Suécia e Turquia): para cada linha da tabela (+0xb1, 0x3b bytes, conta
/// +0x3e), ajusta o byte +0x64 do clube (+0) a `valor` quando a situação da linha (+0x37) não é `fora` (0xfd = nenhuma)
/// e de novo quando é `dentro` (0xfd = nenhuma). Os três primeiros argumentos são bytes empurrados como dword: só o
/// byte baixo conta.
unsafe extern "thiscall" fn limit_club_division_tier(this: usize, valor: u32, fora: u32, dentro: u32, baixar: u32) {
    let (valor, fora, dentro) = (valor as u8 as i8, fora as u8, dentro as u8);
    let mut i: i16 = 0;
    while i < *((this + 0x3e) as *const i16) {
        let linha = || cpp13_d(this + 0xb1).wrapping_add(i as isize as usize * 0x3b);
        if fora != 0xfd && *((linha() + 0x37) as *const u8) != fora {
            cap_professional_status(cpp13_d(linha()), valor, baixar);
        }
        if dentro != 0xfd && *((linha() + 0x37) as *const u8) == dentro {
            cap_professional_status(cpp13_d(linha()), valor, baixar);
        }
        i = i.wrapping_add(1);
    }
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_cup_compare_entries_by_level_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = cup_compare_entries_by_level_desc(a, b);
    PLACAR.registrar(r == n, || format!("cup_compare_entries_by_level_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_cup_compare_entries_by_level_asc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = cup_compare_entries_by_level_asc(a, b);
    PLACAR.registrar(r == n, || format!("cup_compare_entries_by_level_asc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: grava só o byte +0x64 dos clubes da tabela: lado a lado nesses bytes.
unsafe extern "thiscall" fn comparar_limit_club_division_tier(this: usize, valor: u32, fora: u32, dentro: u32, baixar: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32)>(%%I2%%);
    let n = *((this + 0x3e) as *const i16);
    let tabela = cpp13_d(this + 0xb1);
    let reg: Vec<(usize, usize)> = (0..n.max(0) as usize).map(|i| (cpp13_d(tabela + i * 0x3b) + 0x64, 1)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, valor, fora, dentro, baixar),
        || limit_club_division_tier(this, valor, fora, dentro, baixar));
    PLACAR.registrar(igual, || format!("limit_club_division_tier(valor {}, fora {:x}, dentro {:x}, baixar {}) {:x?} x {:x?}",
        valor as u8 as i8, fora as u8, dentro as u8, baixar, mo, mr));
}
