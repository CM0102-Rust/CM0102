// ---------- %%LEVA%%a leva: a tática ativa do clube: a tática do time do clube, a vaga de tática do técnico humano e a tática ativa dele (tactics.cpp, human_manager.cpp) ----------

unsafe fn cpx8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpx8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx8_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O técnico pedido ou, nulo, o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
unsafe fn cpx8_tecnico(tecnico: usize) -> usize {
    if tecnico != 0 {
        return tecnico;
    }
    cpx8_d((cpx8_w(0xb77c7a) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize)
}

/// O índice do registro do técnico humano no gerenciador `this` (id − [0xae2364] + 16), se estiver entre 0 e a conta
/// ([this+4]) − 1; fora disso a original mostra a caixa "Error".
unsafe fn cpx8_indice(this: usize, tecnico: usize) -> Option<usize> {
    let k = cpx8_i(tecnico).wrapping_sub(cpx8_i(0xae2364)).wrapping_add(0x10);
    if k < 0 || k > cpx8_i(this + 4).wrapping_sub(1) {
        return None;
    }
    Some(k as usize)
}

/// O registro do técnico humano `k` (0xa6ef bytes cada, a partir de [this]).
unsafe fn cpx8_registro(this: usize, k: usize) -> usize {
    cpx8_d(this).wrapping_add(k.wrapping_mul(0xa6ef))
}

/// { 0x5fbd00 } (thiscall, ret 4, human_manager.cpp): `HUMAN_MANAGER::get_active_tactic(tecnico)`, pela escalação
/// (0x6db500), pela formação (0x6e0230), pelo dia de jogo (0x6b4360), pela tática do clube (0x896660) e pelas telas. O
/// bloco de tática (0x1ecd bytes) da vaga ativa (byte com sinal em +0x9a69 do registro) do técnico (nulo: o humano da
/// vez). Sem técnico ou fora da conta: a original (caixa "Error", 0).
unsafe extern "thiscall" fn tatica_ativa_do_humano(this: usize, tecnico: usize) -> usize {
    let t = cpx8_tecnico(tecnico);
    let Some(k) = (if t != 0 { cpx8_indice(this, t) } else { None }) else {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, tecnico); // caixa: só a original
    };
    let vaga = cpx8_b(cpx8_registro(this, k) + 0x9a69) as i8 as i32;
    cpx8_d(this).wrapping_add(vaga.wrapping_mul(0x1ecd) as usize).wrapping_add(k.wrapping_mul(0xa6ef))
}

/// Comparador: a devolução (rotina pura); sem técnico ou fora da conta, só a original (caixa).
unsafe extern "thiscall" fn comparar_tatica_ativa_do_humano(this: usize, tecnico: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let t = cpx8_tecnico(tecnico);
    if t == 0 || cpx8_indice(this, t).is_none() {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = tatica_ativa_do_humano(this, tecnico);
    PLACAR.registrar(r == n, || format!("tatica_ativa_do_humano(tecnico {:#x}) original={:#x} rust={:#x}", tecnico, r, n));
    r
}

/// { 0x5fbe20 } (thiscall, ret 0xc, human_manager.cpp): `HUMAN_MANAGER::select_tactic_slot(reserva, clube, tecnico)`,
/// pelos mesmos chamadores da 0x5fbd00. Técnico nulo: o humano da vez (sem nenhum, nada); clube nulo: o do técnico
/// (+0x39). A vaga ativa (+0x9a69 do registro): 4 na segunda seleção (0x5409b0), 3 com `reserva` numa seleção
/// (0x53b900), 2 numa seleção, 1 com `reserva`, 0 no time principal. Fora da conta: a original (caixa "Error").
unsafe extern "thiscall" fn escolher_vaga_de_tatica(this: usize, reserva: u32, clube: usize, tecnico: usize) {
    let t = cpx8_tecnico(tecnico);
    if t == 0 {
        return;
    }
    let Some(k) = cpx8_indice(this, t) else {
        return orig::<unsafe extern "thiscall" fn(usize, u32, usize, usize)>(%%I1%%)(this, reserva, clube, tecnico); // caixa: só a original
    };
    let clube = if clube != 0 { clube } else { cpx8_d(t + 0x39) };
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let vaga: u8 = if clube != 0 && segunda(clube) != 0 {
        4
    } else if reserva != 0 && clube != 0 && selecao(clube) != 0 {
        3
    } else if clube != 0 && selecao(clube) != 0 {
        2 // com `reserva` e fora da seleção a original pergunta de novo (0x53b900 duas vezes)
    } else if reserva != 0 {
        1
    } else {
        0
    };
    *((cpx8_registro(this, k) + 0x9a69) as *mut u8) = vaga;
}

/// Comparador: lado a lado pelo byte da vaga ativa do registro; sem técnico (nada) ou fora da conta (caixa), só a
/// original.
unsafe extern "thiscall" fn comparar_escolher_vaga_de_tatica(this: usize, reserva: u32, clube: usize, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, usize)>(%%I1%%);
    let t = cpx8_tecnico(tecnico);
    let Some(k) = (if t != 0 { cpx8_indice(this, t) } else { None }) else {
        return o(this, reserva, clube, tecnico);
    };
    let reg = [(cpx8_registro(this, k) + 0x9a69, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, reserva, clube, tecnico), || escolher_vaga_de_tatica(this, reserva, clube, tecnico));
    PLACAR.registrar(igual, || format!("escolher_vaga_de_tatica(reserva {}, clube {:#x}, tecnico {:#x}) {:x?} x {:x?}", reserva, clube, tecnico, mo, mr));
}

/// { 0x896660 } (thiscall, ret 8, tactics.cpp): `Tactics::get_club_tactic(clube, time)`, pela formação de cada jogo
/// (0x6e0230), pelas fraquezas do jogador (0x5470d0), pela mídia e pelas telas. Clube humano (0x600a50 com 1, 1) cujo
/// técnico (+0xcf) não tem +0x9a01 (0x5fd210) ou tem a opção 2 das férias (0x5fea50): a vaga pelo time (0x5fbe20) e o
/// bloco ativo dele (0x5fbd00). Nos outros, o índice (palavra com sinal) na linha do clube (id × 0x91 em [this+0x601]):
/// abaixo da conta das táticas prontas ([this+0x5fd]) o bloco pronto ([this+0x5f9]), senão o próprio ([this+0x5f1],
/// índice − conta); blocos de 0x18e3 bytes. Clube nulo: a original (caixa "Error", 0).
unsafe extern "thiscall" fn tatica_do_clube(this: usize, clube: usize, time: u32) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%)(this, clube, time); // caixa: só a original
    }
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 1, 1, 0, 0) != 0 {
        let gerente = cpx8_d(0xb63c98);
        let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
        let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
        if t9a01(gerente, cpx8_d(clube + 0xcf)) == 0 || opcao(gerente, 2, cpx8_d(clube + 0xcf)) != 0 {
            let escolher: unsafe extern "thiscall" fn(usize, u32, usize, usize) = core::mem::transmute(0x5fbe20usize);
            let ativa: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5fbd00usize);
            crate::comparacao::ramo("cpx_8 tatica do humano");
            escolher(gerente, time, clube, cpx8_d(clube + 0xcf)); // cpx8: a vaga pelo time
            return ativa(gerente, cpx8_d(clube + 0xcf));
        }
    }
    let id = cpx8_i(clube);
    let indice = cpx8_w(cpx8_d(this + 0x601).wrapping_add(id.wrapping_mul(0x91) as usize)) as i32;
    let prontas = cpx8_i(this + 0x5fd);
    if indice < prontas { // cpx8: tática pronta ou própria
        crate::comparacao::ramo("cpx_8 tatica pronta");
        cpx8_d(this + 0x5f9).wrapping_add(indice.wrapping_mul(0x18e3) as usize) // cpx8: o bloco pronto
    } else {
        crate::comparacao::ramo("cpx_8 tatica propria");
        cpx8_d(this + 0x5f1).wrapping_add(indice.wrapping_sub(prontas).wrapping_mul(0x18e3) as usize) // cpx8: o bloco próprio
    }
}

/// Comparador: lado a lado pela devolução e, havendo técnico humano no clube, pelo byte da vaga ativa dele (a 0x5fbe20
/// grava). Clube nulo (caixa): só a original.
unsafe extern "thiscall" fn comparar_tatica_do_clube(this: usize, clube: usize, time: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%);
    if clube == 0 {
        return o(this, clube, time);
    }
    let gerente = cpx8_d(0xb63c98);
    let t = cpx8_d(clube + 0xcf);
    let mut reg = Vec::new();
    if gerente != 0 && t != 0 {
        if let Some(k) = cpx8_indice(gerente, t) {
            reg.push((cpx8_registro(gerente, k) + 0x9a69, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, time), || tatica_do_clube(this, clube, time));
    PLACAR.registrar(igual && r == n, || format!("tatica_do_clube(clube {:#x}, time {}) original={:#x} {:x?} rust={:#x} {:x?}", clube, time, r, mo, n, mr));
    r
}
