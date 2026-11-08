use super::*;

// ---------- sexagesima terceira leva: clube controlado por humano ----------

/// { 0x600a50 } (human_manager.cpp): o clube e dirigido por um tecnico humano que decide por ele?
/// O tecnico (`pessoa`, ou o do clube em +0xcf) tem de ser o do clube e um dos 16 humanos. Sem
/// `sem_tatica`: ficam de fora os que tem 0x9ace (0x5fe040) ou +0x9a01 no registro. Com `forcar` ou
/// `qualquer`: sim. Senao, selecao: 0x5fd6b0; clube: 0x5fd480. Se nao, e o clube e do segundo
/// conjunto de selecoes (0x5409b0), repete com a selecao principal da nacao dele (0x53b3d0),
/// ja sem `qualquer`. Clube nulo: a original (caixa de erro, zera 0xb67a34, 0).
pub(crate) unsafe extern "C" fn clube_humano(clube: usize, sem_tatica: u32, qualquer: u8, pessoa: usize, forcar: u32) -> u32 {
    let h = *(G_HUMAN_MANAGER as *const usize);
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
            pessoa = *((clube + club::MANAGER) as *const usize);
        }
        let tecnico = *((clube + club::MANAGER) as *const usize);
        if tecnico != 0 && pessoa == tecnico && *(pessoa as *const i32) >= (*(G_STAFF_COUNT as *const i32)).wrapping_sub(16) {
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
        clube = principal(*((clube + club::NATION) as *const usize));
    }
}

// ---------- sexagesima segunda leva: tecnico humano (0x9ace, 0x9a06) ----------

/// A pessoa (ou, sem ela, o tecnico humano atual) e o registro dela; `None` nos casos de erro do
/// original (caixa de mensagem), que ficam com ele.
pub(crate) unsafe fn pessoa_e_registro(this: usize, pessoa: usize) -> Option<(usize, usize)> {
    let p = if pessoa != 0 { pessoa } else { *((0xb74c26 + 768 * *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize) as *const usize) };
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
            if clube != 0 && *((*((clube + club::NATION) as *const usize) + 0x88) as *const i32) < 400 {
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
        let atual = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        pessoa = *((0xb74c26 + 768 * atual) as *const usize);
        if pessoa == 0 {
            return Err(true);
        }
    }
    let i = (*(pessoa as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(16);
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
                *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
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
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
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
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
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
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        return 0;
    };
    *lado = v;
    1
}

// ---------- 260a leva: a tática ativa do clube: a tática do time do clube, a vaga de tática do técnico humano e a tática ativa dele (tactics.cpp, human_manager.cpp) ----------





/// O técnico pedido ou, nulo, o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
pub(crate) unsafe fn cpx8_tecnico(tecnico: usize) -> usize {
    if tecnico != 0 {
        return tecnico;
    }
    read_usize((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize)
}

/// O índice do registro do técnico humano no gerenciador `this` (id − [0xae2364] + 16), se estiver entre 0 e a conta
/// ([this+4]) − 1; fora disso a original mostra a caixa "Error".
pub(crate) unsafe fn cpx8_indice(this: usize, tecnico: usize) -> Option<usize> {
    let k = read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10);
    if k < 0 || k > read_i32(this + 4).wrapping_sub(1) {
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
        let p = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(G_STAFF_COUNT).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
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
            v.push(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(e + 4).wrapping_mul(0x6e)));
        }
        k += 1;
    }
    v
}



/// Comparador: lado a lado pelas 16 partidas assistidas (a 0x5fe700 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_limpar_partidas_assistidas(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(876);
    let reg: Vec<(usize, usize)> = (0..0x10).map(|i| (cpj5_assistida(this, i), 79)).collect();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || limpar_partidas_assistidas(this));
    PLACAR.registrar(igual && r == n, || format!("limpar_partidas_assistidas {:x?} x {:x?}", mo, mr));
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
        let i = k.wrapping_add(*(G_STAFF_COUNT as *const i32)).wrapping_sub(0x10);
        let h = (*(G_STAFF_TABLE as *const usize)).wrapping_add((i as isize * 110) as usize);
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
        *((0xb74c26 + 0x300 * *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return None;
    }
    let i = (*(h as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
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


// ---------- 197a leva: o registro do técnico humano (hora das férias e escalação guardada) e o técnico da busca (human_manager.cpp, player_search.cpp) ----------

/// O registro de 42735 bytes do técnico humano `pessoa` (nula: o jogador de rede da vaga do humano da
/// vez, [0xb74c26 + 0x300 × vaga]). Os humanos são as 16 últimas pessoas: índice = id − ([0xae2364] − 16),
/// válido de 0 a [gerente+4] − 1. `None` quando a original dá a caixa de erro.
pub(crate) unsafe fn registro_do_tecnico_humano(gerente: usize, pessoa: usize) -> Option<usize> {
    let pessoa = if pessoa != 0 { pessoa } else { *((0xb74c26 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 0x300) as *const usize) };
    if pessoa == 0 {
        return None;
    }
    let d = (*(pessoa as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
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

/// the club's assistant or, without one, the highest-reputation staff member
/// Original: `human_manager_get_stand_in_manager` (0x600e90, human_manager.cpp, cdecl).
///
/// [pt] { 0x600e90 } (cdecl, human_manager.cpp): quem responde pelo técnico do clube: o auxiliar (+0xd3)
/// se houver; senão, dos cinco da comissão (+0x19f) com dados (+0x69), o de maior reputação (+0xa,
/// ficando o primeiro no empate); 0 se nenhum.
pub(crate) unsafe extern "C" fn tecnico_substituto(clube: usize) -> usize {
    let auxiliar = core::ptr::read_unaligned((clube + club::ASSISTANT_MANAGER) as *const usize);
    if auxiliar != 0 {
        return auxiliar;
    }
    let reputacao = |p: usize| core::ptr::read_unaligned((core::ptr::read_unaligned((p + 0x69) as *const usize) + 0xa) as *const i16);
    let mut melhor = 0usize;
    for k in 0..5 {
        let c = core::ptr::read_unaligned((clube + club::COACHES + k * 4) as *const usize);
        if c == 0 || core::ptr::read_unaligned((c + 0x69) as *const usize) == 0 {
            continue;
        }
        if melhor == 0 || reputacao(c) > reputacao(melhor) {
            melhor = c;
        }
    }
    melhor
}

pub(crate) unsafe extern "C" fn comparar_tecnico_substituto(clube: usize) -> usize {
    let r = orig::<unsafe extern "C" fn(usize) -> usize>(444)(clube);
    let n = tecnico_substituto(clube);
    PLACAR.registrar(r == n, || format!("tecnico_substituto({:#x}) original={:#x} rust={:#x}", clube, r, n));
    r
}

/// whether the fixture is the one the human manager chose to watch
/// Original: `HUMAN_MANAGER::is_watching_fixture` (0x5fe290, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fe290 } (thiscall, ret 8, human_manager.cpp): se a `partida` é a que o humano escolheu
/// assistir: o registro dele (+0x9ad4..) com a mesma competição (+0x3f), datas (+0x2a, +0x28), clube
/// (+0), +0x34 e os dois lados (+0xc, +0x10). Partida nula ou humano fora da faixa: a original (caixa
/// de erro e 0).
pub(crate) unsafe extern "thiscall" fn partida_assistida_pelo_humano(this: usize, partida: usize, humano: usize) -> u32 {
    let r = match registro_da_partida_assistida(this, humano) {
        Err(()) => return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(542)(this, partida, humano),
        Ok(_) if partida == 0 => return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(542)(this, partida, humano),
        Ok(0) => return 0,
        Ok(r) => r,
    };
    let b = |p: usize, o: usize| *((p + o) as *const u8);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u16);
    let d = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u32);
    let mesma = b(partida, 0x3f) == b(r, 0x9b13)
        && w(partida, 0x2a) == w(r, 0x9afe)
        && w(partida, 0x28) == w(r, 0x9afc)
        && d(partida, 0) == d(r, 0x9ad4)
        && w(partida, 0x34) == w(r, 0x9b08)
        && d(partida, 0xc) == d(r, 0x9ae0)
        && d(partida, 0x10) == d(r, 0x9ae4);
    if mesma { 1 } else { 0 } // outra partida
}

pub(crate) unsafe extern "thiscall" fn comparar_partida_assistida_pelo_humano(this: usize, partida: usize, humano: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(542);
    let r = o(this, partida, humano);
    // ponytail: os casos de erro (caixa) só na original
    if partida == 0 || registro_da_partida_assistida(this, humano).is_err() {
        return r;
    }
    let n = partida_assistida_pelo_humano(this, partida, humano);
    PLACAR.registrar(r == n, || format!("partida_assistida_pelo_humano original={} rust={}", r, n));
    r
}

/// 1 when the fixture has a negative +0x43 and neither it nor the fixtures of that day (each key nation group) have a
/// club of the human; else 0
/// Original: `HUMAN_MANAGER::can_watch_fixture` (0x5fe700, human_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x5fe700 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::can_watch_fixture(partida, humano)`: o humano
/// (ou, nulo, o da tela atual [0xb74c26 + 0x300 × [0xb77c7a]]) pode ver a partida se ela tem +0x43 negativo, nenhum dos
/// dois clubes (+0x1c, +0x20, com as marcas +0x40, +0x41) é dele (0x600a50) e, em cada grupo das nações-chave
/// ([0xb64738]), nenhuma partida da lista do dia dela (data +0x28/+0x2a, fatia +0x3f; 0x5ae6a0) tem um clube dele: 1.
/// Senão 0. Partida nula: a caixa "Error" (0x707) e 0. Pela limpeza das partidas assistidas (0x5fe8d0) e pela escolha
/// da partida (0x5fe500).
pub(crate) unsafe extern "thiscall" fn pode_ver_a_partida(this: usize, partida: usize, humano: usize) -> u32 {
    if partida == 0 {
        aviso_em(0x9cf11c, 0x707);
        return 0;
    }
    let mut humano = humano;
    if humano == 0 {
        humano = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize).wrapping_mul(0x300)));
        if humano == 0 {
            return 0;
        }
    }
    if read_u8(partida + 0x43) as i8 > -1 {
        return 0;
    }
    let dele = |p: usize| cpr5_humano(read_usize(p + 0x1c), read_u8(p + 0x40), humano) != 0
        || cpr5_humano(read_usize(p + 0x20), read_u8(p + 0x41), humano) != 0;
    if dele(partida) {
        return 0;
    }
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let lista_de: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_FIX_MAN_GET_FIXTURE_LIST_FOR_DATE);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut grupo = 0u8;
    while (grupo as i8) < read_u8(G_KEY_NATION_ACTIVE_COUNT) as i8 {
        let mut d = [0u8; 8];
        let data = data_nova(d.as_mut_ptr() as usize, read_i16(partida + 0x2a) as u16 as u32, read_i16(partida + 0x28) as u16 as u32);
        // ponytail: a original empurra o grupo como palavra inteira (os bytes de cima são sobra de pilha); a 0x5ae6a0
        // repassa o tipo (char) e a 0x5abf10 lê só o byte.
        let lista = lista_de(data, read_u8(partida + 0x3f) as u32, grupo as u32);
        if lista != 0 {
            write_u32(lista + 0xa, read_usize(lista + 2) as u32);
            let mut p = proximo(lista);
            while p != 0 {
                if dele(p) {
                    return 0;
                }
                p = proximo(lista);
            }
        }
        grupo = grupo.wrapping_add(1);
    }
    1
}

/// Comparador: a original e a cópia (as duas só leem; a volta da lista recomeça o cursor), pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_pode_ver_a_partida(this: usize, partida: usize, humano: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1097)(this, partida, humano);
    if partida == 0 {
        return r;
    }
    let n = pode_ver_a_partida(this, partida, humano);
    PLACAR.registrar(r == n, || format!("pode_ver_a_partida(partida {:x}, humano {:x}) original={} rust={}", partida, humano, r, n));
    r
}

/// in each human manager, finds the person nickname (30 bytes), pulls the next ones up, blanks the last one and lowers
/// the count; sorts the last manager array
/// Original: `HUMAN_MANAGER::remove_nickname_from_all` (0x5fff30, human_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x5fff30 } (thiscall, ret 4, human_manager.cpp): `HUMAN_MANAGER::remove_nickname_from_all(id)`: em cada técnico
/// humano ([this], [this+4] registros de 0xa6ef bytes; contador em byte com sinal) não nulo, sem vetor de apelidos
/// (+0xa6e3) sai de vez; senão busca (bsearch com 0x601a10, que lê só o id em +0; a chave tem o byte +4 zerado e o resto
/// sem valor, a cópia usa zero) o apelido da pessoa `*id` entre os [+0xa6eb] de 30 bytes; achou: puxa os seguintes um
/// lugar, o último fica com id −1 e +4 = 0 e a contagem desce. No fim reordena (qsort) só o vetor do último técnico
/// visto (sem técnicos, o `id` faz as vezes do técnico, como no original).
pub(crate) unsafe extern "thiscall" fn tirar_apelido_de_todos(this: usize, id: usize) {
    if id == 0 {
        return;
    }
    let mut ultimo = id;
    if read_u32(this + 4) as i32 > 0 {
        let mut c = 0i8;
        loop {
            let g = (read_u32(this) as usize).wrapping_add((c as i32 as usize).wrapping_mul(0xa6ef));
            ultimo = g;
            if g != 0 {
                if read_u32(g + 0xa6e3) == 0 {
                    return;
                }
                let mut chave = [0u8; 0x1e];
                chave[..4].copy_from_slice(&read_u32(id).to_le_bytes());
                let vetor = read_u32(g + 0xa6e3) as usize;
                let s = cph4_bsearch(chave.as_ptr() as usize, vetor, read_u32(g + 0xa6eb), 0x1e, 0x601a10);
                if s != 0 {
                    let mut k = ((s.wrapping_sub(vetor) as u32 as u64 * 0x88888889) >> 36) as i32;
                    while k < read_u32(g + 0xa6eb) as i32 - 1 {
                        let r = (read_u32(g + 0xa6e3) as usize).wrapping_add(k as usize * 30);
                        core::ptr::copy((r + 30) as *const u8, r as *mut u8, 30); // o seguinte um lugar acima
                        k += 1;
                    }
                    let n = read_u32(g + 0xa6eb) as usize;
                    let fim = (read_u32(g + 0xa6e3) as usize).wrapping_add(n.wrapping_mul(30));
                    *((fim - 0x1a) as *mut u8) = 0;
                    core::ptr::write_unaligned((fim - 0x1e) as *mut u32, u32::MAX);
                    core::ptr::write_unaligned((g + 0xa6eb) as *mut u32, (n as u32).wrapping_sub(1));
                }
            }
            c = c.wrapping_add(1);
            if c as i32 >= read_u32(this + 4) as i32 {
                break;
            }
        }
    }
    cph4_qsort(read_u32(ultimo + 0xa6e3) as usize, read_u32(ultimo + 0xa6eb) as i32, 0x1e, 0x601a10);
}

/// Comparador: lado a lado pelos vetores de apelidos e contagens de todos os técnicos humanos; sem técnicos (o qsort
/// leria pelo `id`), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_tirar_apelido_de_todos(this: usize, id: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(837);
    let n = read_u32(this + 4) as i32;
    if id == 0 || n <= 0 {
        return o(this, id);
    }
    let mut reg = Vec::new();
    for c in 0..n.min(128) {
        let g = (read_u32(this) as usize).wrapping_add(c as usize * 0xa6ef);
        if g != 0 && read_u32(g + 0xa6e3) != 0 {
            reg.push((g + 0xa6e3, 0xc));
            reg.push((read_u32(g + 0xa6e3) as usize, (read_u32(g + 0xa6eb) as i32).max(0) as usize * 30));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, id), || tirar_apelido_de_todos(this, id));
    PLACAR.registrar(igual, || format!("tirar_apelido_de_todos({:x}) {:x?} x {:x?}", id, mo, mr));
}

/// does the current human manager run the club (or the nation's main team)?
/// Original: `human_runs_club` (0x600c90, human_manager.cpp, cdecl).
///
/// [pt] { 0x600c90 } (cdecl, human_manager.cpp): o técnico humano da vez manda no `clube` (ou, nas
/// segundas seleções, na seleção principal da nação, 0x53b3d0)? Precisa ser o técnico do clube
/// (+0xcf); sem `qualquer`, não pode ter as marcas +0x9ace (0x5fe040) nem +0x9a01 no registro de
/// humanos [0xb63c98]; e então basta `forcar`, o clube ser seleção (0x53b900) ou a marca +0x9a05
/// (0x5fd480). Clube nulo: a original (caixa de erro e 0).
pub(crate) unsafe extern "C" fn humano_manda_no_clube(clube: usize, qualquer: u32, forcar: i8) -> u32 {
    let tecnico_9ace: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_COMPUTER_CONTROLLED);
    let tecnico_9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let e_segunda_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let selecao_principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let mut c = clube;
    loop {
        if c == 0 {
            return orig::<unsafe extern "C" fn(usize, u32, i8) -> u32>(371)(0, qualquer, forcar);
        }
        let vez = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        let humano = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const usize);
        let tecnico = core::ptr::read_unaligned((c + 0xcf) as *const usize);
        'proximo: {
            if tecnico == 0 || humano != tecnico {
                break 'proximo;
            }
            let humanos = *(G_HUMAN_MANAGER as *const usize);
            if qualquer == 0 {
                if tecnico_9ace(humanos, humano) != 0 {
                    break 'proximo;
                }
                let i = (*(humano as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
                if i >= 0
                    && i <= (*((humanos + 4) as *const i32)).wrapping_sub(1)
                    && core::ptr::read_unaligned((*(humanos as *const usize) + i as usize * 0xa6ef + 0x9a01) as *const u32) != 0
                {
                    break 'proximo;
                }
            }
            if forcar != 0 || e_selecao(c) != 0 || tecnico_9a05(humanos, humano) != 0 {
                crate::comparacao::ramo("humano manda no clube");
                return 1;
            }
        }
        if e_segunda_selecao(c) == 0 {
            return 0;
        }
        c = selecao_principal(core::ptr::read_unaligned((c + 0x53) as *const usize));
    }
}

pub(crate) unsafe extern "C" fn comparar_humano_manda_no_clube(clube: usize, qualquer: u32, forcar: i8) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, u32, i8) -> u32>(371)(clube, qualquer, forcar);
    let n = humano_manda_no_clube(clube, qualquer, forcar);
    PLACAR.registrar(r == n, || format!("humano_manda_no_clube({:#x}, {}, {}) original={} rust={}", clube, qualquer, forcar, r, n));
    r
}

/// stores +0x9a65 of the human manager record (0xa6ef bytes)
/// Original: `set_human_field` (0x5fdd30, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fdd30 } (thiscall, ret 8, human_manager.cpp): grava `valor` em +0x9a65 do registro do técnico
/// humano `tecnico` (sem ele, o da vez em [0xb74c26 + 0x300 × [0xb77c7a]]) entre os humanos `this`
/// ([this], 0xa6ef bytes cada; o índice é o id − [0xae2364] + 16, até [this+4] − 1). Devolve 1.
/// Sem técnico ou índice fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn gravar_dado_do_humano(t: usize, valor: u32, tecnico: *const i32) -> u32 {
    let mut p = tecnico;
    if p.is_null() {
        let vez = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    if p.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(369)(t, valor, tecnico);
    }
    let i = (*p).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((t + 4) as *const i32)).wrapping_sub(1) {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(369)(t, valor, tecnico);
    }
    crate::comparacao::ramo("dado do humano gravado");
    core::ptr::write_unaligned((*(t as *const usize) + i as usize * 0xa6ef + 0x9a65) as *mut u32, valor);
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_gravar_dado_do_humano(t: usize, valor: u32, tecnico: *const i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(369);
    let mut p = tecnico;
    if p.is_null() {
        let vez = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    let reg = if p.is_null() {
        vec![]
    } else {
        let i = (*p).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
        if i < 0 || i > (*((t + 4) as *const i32)).wrapping_sub(1) { vec![] } else { vec![(*(t as *const usize) + i as usize * 0xa6ef + 0x9a65, 4)] }
    };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, valor, tecnico), || gravar_dado_do_humano(t, valor, tecnico));
    PLACAR.registrar(igual && r == v, || format!("gravar_dado_do_humano({:#x}, {:#x}, {:p}) original={} {:x?} rust={} {:x?}", t, valor, tecnico, r, mo, v, mr));
    r
}

/// turns the human manager's holiday on or off, keeping the count of humans on holiday; when turning on, answers
/// pending news
/// Original: `set_on_holiday` (0x5fd2f0, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fd2f0 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::set_on_holiday(férias, técnico)`: sem técnico, o
/// humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); nenhum: a caixa da linha 0x41c. O índice do técnico (id − [0xae2364]
/// + 16) fora de 0..[this+4]: a caixa da linha 0x428. A marca de férias (+0x9a01 do registro de 0xa6ef bytes em [this])
/// passa a `férias`, com a conta dos humanos de férias (+0xc) subindo ao ligar e descendo ao desligar; ao ligar, as
/// notícias pendentes do técnico com o clube (+0x39) e com a seleção (+0x24) são respondidas (0x7874b0); com o dia em
/// [0xb67a50], o interesse do dia (0x6b5040).
pub(crate) unsafe extern "thiscall" fn ferias_do_humano(this: usize, ferias: u32, tecnico: usize) {
    let mut t = tecnico;
    if t == 0 {
        t = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * (*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize));
        if t == 0 {
            aviso_em(0x9cf11c, 0x41c);
            return;
        }
    }
    let i = (read_usize(t) as i32).wrapping_sub(read_usize(G_STAFF_COUNT) as i32).wrapping_add(0x10);
    if i < 0 || i > (read_usize(this + 4) as i32).wrapping_sub(1) {
        aviso_em(0x9cf11c, 0x428);
        return;
    }
    let marca = read_usize(this).wrapping_add((i as usize).wrapping_mul(0xa6ef)).wrapping_add(0x9a01);
    let conta = (this + 0xc) as *mut i32;
    let antes = read_usize(marca);
    if ferias != 0 {
        if antes == 0 {
            *conta = (*conta).wrapping_add(1);
        }
    } else if antes != 0 {
        *conta = (*conta).wrapping_sub(1);
    }
    core::ptr::write_unaligned(marca as *mut u32, ferias);
    crate::comparacao::ramo("cpy_13 ferias do humano");
    if ferias != 0 {
        let responder: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_AUTO_RESPOND_ALL);
        if read_usize(t + 0x39) != 0 {
            responder(read_usize(G_NEWS), t, read_usize(t + 0x39));
        }
        if read_usize(t + 0x24) != 0 {
            responder(read_usize(G_NEWS), t, read_usize(t + 0x24));
        }
    }
    let dia = read_usize(G_MATCH_DAY);
    if dia != 0 {
        let interesse: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_MATCH_DAY_REFRESH_HUMAN_INTEREST_FLAGS);
        interesse(dia);
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_ferias_do_humano(this: usize, ferias: u32, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize)>(1378);
    let t = if tecnico != 0 { tecnico } else { read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * (*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize)) };
    let i = if t == 0 { -1 } else { (read_usize(t) as i32).wrapping_sub(read_usize(G_STAFF_COUNT) as i32).wrapping_add(0x10) };
    if i < 0 || i > (read_usize(this + 4) as i32).wrapping_sub(1) {
        return o(this, ferias, tecnico); // a caixa: só a original
    }
    let marca = read_usize(this).wrapping_add((i as usize).wrapping_mul(0xa6ef)).wrapping_add(0x9a01);
    let (antes, conta) = (read_usize(marca), read_usize(this + 0xc) as i32);
    let esperada = conta + if ferias != 0 && antes == 0 { 1 } else if ferias == 0 && antes != 0 { -1 } else { 0 };
    o(this, ferias, tecnico);
    let (depois, conta2) = (read_usize(marca) as u32, read_usize(this + 0xc) as i32);
    PLACAR.registrar(depois == ferias && conta2 == esperada, || format!("ferias_do_humano({}, técnico {:x}): marca {:x} -> {:x}, conta {} -> {} (esperava {})", ferias, t, antes, depois, conta, conta2, esperada));
}
