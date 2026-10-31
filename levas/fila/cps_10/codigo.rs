// ---------- %%LEVA%%a leva: os gols do jogador pelo clube, o registro da seleção do ano e a ordem dos clubes pelos pontos do técnico (player_stats.cpp, staff_records.cpp) ----------

unsafe fn cps10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x7c3bb0 } (stdcall, ret 8, player_stats.cpp; a original recebe o PlayerStats no ecx e não o lê):
/// `player_stats_get_goals_for_club(pessoa, clube)` (pela MATCH_PLAYER::setup 0x6eaea0, que grava em +0x3d do jogador
/// da partida): pessoa nula, a caixa "Error" (linha 0xb04) e 0; clube nulo, a da linha 0xb0c e 0; id a partir de
/// [0xae2364], 0; sem a ficha de jogador (+0x61), a da linha 0xb1e e 0. Senão a tabela da carreira (0x7c3e40 do
/// [0xb74350]: entradas de 0x2b bytes, seis por temporada e clube) e a soma, em 16 bits, do byte +0xdf dos grupos cujo
/// clube (+0xd9, com sinal, não negativo) é o id do `clube`; a tabela é liberada (0x9452ca, também quando nula).
/// Quente (cada jogador de cada partida): sem contador.
unsafe extern "stdcall" fn gols_pelo_clube(pessoa: usize, clube: usize) -> u16 {
    if pessoa == 0 {
        aviso_em(0xa78310, 0xb04);
        return 0;
    }
    if clube == 0 {
        aviso_em(0xa78310, 0xb0c);
        return 0;
    }
    if cps10_i(pessoa) >= cps10_i(0xae2364) {
        return 0;
    }
    if cps10_d(pessoa + 0x61) == 0 {
        aviso_em(0xa78310, 0xb1e);
        return 0;
    }
    let montar: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x7c3e40usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut n = 0i32;
    let t = montar(cps10_d(0xb74350), pessoa, &mut n);
    let mut s = 0u16;
    if t != 0 {
        let mut i = 0i32;
        while i < n {
            let g = t + 0xd9 + (i / 6) as usize * 0x102;
            let v = cps10_i(g);
            if v >= 0 && v == cps10_i(clube) { // cps10: só os grupos do clube
                s = s.wrapping_add(*((g + 6) as *const u8) as u16);
            }
            i += 6;
        }
    }
    liberar(t);
    s // cps10: os gols pelo clube
}

/// Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
unsafe extern "stdcall" fn comparar_gols_pelo_clube(pessoa: usize, clube: usize) -> u16 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> u16>(%%I0%%);
    if pessoa == 0 || clube == 0 || (cps10_i(pessoa) < cps10_i(0xae2364) && cps10_d(pessoa + 0x61) == 0) {
        return o(pessoa, clube);
    }
    let r = o(pessoa, clube);
    let n = gols_pelo_clube(pessoa, clube);
    PLACAR.registrar(r == n, || format!("gols_pelo_clube(pessoa {:x}, clube {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}

/// { 0x86c730 } (thiscall, ret 0x14, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_team_of_year_record(pessoa, clube, prêmio, a, b)` (pela TEAM_AWARD::record_winners 0x8affb0):
/// pessoa ou prêmio nulos, a caixa "Error" (linha 0x7ee). Senão o histórico da pessoa (tipo 0x32: o id do clube ou −1,
/// o id do prêmio, `a` em 16 bits sem sinal e `b` como byte com sinal), no arquivo (0x58e340).
unsafe extern "thiscall" fn registrar_a_selecao_do_ano(this: usize, pessoa: usize, clube: usize, premio: usize, a: u32, b: u32) {
    crate::comparacao::ramo("cps_10 selecao do ano");
    if pessoa == 0 || premio == 0 {
        aviso_em(0xa83b78, 0x7ee);
        return;
    }
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x32;
    cpq4_por(&mut e, 8, cpq4_id_ou_menos_1(clube));
    cpq4_por(&mut e, 0xd, cps10_i(premio));
    cpq4_por(&mut e, 0x11, (a & 0xffff) as i32);
    cpq4_por(&mut e, 0x15, b as u8 as i8 as i32);
    cpq4_historico(true, this, cps10_d(pessoa), &e);
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn comparar_registrar_a_selecao_do_ano(this: usize, pessoa: usize, clube: usize, premio: usize, a: u32, b: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(%%I1%%)(this, pessoa, clube, premio, a, b)
}

/// O registro de pontos do técnico do clube (+0xcf; 0x86f5b0 do [0xdd7f00], criando), ou 0 sem técnico.
unsafe fn cps10_registro(clube: usize) -> usize {
    let tecnico = cps10_d(clube + 0xcf);
    if tecnico == 0 {
        return 0;
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    registro(cps10_d(0xdd7f00), tecnico, 1)
}

/// { 0x870220 } (cdecl, staff_records.cpp): `staff_records_compare_clubs_by_manager_points(a, b)`, a ordem do qsort dos
/// candidatos a técnico do ano (0x86fab0): os registros dos técnicos dos dois clubes (cps10_registro); sem os dois, 0;
/// só sem o de `a`, 1; só sem o de `b`, −1; com os dois, os pontos da temporada na liga (+0xba) de `b` menos os de
/// `a` (mais pontos primeiro).
unsafe extern "C" fn ordem_pelos_pontos_do_tecnico(a: usize, b: usize) -> i32 {
    let (ca, cb) = (cps10_d(a), cps10_d(b));
    let ra = cps10_registro(ca);
    let rb = cps10_registro(cb);
    if ra == 0 {
        return if rb != 0 { 1 } else { 0 };
    }
    if rb == 0 {
        return -1;
    }
    crate::comparacao::ramo("cps_10 pontos dos dois");
    cps10_i(rb + 0xba).wrapping_sub(cps10_i(ra + 0xba)) // cps10: mais pontos primeiro
}

// ponytail: a 0x86f5b0 cria o registro que falta; a original roda antes e a cópia só acha os que ela criou.
unsafe extern "C" fn comparar_ordem_pelos_pontos_do_tecnico(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = ordem_pelos_pontos_do_tecnico(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pelos_pontos_do_tecnico(clubes {:x}, {:x}) original={} rust={}", cps10_d(a), cps10_d(b), r, n));
    r
}
