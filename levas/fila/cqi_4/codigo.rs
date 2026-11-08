// ---------- %%LEVA%%a leva: as regras das quatro divisões inglesas: turnos, pontos, vagas de acesso, mata-mata e rebaixamento, desempates e a tabela de rodadas (eng_prm.cpp, eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------

unsafe fn cqi4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut CQI4_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn cqi4_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQI4_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqi4_d(cqi4_d(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// As regras comuns às quatro divisões (os turnos +0x3c e os pontos da vitória +0xc2 vão em cada uma): 1 ponto pelo empate (+0xc3),
/// categoria 2 nas partidas (+0xc4, liga), clubes nacionais (+0x42 = 1), os desempates 1, 2 e 3 (+0xc5..+0xc7: saldo,
/// gols pró e a posição [inferência pelos códigos da Inglaterra]) e +0x52 = 2 [sem nome].
fn cqi4_comuns() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0x52, 1, 2)]
}

/// { 0x574b70 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::set_up_rules()`. A Premier: as comuns, +0xab e +0xdb = 0
/// (as palavras copiadas em cada partida), ninguém sobe (+0xbe = 0, +0xbf = 0, +0xc0 = 0), 3 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1) e a First abaixo (+0x20 = [0x9cf5c0]).
unsafe extern "thiscall" fn eng_prm_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da premier");
    cqi4_regras(this, &cqi4_premier())
}

unsafe fn cqi4_premier() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Premier, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqi4: a Premier rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, cqi4_d(0x9cf5c0) as u32)]);
    v
}

/// { 0x572d90 } (thiscall, ret 0, eng_first.cpp): `EngFirst::set_up_rules()`. A First: as comuns, 2 sobem direto
/// (+0xbe), 4 vão ao mata-mata de acesso (+0xbf, do 3º ao 6º), nenhum play-off de permanência (+0xc0), 3 caem
/// (+0xc1), a Premier acima (+0x1c = [0x9cf5bc]) e a Second abaixo (+0x20 = [0x9cf5c4]), 5 reservas no banco
/// (+0x49) e 3 substituições (+0x4a).
unsafe extern "thiscall" fn eng_first_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da first");
    cqi4_regras(this, &cqi4_first())
}

unsafe fn cqi4_first() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a First joga 2 turnos
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, cqi4_d(0x9cf5bc) as u32), (0x4a, 1, 3), (0x20, 4, cqi4_d(0x9cf5c4) as u32), (0x49, 1, 5)]);
    v
}

/// { 0x576780 } (thiscall, ret 0, eng_second.cpp): `EngSecond::set_up_rules()`. A Second: como a First, mas 4
/// caem (+0xc1); a First acima ([0x9cf5c0]) e a Third abaixo ([0x9cf5c8]).
unsafe extern "thiscall" fn eng_second_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da second");
    cqi4_regras(this, &cqi4_second())
}

unsafe fn cqi4_second() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Second dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, cqi4_d(0x9cf5c0) as u32), (0x4a, 1, 3), (0x20, 4, cqi4_d(0x9cf5c8) as u32), (0x49, 1, 5)]);
    v
}

/// { 0x5780c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::set_up_rules()`. A Third: as comuns, 3 sobem direto
/// (+0xbe), 4 ao mata-mata (+0xbf, do 4º ao 7º), 1 cai (+0xc1), a Second acima ([0x9cf5c4]) e abaixo a Conference
/// ([0x9cf69c]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da nação, [[+4]+0x5d]
/// [inferência]), senão −1; 5 reservas e 3 substituições.
unsafe extern "thiscall" fn eng_prm_set_up_rules_5780c0(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da third");
    cqi4_regras(this, &cqi4_third(this))
}

unsafe fn cqi4_third(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Third, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xbe, 1, 3), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 1), (0x1c, 4, cqi4_d(0x9cf5c4) as u32)]);
    let nacao = cqi4_d(cqi4_d(this + 4) + 0x5d);
    let abaixo = if *((nacao + 0x11c) as *const u8) & 4 != 0 { cqi4_d(0x9cf69c) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, abaixo), (0x4a, 1, 3), (0x49, 1, 5)]);
    v
}

/// O comparador das quatro: a original monta a tabela de rodadas (aloca; não roda duas vezes), então é ensaio. A
/// original roda inteira; a faixa +0x1c..+0xec volta como estava e a cópia grava as regras sem a virtual +0x3c; os
/// campos que a cópia grava têm de sair iguais aos da original, e a devolução 1; no fim fica a faixa da original
/// (com a tabela de rodadas dela em +0xba, +0xa9 e +0x3a).
unsafe fn cqi4_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQI4_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQI4_ENSAIO = false;
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

unsafe extern "thiscall" fn comparar_eng_prm_set_up_rules(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqi4_comparar("eng_prm_set_up_rules", this, cqi4_premier(), || o(this), || eng_prm_set_up_rules(this))
}

unsafe extern "thiscall" fn comparar_eng_first_set_up_rules(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    cqi4_comparar("eng_first_set_up_rules", this, cqi4_first(), || o(this), || eng_first_set_up_rules(this))
}

unsafe extern "thiscall" fn comparar_eng_second_set_up_rules(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqi4_comparar("eng_second_set_up_rules", this, cqi4_second(), || o(this), || eng_second_set_up_rules(this))
}

unsafe extern "thiscall" fn comparar_eng_prm_set_up_rules_5780c0(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I3%%);
    cqi4_comparar("eng_prm_set_up_rules_5780c0", this, cqi4_third(this), || o(this), || eng_prm_set_up_rules_5780c0(this))
}
