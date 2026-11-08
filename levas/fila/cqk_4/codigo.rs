// ---------- %%LEVA%%a leva: as regras das divisões espanholas (Primera, Segunda e a divisão de baixo): turnos, pontos, vagas, o confronto direto primeiro nos desempates e a tabela de rodadas (spa_first.cpp, spa_second.cpp, spa_lower.cpp) ----------

unsafe fn cqk4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut SPA_FIRST_RULES_DRY_RUN: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn spa_first_apply_league_rules(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !SPA_FIRST_RULES_DRY_RUN {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqk4_d(cqk4_d(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// As regras comuns às três: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), clubes nacionais (+0x42
/// = 1), os desempates 4, 1 e 2 (+0xc5..+0xc7: o confronto direto primeiro [inferência pelo código 4], depois o
/// saldo e os gols pró), nenhum play-off de permanência (+0xc0 = 0), os bits de regra +0xd9 = 0x43 (campeão, vagas de
/// cima de acesso e a ordem fixa das rodadas), 7 reservas (+0x49) e 3 substituições (+0x4a).
fn spanish_common_rules() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2), (0xc0, 1, 0), (0xd9, 2, 0x43), (0x49, 1, 7), (0x4a, 1, 3)]
}

/// { 0x84fdf0 } (thiscall, ret 0, spa_first.cpp): `SpaFirst::set_up_rules()`. A Primera: +0xa9 = 38 (as rodadas;
/// a virtual reescreve), as comuns, 2 turnos, 3 pontos, o 4º desempate 3 (+0xc8), +0xab e +0xdb = 0, ninguém sobe
/// (+0xbe = +0xbf = 0), 3 caem (+0xc1), nenhuma liga acima (+0x1c = −1), a Segunda abaixo (+0x20 = [0x9cf628]) e
/// +0x52 = 2 [sem nome].
unsafe extern "thiscall" fn spa_first_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqk_4 primera rules");
    core::ptr::write_unaligned((this + 0xa9) as *mut u16, 0x26);
    spa_first_apply_league_rules(this, &primera_rules())
}

unsafe fn primera_rules() -> Vec<(usize, u8, u32)> {
    let mut v = spanish_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc8, 1, 3), (0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqk4: a Primera rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, cqk4_d(0x9cf628) as u32), (0x52, 1, 2)]);
    v
}

/// { 0x855cc0 } (thiscall, ret 0, spa_second.cpp): `SpaSecond::set_up_rules()`. A Segunda: as comuns, 2 turnos, 3
/// pontos, o 4º desempate 3, 3 sobem direto (+0xbe), nenhum play-off de acesso, 4 caem (+0xc1), a Primera acima
/// ([0x9cf624]) e abaixo a Segunda B ([0x9cf7b8]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c
/// da nação [inferência]), senão −1; +0x52 = 2.
unsafe extern "thiscall" fn spa_second_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqk_4 segunda rules");
    spa_first_apply_league_rules(this, &segunda_rules(this))
}

unsafe fn segunda_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = spanish_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: a Segunda joga 2 turnos
    v.extend_from_slice(&[(0xc8, 1, 3), (0xbe, 1, 3), (0xbf, 1, 0), (0xc1, 1, 4), (0x1c, 4, cqk4_d(0x9cf624) as u32)]);
    let nacao = cqk4_d(cqk4_d(this + 4) + 0x5d);
    let abaixo = if *((nacao + 0x11c) as *const u8) & 4 != 0 { cqk4_d(0x9cf7b8) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, abaixo), (0x52, 1, 2)]);
    v
}

/// { 0x853790 } (thiscall, ret 0, spa_lower.cpp): `SpaLower::set_up_rules()`. A divisão de baixo (os times B e os
/// melhores das divisões menores): as comuns, 2 turnos, 3 pontos, 2 sobem direto (+0xbe) e 2 vão ao play-off de
/// acesso (+0xbf), 4 caem (+0xc1), a Primera como liga de cima ([0x9cf624], assim no original) e nenhuma abaixo.
unsafe extern "thiscall" fn spa_lower_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqk_4 lower division rules");
    spa_first_apply_league_rules(this, &lower_division_rules())
}

unsafe fn lower_division_rules() -> Vec<(usize, u8, u32)> {
    let mut v = spanish_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: a divisão de baixo dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 2), (0xc1, 1, 4), (0x1c, 4, cqk4_d(0x9cf624) as u32), (0x20, 4, 0xffff_ffff)]);
    v
}

/// O comparador das três (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos da lista saem iguais aos da original (o +0xa9 da Primera, que a
/// virtual reescreve, fica de fora), e a devolução 1; no fim fica a faixa da original.
unsafe fn spa_first_compare_league_rules(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let band = [(this + 0x1c, 0xd0)];
    let antes = crate::comparison::MemorySnapshot::read(&band);
    let r = original();
    let mo = crate::comparison::MemorySnapshot::read(&band);
    antes.write_back();
    SPA_FIRST_RULES_DRY_RUN = true;
    let rr = crate::rng::pause_trace(copia);
    SPA_FIRST_RULES_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&band);
    mo.write_back();
    let campo = |m: &crate::comparison::MemorySnapshot, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    SCOREBOARD.record(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

unsafe extern "thiscall" fn compare_spa_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    spa_first_compare_league_rules("spa_first_set_up_rules", this, primera_rules(), || o(this), || spa_first_set_up_rules(this))
}

unsafe extern "thiscall" fn compare_spa_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    spa_first_compare_league_rules("spa_second_set_up_rules", this, segunda_rules(this), || o(this), || spa_second_set_up_rules(this))
}

unsafe extern "thiscall" fn compare_spa_lower_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    spa_first_compare_league_rules("spa_lower_set_up_rules", this, lower_division_rules(), || o(this), || spa_lower_set_up_rules(this))
}
