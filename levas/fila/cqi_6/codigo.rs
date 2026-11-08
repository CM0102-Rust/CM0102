// ---------- %%LEVA%%a leva: as regras das divisões italianas (Série A, B, C1/A e C1/B): turnos, pontos, vagas, desempates, o empate que vai ao jogo-desempate e a tabela de rodadas (ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

unsafe fn cqi6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut CQI6_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn cqi6_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQI6_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqi6_d(cqi6_d(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// As regras comuns às quatro: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), +0xab e +0xdb = 0 (as
/// palavras copiadas nas partidas), clubes nacionais (+0x42 = 1), o 1º desempate 1 (+0xc5) e o 2º 2 (+0xc6), a ordem
/// fixa das rodadas na grade (bit 0x40 de +0xd9, com o byte de baixo como está), 7 reservas (+0x49) e 3 substituições
/// (+0x4a).
unsafe fn cqi6_comuns(this: usize) -> Vec<(usize, u8, u32)> {
    let d9 = *((this + 0xd9) as *const u8) | 0x40;
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xd9, 1, d9 as u32), (0x49, 1, 7), (0x4a, 1, 3)]
}

/// { 0x641640 } (thiscall, ret 0, ita_ser_a.cpp): `ItaSerA::set_up_rules()`. A Série A: as comuns, 2 turnos, 3
/// pontos, os desempates 3º = 4 e 4º = 0 (+0xc7, +0xc8), ninguém sobe, nenhum play-off, 4 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1), a Série B abaixo (+0x20 = [0x9cf574]), +0x52 = 2 [sem nome] e +0xe6 = 1 (o
/// empate em pontos na fronteira de uma vaga não se decide pelos critérios: vai ao jogo-desempate, ver 0x642880).
unsafe extern "thiscall" fn ita_ser_a_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da serie a");
    cqi6_regras(this, &cqi6_serie_a(this))
}

unsafe fn cqi6_serie_a(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 4)); // cqi6: a Série A rebaixa 4
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, cqi6_d(0x9cf574) as u32), (0xe6, 4, 1)]);
    v
}

/// { 0x6468a0 } (thiscall, ret 0, ita_ser_b.cpp): `ItaSerB::set_up_rules()`. A Série B: como a A, mas 4 sobem
/// direto (+0xbe) e 4 caem (+0xc1); a A acima ([0x9cf570]) e a C1/A abaixo ([0x9cf578]).
unsafe extern "thiscall" fn ita_ser_a_set_up_rules_6468a0(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da serie b");
    cqi6_regras(this, &cqi6_serie_b(this))
}

unsafe fn cqi6_serie_b(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a Série B joga 2 turnos
    v.extend_from_slice(&[(0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, cqi6_d(0x9cf570) as u32), (0x20, 4, cqi6_d(0x9cf578) as u32), (0x52, 1, 2), (0xe6, 4, 1)]);
    v
}

/// { 0x64b770 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::set_up_rules()`. A C1/A: as comuns, 2 turnos, 3
/// pontos, os desempates 3º = 0 (+0xc7), o campeão sobe direto (+0xbe = 1), do 2º ao 5º vão ao play-off de acesso
/// (+0xbf = 4), do 14º ao 17º ao play-out de permanência (+0xc0 = 4), o último cai (+0xc1 = 1), a Série B acima
/// ([0x9cf574]) e abaixo o id de [0x9cf57c] (o da C1/B, o outro grupo: assim no original).
unsafe extern "thiscall" fn ita_ser_c1a_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da c1a");
    cqi6_regras(this, &cqi6_c1a(this))
}

unsafe fn cqi6_c1a(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a C1/A dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, cqi6_d(0x9cf574) as u32), (0x20, 4, cqi6_d(0x9cf57c) as u32)]);
    v
}

/// { 0x64f920 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::set_up_rules()`. A C1/B: como a C1/A; acima o id de
/// [0x9cf578] e abaixo a C2/A ([0x9cf580]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da
/// nação [inferência]), senão −1.
unsafe extern "thiscall" fn ita_ser_a_set_up_rules_64f920(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da c1b");
    cqi6_regras(this, &cqi6_c1b(this))
}

unsafe fn cqi6_c1b(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, cqi6_d(0x9cf578) as u32)]);
    let nacao = cqi6_d(cqi6_d(this + 4) + 0x5d);
    let abaixo = if *((nacao + 0x11c) as *const u8) & 4 != 0 { cqi6_d(0x9cf580) as u32 } else { 0xffff_ffff };
    v.push((0x20, 4, abaixo));
    v
}

/// O comparador das quatro (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos que a cópia grava saem iguais aos da original, e a devolução 1; no
/// fim fica a faixa da original.
unsafe fn cqi6_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQI6_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQI6_ENSAIO = false;
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

unsafe extern "thiscall" fn comparar_ita_ser_a_set_up_rules(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqi6_comparar("ita_ser_a_set_up_rules", this, cqi6_serie_a(this), || o(this), || ita_ser_a_set_up_rules(this))
}

unsafe extern "thiscall" fn comparar_ita_ser_a_set_up_rules_6468a0(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    cqi6_comparar("ita_ser_a_set_up_rules_6468a0", this, cqi6_serie_b(this), || o(this), || ita_ser_a_set_up_rules_6468a0(this))
}

unsafe extern "thiscall" fn comparar_ita_ser_c1a_set_up_rules(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqi6_comparar("ita_ser_c1a_set_up_rules", this, cqi6_c1a(this), || o(this), || ita_ser_c1a_set_up_rules(this))
}

unsafe extern "thiscall" fn comparar_ita_ser_a_set_up_rules_64f920(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I3%%);
    cqi6_comparar("ita_ser_a_set_up_rules_64f920", this, cqi6_c1b(this), || o(this), || ita_ser_a_set_up_rules_64f920(this))
}
