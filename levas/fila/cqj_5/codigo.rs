// ---------- %%LEVA%%a leva: as regras das três divisões francesas: turnos, pontos, acesso e rebaixamento pelo ano (a virada para 20 clubes em 2002), desempates e a tabela de rodadas (fra_first.cpp, fra_second.cpp, fra_third.cpp) ----------

unsafe fn cqj5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut CQJ5_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn fra_first_apply_league_rules(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQJ5_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqj5_d(cqj5_d(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// A temporada 2001 (+0x40, o ano da liga, = 0x7d1): a última com 18 clubes na primeira divisão.
unsafe fn is_2001_season(this: usize) -> bool {
    core::ptr::read_unaligned((this + 0x40) as *const u16) == 0x7d1
}

/// As regras comuns às três: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), clubes nacionais (+0x42
/// = 1), os desempates 1, 2 e 3 (+0xc5..+0xc7), nenhum play-off (+0xbf = +0xc0 = 0), os bits de regra +0xd9 = 0x83
/// (a liga tem campeão, as vagas de cima são de acesso e a ordem das rodadas tackle ao meio) e 3 substituições
/// (+0x4a).
fn common_rules() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0xbf, 1, 0), (0xc0, 1, 0), (0xd9, 2, 0x83), (0x4a, 1, 3)]
}

/// { 0x5bc410 } (thiscall, ret 0, fra_first.cpp): `FraFirst::set_up_rules()`. A primeira divisão: as comuns, 2
/// turnos, 3 pontos, +0xab e +0xdb = 0, ninguém sobe (+0xbe = 0), caem 2 em 2001 e 3 depois (+0xc1), 5 reservas
/// (+0x49), nenhuma liga acima (+0x1c = −1), +0x52 = 2 [sem nome] e a segunda abaixo (+0x20 = [0x9cf5d0]).
unsafe extern "thiscall" fn fra_first_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_5 regras da primeira");
    fra_first_apply_league_rules(this, &fra_first_first_division_rules(this))
}

unsafe fn fra_first_first_division_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0)]);
    let caem = if is_2001_season(this) { 2 } else { 3 }; // cqj5: a primeira rebaixa 2 em 2001
    v.extend_from_slice(&[(0xc1, 1, caem), (0x49, 1, 5), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, cqj5_d(0x9cf5d0) as u32)]);
    v
}

/// { 0x5bf0a0 } (thiscall, ret 0, fra_second.cpp): `FraSecond::set_up_rules()`. A segunda: as comuns, 2 turnos, 3
/// pontos; em 2001 sobem 4 (+0xbe) e caem 2 (+0xc1), depois sobem 3 e caem 3; a primeira acima ([0x9cf5cc]), a
/// terceira abaixo ([0x9cf5d4]), +0x52 = 2 e 5 reservas.
unsafe extern "thiscall" fn fra_second_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_5 regras da segunda");
    fra_first_apply_league_rules(this, &fra_first_second_division_rules(this))
}

unsafe fn fra_first_second_division_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqj5: a segunda dá 3 pontos pela vitória
    let (sobem, caem) = if is_2001_season(this) { (4, 2) } else { (3, 3) };
    v.extend_from_slice(&[(0xbe, 1, sobem), (0xc1, 1, caem), (0x1c, 4, cqj5_d(0x9cf5cc) as u32), (0x52, 1, 2), (0x20, 4, cqj5_d(0x9cf5d4) as u32), (0x49, 1, 5)]);
    v
}

/// { 0x5c0700 } (thiscall, ret 0, fra_third.cpp): `FraThird::set_up_rules()`. O National: as comuns, 2 turnos, 3
/// pontos; sobem 4 em 2001 e 3 depois (+0xbe), caem 4 (+0xc1); a segunda acima ([0x9cf5d0]), nenhuma abaixo (+0x20
/// = −1), +0xa7 (os jogos da temporada) = −1 (recalculado pela 0x687430) e 3 reservas (+0x49).
unsafe extern "thiscall" fn fra_third_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_5 regras do national");
    fra_first_apply_league_rules(this, &national_division_rules(this))
}

unsafe fn national_division_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqj5: o National joga 2 turnos
    let sobem = if is_2001_season(this) { 4 } else { 3 };
    v.extend_from_slice(&[(0xbe, 1, sobem), (0xc1, 1, 4), (0x1c, 4, cqj5_d(0x9cf5d0) as u32), (0x49, 1, 3), (0x20, 4, 0xffff_ffff), (0xa7, 2, 0xffff)]);
    v
}

/// O comparador das três (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos que a cópia grava saem iguais aos da original, e a devolução 1; no
/// fim fica a faixa da original.
unsafe fn fra_first_compare_league_rules(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQJ5_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQJ5_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&faixa);
    mo.escrever();
    let campo = |m: &crate::comparacao::Memoria, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    PLACAR.registrar(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

unsafe extern "thiscall" fn comparar_fra_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    fra_first_compare_league_rules("fra_first_set_up_rules", this, fra_first_first_division_rules(this), || o(this), || fra_first_set_up_rules(this))
}

unsafe extern "thiscall" fn comparar_fra_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    fra_first_compare_league_rules("fra_second_set_up_rules", this, fra_first_second_division_rules(this), || o(this), || fra_second_set_up_rules(this))
}

unsafe extern "thiscall" fn comparar_fra_third_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    fra_first_compare_league_rules("fra_third_set_up_rules", this, national_division_rules(this), || o(this), || fra_third_set_up_rules(this))
}
