use super::*;

// ---------- sexagesima terceira leva: clube controlado por humano ----------

/// { 0x600a50 } (human_manager.cpp): o clube e dirigido por um tecnico humano que decide por ele?
/// O tecnico (`pessoa`, ou o do clube em +0xcf) tem de ser o do clube e um dos 16 humanos. Sem
/// `sem_tatica`: ficam de fora os que tem 0x9ace (0x5fe040) ou +0x9a01 no registro. Com `forcar` ou
/// `qualquer`: sim. Senao, selecao: 0x5fd6b0; clube: 0x5fd480. Se nao, e o clube e do segundo
/// conjunto de selecoes (0x5409b0), repete com a selecao principal da nacao dele (0x53b3d0),
/// ja sem `qualquer`. Clube nulo: a original (caixa de erro, zera 0xb67a34, 0).
pub(crate) unsafe extern "C" fn clube_humano(clube: usize, sem_tatica: u32, qualquer: u8, pessoa: usize, forcar: u32) -> u32 {
    let h = *(0xb63c98 as *const usize);
    let (mut clube, mut qualquer, mut pessoa) = (clube, qualquer, pessoa);
    let t9ace: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_COMPUTER_CONTROLLED);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
    let t9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    loop {
        if clube == 0 {
            return orig::<unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32>(180)(0, sem_tatica, qualquer, pessoa, forcar);
        }
        if pessoa == 0 {
            pessoa = *((clube + 0xcf) as *const usize);
        }
        let tecnico = *((clube + 0xcf) as *const usize);
        if tecnico != 0 && pessoa == tecnico && *(pessoa as *const i32) >= (*(0xae2364 as *const i32)).wrapping_sub(16) {
            let tatica = sem_tatica == 0
                && (t9ace(h, pessoa) != 0 || matches!(tecnico_humano(h, pessoa), Ok(r) if *((r + 0x9a01) as *const u32) != 0));
            if !tatica {
                if forcar != 0 || qualquer != 0 {
                    return 1;
                }
                let sim = if e_selecao(clube) != 0 { t9a06(h, pessoa, core::ptr::null_mut()) as u32 } else { t9a05(h, pessoa) };
                if sim != 0 {
                    return 1;
                }
            }
        }
        if segunda(clube) == 0 {
            return 0;
        }
        qualquer = 0;
        clube = principal(*((clube + 0x53) as *const usize));
    }
}

// ---------- sexagesima segunda leva: tecnico humano (0x9ace, 0x9a06) ----------

/// A pessoa (ou, sem ela, o tecnico humano atual) e o registro dela; `None` nos casos de erro do
/// original (caixa de mensagem), que ficam com ele.
pub(crate) unsafe fn pessoa_e_registro(this: usize, pessoa: usize) -> Option<(usize, usize)> {
    let p = if pessoa != 0 { pessoa } else { *((0xb74c26 + 768 * *(0xb77c7a as *const i16) as isize) as *const usize) };
    if p == 0 {
        return None;
    }
    tecnico_humano(this, p).ok().map(|r| (p, r))
}

/// { 0x5fe040 } (human_manager.cpp): campo +0x9ace do registro do tecnico humano da pessoa.
/// Sem pessoa nenhuma ou fora da faixa: a original (caixa de erro, zera 0xb67a34, 0).
pub(crate) unsafe extern "thiscall" fn tecnico_9ace(this: usize, pessoa: usize) -> u32 {
    match pessoa_e_registro(this, pessoa) {
        Some((_, r)) => *((r + 0x9ace) as *const u32),
        None => orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(178)(this, pessoa),
    }
}

/// { 0x5fd6b0 } (human_manager.cpp): byte +0x9a06 do registro do tecnico humano e diferente de
/// zero? Se a pessoa tem clube (+0x24) de nacao com +0x88 abaixo de 400: falso, e `saida` = 1.
/// `saida` (se houver) comeca em 0. Erros como em `tecnico_9ace`.
pub(crate) unsafe extern "thiscall" fn tecnico_9a06(this: usize, pessoa: usize, saida: *mut u32) -> u8 {
    if !saida.is_null() {
        *saida = 0;
    }
    match pessoa_e_registro(this, pessoa) {
        Some((p, r)) => {
            let clube = *((p + 0x24) as *const usize);
            if clube != 0 && *((*((clube + 0x53) as *const usize) + 0x88) as *const i32) < 400 {
                if !saida.is_null() {
                    *saida = 1;
                }
                return 0;
            }
            (*((r + 0x9a06) as *const u8) != 0) as u8
        }
        None => orig::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8>(179)(this, pessoa, saida),
    }
}

// ---------- quadragesima nona leva: tecnico humano e mascaras de formacao ----------

/// Registro do tecnico humano da pessoa (human_manager.cpp): os 16 tecnicos humanos sao as 16
/// ultimas pessoas (indice = id - quantidade de pessoas em 0xae2364 + 16), registros de 42.735
/// bytes a partir de +0 do objeto, quantidade em +4. Sem pessoa, usa a do tecnico atual (tabela
/// 0xb74c26, 768 bytes por tecnico, indice na palavra 0xb77c7a). `None`: sem pessoa nenhuma, ou
/// indice fora da faixa (o bool diz qual).
pub(crate) unsafe fn tecnico_humano(this: usize, pessoa: usize) -> Result<usize, bool> {
    let mut pessoa = pessoa;
    if pessoa == 0 {
        let atual = *(0xb77c7a as *const i16) as isize;
        pessoa = *((0xb74c26 + 768 * atual) as *const usize);
        if pessoa == 0 {
            return Err(true);
        }
    }
    let i = (*(pessoa as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(16);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Err(false);
    }
    Ok((*(this as *const usize)).wrapping_add((42735 * i) as usize))
}

/// { 0x5fd210 } (human_manager.cpp): campo +0x9a01 do registro do tecnico humano da pessoa (0 fora
/// da faixa). Sem pessoa nenhuma: o original mostra uma caixa de erro e zera 0xb67a34.
pub(crate) unsafe extern "thiscall" fn tecnico_9a01(this: usize, pessoa: usize) -> u32 {
    match tecnico_humano(this, pessoa) {
        Ok(r) => *((r + 0x9a01) as *const u32),
        Err(sem_pessoa) => {
            if sem_pessoa {
                *(0xb67a34 as *mut u32) = 0;
            }
            0
        }
    }
}

/// { 0x5fd480 } (human_manager.cpp): byte +0x9a05 do registro do tecnico humano da pessoa e diferente
/// de zero? Sem pessoa nenhuma ou fora da faixa: o original mostra uma caixa de erro e zera 0xb67a34.
pub(crate) unsafe extern "thiscall" fn tecnico_9a05(this: usize, pessoa: usize) -> u32 {
    match tecnico_humano(this, pessoa) {
        Ok(r) => (*((r + 0x9a05) as *const u8) != 0) as u32,
        Err(_) => {
            *(0xb67a34 as *mut u32) = 0;
            0
        }
    }
}

/// { 0x5b5a60 } (formation.cpp): bit da mascara de formacao pelo codigo da casa tatica (byte alto:
/// linha 1, 2, 4, 8; byte baixo: coluna). Tabela tirada do proprio jogo; 0x802 e 0x804 dao o mesmo
/// bit no original. Codigo desconhecido: 0.
pub(crate) unsafe extern "stdcall" fn bit_da_casa(codigo: i16) -> u32 {
    match codigo {
        0x1 => 0x1, 0x82 => 0x2, 0x84 => 0x40, 0x88 => 0x800, 0x90 => 0x10000, 0xa0 => 0x200000, 0xc0 => 0x4000000,
        0x102 => 0x4, 0x104 => 0x80, 0x108 => 0x1000, 0x110 => 0x20000, 0x120 => 0x400000, 0x140 => 0x8000000,
        0x202 => 0x8, 0x204 => 0x100, 0x208 => 0x2000, 0x210 => 0x40000, 0x220 => 0x800000, 0x240 => 0x10000000,
        0x402 => 0x10, 0x404 => 0x200, 0x408 => 0x4000, 0x410 => 0x80000, 0x420 => 0x1000000, 0x440 => 0x20000000,
        0x802 | 0x804 => 0x400, 0x808 => 0x8000, 0x810 => 0x100000, 0x820 => 0x2000000, 0x840 => 0x40000000,
        _ => 0,
    }
}

/// { 0x5b73a0 } (formation.cpp): separa a mascara em setor (o primeiro de 1, 2, 4 ... 0x40, em
/// `setor`) e lado (0x80, 0x100, 0x200 (tambem com o bit 1), 0x400, 0x800, nessa ordem, em `lado`).
/// Sem uma das partes: o original mostra uma caixa de erro, zera 0xb67a34 e devolve 0.
pub(crate) unsafe extern "C" fn separar_mascara(m: u32, setor: *mut u16, lado: *mut u16) -> u32 {
    match [1u16, 2, 4, 8, 0x10, 0x20, 0x40].iter().find(|&&b| m & b as u32 != 0) {
        Some(&b) => *setor = b,
        None => {
            *(0xb67a34 as *mut u32) = 0;
            return 0;
        }
    }
    let v = if m & 0x80 != 0 {
        0x80
    } else if m & 0x100 != 0 {
        0x100
    } else if m & 0x200 != 0 || m & 1 != 0 {
        0x200
    } else if m & 0x400 != 0 {
        0x400
    } else if m & 0x800 != 0 {
        0x800
    } else {
        *(0xb67a34 as *mut u32) = 0;
        return 0;
    };
    *lado = v;
    1
}

// ---------- 260a leva: a tática ativa do clube: a tática do time do clube, a vaga de tática do técnico humano e a tática ativa dele (tactics.cpp, human_manager.cpp) ----------


pub(crate) unsafe fn cpx8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



/// O técnico pedido ou, nulo, o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
pub(crate) unsafe fn cpx8_tecnico(tecnico: usize) -> usize {
    if tecnico != 0 {
        return tecnico;
    }
    read_usize((read_i16(0xb77c7a) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize)
}

/// O índice do registro do técnico humano no gerenciador `this` (id − [0xae2364] + 16), se estiver entre 0 e a conta
/// ([this+4]) − 1; fora disso a original mostra a caixa "Error".
pub(crate) unsafe fn cpx8_indice(this: usize, tecnico: usize) -> Option<usize> {
    let k = cpx8_i(tecnico).wrapping_sub(cpx8_i(0xae2364)).wrapping_add(0x10);
    if k < 0 || k > cpx8_i(this + 4).wrapping_sub(1) {
        return None;
    }
    Some(k as usize)
}

/// O registro do técnico humano `k` (0xa6ef bytes cada, a partir de [this]).
pub(crate) unsafe fn cpx8_registro(this: usize, k: usize) -> usize {
    read_usize(this).wrapping_add(k.wrapping_mul(0xa6ef))
}

/// the tactic block (0x1ecd bytes) of the active slot of the human manager record
/// Original: `HUMAN_MANAGER::get_active_tactic` (0x5fbd00, human_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x5fbd00 } (thiscall, ret 4, human_manager.cpp): `HUMAN_MANAGER::get_active_tactic(tecnico)`, pela escalação
/// (0x6db500), pela formação (0x6e0230), pelo dia de jogo (0x6b4360), pela tática do clube (0x896660) e pelas telas. O
/// bloco de tática (0x1ecd bytes) da vaga ativa (byte com sinal em +0x9a69 do registro) do técnico (nulo: o humano da
/// vez). Sem técnico ou fora da conta: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn tatica_ativa_do_humano(this: usize, tecnico: usize) -> usize {
    let t = cpx8_tecnico(tecnico);
    let Some(k) = (if t != 0 { cpx8_indice(this, t) } else { None }) else {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1328)(this, tecnico); // caixa: só a original
    };
    let vaga = read_u8(cpx8_registro(this, k) + 0x9a69) as i8 as i32;
    read_usize(this).wrapping_add(vaga.wrapping_mul(0x1ecd) as usize).wrapping_add(k.wrapping_mul(0xa6ef))
}

/// Comparador: a devolução (rotina pura); sem técnico ou fora da conta, só a original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_tatica_ativa_do_humano(this: usize, tecnico: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1328);
    let t = cpx8_tecnico(tecnico);
    if t == 0 || cpx8_indice(this, t).is_none() {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = tatica_ativa_do_humano(this, tecnico);
    PLACAR.registrar(r == n, || format!("tatica_ativa_do_humano(tecnico {:#x}) original={:#x} rust={:#x}", tecnico, r, n));
    r
}

/// the human manager active slot: 4 for the second national team, 3 with reserve in a national team, 2 in a national
/// team, 1 with reserve, 0 for the first team
/// Original: `HUMAN_MANAGER::select_tactic_slot` (0x5fbe20, human_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5fbe20 } (thiscall, ret 0xc, human_manager.cpp): `HUMAN_MANAGER::select_tactic_slot(reserva, clube, tecnico)`,
/// pelos mesmos chamadores da 0x5fbd00. Técnico nulo: o humano da vez (sem nenhum, nada); clube nulo: o do técnico
/// (+0x39). A vaga ativa (+0x9a69 do registro): 4 na segunda seleção (0x5409b0), 3 com `reserva` numa seleção
/// (0x53b900), 2 numa seleção, 1 com `reserva`, 0 no time principal. Fora da conta: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn escolher_vaga_de_tatica(this: usize, reserva: u32, clube: usize, tecnico: usize) {
    let t = cpx8_tecnico(tecnico);
    if t == 0 {
        return;
    }
    let Some(k) = cpx8_indice(this, t) else {
        return orig::<unsafe extern "thiscall" fn(usize, u32, usize, usize)>(1327)(this, reserva, clube, tecnico); // caixa: só a original
    };
    let clube = if clube != 0 { clube } else { read_usize(t + 0x39) };
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
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
pub(crate) unsafe extern "thiscall" fn comparar_escolher_vaga_de_tatica(this: usize, reserva: u32, clube: usize, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, usize)>(1327);
    let t = cpx8_tecnico(tecnico);
    let Some(k) = (if t != 0 { cpx8_indice(this, t) } else { None }) else {
        return o(this, reserva, clube, tecnico);
    };
    let reg = [(cpx8_registro(this, k) + 0x9a69, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, reserva, clube, tecnico), || escolher_vaga_de_tatica(this, reserva, clube, tecnico));
    PLACAR.registrar(igual, || format!("escolher_vaga_de_tatica(reserva {}, clube {:#x}, tecnico {:#x}) {:x?} x {:x?}", reserva, clube, tecnico, mo, mr));
}

/// for a human club (without +0x9a01 or with holiday option 2), the slot for the team and the manager active block;
/// otherwise, by the index in the club row, the preset block (below the preset count) or the own one
/// Original: `Tactics::get_club_tactic` (0x896660, tactics.cpp, thiscall, ret 8).
///
/// [pt] { 0x896660 } (thiscall, ret 8, tactics.cpp): `Tactics::get_club_tactic(clube, time)`, pela formação de cada jogo
/// (0x6e0230), pelas fraquezas do jogador (0x5470d0), pela mídia e pelas telas. Clube humano (0x600a50 com 1, 1) cujo
/// técnico (+0xcf) não tem +0x9a01 (0x5fd210) ou tem a opção 2 das férias (0x5fea50): a vaga pelo time (0x5fbe20) e o
/// bloco ativo dele (0x5fbd00). Nos outros, o índice (palavra com sinal) na linha do clube (id × 0x91 em [this+0x601]):
/// abaixo da conta das táticas prontas ([this+0x5fd]) o bloco pronto ([this+0x5f9]), senão o próprio ([this+0x5f1],
/// índice − conta); blocos de 0x18e3 bytes. Clube nulo: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn tatica_do_clube(this: usize, clube: usize, time: u32) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1326)(this, clube, time); // caixa: só a original
    }
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if humano(clube, 1, 1, 0, 0) != 0 {
        let gerente = read_usize(0xb63c98);
        let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
        let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
        if t9a01(gerente, read_usize(clube + 0xcf)) == 0 || opcao(gerente, 2, read_usize(clube + 0xcf)) != 0 {
            let escolher: unsafe extern "thiscall" fn(usize, u32, usize, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SELECT_TACTIC_SLOT);
            let ativa: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_ACTIVE_TACTIC);
            crate::comparacao::ramo("cpx_8 tatica do humano");
            escolher(gerente, time, clube, read_usize(clube + 0xcf)); // cpx8: a vaga pelo time
            return ativa(gerente, read_usize(clube + 0xcf));
        }
    }
    let id = cpx8_i(clube);
    let indice = read_i16(read_usize(this + 0x601).wrapping_add(id.wrapping_mul(0x91) as usize)) as i32;
    let prontas = cpx8_i(this + 0x5fd);
    if indice < prontas { // cpx8: tática pronta ou própria
        crate::comparacao::ramo("cpx_8 tatica pronta");
        read_usize(this + 0x5f9).wrapping_add(indice.wrapping_mul(0x18e3) as usize) // cpx8: o bloco pronto
    } else {
        crate::comparacao::ramo("cpx_8 tatica propria");
        read_usize(this + 0x5f1).wrapping_add(indice.wrapping_sub(prontas).wrapping_mul(0x18e3) as usize) // cpx8: o bloco próprio
    }
}

/// Comparador: lado a lado pela devolução e, havendo técnico humano no clube, pelo byte da vaga ativa dele (a 0x5fbe20
/// grava). Clube nulo (caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_tatica_do_clube(this: usize, clube: usize, time: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1326);
    if clube == 0 {
        return o(this, clube, time);
    }
    let gerente = read_usize(0xb63c98);
    let t = read_usize(clube + 0xcf);
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

// ---------- 246a leva: limpar as partidas assistidas que os humanos não podem ver, os alvos dos olheiros do clube e trocar o olheiro da busca ----------


/// A partida assistida do técnico humano `i` (79 bytes em [this] + 0x9ad4, 0xa6ef bytes por humano).
pub(crate) unsafe fn cpj5_assistida(this: usize, i: usize) -> usize {
    read_usize(this).wrapping_add(0x9ad4).wrapping_add(i.wrapping_mul(0xa6ef))
}

/// clears (79 bytes) the watched fixture of each active human manager who may no longer watch it (0x5fe700); returns 1
/// Original: `HUMAN_MANAGER::clear_invalid_watched_fixtures` (0x5fe8d0, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fe8d0 } (thiscall, human_manager.cpp): `HUMAN_MANAGER::clear_invalid_watched_fixtures()`: para cada um dos 16
/// técnicos humanos (os últimos 16 de [0xae23bc]) ativo (+0x5f ≠ 0) que não pode mais ver a partida assistida dele
/// (0x5fe700), zera os 79 bytes dela. Devolve 1.
pub(crate) unsafe extern "thiscall" fn limpar_partidas_assistidas(this: usize) -> u32 {
    let pode_ver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CAN_WATCH_FIXTURE);
    for i in 0..0x10usize {
        let p = read_usize(0xae23bc).wrapping_add(read_usize(0xae2364).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
        if *((p + 0x5f) as *const u8) == 0 {
            continue;
        }
        let f = cpj5_assistida(this, i);
        if pode_ver(this, f, p) == 0 {
            core::ptr::write_bytes(f as *mut u8, 0, 79); // a partida sai
        }
    }
    1
}

/// Os pedidos de observação (0x11 bytes em [this+0x5c], [this+0x64] deles) do clube `id` (+0): as pessoas (+4).
pub(crate) unsafe fn cpj5_alvos(this: usize, id: u32) -> Vec<usize> {
    let mut v = Vec::new();
    let mut k = 0i32;
    while k < read_usize(this + 0x64) as i32 {
        let e = read_usize(this + 0x5c).wrapping_add(k as usize * 0x11);
        if read_usize(e) as u32 == id { // o pedido do clube
            v.push(read_usize(0xae23bc).wrapping_add(read_usize(e + 4).wrapping_mul(0x6e)));
        }
        k += 1;
    }
    v
}

/// empties the list and adds the person of each scout request of the club; returns whether there is any
/// Original: `SCOUT_MANAGER::collect_club_scout_targets` (0x7f9690, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f9690 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::collect_club_scout_targets(clube, lista)`: com
/// `lista`, esvazia-a (0x68d080) e acrescenta (0x68d0d0, 0x6e bytes) a pessoa de cada pedido de observação do clube
/// `*clube`. Devolve se há algum. Clube nulo: 0 (a lista fica vazia).
pub(crate) unsafe extern "thiscall" fn alvos_dos_olheiros_do_clube(this: usize, clube: *const u32, lista: usize) -> u32 {
    if lista != 0 {
        let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
        esvaziar(lista);
    }
    if clube.is_null() {
        return 0;
    }
    let alvos = cpj5_alvos(this, *clube);
    if lista != 0 {
        let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
        for &p in &alvos {
            acrescentar(lista, p, 0x6e);
        }
    }
    (!alvos.is_empty()) as u32
}

/// new scout: stores the ids (0x7e4920) or −1, and the nation, region and competition assignments go back to −1;
/// returns 1
/// Original: `PLAYER_SEARCH::set_scout` (0x7ae8a0, player_search.cpp, thiscall, ret 4).
///
/// [pt] { 0x7ae8a0 } (thiscall, ret 4, player_search.cpp): `PLAYER_SEARCH::set_scout(olheiro)`: se o olheiro muda (o atual
/// pela 0x7ae950), grava os ids dele em +0x174 (0x7e4920) ou, sem olheiro, o índice +0x180 = −1; e as missões de nação
/// (+0x18c), região (+0x194) e competição (+0x195) voltam a −1. Devolve 1.
pub(crate) unsafe extern "thiscall" fn trocar_olheiro_da_busca(this: usize, olheiro: usize) -> u32 {
    if olheiro == olheiro_da_busca(this) {
        return 1; // o mesmo olheiro
    }
    if olheiro != 0 {
        let gravar: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_FILL_STAFF_NAME_REF);
        gravar(olheiro, this + 0x174);
    } else {
        core::ptr::write_unaligned((this + 0x180) as *mut u32, u32::MAX);
    }
    core::ptr::write_unaligned((this + 0x18c) as *mut u32, u32::MAX);
    *((this + 0x194) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x195) as *mut u32, u32::MAX);
    1
}

/// Comparador: lado a lado pelas 16 partidas assistidas (a 0x5fe700 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_limpar_partidas_assistidas(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(876);
    let reg: Vec<(usize, usize)> = (0..0x10).map(|i| (cpj5_assistida(this, i), 79)).collect();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || limpar_partidas_assistidas(this));
    PLACAR.registrar(igual && r == n, || format!("limpar_partidas_assistidas {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: os alvos da cópia antes; a original enche a lista; confere o valor e a lista (cada item, os 0x6e bytes
/// da pessoa).
pub(crate) unsafe extern "thiscall" fn comparar_alvos_dos_olheiros_do_clube(this: usize, clube: *const u32, lista: usize) -> u32 {
    let alvos = if clube.is_null() { Vec::new() } else { cpj5_alvos(this, *clube) };
    let r = orig::<unsafe extern "thiscall" fn(usize, *const u32, usize) -> u32>(877)(this, clube, lista);
    let mut igual = r == (!alvos.is_empty()) as u32;
    if lista != 0 {
        let itens = cpd1_itens(lista);
        igual &= itens.len() == alvos.len()
            && itens.iter().zip(&alvos).all(|(&i, &p)| core::slice::from_raw_parts(i as *const u8, 0x6e) == core::slice::from_raw_parts(p as *const u8, 0x6e));
    }
    PLACAR.registrar(igual, || format!("alvos_dos_olheiros_do_clube original={} alvos {}", r, alvos.len()));
    r
}

/// Comparador: lado a lado pelo olheiro e as missões (+0x174 a +0x199; a 0x53e990 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_trocar_olheiro_da_busca(this: usize, olheiro: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(878);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x174, 0x25)], || o(this, olheiro), || trocar_olheiro_da_busca(this, olheiro));
    PLACAR.registrar(igual && r == n, || format!("trocar_olheiro_da_busca({:x}) {:x?} x {:x?}", olheiro, mo, mr));
    r
}

// ---------- 216a leva: a partida assistida por algum humano, o apelido dado pelo humano e a cópia da aposentadoria pendente (human_manager.cpp, player_regen.cpp) ----------

/// whether any of the active human managers chose to watch the fixture
/// Original: `HUMAN_MANAGER::is_fixture_watched` (0x5fe420, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fe420 } (thiscall, ret 4, human_manager.cpp): 1 se algum dos 16 técnicos humanos ativos
/// (registros de 110 bytes em [0xae23bc], a partir de [0xae2364] − 16, +0x5f não nulo) escolheu
/// assistir a `partida` (0x5fe290); senão 0. Partida nula: a original (caixa de erro e 0).
pub(crate) unsafe extern "thiscall" fn humano_assiste_a_partida(this: usize, partida: usize) -> u32 {
    if partida == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(551)(this, partida);
    }
    let assiste: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_WATCHING_FIXTURE);
    for k in 0..16i32 {
        let i = k.wrapping_add(*(0xae2364 as *const i32)).wrapping_sub(0x10);
        let h = (*(0xae23bc as *const usize)).wrapping_add((i as isize * 110) as usize);
        if *((h + 0x5f) as *const u8) != 0 && assiste(this, partida, h) != 0 {
            return 1;
        }
    }
    0 // ninguém assiste
}

/// O índice (0..[this+4]) do técnico humano `humano` (ou o da vez, [0xb74c26 + 0x300 × [0xb77c7a]])
/// para a 0x600740: None se não há humano; Some(None) fora da faixa (caso de erro da original).
pub(crate) unsafe fn indice_do_humano_do_apelido(this: usize, humano: usize) -> Option<Option<usize>> {
    let h = if humano != 0 {
        humano
    } else {
        *((0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return None;
    }
    let i = (*(h as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Some(None);
    }
    Some(Some(i as usize))
}

/// whether the human manager (or the current one) gave the person a nickname, by binary search in his list
/// Original: `HUMAN_MANAGER::has_nickname` (0x600740, human_manager.cpp, thiscall).
///
/// [pt] { 0x600740 } (thiscall, ret 8, human_manager.cpp): 1 se o técnico humano `humano` (0 = o da vez)
/// deu apelido à pessoa de id [`pessoa`]: busca binária (0x946464 com 0x601a10, itens de 0x1e bytes)
/// na lista +0xa6e3 (quantos em +0xa6eb) do registro dele (0xa6ef bytes em [this]). Pessoa ou humano
/// nulos, ou lista vazia: 0. Humano fora da faixa: a original (caixa de erro e 0).
pub(crate) unsafe extern "thiscall" fn humano_deu_apelido(this: usize, pessoa: usize, humano: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let i = match indice_do_humano_do_apelido(this, humano) {
        None => return 0,
        Some(None) => return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(552)(this, pessoa, humano),
        Some(Some(i)) => i,
    };
    let buscar: unsafe extern "C" fn(*const u32, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let r = (*(this as *const usize)).wrapping_add(i.wrapping_mul(0xa6ef));
    let lista = core::ptr::read_unaligned((r + 0xa6e3) as *const usize);
    // a chave: o id e um byte 0 (a 0x601a10 só compara o id)
    let chave = [*(pessoa as *const u32), 0u32];
    let achou = if lista == 0 { 0 } else { (buscar(chave.as_ptr(), lista, core::ptr::read_unaligned((r + 0xa6eb) as *const u32), 0x1e, 0x601a10) != 0) as u32 };
    achou // apelido
}

/// copies the pending retirement record: the person, the club, the two dates, the reason and the announced flag
/// Original: `retire_record_assign` (0x7a8780, player_regen.cpp, thiscall).
///
/// [pt] { 0x7a8780 } (thiscall, ret 4, player_regen.cpp): copia a aposentadoria pendente (0x1a bytes) de
/// `origem`: o id da pessoa (+0) e do clube (+4), as datas em +8 e +0x10 (0x417c10, com o retorno numa
/// local), o motivo (+0x18) e a marca de anunciada (+0x19). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_aposentadoria_pendente(this: usize, origem: usize) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut local = [0u8; 8];
    *(this as *mut u32) = *(origem as *const u32);
    *((this + 4) as *mut u32) = *((origem + 4) as *const u32);
    copiar(this + 8, &mut local, origem + 8);
    copiar(this + 0x10, &mut local, origem + 0x10);
    *((this + 0x18) as *mut u8) = *((origem + 0x18) as *const u8); // motivo
    *((this + 0x19) as *mut u8) = *((origem + 0x19) as *const u8);
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_humano_assiste_a_partida(this: usize, partida: usize) -> u32 {
    // a 0x5fe290 só lê: pode rodar nos dois lados
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(551);
    if partida == 0 {
        return o(this, partida);
    }
    let r = o(this, partida);
    let n = humano_assiste_a_partida(this, partida);
    PLACAR.registrar(r == n, || format!("humano_assiste_a_partida original={} rust={}", r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_humano_deu_apelido(this: usize, pessoa: usize, humano: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(552);
    if pessoa != 0 && matches!(indice_do_humano_do_apelido(this, humano), Some(None)) {
        return o(this, pessoa, humano);
    }
    let r = o(this, pessoa, humano);
    let n = humano_deu_apelido(this, pessoa, humano);
    PLACAR.registrar(r == n, || format!("humano_deu_apelido original={} rust={}", r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_aposentadoria_pendente(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(553);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, origem), || copiar_aposentadoria_pendente(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_aposentadoria_pendente {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 197a leva: o registro do técnico humano (hora das férias e escalação guardada) e o técnico da busca (human_manager.cpp, player_search.cpp) ----------

/// O registro de 42735 bytes do técnico humano `pessoa` (nula: o jogador de rede da vaga do humano da
/// vez, [0xb74c26 + 0x300 × vaga]). Os humanos são as 16 últimas pessoas: índice = id − ([0xae2364] − 16),
/// válido de 0 a [gerente+4] − 1. `None` quando a original dá a caixa de erro.
pub(crate) unsafe fn registro_do_tecnico_humano(gerente: usize, pessoa: usize) -> Option<usize> {
    let pessoa = if pessoa != 0 { pessoa } else { *((0xb74c26 + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize) };
    if pessoa == 0 {
        return None;
    }
    let d = (*(pessoa as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if d < 0 || d > *((gerente + 4) as *const i32) - 1 {
        return None;
    }
    Some((*(gerente as *const usize) as isize + d as isize * 42735) as usize)
}

/// the clock stamp stored when the human manager went on holiday
/// Original: `HUMAN_MANAGER::get_holiday_stamp` (0x5fdc30, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fdc30 } (thiscall, ret 4, human_manager.cpp): a hora (relógio em ms) guardada quando o técnico
/// humano saiu de férias (+0x9a65 do registro). Fora da faixa: a original (caixa de erro, 0).
pub(crate) unsafe extern "thiscall" fn hora_das_ferias(gerente: usize, pessoa: usize) -> u32 {
    match registro_do_tecnico_humano(gerente, pessoa) {
        Some(r) => core::ptr::read_unaligned((r + 0x9a65) as *const u32),
        None => orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(463)(gerente, pessoa),
    }
}

/// the address of the human manager's saved line-up (16 pairs)
/// Original: `HUMAN_MANAGER::get_saved_lineup` (0x5fed50, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fed50 } (thiscall, ret 4, human_manager.cpp): o endereço da escalação guardada do técnico
/// humano (16 pares jogador e posição, +0x9b28 do registro). Fora da faixa: a original (caixa de erro, 0).
pub(crate) unsafe extern "thiscall" fn escalacao_guardada(gerente: usize, pessoa: usize) -> usize {
    match registro_do_tecnico_humano(gerente, pessoa) {
        Some(r) => r + 0x9b28,
        None => orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(464)(gerente, pessoa),
    }
}

/// stores the human manager id in the player search
/// Original: `PLAYER_SEARCH::set_manager` (0x7ae570, player_search.cpp, thiscall).
///
/// [pt] { 0x7ae570 } (thiscall, ret 4, player_search.cpp): grava na busca (+0) o id do técnico humano
/// `pessoa` e devolve 1. Pessoa nula ou que não é humana (id abaixo de [0xae2364] − 16): a original
/// (caixa de erro, 0).
pub(crate) unsafe extern "thiscall" fn tecnico_da_busca(busca: usize, pessoa: usize) -> u32 {
    if pessoa == 0 || *(pessoa as *const i32) < *(0xae2364 as *const i32) - 0x10 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(465)(busca, pessoa);
    }
    *(busca as *mut i32) = *(pessoa as *const i32);
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_hora_das_ferias(gerente: usize, pessoa: usize) -> u32 {
    if registro_do_tecnico_humano(gerente, pessoa).is_none() {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(463)(gerente, pessoa);
    }
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(463)(gerente, pessoa);
    let n = hora_das_ferias(gerente, pessoa);
    PLACAR.registrar(r == n, || format!("hora_das_ferias original={} rust={}", r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_escalacao_guardada(gerente: usize, pessoa: usize) -> usize {
    if registro_do_tecnico_humano(gerente, pessoa).is_none() {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(464)(gerente, pessoa);
    }
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(464)(gerente, pessoa);
    let n = escalacao_guardada(gerente, pessoa);
    PLACAR.registrar(r == n, || format!("escalacao_guardada original={:#x} rust={:#x}", r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_tecnico_da_busca(busca: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(465);
    if pessoa == 0 || *(pessoa as *const i32) < *(0xae2364 as *const i32) - 0x10 {
        return o(busca, pessoa);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca, 4)], || o(busca, pessoa), || tecnico_da_busca(busca, pessoa));
    PLACAR.registrar(igual && ro == rr, || format!("tecnico_da_busca original={} rust={} {:x?} x {:x?}", ro, rr, mo, mr));
    ro
}

