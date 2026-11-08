// ---------- %%LEVA%%a leva: as estatísticas das táticas: o resultado de cada partida na linha da tática do clube do computador, o capitão guardado e o capitão escolhido (tactics.cpp) ----------

unsafe fn cpz5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A linha da tabela de estatísticas das táticas ([this], 0x28 bytes por tática) usada pelo clube: a palavra +0 do
/// registro de 0x91 bytes do clube (id em +0) na tabela tática (+0x601).
unsafe fn tactic_stats_row(this: usize, clube: usize) -> usize {
    let id = cpz5_d(clube);
    let k = cpz5_w(cpz5_d(this + 0x601).wrapping_add(id.wrapping_mul(0x91))) as i32;
    cpz5_d(this).wrapping_add((k as isize * 0x28) as usize)
}

/// x < 1 vale 1 (palavra com sinal), como as três comparações antes de cada `fild`.
fn at_least_one(x: i16) -> f64 {
    if x < 1 { 1.0 } else { x as f64 }
}

/// { 0x896290 } (thiscall, ret 4, tactics.cpp): `Tactics::record_tactic_result(jogo)`, do fim de cada partida no laço
/// do motor (0x6ba9a0, com o placar agregado; fixture +0 ≠ −2). Jogo nulo: a caixa da linha 0x1f9 e 0. Mandante (+0x1c)
/// ou visitante (+0x20) dirigido por humano (0x600a50 com 0, 1, 0, 0; cdecl 5): 0. Senão, para cada lado (clube nulo:
/// a caixa da linha 0x20a e segue), o registro da partida pelo clube (0x7e4750, num registro local que começa com a
/// data padrão 0x54a110) soma na linha da tática do clube (tactic_stats_row): +2 jogos, +4 vitórias (+3 pontos em +0xe), +6
/// empates (+1 ponto), +8 derrotas, +0xa gols pró (+0x19 do registro, com sinal), +0xc contra (+0x1a); com p = jogos,
/// a = pró, b = contra e c = pontos (cada um pelo menos 1), q = c/p, g = a/p, h = g·q², r = p/b: +0x10 = r·h·g, +0x18
/// = r·g·q²·q, +0x20 = r·r·h (a ordem das contas do x87). A escolha da formação do computador (0x6e0230) lê +2 e as
/// três razões. Devolve 1.
unsafe extern "thiscall" fn record_tactic_result(this: usize, jogo: usize) -> u32 {
    if jogo == 0 {
        show_error_at(0xa8b908, 0x1f9);
        return 0;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    for k in 0..2 {
        let c = cpz5_d(jogo + 0x1c + 4 * k);
        if c != 0 && humano(c, 0, 1, 0, 0) != 0 {
            return 0;
        }
    }
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let registro: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7e4750usize);
    let mut reg = [0u8; 0x20];
    let pr = reg.as_mut_ptr() as usize;
    data_padrao(pr);
    for k in 0..2 {
        let c = cpz5_d(jogo + 0x1c + 4 * k);
        if c == 0 {
            show_error_at(0xa8b908, 0x20a);
            continue;
        }
        let l = tactic_stats_row(this, c);
        registro(jogo, c, pr);
        let m = |o: usize| (l + o) as *mut i16;
        crate::comparison::branch("cpz_5 linha da tatica");
        *m(2) = (*m(2)).wrapping_add(1);
        let (pro, contra) = (reg[0x19] as i8, reg[0x1a] as i8);
        if pro > contra {
            *m(4) = (*m(4)).wrapping_add(1);
            *m(0xe) = (*m(0xe)).wrapping_add(3); // cpz5: a vitória vale 3 pontos
        } else if pro == contra {
            *m(6) = (*m(6)).wrapping_add(1);
            *m(0xe) = (*m(0xe)).wrapping_add(1);
        } else {
            *m(8) = (*m(8)).wrapping_add(1);
        }
        *m(0xa) = (*m(0xa)).wrapping_add(pro as i16); // cpz5: os gols pró
        *m(0xc) = (*m(0xc)).wrapping_add(contra as i16);
        let a = at_least_one(*m(0xa));
        let b = at_least_one(*m(0xc));
        let c = at_least_one(*m(0xe));
        let p = at_least_one(*m(2));
        let q = c / p;
        let q2 = q * q;
        let g = a / p;
        let h = g * q2;
        let r = p / b;
        let f = |o: usize| (l + o) as *mut f64;
        core::ptr::write_unaligned(f(0x10), r * h * g);
        core::ptr::write_unaligned(f(0x18), r * g * q2 * q); // cpz5: a razão da postura normal
        core::ptr::write_unaligned(f(0x20), r * r * h);
    }
    1
}

/// Comparador da 0x896290: lado a lado nas linhas das táticas dos dois clubes (0x28 bytes cada) e na devolução; o
/// registro da partida é local. Jogo ou clube nulos: a caixa, só a original.
unsafe extern "thiscall" fn comparar_record_tactic_result(this: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    if jogo == 0 || cpz5_d(jogo + 0x1c) == 0 || cpz5_d(jogo + 0x20) == 0 {
        return o(this, jogo);
    }
    let reg = [(tactic_stats_row(this, cpz5_d(jogo + 0x1c)), 0x28), (tactic_stats_row(this, cpz5_d(jogo + 0x20)), 0x28)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, jogo), || record_tactic_result(this, jogo));
    SCOREBOARD.record(igual && r == n, || format!("record_tactic_result(jogo {:x}) original={} rust={} {:x?} x {:x?}", jogo, r, n, mo, mr));
    r
}

/// { 0x8976a0 } (thiscall, ret 8, tactics.cpp): `Tactics::get_captain(clube, time)`, das telas de tática (0x8a7440). Clube
/// nulo: a caixa da linha 0x527 e 0. A linha do clube (0x8982b0, thiscall(this, clube, time), ret 8; −1: 0); devolve o
/// dword +0x52 do registro de 0x91 bytes da linha na tabela tática (+0x601).
unsafe extern "thiscall" fn get_captain(this: usize, clube: usize, time: u32) -> u32 {
    if clube == 0 {
        show_error_at(0xa8b908, 0x527);
        return 0;
    }
    let linha: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(this, clube, time);
    if k == -1 {
        return 0;
    }
    cpz5_d(cpz5_d(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52) as u32
}

/// Comparador da 0x8976a0: pela devolução (só lê; a linha nula ou fora da faixa mostra a caixa nas duas).
unsafe extern "thiscall" fn comparar_get_captain(this: usize, clube: usize, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    if clube == 0 {
        return o(this, clube, time);
    }
    let r = o(this, clube, time);
    let n = get_captain(this, clube, time);
    SCOREBOARD.record(r == n, || format!("get_captain(clube {:x}, time {}) original={:x} rust={:x}", clube, time, r, n));
    r
}

/// { 0x897880 } (thiscall, ret 0xc, tactics.cpp): `Tactics::set_captain(clube, &pessoa, time)`, ao aplicar as mudanças da
/// tela de tática (0x8a80d0). Clube nulo: a caixa da linha 0x579; ponteiro nulo: a da 0x581; 0. A linha do clube
/// (0x8982b0; −1: 0); grava o dword apontado em +0x52 do registro da linha (+0x601) e devolve 1.
unsafe extern "thiscall" fn set_captain(this: usize, clube: usize, pessoa: usize, time: u32) -> u32 {
    if clube == 0 {
        show_error_at(0xa8b908, 0x579);
        return 0;
    }
    if pessoa == 0 {
        show_error_at(0xa8b908, 0x581);
        return 0;
    }
    let linha: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(this, clube, time);
    if k == -1 {
        return 0;
    }
    let p = cpz5_d(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    core::ptr::write_unaligned(p as *mut u32, core::ptr::read_unaligned(pessoa as *const u32));
    1
}

/// Comparador da 0x897880: lado a lado no dword do capitão da linha (achada antes pela 0x8982b0, que só lê) e na
/// devolução. Clube ou ponteiro nulos, ou sem linha: só a original.
unsafe extern "thiscall" fn comparar_set_captain(this: usize, clube: usize, pessoa: usize, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I2%%);
    if clube == 0 || pessoa == 0 {
        return o(this, clube, pessoa, time);
    }
    let linha: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(this, clube, time);
    if k == -1 {
        return o(this, clube, pessoa, time);
    }
    let p = cpz5_d(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(p, 4)], || o(this, clube, pessoa, time), || set_captain(this, clube, pessoa, time));
    SCOREBOARD.record(igual && r == n, || format!("set_captain(clube {:x}, time {}) original={} rust={} {:x?} x {:x?}", clube, time, r, n, mo, mr));
    r
}
