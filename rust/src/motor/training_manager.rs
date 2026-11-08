use super::*;




/// Staff id -> person record (0x6e bytes in G_STAFF_TABLE).
///
/// [pt] O registro da pessoa `id` (tabela [0xae23bc], 0x6e bytes).
pub(crate) unsafe fn training_manager_staff_by_id(id: i32) -> usize {
    read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as isize * 0x6e) as usize)
}

/// Resets a 9-byte coaching slot to empty (id -1, ratings 0).
///
/// [pt] A vaga de 9 bytes da comissão (+0 id do treinador, +4..+8 as notas das cinco categorias) volta a vazia: −1, 0, 0.
pub(crate) unsafe fn clear_coach_slot(e: usize) {
    core::ptr::write_unaligned(e as *mut i32, -1);
    core::ptr::write_unaligned((e + 4) as *mut u32, 0);
    *((e + 8) as *mut u8) = 0;
}

/// Os divisores da nota pela quantidade de categorias do treinador (1 a 5), floats montados na pilha pela original:
/// 1,0; 1,2; 1,4; 1,6; 2,0.
pub(crate) const RATING_DIVISORS: [u32; 5] = [0x3f80_0000, 0x3f99_999a, 0x3fb3_3333, 0x3fcc_cccd, 0x4000_0000];

/// Non-player attribute rating (0x544410) for the attribute byte at f + k.
///
/// [pt] A chance de conhecer (0x544410, cdecl: a ficha de não jogador e o byte do atributo; a original empurra o
/// registrador com lixo em cima: a chamada lê o byte).
pub(crate) unsafe fn non_player_attribute_rating(f: usize, k: usize) -> i32 {
    let c: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    c(f, read_i8(f + k)) as i8 as i32 // a original estende o al (movsx)
}

/// the six training staff slots of the human manager: drops who left the club, adds the new assistant and coaches with
/// all five categories and recomputes each category rating (the chance from the coaching attributes, divided by the
/// coach category count, 1 to 20)
/// Original: `TRAINING_MANAGER::update_coach_assignments` (0x8b2880, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b2880 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::update_coach_assignments(clube, técnico)`,
/// pelas contratações e saídas de treinadores (0x4ce4e0, 0x4cf720, 0x8c9720, a regeneração 0x7aaeb0) e pela troca de
/// clube (0x8b2470). Sem técnico e o clube sem humano (0x600a50 com 1, 1, 0, 0) ou clube de seleção (0x53b900): nada.
/// Técnico nulo: o do clube (+0xcf). A comissão do humano (0x8b4cd0, seis vagas de 9 bytes): (1) a vaga cujo
/// treinador não tem ficha de não jogador (+0x69) ou não é o auxiliar (0x53cb80) nem um dos 5 treinadores (0x53cc30)
/// fica vazia; (2) o auxiliar e os treinadores com ficha que não estão nas vagas entram na primeira vazia com as cinco
/// categorias (0x01010101 e 1); (3) cada categoria marcada recebe a nota: 0 (goleiros, ficha +0x10) (chance − ficha
/// +0x12 + 21) / 2; 1 (tática, +0x21 e +0x10) (chance + chance + +0x12) / 3; 2 (+0x10) (chance + +0xe + +0x12) / 3;
/// 3 (+0x10) (chance + +0x12) / 2; 4 (+0x11) a chance (divisões truncadas para zero; a chance 0x544410), o byte com
/// sinal dividido pelo divisor da quantidade de categorias marcadas (1,0 a 2,0, float), truncado e posto em 1..20.
pub(crate) unsafe extern "thiscall" fn update_coach_assignments(this: usize, clube: usize, tecnico: usize) {
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if tecnico == 0 && humano(clube, 1, 1, 0, 0) == 0 {
        return;
    }
    if selecao(clube) != 0 {
        return;
    }
    let tec = if tecnico == 0 { read_u32_as_usize(clube + club::MANAGER) } else { tecnico };
    let comissao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_COACH_ASSIGNMENTS);
    let auxiliar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_ASSISTANT);
    let treinadores: unsafe extern "C" fn(usize, *mut usize) -> u32 = core::mem::transmute(FN_DATABASE_GET_CLUB_COACHES);
    let t = comissao(this, tec);
    let aux = auxiliar(clube);
    let mut tr = [0usize; 5];
    treinadores(clube, tr.as_mut_ptr());
    let candidato = |k: i32| if k < 0 { aux } else { tr[k as usize] };
    for v in 0..6 {
        let e = t + 9 * v;
        let id = read_i32(e);
        if id == -1 {
            continue;
        }
        if read_u32_as_usize(training_manager_staff_by_id(id) + 0x69) == 0 {
            clear_coach_slot(e);
            continue;
        }
        if !(-1..5).any(|k| {
            let p = candidato(k);
            p != 0 && read_i32(p) == id
        }) {
            clear_coach_slot(e);
        }
    }
    for k in -1..5 {
        let p = candidato(k);
        if p == 0 || read_u32_as_usize(p + 0x69) == 0 {
            continue;
        }
        if (0..6).any(|v| read_i32(t + 9 * v) == read_i32(p)) {
            continue;
        }
        if let Some(v) = (0..6).find(|&v| read_i32(t + 9 * v) == -1) {
            let e = t + 9 * v;
            core::ptr::write_unaligned(e as *mut i32, read_i32(p));
            core::ptr::write_unaligned((e + 4) as *mut u32, 0x0101_0101);
            *((e + 8) as *mut u8) = 1;
        }
    }
    for v in 0..6 {
        let e = t + 9 * v;
        let id = read_i32(e);
        if id == -1 {
            continue;
        }
        let f = read_u32_as_usize(training_manager_staff_by_id(id) + 0x69);
        if f == 0 {
            clear_coach_slot(e);
            continue;
        }
        let n = (0..5).filter(|&j| *((e + 4 + j) as *const u8) != 0).count();
        for j in 0..5 {
            if *((e + 4 + j) as *const u8) == 0 {
                continue;
            }
            let b = |k: usize| read_i8(f + k) as i32;
            let x = match j {
                0 => (non_player_attribute_rating(f, 0x10) - b(0x12) + 0x15) / 2, // cqf7: a nota de goleiros
                1 => {
                    let a = non_player_attribute_rating(f, 0x21);
                    (a + non_player_attribute_rating(f, 0x10) + b(0x12)) / 3 // cqf7: a nota de tática
                }
                2 => (non_player_attribute_rating(f, 0x10) + b(0xe) + b(0x12)) / 3,
                3 => (non_player_attribute_rating(f, 0x10) + b(0x12)) / 2,
                _ => non_player_attribute_rating(f, 0x11),
            };
            // o byte de baixo, com sinal, em float dividido pelo float da quantidade (precisão dupla do x87)
            let q = match_pl_ftol(x as i8 as f64 / f32::from_bits(RATING_DIVISORS[n - 1]) as f64) as i8; // cqf7: o divisor da quantidade
            crate::comparacao::ramo("cqf_7 nota do treinador");
            *((e + 4 + j) as *mut i8) = q.clamp(1, 20);
        }
    }
}

/// Comparer for `update_coach_assignments` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na comissão do humano (54 bytes) e na marca +0x244 do clube (a 0x53cb80/0x53cc30 pela
/// 0x540a50 podem zerá-la). Sem humano ou clube de seleção: lado a lado só na marca. Técnico fora dos 16 humanos
/// (a 0x8b4cd0 mostra a caixa e a original escreveria no nulo): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_update_coach_assignments(this: usize, clube: usize, tecnico: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_COACH_ASSIGNMENTS);
    if clube == 0 {
        return o(this, clube, tecnico);
    }
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let mut reg = vec![(clube + club::HAS_LINKED_CLUB, 1)];
    if (tecnico != 0 || humano(clube, 1, 1, 0, 0) != 0) && selecao(clube) == 0 {
        let tec = if tecnico == 0 { read_u32_as_usize(clube + club::MANAGER) } else { tecnico };
        if tec == 0 || !(0..0x10).contains(&read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(tec)).wrapping_sub(1)) {
            return o(this, clube, tecnico);
        }
        reg.push((this + 0x652d + 54 * read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(tec)).wrapping_sub(1) as usize, 54));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, tecnico), || update_coach_assignments(this, clube, tecnico));
    PLACAR.registrar(igual, || format!("update_coach_assignments(clube {:x}, tecnico {:x}) {:x?} x {:x?}", clube, tecnico, mo.0, mr.0));
}

/// Training schedule `vaga` (0..9, else 1) of human manager `tec`; outside the 16 humans shows the error box and
/// returns 0.
///
/// [pt] O programa `vaga` (0..9; fora disso o 1) do técnico humano `tec`: this + (vaga + 7 + 10 × ([0xae2364] − id − 1)) ×
/// 0x9b; fora dos 16 humanos, a caixa "Error" (linha 0x68f) e 0 (a original lê então o byte +0x15 do nulo).
pub(crate) unsafe fn human_training_schedule(this: usize, tec: usize, vaga: i8) -> usize {
    let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(tec)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    let v = if (0..10).contains(&vaga) { vaga as i32 } else { 1 };
    this.wrapping_add(((v + 7 + k * 10) * 0x9b) as usize)
}

/// 6-byte training entry of player data `f` ([this+0x651d] + 6 x id).
///
/// [pt] A entrada de 6 bytes do treino da ficha `f` (+0 a vaga do programa).
pub(crate) unsafe fn training_entry_of_player_data(this: usize, f: usize) -> usize {
    read_u32_as_usize(this + 0x651d).wrapping_add((read_i32(f) as isize * 6) as usize)
}

/// on the human manager club change: the old squad goes back to the default schedules (goalkeeper or general) with load
/// 50; the new one goes to the manager goalkeeper or general schedule, with the schedule load; and the new club
/// training staff
/// Original: `TRAINING_MANAGER::reassign_squad_schedules` (0x8b2470, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b2470 } (thiscall, ret 0xc, training_manager.cpp): `TRAINING_MANAGER::reassign_squad_schedules(técnico, antigo,
/// novo)`, quando o técnico humano troca de clube (0x690890 e 0x68f9c0/0x69ebc0, manager_manager.cpp). Elenco do clube
/// antigo (0x541f90, 50): cada pessoa válida (id abaixo de [0xae2364]) leva carga 50 (0x62c660) e a vaga 6 (goleiro de
/// 15 para cima, ficha +0xf) ou 1 na entrada do treino; sem ficha, a caixa (linha 0x34c). Com clube novo: dos dez
/// programas do técnico, o último de tipo 6 (+0x15) é o dos goleiros e o último de tipo 1 o geral; cada jogador do
/// elenco novo com ficha vai para o dos goleiros (goleiro de 15 para cima e havendo um), senão o geral (havendo), senão
/// a vaga 1, com a carga do programa (0x8b4f60) na ficha de forma; por fim a comissão (0x8b2880 com o clube novo e o
/// técnico).
pub(crate) unsafe extern "thiscall" fn reassign_squad_schedules(this: usize, tec: usize, antigo: usize, novo: usize) {
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let gravar_carga: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_INJURY_SET_TRAINING_LOAD);
    let (mut gol, mut geral) = (-1i8, -1i8);
    if antigo != 0 {
        let mut v = [0usize; 50];
        elenco(antigo, v.as_mut_ptr(), 0);
        for &p in &v {
            if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            gravar_carga(read_u32_as_usize(G_INJURY), p, 0x32);
            let f = read_u32_as_usize(p + 0x61);
            if f == 0 {
                show_error_at(0xa8ccd0, 0x34c);
                continue;
            }
            *(training_entry_of_player_data(this, f) as *mut u8) = if read_i8(f + 0xf) >= 0xf { 6 } else { 1 };
        }
    }
    if novo == 0 {
        return;
    }
    for vaga in 0..10i8 {
        if *((human_training_schedule(this, tec, vaga) + 0x15) as *const u8) == 6 {
            gol = vaga;
        } else if *((human_training_schedule(this, tec, vaga) + 0x15) as *const u8) == 1 {
            geral = vaga;
        }
    }
    let mut v = [0usize; 50];
    elenco(novo, v.as_mut_ptr(), 0);
    let carga: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_TRAINING_SCHEDULE_GET_TRAINING_LOAD);
    for &p in &v {
        if p == 0 {
            continue;
        }
        let f = read_u32_as_usize(p + 0x61);
        if f == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let e = training_entry_of_player_data(this, f) as *mut i8;
        *e = if read_i8(f + 0xf) >= 0xf && gol != -1 {
            gol // cqf7: o programa dos goleiros
        } else if geral != -1 {
            geral
        } else {
            1
        };
        crate::comparacao::ramo("cqf_7 programa do elenco novo");
        let c = carga(human_training_schedule(this, tec, *e));
        gravar_carga(read_u32_as_usize(G_INJURY), p, c as u32);
    }
    let comissao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRAINING_MANAGER_UPDATE_COACH_ASSIGNMENTS);
    comissao(this, novo, tec);
}

/// Comparer for `reassign_squad_schedules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na entrada do treino e na carga de forma de cada jogador dos dois elencos, na comissão do
/// técnico (a 0x8b2880 do fim) e na marca +0x244 dos clubes (a 0x541f90 pela 0x540a50). Com caixa (jogador sem
/// ficha no elenco antigo ou técnico fora dos 16 humanos com clube novo): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_reassign_squad_schedules(this: usize, tec: usize, antigo: usize, novo: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_REASSIGN_SQUAD_SCHEDULES);
    let k = if tec == 0 { -1 } else { read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(tec)).wrapping_sub(1) };
    if novo != 0 && !(0..0x10).contains(&k) {
        return o(this, tec, antigo, novo);
    }
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let les = read_u32_as_usize(read_u32_as_usize(G_INJURY));
    let mut reg = Vec::new();
    for (clube, antigo_) in [(antigo, true), (novo, false)] {
        if clube == 0 {
            continue;
        }
        reg.push((clube + club::HAS_LINKED_CLUB, 1));
        let mut v = [0usize; 50];
        elenco(clube, v.as_mut_ptr(), 0);
        for &p in &v {
            if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            let f = read_u32_as_usize(p + 0x61);
            if f == 0 {
                if antigo_ {
                    return o(this, tec, antigo, novo);
                }
                continue;
            }
            reg.push((training_entry_of_player_data(this, f), 6));
            reg.push((les.wrapping_add((read_i32(p) as isize * 31 + 0x1e) as usize), 1));
        }
    }
    if novo != 0 {
        reg.push((this + 0x652d + 54 * k as usize, 54));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tec, antigo, novo), || reassign_squad_schedules(this, tec, antigo, novo));
    PLACAR.registrar(igual, || format!("reassign_squad_schedules(tecnico {:x}, {:x} -> {:x}) {:x?} x {:x?}", tec, antigo, novo, mo.0, mr.0));
}

/// the default names of the 167 training schedules
/// Original: `(training_manager.cpp, 0x8b2230)` (0x8b2230, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b2230 } (thiscall, ret 0, training_manager.cpp): os nomes padrão dos 167 programas de treino (0x8b50f0 em cada
/// um, de 0x9b em 0x9b bytes), pela criação dos programas padrão (0x8b3930).
pub(crate) unsafe extern "thiscall" fn training_manager_cpp_0x8b2230(this: usize) {
    let nome: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRAINING_SCHEDULE_SET_DEFAULT_NAME);
    for k in 0..0xa7 {
        nome(this + k * 0x9b);
    }
}

/// Comparador: lado a lado nos 167 programas (0x9b bytes cada).
pub(crate) unsafe extern "thiscall" fn comparar_training_manager_cpp_0x8b2230(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TRAINING_MANAGER_CPP_0X8B2230);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(this, 0xa7 * 0x9b)], || o(this), || training_manager_cpp_0x8b2230(this));
    PLACAR.registrar(igual, || format!("training_manager_cpp_0x8b2230({:x})", this));
}






/// 6-byte training entry of a person ([this+0x651d] + 6 x id of player data +0x61).
///
/// [pt] A entrada de 6 bytes do treino do jogador: [this+0x651d] + 6 × id da ficha (+0x61).
pub(crate) unsafe fn training_entry_of_staff(this: usize, p: usize) -> usize {
    read_u32_as_usize(this + 0x651d).wrapping_add((read_i32(read_u32_as_usize(p + 0x61)) as isize * 6) as usize)
}

/// Person -> contract (get_contract), stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa (CONTRACT_MANAGER 0x4cf5a0, thiscall no gerente [0xae1050]), guardado em [0xae1080] como a
/// original faz a cada consulta.
pub(crate) unsafe fn training_manager_get_contract_stored(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_u32_as_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Contract -> club record (id +4), or 0 without contract.
///
/// [pt] O clube do contrato (+4 o id; registros de 0x245 bytes em [0xae23b4]), ou 0 sem contrato.
pub(crate) unsafe fn training_manager_club_of_contract(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        read_u32_as_usize(G_CLUB_TABLE).wrapping_add((read_i32(c + 4) as isize * 0x245) as usize)
    }
}

/// Manager's id is among the last 16 staff ids (the human managers).
///
/// [pt] O técnico humano é um dos 16 últimos ids de pessoa ([0xae2364] − 16 para cima).
pub(crate) unsafe fn is_human_manager_id(tec: usize) -> bool {
    read_i32(tec) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10)
}



/// brings the player training weeks up to date (up to 30): the human club schedule, the progress rolls of each category
/// by the schedule and coach weight, the position training (the rating, the end and the player complaint) and the
/// progress on the attributes
/// Original: `TRAINING_MANAGER::update_player_training` (0x8b1750, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b1750 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::update_player_training(pessoa, pular)`,
/// pela iniciação do jogador na partida (0x6eaea0, match_man.cpp, com 0: todo jogador de toda partida), pela lesão nova
/// (0x62d250) e pelas telas. Pessoa nula (linha 0x18a), sem a ficha (0x195) ou sem programa (0x8b39f0 nulo, 0x1a3): a
/// caixa "Error"; id a partir de [0xae2364]: nada. Jogador de clube de técnico humano sem a marca 0x20 em +5 da entrada
/// do treino: a marca, a vaga do programa (o de goleiro, tipo 6, para goleiro de 15 para cima; senão o geral, tipo 1;
/// senão 1) e a carga dele (0x8b4f60) na ficha de forma (0x62c660). Semanas = semana do treino (+0x688d) − a da entrada
/// (+1); nenhuma e sem `pular`: nada. Com a marca e o clube já sem técnico humano: tira a marca, carga 50, vaga 6 ou 1 e
/// os treinos de posição param (0x8b4450 com 0). Até 30 semanas: põe a entrada em dia; cada categoria (0..4) tem o
/// peso nível × 2 + 60 + (fator do treinador − 100) / 5 (0..200) e, por semana, sorteios do progresso: sobe 1 com peso
/// acima de 100, sorteio(210 − peso) < 20, valor abaixo de (+0x58) / 3 e sorteio(|valor| × 2 + 14) < 10; senão desce 1
/// com peso abaixo de 110, sorteio(peso + 60) < 20, valor acima de −7 e de (+0x5b) / 3 − 6 e o mesmo último sorteio
/// (goleiro de 19 para cima: só com sorteio(4), ou sorteio(2) na categoria 4, nulo). Os treinos de posição (0x8b4b20,
/// itens de 8 bytes: +4 a máscara, +6 a nota, +7 a taxa) andam por semana (ou uma vez, com `pular` na máscara): taxa
/// positiva, sorteio(taxa) < 5 dá nota + 2 (até 20) e o jogador pode reclamar (contrato do clube 0x4cf5f0 sem 0x8000
/// em +0x45: pela taxa acima de 50 ou pela aptidão 0x8b4770 abaixo de 12, com +0x5b abaixo de 15, sorteio(+0x5b) < 10
/// e sem lesão 0x62c400: a marca 0x8000 (0x867700), −5 em +0x44 (−100..100) e 0x5885c0); taxa negativa, sorteio(−taxa)
/// < 5 dá nota − 2 e, abaixo de 1, o item acaba (taxa 0, nota 1). Aplica o progresso (0x8b2be0) e, com itens acabados,
/// tira-os da lista (o último no lugar) e reordena (qsort 0x945333 com 0x8b4e00).
pub(crate) unsafe extern "thiscall" fn update_player_training(this: usize, p: usize, pular: u32) {
    if p == 0 {
        show_error_at(0xa8ccd0, 0x18a);
        return;
    }
    if read_i32(p) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    if read_u32_as_usize(p + 0x61) == 0 {
        show_error_at(0xa8ccd0, 0x195);
        return;
    }
    let mut d = [0i8; 8]; // o progresso das cinco categorias (a original zera um dword e um byte)
    let e = training_entry_of_staff(this, p);
    let programa: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_PLAYER_SCHEDULE);
    let prog = programa(this, p);
    if prog == 0 {
        show_error_at(0xa8ccd0, 0x1a3);
        return;
    }
    let gravar_carga: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_INJURY_SET_TRAINING_LOAD);
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(FN_TRAINING_MANAGER_FIND_POSITION_TRAINING);
    let marca = (e + 5) as *mut u8;
    if *marca & 0x20 == 0 {
        let c = training_manager_get_contract_stored(p);
        if c != 0 && training_manager_club_of_contract(c) != 0 && read_u32_as_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf) != 0
            && is_human_manager_id(read_u32_as_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf)) {
            let tec = read_u32_as_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf);
            *marca |= 0x20;
            let (mut goleiro, mut geral) = (-1i8, -1i8);
            for k in 0..10i8 {
                if *((human_training_schedule(this, tec, k) + 0x15) as *const u8) == 6 {
                    goleiro = k;
                } else if *((human_training_schedule(this, tec, k) + 0x15) as *const u8) == 1 {
                    geral = k;
                }
            }
            let vaga = if read_i8(read_u32_as_usize(p + 0x61) + 0xf) >= 0xf && goleiro != -1 {
                goleiro
            } else if geral != -1 {
                geral
            } else {
                1
            };
            *(e as *mut i8) = vaga;
            let carga: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_TRAINING_SCHEDULE_GET_TRAINING_LOAD);
            let v = carga(human_training_schedule(this, tec, vaga));
            crate::comparacao::ramo("cqd_8 programa do humano");
            gravar_carga(read_u32_as_usize(G_INJURY), p, v as u32);
        }
    }
    let mut semanas = read_u16(this + 0x688d).wrapping_sub(read_u16(e + 1));
    let pular = pular as u16;
    if semanas == 0 && pular == 0 {
        return;
    }
    if *marca & 0x20 != 0 {
        let c = training_manager_get_contract_stored(p);
        let ainda = c != 0 && training_manager_club_of_contract(c) != 0 && read_u32_as_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf) != 0
            && is_human_manager_id(read_u32_as_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf));
        if !ainda {
            crate::comparacao::ramo("cqd_8 saiu do clube humano");
            *marca &= 0xdf;
            gravar_carga(read_u32_as_usize(G_INJURY), p, 0x32);
            let f = read_u32_as_usize(p + 0x61);
            *(training_entry_of_staff(this, p) as *mut u8) = if read_i8(f + 0xf) >= 0xf { 6 } else { 1 };
            let mut q = 0usize;
            let n = achar(this, p, &mut q);
            let parar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRAINING_MANAGER_SET_POSITION_TRAINING_RATE);
            let mut k = 0i8;
            if n > 0 {
                loop {
                    parar(this, q.wrapping_add((k as isize * 8) as usize), 0);
                    k = k.wrapping_add(1);
                    if k as i32 >= n {
                        break;
                    }
                }
            }
        }
    }
    if semanas > 0x1e {
        semanas = 0x1e;
    }
    core::ptr::write_unaligned((e + 1) as *mut u16, read_u16(this + 0x688d));
    let mut q = 0usize;
    let npos = achar(this, p, &mut q);
    if *marca & 0x10 != 0 && npos == 0 {
        *marca &= 0xef;
    }
    let nivel: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_TRAINING_SCHEDULE_GET_CATEGORY_LEVEL);
    let fator: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRAINING_MANAGER_GET_COACHING_FACTOR);
    for k in 0..5usize {
        // ponytail: a original empurra a categoria num dword cujos bytes de cima sobram da pilha; a 0x8b4f50 e a
        // 0x8b41d0 leem o byte
        let lv = nivel(prog, k as u32) as u8 as i8;
        let base = (lv as i16).wrapping_mul(2).wrapping_add(0x3c); // cqd8: o peso do nível
        let clube = training_manager_club_of_contract(training_manager_get_contract_stored(p));
        let fa = fator(this, clube, k as u32) as u16 as i16 as i32;
        let x = base.wrapping_add(((fa - 100) / 5) as i16);
        let peso: i16 = if x < 0 { 0 } else if x > 200 { 200 } else { x };
        let piso = ((read_i8(p + 0x5b) as i32 / 3) as i8).wrapping_sub(6);
        let teto = (read_i8(p + 0x58) as i32 / 3) as i8;
        let meio = training_nibble(e, k);
        for _ in 0..semanas {
            let v = meio.wrapping_add(d[k]);
            if read_i8(read_u32_as_usize(p + 0x61) + 0xf) > 0x12 {
                let r = if k == 4 { friendly_random(2) } else { friendly_random(4) };
                if r != 0 {
                    continue; // goleiro: a categoria quase não anda
                }
            }
            if peso > 0x64 {
                if friendly_random(0xd2 - peso as i32) < 0x14 && v < teto { // cqd8: o sorteio da subida
                    if friendly_random((v as i32).abs() * 2 + 0xe) < 10 {
                        crate::comparacao::ramo("cqd_8 progresso sobe");
                        d[k] = d[k].wrapping_add(1); // cqd8: o progresso sobe
                    }
                    continue;
                }
            }
            if peso >= 0x6e {
                continue;
            }
            if friendly_random(peso as i32 + 0x3c) >= 0x14 || v <= -7 || v <= piso {
                continue;
            }
            if friendly_random((v as i32).abs() * 2 + 0xe) < 10 {
                crate::comparacao::ramo("cqd_8 progresso desce");
                d[k] = d[k].wrapping_sub(1); // cqd8: o progresso desce
            }
        }
    }
    let mut voltas = semanas;
    let mut contrato_do_clube = 0usize;
    if pular != 0 || voltas != 0 {
        if pular != 0 {
            voltas = 1;
        }
        let ficha: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        contrato_do_clube = ficha(read_u32_as_usize(G_CONTRACT_MANAGER), p, read_u32_as_usize(p + 0x39) as u32);
    }
    let mut acabou = false;
    if npos > 0 {
        let aptidao: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRAINING_MANAGER_GET_POSITION_APTITUDE);
        let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
        let reclamar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
        let infeliz: unsafe extern "C" fn(usize) = core::mem::transmute(FN_NEWS_POST_PLAYER_UNHAPPY_WITH_TRAINING);
        let mut k = 0i8;
        let mut it = q + 7; // a taxa do item (+7); a nota em +6, a máscara em +4
        loop {
            for _ in 0..voltas {
                let taxa = read_i8(it);
                if (pular != 0 && read_u16(it - 3) == pular) || taxa > 0 {
                    if friendly_random(read_i8(it) as i32) < 5 {
                        let n = read_i8(it - 1).wrapping_add(2);
                        *((it - 1) as *mut i8) = if n > 0x14 { 0x14 } else { n };
                    }
                    let c = contrato_do_clube;
                    if c != 0 && read_u32_as_usize(c + 0x45) & 0x8000 == 0 {
                        // ponytail: a original empurra a máscara no ecx com os 16 bits de cima que sobram; a 0x8b4770
                        // lê a palavra
                        let apt = aptidao(this, p, read_u16(it - 3) as u32) as u8 as i8;
                        let t = read_i8(it);
                        let quer = (t > 0x32 && friendly_random(100 - t as i32) < 10) || (apt < 0xc && friendly_random(apt as i32) < 2);
                        let a = read_i8(p + 0x5b);
                        if quer && a < 0xf && friendly_random(a as i32) < 10 && read_u32_as_usize(G_INJURY) != 0
                            && estado(read_u32_as_usize(G_INJURY), p) == 0 {
                            crate::comparacao::ramo("cqd_8 reclama do treino de posicao");
                            reclamar(c, 0x8000, 0);
                            let m = (c + 0x44) as *mut i8;
                            let v = *m as i32 - 5;
                            *m = if v > 100 { 100 } else if v < -100 { -100 } else { (*m).wrapping_sub(5) };
                            infeliz(p);
                            contrato_do_clube = 0;
                        }
                    }
                } else if taxa < 0 && friendly_random((taxa as i32).abs()) < 5 {
                    let n = read_i8(it - 1).wrapping_sub(2);
                    *((it - 1) as *mut i8) = n;
                    if n < 1 {
                        *(it as *mut i8) = 0;
                        *((it - 1) as *mut i8) = 1;
                        acabou = true;
                    }
                }
            }
            k = k.wrapping_add(1);
            it += 8;
            if k as i32 >= npos {
                break;
            }
        }
    }
    let aplicar: unsafe extern "thiscall" fn(usize, usize, *const i8) = core::mem::transmute(FN_TRAINING_MANAGER_APPLY_TRAINING_PROGRESS);
    aplicar(this, p, d.as_ptr());
    if acabou {
        crate::comparacao::ramo("cqd_8 treino de posicao acabou");
        let n0 = read_i32(this + 0x6525);
        let mut i = 0i32;
        if n0 > 0 {
            loop {
                let base = read_u32_as_usize(this + 0x6521);
                let n = read_i32(this + 0x6525);
                let item = base.wrapping_add((i as isize * 8) as usize);
                if *((item + 7) as *const u8) == 0 {
                    let ultimo = base.wrapping_add((n as isize * 8 - 8) as usize);
                    core::ptr::write_unaligned(item as *mut u32, read_u32_as_usize(ultimo) as u32);
                    core::ptr::write_unaligned((item + 4) as *mut u32, read_u32_as_usize(ultimo + 4) as u32);
                    core::ptr::write_unaligned((this + 0x6525) as *mut i32, read_i32(this + 0x6525) - 1);
                }
                i += 1;
                if i >= read_i32(this + 0x6525) {
                    break;
                }
            }
        }
        if n0 != read_i32(this + 0x6525) {
            let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
            qsort(read_u32_as_usize(this + 0x6521), read_u32_as_usize(this + 0x6525) as u32, 8, 0x8b4e00);
        }
    }
}

/// Comparador: lado a lado no estado do sorteador, em [0xae1080], na entrada do treino (6 bytes), na ficha (+0xf..+0x41,
/// o que a 0x8b2be0 grava) e na carga da ficha de forma (31 bytes por jogador em [[0xb63d28]], +0x1e), com os sorteios
/// da cópia fora da contagem. Só a original: os casos de erro e o jogador com treino de posição (0x8b4b20 antes de
/// rodar): ali a 0x8b4450, a marca do contrato (0x867700) e a 0x5885c0 mexem fora dessas regiões.
pub(crate) unsafe extern "thiscall" fn comparar_update_player_training(this: usize, p: usize, pular: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_UPDATE_PLAYER_TRAINING);
    if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || read_u32_as_usize(p + 0x61) == 0 {
        return o(this, p, pular);
    }
    let programa: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_PLAYER_SCHEDULE);
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(FN_TRAINING_MANAGER_FIND_POSITION_TRAINING);
    let mut q = 0usize;
    if programa(this, p) == 0 || achar(this, p, &mut q) > 0 {
        return o(this, p, pular);
    }
    let f = read_u32_as_usize(p + 0x61);
    let mut reg = Vec::from(crate::aleatorio::REGIOES_DO_ESTADO);
    reg.extend_from_slice(&[(G_CONTRACT_LOOKUP_RESULT, 4), (training_entry_of_staff(this, p), 6), (f + 0xf, 0x33)]);
    let les = read_u32_as_usize(G_INJURY);
    if les != 0 {
        reg.push((read_u32_as_usize(les).wrapping_add((read_i32(p) as isize * 31 + 0x1e) as usize), 1));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p, pular),
        || crate::aleatorio::sem_contar(|| update_player_training(this, p, pular)));
    PLACAR.registrar(igual, || format!("update_player_training(pessoa {:x}, pular {:x}) {:x?} x {:x?}", p, pular as u16, mo.0, mr.0));
}

/// the player training schedule: the club human manager one by the entry slot (the rest one when injured) or the
/// default schedule of the slot
/// Original: `TRAINING_MANAGER::get_player_schedule` (0x8b39f0, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b39f0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::get_player_schedule(pessoa)`, pelo diário
/// das pessoas (game_daily_staff_update 0x5ce2d0), pela 0x5d6aa0 e pela update_player_training (0x8b1750). Pessoa nula
/// ou sem a ficha: a caixa "Error" (linha 0x6ad) e 0. No clube de técnico humano (contrato 0x4cf5a0, guardado em
/// [0xae1080]; técnico +0xcf entre os 16 últimos ids): lesionado (0x62c400 = 2) o programa 1 do humano, this + 0x4d8 +
/// 0x60e × ([0xae2364] − id − 1); senão o da vaga da entrada do treino (+0, fora de 0..9 a 1); fora dos 16, a caixa
/// (linha 0x68f) e 0. Senão: lesionado, o programa 0 (this); senão this + 0x9b × vaga (com sinal).
pub(crate) unsafe extern "thiscall" fn get_player_schedule(this: usize, p: usize) -> usize {
    if p == 0 || read_u32_as_usize(p + 0x61) == 0 {
        show_error_at(0xa8ccd0, 0x6ad);
        return 0;
    }
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let c = training_manager_get_contract_stored(p);
    if c != 0 && training_manager_club_of_contract(c) != 0 {
        let tec = read_u32_as_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf);
        if tec != 0 && is_human_manager_id(tec) {
            if estado(read_u32_as_usize(G_INJURY), p) == 2 {
                let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(tec)).wrapping_sub(1);
                if !(0..0x10).contains(&k) {
                    show_error_at(0xa8ccd0, 0x68f);
                    return 0;
                }
                return this.wrapping_add(0x4d8 + 0x60e * k as usize);
            }
            return human_training_schedule(this, tec, *(training_entry_of_staff(this, p) as *const i8));
        }
    }
    if estado(read_u32_as_usize(G_INJURY), p) == 2 {
        return this;
    }
    this.wrapping_add((*(training_entry_of_staff(this, p) as *const i8) as isize * 0x9b) as usize)
}

/// Comparer for `get_player_schedule` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução e [0xae1080] lado a lado; sem pessoa ou sem ficha (a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_get_player_schedule(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_PLAYER_SCHEDULE);
    if p == 0 || read_u32_as_usize(p + 0x61) == 0 {
        return o(this, p);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, p), || get_player_schedule(this, p));
    PLACAR.registrar(igual && r == n, || format!("get_player_schedule(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo.0, mr.0));
    r
}

/// the level of one of the five training schedule categories
/// Original: `TRAINING_SCHEDULE::get_category_level` (0x8b4f50, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b4f50 } (thiscall, ret 4, training_manager.cpp): `TRAINING_SCHEDULE::get_category_level(categoria)`: o nível da
/// categoria (+4 + categoria, com sinal) do programa, no al; os bytes de cima do eax ficam com a categoria estendida
/// (movsx), como na original. Pela update_player_training (0x8b1750, as cinco categorias de cada atualização).
pub(crate) unsafe extern "thiscall" fn get_category_level(this: usize, categoria: u32) -> u32 {
    let k = categoria as u8 as i8 as i32;
    let v = *((this as isize + k as isize + 4) as *const u8);
    (k as u32 & 0xffff_ff00) | v as u32
}

/// Comparer for `get_category_level` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução inteira (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_get_category_level(this: usize, categoria: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_GET_CATEGORY_LEVEL)(this, categoria);
    let n = get_category_level(this, categoria);
    PLACAR.registrar(r == n, || format!("get_category_level({:x}, {}) original={:x} rust={:x}", this, categoria as u8 as i8, r, n));
    r
}



/// 6-byte training entry of a person ([this+0x651d] + 6 x id of player data +0x61).
///
/// [pt] A entrada de 6 bytes do treino do jogador: [this+0x651d] + 6 × id da ficha (+0x61).
pub(crate) unsafe fn player_training_entry(this: usize, p: usize) -> usize {
    read_u32_as_usize(this + 0x651d).wrapping_add((*(read_u32_as_usize(p + 0x61) as *const i32) as isize * 6) as usize)
}

/// Signed nibble of category k in a training entry (low/high of +3, +4, low of +5).
///
/// [pt] O meio byte com sinal da categoria `k` na entrada do treino: 0 o baixo de +3, 1 o alto de +3, 2 o baixo de +4, 3 o
/// alto de +4, 4 o baixo de +5.
pub(crate) unsafe fn training_nibble(e: usize, k: usize) -> i8 {
    let b = *((e + 3 + k / 2) as *const u8);
    if k % 2 == 0 {
        ((b << 4) as i8) >> 4
    } else {
        (b as i8) >> 4
    }
}

/// TRAINING_MANAGER apply_training_progress(person, deltas).
pub(crate) unsafe fn training_manager_apply_training_progress(this: usize, p: usize, d: &[i8; 8]) {
    let f: unsafe extern "thiscall" fn(usize, usize, *const i8) = core::mem::transmute(FN_TRAINING_MANAGER_APPLY_TRAINING_PROGRESS);
    f(this, p, d.as_ptr());
}

/// takes the progress stored in the training entry off the attributes (keeping the five nibbles) or puts it back,
/// around the player attribute normalisations
/// Original: `TRAINING_MANAGER::remove_or_restore_training_progress` (0x8b3640, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b3640 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::remove_or_restore_training_progress(pessoa,
/// devolver)`, pelas normalizações dos atributos do jogador (0x538bc0 e 0x538ec0: no envelhecimento do diário das
/// pessoas 0x5ce2d0, no regen jovem 0x7abac0 e na carga), que tiram o progresso antes e o devolvem depois. Pessoa nula
/// ou sem a ficha: a caixa "Error" (linha 0x621). Tirar (`devolver` = 0): guarda os cinco meios bytes do progresso da
/// entrada do treino em 0xde1f3c..0xde1f40 (categoria 3, 2, 4, 0, 1) e aplica o progresso negado (0x8b2be0); devolver:
/// aplica o guardado.
pub(crate) unsafe extern "thiscall" fn remove_or_restore_training_progress(this: usize, p: usize, devolver: u32) {
    if p == 0 || read_u32_as_usize(p + 0x61) == 0 {
        show_error_at(0xa8ccd0, 0x621);
        return;
    }
    let e = player_training_entry(this, p);
    let mut d = [0i8; 8];
    let g = |a: usize| *(a as *const i8);
    if devolver != 0 {
        crate::comparacao::ramo("cqd_9 devolver");
        d[0] = g(0xde1f3f); // cqd9: devolve a categoria 0
        d[1] = g(0xde1f40);
        d[2] = g(0xde1f3d);
        d[3] = g(0xde1f3c);
        d[4] = g(0xde1f3e); // cqd9: devolve a categoria 4
    } else {
        crate::comparacao::ramo("cqd_9 tirar");
        *(G_TRAINING_PROGRESS_SAVED_DELTA_0 as *mut i8) = training_nibble(e, 0);
        *(G_TRAINING_PROGRESS_SAVED_DELTA_1 as *mut i8) = training_nibble(e, 1); // cqd9: guarda a categoria 1
        *(G_TRAINING_PROGRESS_SAVED_DELTA_2 as *mut i8) = training_nibble(e, 2);
        *(G_TRAINING_PROGRESS_SAVED_DELTA_3 as *mut i8) = training_nibble(e, 3);
        *(G_TRAINING_PROGRESS_SAVED_DELTA_4 as *mut i8) = training_nibble(e, 4);
        for k in 0..5 {
            d[k] = training_nibble(e, k).wrapping_neg();
        }
    }
    training_manager_apply_training_progress(this, p, &d);
}

/// Comparador: lado a lado na ficha (+0xf..+0x41), na entrada do treino (6 bytes) e no guardado (0xde1f3c, 5 bytes);
/// sem pessoa ou sem ficha (a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_remove_or_restore_training_progress(this: usize, p: usize, devolver: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_REMOVE_OR_RESTORE_TRAINING_PROGRESS);
    if p == 0 || read_u32_as_usize(p + 0x61) == 0 {
        return o(this, p, devolver);
    }
    let f = read_u32_as_usize(p + 0x61);
    let reg = [(f + 0xf, 0x33), (player_training_entry(this, p), 6), (0xde1f3c, 5)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, p, devolver), || remove_or_restore_training_progress(this, p, devolver));
    PLACAR.registrar(igual, || format!("remove_or_restore_training_progress(pessoa {:x}, devolver {}) {:x?} x {:x?}", p, devolver, mo.0, mr.0));
}

/// how much the training progress adds to an attribute: the attribute minus the same attribute without the progress
/// Original: `TRAINING_MANAGER::get_training_attribute_change` (0x8b37c0, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b37c0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_training_attribute_change(pessoa,
/// atributo)`, pela tela do treino (0x8b8184): o atributo (0x5431e0) menos o mesmo atributo sem o progresso do treino
/// (aplica o progresso negado, lê e reaplica o progresso), no al. Pessoa nula ou sem a ficha: a caixa "Error" (linha
/// 0x657) e 0.
pub(crate) unsafe extern "thiscall" fn get_training_attribute_change(this: usize, p: usize, atributo: u32) -> u8 {
    if p == 0 || read_u32_as_usize(p + 0x61) == 0 {
        show_error_at(0xa8ccd0, 0x657);
        return 0;
    }
    let e = player_training_entry(this, p);
    let ler: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(FN_STAFF_GET_PLAYER_ATTRIBUTE);
    let antes = ler(p, atributo);
    let mut d = [0i8; 8];
    for k in 0..5 {
        d[k] = training_nibble(e, k).wrapping_neg();
    }
    training_manager_apply_training_progress(this, p, &d);
    let sem = ler(p, atributo);
    let mut v = [0i8; 8];
    for k in 0..5 {
        v[k] = d[k].wrapping_neg();
    }
    training_manager_apply_training_progress(this, p, &v);
    antes.wrapping_sub(sem)
}

/// Comparer for `get_training_attribute_change` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na ficha e na entrada do treino, com a devolução; sem pessoa ou sem ficha, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_get_training_attribute_change(this: usize, p: usize, atributo: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_GET_TRAINING_ATTRIBUTE_CHANGE);
    if p == 0 || read_u32_as_usize(p + 0x61) == 0 {
        return o(this, p, atributo);
    }
    let f = read_u32_as_usize(p + 0x61);
    let reg = [(f + 0xf, 0x33), (player_training_entry(this, p), 6)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, p, atributo), || get_training_attribute_change(this, p, atributo));
    PLACAR.registrar(igual && r == n, || format!("get_training_attribute_change(pessoa {:x}, atributo {}) original={} rust={} {:x?} x {:x?}", p, atributo as u8 as i8, r as i8, n as i8, mo.0, mr.0));
    r
}

/// Position masks accepted by 0x8b4770 (others show the error box).
///
/// [pt] As máscaras de posição que a 0x8b4770 aceita (as outras dão a caixa).
pub(crate) fn is_valid_position_mask(m: i32) -> bool {
    matches!(m, 1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x40 | 0x80 | 0x200 | 0x800)
}

/// the player aptitude to learn a position or side: the average of the adaptability and the attributes the position
/// needs
/// Original: `TRAINING_MANAGER::get_position_aptitude` (0x8b4770, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b4770 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_position_aptitude(pessoa, mascara)`
/// (this não usado), pela update_player_training (0x8b1750, a reclamação do treino de posição) e pela
/// set_position_training_rate (0x8b4450): a média da adaptabilidade (+0x56 da pessoa) com os atributos que a posição
/// pede (notas de 1 a 20 pela 0x544380 sobre a ficha +0x61): goleiro (1) +0x39, +0x2a e +0x35, por 4; defesa (2, 4)
/// +0x3e, +0x39 e +0x32, por 4; meio e frente (8, 0x10, 0x20) +0x37, +0x32, +0x40 e +0x3f, por 5; 0x40 +0x33, +0x27,
/// +0x40 e +0x36, por 5; 0x80 +0x30, por 2; 0x200 só a adaptabilidade (no al, os bytes de cima com a máscara); 0x800
/// +0x3b, por 2 (divisões truncadas). Outra máscara: a caixa "Error" (linha 0x945) e 0.
pub(crate) unsafe extern "thiscall" fn get_position_aptitude(this: usize, p: usize, mascara: u32) -> u32 {
    let _ = this;
    let m = mascara as u16 as i16 as i32;
    if !is_valid_position_mask(m) {
        show_error_at(0xa8ccd0, 0x945);
        return 0;
    }
    let f = read_u32_as_usize(p + 0x61);
    let nota_de: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_PLAYER_ATTRIBUTE_RATING);
    // ponytail: a original empurra o byte do atributo num registrador com os bytes de cima que sobram; a 0x544380 lê
    // o byte
    let nota = |o: usize| nota_de(f, *((f + o) as *const u8) as u32) as u8 as i8 as i32;
    let b = |o: usize| read_i8(f + o) as i32;
    let adapt = read_i8(p + 0x56) as i32;
    let r = match m {
        1 => {
            let x = nota(0x39);
            let y = nota(0x2a);
            (b(0x35) + x + y + adapt) / 4
        }
        2 | 4 => {
            let x = nota(0x3e);
            let y = nota(0x39);
            let z = nota(0x32);
            (x + y + z + adapt) / 4
        }
        8 | 0x10 | 0x20 => {
            let x = nota(0x37);
            let y = nota(0x32);
            (x + y + adapt + b(0x40) + b(0x3f)) / 5
        }
        0x40 => {
            let x = nota(0x33);
            let y = nota(0x27);
            (x + y + adapt + b(0x40) + b(0x36)) / 5
        }
        0x80 => (adapt + b(0x30)) / 2,
        0x200 => return (m as u32 & 0xffff_ff00) | *((p + 0x56) as *const u8) as u32,
        _ => (adapt + b(0x3b)) / 2,
    };
    r as u32
}

/// Comparer for `get_position_aptitude` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução inteira (só lê); máscara fora (a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_get_position_aptitude(this: usize, p: usize, mascara: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_GET_POSITION_APTITUDE);
    if !is_valid_position_mask(mascara as u16 as i16 as i32) {
        return o(this, p, mascara);
    }
    let r = o(this, p, mascara);
    let n = get_position_aptitude(this, p, mascara);
    PLACAR.registrar(r == n, || format!("get_position_aptitude(pessoa {:x}, mascara {:x}) original={:x} rust={:x}", p, mascara as u16, r, n));
    r
}


/// Writes the five levels (+4..+8) of a training schedule.
///
/// [pt] Os cinco níveis (+4..+8) de um programa de treino.
pub(crate) unsafe fn set_schedule_levels(this: usize, n: [u8; 5]) {
    for (k, v) in n.iter().enumerate() {
        *((this + 4 + k) as *mut u8) = *v;
    }
}

/// stores the schedule's slot and type and its five category levels from the type's mix (general, one strong category,
/// goalkeepers or no training)
/// Original: `TRAINING_SCHEDULE::set_type` (0x8b4e50, training_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8b4e50 } (thiscall, ret 8, training_manager.cpp): `TRAINING_SCHEDULE::set_type(vaga, tipo)`: a vaga (com sinal,
/// em dword) em +0, o tipo em +0x15 e os cinco níveis das categorias (+4..+8) pela mistura do tipo (tabela de saltos
/// 0x8b4f08): 1 geral (25 nas quatro primeiras), 2 a 5 uma categoria com 50 e as outras com 10, 6 goleiros (50 na quinta,
/// 0 na terceira), 0 e 7 sem treino (tudo 0); fora de 0..=7 (o −1 da primeira vaga do humano) só a vaga e o tipo. Pela
/// set_default_schedules (0x8b3d40, 167 vezes no começo do jogo) e pelas telas do treino.
pub(crate) unsafe extern "thiscall" fn set_type(this: usize, vaga: i8, tipo: i8) {
    *(this as *mut i32) = vaga as i32;
    *((this + 0x15) as *mut i8) = tipo;
    match tipo {
        1 => {
            crate::comparacao::ramo("cpu_8 geral");
            set_schedule_levels(this, [0x19, 0x19, 0x19, 0x19, 0]); // cpu8: o treino geral
        }
        2 => set_schedule_levels(this, [0x32, 0xa, 0xa, 0xa, 0]),
        3 => set_schedule_levels(this, [0xa, 0xa, 0x32, 0xa, 0]),
        4 => set_schedule_levels(this, [0xa, 0xa, 0xa, 0x32, 0]),
        5 => set_schedule_levels(this, [0xa, 0x32, 0xa, 0xa, 0]),
        6 => {
            crate::comparacao::ramo("cpu_8 goleiros");
            set_schedule_levels(this, [0xa, 0xa, 0, 0xa, 0x32]); // cpu8: o treino dos goleiros
        }
        0 | 7 => set_schedule_levels(this, [0; 5]),
        _ => {}
    }
}

/// Comparer for `set_type` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nos 0x16 bytes do programa (a vaga, os níveis e o tipo).
pub(crate) unsafe extern "thiscall" fn comparar_set_type(this: usize, vaga: i8, tipo: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8)>(IDX_SET_TYPE);
    let regioes = [(this, 0x16)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, vaga, tipo), || set_type(this, vaga, tipo));
    PLACAR.registrar(igual, || format!("set_type({:x}, {}, {}) {:x?} x {:x?}", this, vaga, tipo, mo, mr));
}

/// Training schedule slot of human manager `pessoa` as 0x8b3d40 computes it; outside the 16 humans shows the box and
/// returns 0.
///
/// [pt] O programa `vaga` (0..=9) do técnico humano `pessoa` como a 0x8b3d40 o calcula sem chamar a 0x8b3930 (vagas 0 a
/// 5): this + 0x43d + 0x9b × vaga + 0x60e × humano; fora dos 16 humanos, a caixa (linha 0x68f) e 0.
pub(crate) unsafe fn human_schedule_slot(this: usize, pessoa: usize, vaga: usize) -> usize {
    let v = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(pessoa)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    this + 0x43d + 0x9b * vaga + 0x60e * v as usize
}

/// sets up the computer's seven schedules (type k in slot k) and the ten schedules of each of the 16 human managers,
/// then gives the default names
/// Original: `TRAINING_MANAGER::set_default_schedules` (0x8b3d40, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b3d40 } (thiscall, ret 0, training_manager.cpp): `TRAINING_MANAGER::set_default_schedules()`, pelo construtor
/// (0x8b0ec0) num jogo novo (sem training.dat): os sete programas do computador (0x9b bytes cada a partir de this; o
/// tipo k na vaga k: o computador treina pela vaga 1, geral, e pela 6, goleiros, conforme a ficha de treino do
/// jogador) e, para cada um dos 16 técnicos humanos (pessoa [0xae2364] − h − 1, registro de 0x6e bytes em
/// [0xae23bc]), os dez programas: vagas 0 a 5 calculadas aqui (tipos −1, 0, 1, 2, 3, 4), 6 a 9 pela 0x8b3930 (tipos
/// 5, 6, 7, 7). No fim os nomes padrão (0x8b2230). Chamadas pelo endereço (0x8b4e50 e 0x8b3930 nesta leva).
pub(crate) unsafe extern "thiscall" fn set_default_schedules(this: usize) {
    let tipo: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(FN_TRAINING_SCHEDULE_SET_TYPE);
    let humano: unsafe extern "thiscall" fn(usize, usize, i8) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_MANAGER_SCHEDULE);
    tipo(this, 0, 0);
    crate::comparacao::ramo("cpu_8 programas padrao");
    tipo(this + 0x9b, 1, 1); // cpu8: o programa geral do computador
    for k in 2..7i8 {
        tipo(this + 0x9b * k as usize, k, k);
    }
    for h in 0..0x10i32 {
        let id = read_i32(G_STAFF_COUNT).wrapping_sub(h).wrapping_sub(1);
        let pessoa = (*(G_STAFF_TABLE as *const usize)).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        for (vaga, t) in [(0usize, -1i8), (1, 0), (2, 1), (3, 2), (4, 3), (5, 4)] {
            tipo(human_schedule_slot(this, pessoa, vaga), vaga as i8, t);
        }
        // ponytail: a original empurra a vaga 8 e 9 num dword só com o byte baixo gravado; as duas chamadas leem o byte.
        for (vaga, t) in [(6i8, 5i8), (7, 6), (8, 7), (9, 7)] {
            tipo(humano(this, pessoa, vaga), vaga, t);
        }
    }
    let nomes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRAINING_MANAGER_SET_DEFAULT_SCHEDULE_NAMES);
    nomes(this);
}

/// Comparer for `set_default_schedules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nos 167 programas (0x651d bytes a partir de this; nomes incluídos) e no marcador de erro.
pub(crate) unsafe extern "thiscall" fn comparar_set_default_schedules(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SET_DEFAULT_SCHEDULES);
    let regioes = [(this, 0x651d), (0xb67a34, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this), || set_default_schedules(this));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("set_default_schedules({:x}) diferencas em {:x?}", this, dif)
    });
}

/// a human manager's schedule for slot 0 to 9 (otherwise 1); outside the 16 human managers, the error box and 0
/// Original: `TRAINING_MANAGER::get_manager_schedule` (0x8b3930, training_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8b3930 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_manager_schedule(tecnico, vaga)`: o
/// programa da vaga (0..=9; fora disso, 1) do técnico humano: this + 0x9b × (vaga + 10 × humano + 7), humano =
/// [0xae2364] − id − 1. Fora dos 16 humanos: a caixa (linha 0x68f) e 0. Pela 0x8b3d40 (vagas 6 a 9) e pelas telas.
pub(crate) unsafe extern "thiscall" fn get_manager_schedule(this: usize, tecnico: usize, vaga: i8) -> usize {
    let v = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(tecnico)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    let vaga = if (0..0xa).contains(&vaga) { vaga } else { 1 };
    this.wrapping_add(0x9b * (vaga as i32 + 10 * v + 7) as usize)
}

/// Comparer for `get_manager_schedule` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução, sem os casos de erro (só a original).
pub(crate) unsafe extern "thiscall" fn comparar_get_manager_schedule(this: usize, tecnico: usize, vaga: i8) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> usize>(IDX_GET_MANAGER_SCHEDULE);
    if !(0..0x10).contains(&read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(tecnico)).wrapping_sub(1)) {
        return o(this, tecnico, vaga);
    }
    let r = o(this, tecnico, vaga);
    let n = get_manager_schedule(this, tecnico, vaga);
    PLACAR.registrar(r == n, || format!("get_manager_schedule(tecnico {:x}, vaga {}) original={:x} rust={:x}", tecnico, vaga, r, n));
    r
}



/// Divisor of the coaches' average by number of coaches (1..6): 1, 1.45, 1.7, 1.9, 2.05, 2.2.
///
/// [pt] Os seis divisores da média dos treinadores (floats imediatos da 0x8b41d0: 1; 1,45; 1,7; 1,9; 2,05; 2,2), pela
/// quantidade de treinadores (1 a 6).
pub(crate) fn coach_average_divisor(n: i8) -> f64 {
    const T: [u32; 6] = [0x3f800000, 0x3fb9999a, 0x3fd9999a, 0x3ff33333, 0x40033333, 0x400ccccd];
    f32::from_bits(T[(n - 1) as usize]) as f64
}

/// the coaching factor of a training category: 100 without a manager or with a computer manager; at a human club, the
/// average rating of the coach assignments (divided by the divisor for their number) × 40/7, at least 50
/// Original: `TRAINING_MANAGER::get_coaching_factor` (0x8b41d0, training_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8b41d0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_coaching_factor(clube, categoria)`,
/// pela TRAINING_MANAGER::update_player_training (0x8b1750, cinco vezes por jogador em cada atualização do treino; o
/// valor entra como (fator − 100) / 5 no peso do sorteio do progresso). Clube nulo: 0. Sem técnico (+0xcf) ou com técnico
/// do computador (id abaixo de [0xae2364] − 16): 100 (o eax leva os 16 bits altos do ponteiro do técnico, como na
/// original). Técnico humano: a comissão dele (0x8b4cd0, seis vagas de 9 bytes: id −1 é vaga vazia, a nota da categoria
/// em +4 + categoria); com n treinadores de nota, a soma (16 bits) dividida pelo n-ésimo divisor e truncada (_ftol
/// 0x945640), × 40/7 (0x970400) e truncada; abaixo de 50, 50. Quente no ramo do computador: sem contador ali.
pub(crate) unsafe extern "thiscall" fn get_coaching_factor(this: usize, clube: usize, categoria: i8) -> u32 {
    let _ = this;
    if clube == 0 {
        return 0;
    }
    let t = read_u32_as_usize(clube + club::MANAGER);
    if t == 0 {
        crate::comparacao::ramo("cpt_8 sem tecnico");
        return 100; // cpt8: sem técnico
    }
    if read_i32(t) < read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        return (t as u32 & 0xffff_0000) | 100; // cpt8: técnico do computador
    }
    let comissao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_COACH_ASSIGNMENTS);
    let s = comissao(this, t);
    let (mut n, mut soma) = (0i8, 0i16);
    for k in 0..6usize {
        let e = s + 9 * k;
        if read_i32(e) == -1 {
            continue;
        }
        let nota = *((e as isize + 4 + categoria as isize) as *const i8);
        if nota != 0 {
            n += 1;
            soma = soma.wrapping_add(nota as i16);
        }
    }
    if n != 0 {
        crate::comparacao::ramo("cpt_8 treinadores");
        soma = match_pl_ftol(soma as f64 / coach_average_divisor(n)) as i16; // cpt8: a média dos treinadores
    }
    let r = match_pl_ftol(soma as f64 * read_f64_at(0x970400)); // 0x970400 = 5.714285714
    if (r as i16) < 0x32 {
        return 0x32;
    }
    r as u32
}

/// Comparer for `get_coaching_factor` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução inteira (a consulta só lê; a comissão do humano é lida pelas duas).
pub(crate) unsafe extern "thiscall" fn comparar_get_coaching_factor(this: usize, clube: usize, categoria: i8) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> u32>(IDX_GET_COACHING_FACTOR)(this, clube, categoria);
    let n = get_coaching_factor(this, clube, categoria);
    PLACAR.registrar(r == n, || format!("get_coaching_factor(clube {:x}, categoria {}) original={:x} rust={:x}", clube, categoria, r, n));
    r
}

/// the coach assignment table (six 9-byte slots) of one of the 16 human managers; otherwise the error box and 0
/// Original: `TRAINING_MANAGER::get_coach_assignments` (0x8b4cd0, training_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8b4cd0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::get_coach_assignments(tecnico)`: a comissão
/// de treino (seis vagas de 9 bytes) de um dos 16 técnicos humanos, em this + 0x652d + 54 × ([0xae2364] − id − 1).
/// Técnico nulo: a caixa "Error" (linha 0xa20) e 0; fora dos 16 humanos: a caixa (linha 0xa2a) e 0.
pub(crate) unsafe extern "thiscall" fn get_coach_assignments(this: usize, tecnico: usize) -> usize {
    if tecnico == 0 {
        show_error_at(0xa8ccd0, 0xa20);
        return 0;
    }
    let v = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(tecnico)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0xa2a);
        return 0;
    }
    this + 0x652d + 54 * v as usize
}

/// Comparer for `get_coach_assignments` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução, sem os casos de erro (só a original).
pub(crate) unsafe extern "thiscall" fn comparar_get_coach_assignments(this: usize, tecnico: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_COACH_ASSIGNMENTS);
    if tecnico == 0 || !(0..0x10).contains(&read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(tecnico)).wrapping_sub(1)) {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = get_coach_assignments(this, tecnico);
    PLACAR.registrar(r == n, || format!("get_coach_assignments(tecnico {:x}) original={:x} rust={:x}", tecnico, r, n));
    r
}

/// the training schedule load from its five category levels with fixed weights, up to 100
/// Original: `TRAINING_SCHEDULE::get_training_load` (0x8b4f60, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b4f60 } (thiscall, ret 0, training_manager.cpp): `TRAINING_SCHEDULE::get_training_load()`: a carga do
/// programa pelos cinco níveis (+4..+8, com sinal): trunc(+8 × 0,5) − trunc(+4 × −1,3) − trunc(+7 × −0,5) −
/// trunc(+6 × −0,5) − trunc(+5 × −0,5) (constantes 0x968968, 0x96f878, 0x969540; _ftol 0x945640), em 16 bits; de 100
/// para cima, 100. Pela update_player_training (0x8b1750, o jogador do clube humano), pela set_player_schedule
/// (0x8b2250) e pela reassign_squad_schedules (0x8b2470); vai para a ficha de forma (Injury::set_training_load 0x62c660).
pub(crate) unsafe extern "thiscall" fn get_training_load(this: usize) -> i32 {
    let nivel = |k: usize| *((this + k) as *const i8) as f64;
    let a = match_pl_ftol(nivel(4) * read_f64_at(0x96f878)) as i16; // 0x96f878 = -1.3
    let mut s = match_pl_ftol(nivel(8) * read_f64_at(0x968968)) as i16; // 0x968968 = 0.5
    s = s.wrapping_sub(a);
    s = s.wrapping_sub(match_pl_ftol(nivel(7) * read_f64_at(0x969540)) as i16); // 0x969540 = -0.5
    s = s.wrapping_sub(match_pl_ftol(nivel(6) * read_f64_at(0x969540)) as i16); // 0x969540 = -0.5
    s = s.wrapping_sub(match_pl_ftol(nivel(5) * read_f64_at(0x969540)) as i16); // 0x969540 = -0.5
    if s >= 0x64 {
        return 0x64;
    }
    s as i32
}

/// Comparer for `get_training_load` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_get_training_load(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GET_TRAINING_LOAD)(this);
    let n = get_training_load(this);
    PLACAR.registrar(r == n, || format!("get_training_load({:x}) original={} rust={}", this, r, n));
    r
}




/// Adds v to the 1..20 attribute byte at a, clamped to 1..20.
///
/// [pt] O atributo de 1 a 20 em `a` mais `v`, entre 1 e 20.
pub(crate) unsafe fn add_clamped_1_20(a: usize, v: i32) {
    *(a as *mut i8) = (read_i8(a) as i32 + v).clamp(1, 20) as i8;
}

/// Adds v to the -125..125 attribute byte at a, clamped.
///
/// [pt] O atributo de −125 a 125 em `a` mais `v`, entre −125 e 125.
pub(crate) unsafe fn add_clamped_125(a: usize, v: i32) {
    *(a as *mut i8) = (read_i8(a) as i32 + v).clamp(-125, 125) as i8;
}

/// How much the attribute moves for progress d over the stored nibble n: (d + n)/2 - n/2 in 8 bits.
///
/// [pt] Quanto o atributo anda com o progresso `d` sobre o meio byte guardado `n` (com sinal): (d + n) / 2 − n / 2, as
/// divisões truncadas para o zero e a conta em 8 bits.
pub(crate) fn progress_step(d: i8, n: i32) -> i32 {
    (((d as i32 + n) / 2) as i8).wrapping_sub((n / 2) as i8) as i32
}

/// Adds d to the high nibble of the byte at a (low kept).
///
/// [pt] O meio byte alto de `a` mais `d` (o baixo fica).
pub(crate) unsafe fn add_high_nibble(a: usize, d: i8) {
    let v = *(a as *const u8);
    *(a as *mut u8) = ((v & 0xf0).wrapping_add((d as u8) << 4)) | (v & 0x0f);
}

/// Adds d to the low nibble of the byte at a (high kept).
///
/// [pt] O meio byte baixo de `a` mais `d` (o alto fica).
pub(crate) unsafe fn add_low_nibble(a: usize, d: i8) {
    let v = *(a as *const u8);
    *(a as *mut u8) = (v & 0xf0) | (v.wrapping_add(d as u8) & 0x0f);
}

/// adds each category's progress to the player's stored nibble; every two steps the category attributes move (1 to 20
/// ones by one, −125 to 125 ones by six); then the trained position or side goes to 20 when learnt, or back to 1 when
/// the training stops
/// Original: `TRAINING_MANAGER::apply_training_progress` (0x8b2be0, training_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8b2be0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::apply_training_progress(pessoa, d)`, pela
/// update_player_training (0x8b1750, cada jogador em cada atualização do treino) e pelas 0x8b3640 e 0x8b37c0. Pessoa
/// nula: a caixa "Error" (linha 0x4b3); sem a ficha (+0x61): a caixa (linha 0x4ba); id a partir de [0xae2364]: nada.
/// Com a entrada de 6 bytes do jogador (this +0x651d + 6 × id da ficha), cada categoria com progresso `d[k]` diferente
/// de zero anda o passo (progress_step sobre o meio byte guardado) nos seus atributos da ficha (os de 1 a 20 o passo, os de
/// −125 a 125 seis vezes o passo) e guarda o meio byte mais `d[k]`: a 3 (+4 alto), a 0 (+3 baixo), a 2 (+4 baixo), a 1
/// (+3 alto) e a 4 (+5 baixo). Depois o treino de posição (0x8b4b20): com a entrada de aprendizado de 15 para cima, a
/// posição ou o lado da máscara vai a 20; abaixo, volta a 1 se ainda sobrar mais de uma posição (+0xf..+0x15) ou lado
/// (+0x17..+0x19) de 15 para cima. Quente: sem contador.
pub(crate) unsafe extern "thiscall" fn apply_training_progress(this: usize, pessoa: usize, d: usize) {
    if pessoa == 0 {
        show_error_at(0xa8ccd0, 0x4b3);
        return;
    }
    let f = read_u32_as_usize(pessoa + staff::PLAYER);
    if f == 0 {
        show_error_at(0xa8ccd0, 0x4ba);
        return;
    }
    if read_i32(pessoa) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let e = read_u32_as_usize(this + 0x651d) + 6 * read_i32(f) as usize;
    // a categoria 3 (o meio byte alto de +4)
    let k = read_i8(d + 3);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 4) >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            add_clamped_1_20(f + 0x22, dd);
            add_clamped_125(f + 0x23, seis);
            add_clamped_125(f + 0x26, seis);
            add_clamped_1_20(f + 0x29, dd);
            add_clamped_125(f + 0x2b, seis);
            add_clamped_125(f + 0x37, seis);
            add_clamped_125(f + 0x3e, seis);
            add_clamped_1_20(f + 0x40, dd);
            add_clamped_125(f + 0x41, seis);
        }
        add_high_nibble(e + 4, k); // cpt9: guarda o progresso da categoria 3
    }
    // a categoria 0 (o meio byte baixo de +3)
    let k = read_i8(d);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 3) << 4 >> 4) as i32);
        if dd != 0 {
            add_clamped_1_20(f + 0x1b, dd); // cpt9: o primeiro atributo da categoria 0
            add_clamped_1_20(f + 0x1d, dd);
            add_clamped_1_20(f + 0x2e, dd);
            add_clamped_1_20(f + 0x34, dd);
            add_clamped_1_20(f + 0x36, dd);
            add_clamped_125(f + 0x3a, dd * 6);
            add_clamped_1_20(f + 0x3c, dd);
            add_clamped_1_20(f + 0x3d, dd);
        }
        add_low_nibble(e + 3, k);
    }
    // a categoria 2 (o meio byte baixo de +4)
    let k = read_i8(d + 2);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 4) << 4 >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            add_clamped_125(f + 0x27, seis);
            add_clamped_125(f + 0x31, seis);
            add_clamped_125(f + 0x38, seis);
        }
        add_low_nibble(e + 4, k);
    }
    // a categoria 1 (o meio byte alto de +3)
    let k = read_i8(d + 1);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 3) >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6; // cpt9: a categoria 1, seis vezes o passo
            add_clamped_125(f + 0x24, seis);
            add_clamped_125(f + 0x32, seis);
            add_clamped_125(f + 0x33, seis);
            add_clamped_1_20(f + 0x3f, dd);
            add_clamped_125(f + 0x39, seis);
        }
        add_high_nibble(e + 3, k);
    }
    // a categoria 4 (o meio byte baixo de +5)
    let k = read_i8(d + 4);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 5) << 4 >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            add_clamped_125(f + 0x2a, seis);
            add_clamped_125(f + 0x35, seis);
        }
        add_low_nibble(e + 5, k);
    }
    // o treino de posição
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(FN_TRAINING_MANAGER_FIND_POSITION_TRAINING);
    let mut p = 0usize;
    let n = achar(this, pessoa, &mut p);
    if n <= 0 {
        return;
    }
    let de15 = |o: usize| (read_i8(f + o) >= 0xf) as i8;
    let mut posicoes = de15(0xf) + de15(0x10) + de15(0x11) + de15(0x12) + de15(0x13) + de15(0x14) + de15(0x15);
    let mut lados = de15(0x18) + de15(0x19) + de15(0x17);
    for j in 0..n as usize {
        let item = p + 8 * j;
        let campo = match *((item + 4) as *const i16) as i32 {
            1 => 0xf,
            2 => 0x10,
            4 => 0x11,
            8 => 0x12,
            0x10 => 0x13,
            0x20 => 0x14,
            0x40 => 0x15,
            0x80 => 0x18,
            0x200 => 0x19,
            0x800 => 0x17,
            _ => continue,
        };
        if read_i8(item + 6) >= 0xf {
            *((f + campo) as *mut u8) = 0x14;
        } else if campo == 0x18 || campo == 0x19 || campo == 0x17 {
            if lados > 1 {
                *((f + campo) as *mut u8) = 1;
                lados -= 1;
            }
        } else if posicoes > 1 {
            *((f + campo) as *mut u8) = 1;
            posicoes -= 1;
        }
    }
}

/// Comparer for `apply_training_progress` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na ficha (+0xf..+0x41) e na entrada de treino do jogador (6 bytes), sem os casos de erro e
/// sem as pessoas que não são jogadores (só a original).
pub(crate) unsafe extern "thiscall" fn comparar_apply_training_progress(this: usize, pessoa: usize, d: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_APPLY_TRAINING_PROGRESS);
    if pessoa == 0 || read_u32_as_usize(pessoa + staff::PLAYER) == 0 || read_i32(pessoa) >= read_i32(G_STAFF_COUNT) {
        return o(this, pessoa, d);
    }
    let f = read_u32_as_usize(pessoa + staff::PLAYER);
    let e = read_u32_as_usize(this + 0x651d) + 6 * read_i32(f) as usize;
    let regioes = [(f + 0xf, 0x33), (e, 6)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, pessoa, d), || apply_training_progress(this, pessoa, d));
    PLACAR.registrar(igual, || format!("apply_training_progress(pessoa {:x}, d {:x?}) {:x?} x {:x?}", pessoa, core::slice::from_raw_parts(d as *const i8, 5), mo.0, mr.0));
}

/// finds the player's first entry in the sorted position training list and returns how many are his (0 without the
/// position training mark)
/// Original: `TRAINING_MANAGER::find_position_training` (0x8b4b20, training_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8b4b20 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::find_position_training(pessoa, &primeira)`:
/// pessoa nula, a caixa "Error" (linha 0x995); saída nula, a caixa (linha 0x99c); sem a ficha (+0x61), a caixa (linha
/// 0x9a3): 0. Sem a marca 0x10 em +5 da entrada de treino do jogador (this +0x651d + 6 × id da ficha): 0. Senão a busca
/// binária (0x946464, ordem 0x8b4e00) do id da pessoa na lista do treino de posição (this +0x6521, conta +0x6525, itens
/// de 8 bytes); não achou: 0; achou: recua até o primeiro item da pessoa, grava-o na saída e devolve quantos seguidos são
/// dela.
pub(crate) unsafe extern "thiscall" fn find_position_training(this: usize, pessoa: usize, saida: *mut usize) -> i32 {
    if pessoa == 0 {
        show_error_at(0xa8ccd0, 0x995);
        return 0;
    }
    if saida.is_null() {
        show_error_at(0xa8ccd0, 0x99c);
        return 0;
    }
    let f = read_u32_as_usize(pessoa + staff::PLAYER);
    if f == 0 {
        show_error_at(0xa8ccd0, 0x9a3);
        return 0;
    }
    if *((read_u32_as_usize(this + 0x651d) + 6 * read_i32(f) as usize + 5) as *const u8) & 0x10 == 0 {
        return 0;
    }
    let chave = read_i32(pessoa);
    let busca: unsafe extern "C" fn(*const i32, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let base = read_u32_as_usize(this + 0x6521);
    let mut a = busca(&chave, base, read_u32_as_usize(this + 0x6525), 8, 0x8b4e00);
    if a == 0 {
        return 0;
    }
    while a > base {
        if read_i32(a - 8) != chave {
            break;
        }
        a -= 8;
    }
    *saida = a;
    let ultimo = read_u32_as_usize(this + 0x6521) + read_u32_as_usize(this + 0x6525) * 8 - 8;
    let mut n = 0i32;
    while a <= ultimo && read_i32(a) == chave {
        a += 8;
        n += 1;
    }
    n
}

/// Comparer for `find_position_training` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução e a saída (reposta entre as duas), sem os casos de erro (só a original).
pub(crate) unsafe extern "thiscall" fn comparar_find_position_training(this: usize, pessoa: usize, saida: *mut usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32>(IDX_FIND_POSITION_TRAINING);
    if pessoa == 0 || saida.is_null() || read_u32_as_usize(pessoa + staff::PLAYER) == 0 {
        return o(this, pessoa, saida);
    }
    let antes = *saida;
    let r = o(this, pessoa, saida);
    let so = *saida;
    *saida = antes;
    let n = find_position_training(this, pessoa, saida);
    let sr = *saida;
    *saida = so;
    PLACAR.registrar(r == n && so == sr, || format!("find_position_training(pessoa {:x}) original={} {:x} rust={} {:x}", pessoa, r, so, n, sr));
    r
}

/// 1 when one of the player position training entries has a learning rate above 50
/// Original: `TRAINING_MANAGER::has_fast_position_training` (0x8b4db0, training_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8b4db0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::has_fast_position_training(pessoa)`, pelo
/// diário das pessoas (game_daily_staff_update 0x5ce2d0): 1 se algum item do treino de posição da pessoa (0x8b4b20) tem
/// aprendizado (+7) acima de 50; senão 0.
pub(crate) unsafe extern "thiscall" fn has_fast_position_training(this: usize, pessoa: usize) -> u32 {
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(FN_TRAINING_MANAGER_FIND_POSITION_TRAINING);
    let mut p = 0usize;
    let n = achar(this, pessoa, &mut p);
    for j in 0..n.max(0) as usize {
        if read_i8(p + 8 * j + 7) > 0x32 {
            return 1;
        }
    }
    0
}

/// Comparer for `has_fast_position_training` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (as duas só leem; a busca roda pelas duas).
pub(crate) unsafe extern "thiscall" fn comparar_has_fast_position_training(this: usize, pessoa: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HAS_FAST_POSITION_TRAINING)(this, pessoa);
    let n = has_fast_position_training(this, pessoa);
    PLACAR.registrar(r == n, || format!("has_fast_position_training(pessoa {:x}) original={} rust={}", pessoa, r, n));
    r
}


/// on a day of the year that is a multiple of 7 the training week goes up by 1 (each player training is caught up by
/// the missing weeks); returns the day / 7
/// Original: `TRAINING_MANAGER::advance_week_counter` (0x8b1730, training_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8b1730 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::advance_week_counter(&dia)`, todo dia pelo
/// laço do dia (game_main_day_loop 0x5cccd0, com o dia do ano da data do jogo 0xae2c90): no dia múltiplo de 7 (resto
/// com sinal), a semana do treino (+0x688d, 16 bits) sobe 1; a update_player_training (0x8b1750) põe em dia as semanas
/// que faltam de cada jogador. Devolve dia / 7 (o chamador não lê).
pub(crate) unsafe extern "thiscall" fn advance_week_counter(this: usize, dia: *const i16) -> i32 {
    let v = *dia as i32;
    let resto = v % 7; // cpt10: a semana de 7 dias
    if resto == 0 { // cpt10: o sétimo dia
        crate::comparacao::ramo("cpt_10 semana");
        let s = (this + 0x688d) as *mut u16;
        *s = (*s).wrapping_add(1); // cpt10: mais uma semana
    }
    v / 7
}

/// Comparer for `advance_week_counter` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na semana do treino, com a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_advance_week_counter(this: usize, dia: *const i16) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i16) -> i32>(IDX_ADVANCE_WEEK_COUNTER);
    let regioes = [(this + 0x688d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, dia), || advance_week_counter(this, dia));
    PLACAR.registrar(igual && r == n, || format!("advance_week_counter(dia {}) original={} rust={} {:x?} x {:x?}", *dia, r, n, mo.0, mr.0));
    r
}


// ponytail: a reserva perde o item (não roda duas vezes): só a original; a cópia leva o ramo.

/// allocates the 6-byte training entry of every player data record and puts goalkeepers (goalkeeping rating 15 or more)
/// on schedule 6 and the others on 1
/// Original: `TRAINING_MANAGER::init_player_entries` (0x8b3c50, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b3c50 } (thiscall, ret 0, training_manager.cpp): `TRAINING_MANAGER::init_player_entries()`, pelo construtor
/// (0x8b0ec0) num jogo novo: aloca (calloc 0x945501) as fichas de treino de 6 bytes, uma por ficha de jogador
/// ([0xae236c] fichas de 0x46 bytes em [0xae23c4], a ficha pelo id +0), em this + 0x651d; sem memória, a caixa (linha
/// 0x6e3) e volta. Em cada ficha apaga o bit 0x20 do +5 e põe a vaga do programa (+0): 6 (goleiros) com a nota de
/// goleiro (+0xf, com sinal) de 15 para cima, senão 1 (geral). A primeira atualização do treino de cada jogador
/// (0x8b1750, que só depois refaz a vaga dos jogadores do computador) e todo o treino do clube humano usam essa vaga.
pub(crate) unsafe extern "thiscall" fn init_player_entries(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let p = calloc(read_u32_as_usize(G_PLAYER_DATA_COUNT) as u32, 6);
    *((this + 0x651d) as *mut u32) = p as u32;
    if p == 0 {
        show_error_at(0xa8ccd0, 0x6e3);
        return;
    }
    let mut f = read_u32_as_usize(G_PLAYER_DATA_TABLE);
    let mut i = 0i32;
    while i < read_i32(G_PLAYER_DATA_COUNT) {
        let ficha = || read_u32_as_usize(this + 0x651d).wrapping_add((read_i32(f) as usize).wrapping_mul(6));
        *((ficha() + 5) as *mut u8) &= 0xdf;
        let gol = *((f + 0xf) as *const i8);
        if gol >= 0xf { // cpu9: goleiro de 15 para cima
            crate::comparacao::ramo("cpu_9 goleiro");
            *(ficha() as *mut u8) = 6; // cpu9: o programa dos goleiros
        } else {
            crate::comparacao::ramo("cpu_9 linha");
            *(ficha() as *mut u8) = 1; // cpu9: o programa geral
        }
        i += 1;
        f += 0x46;
    }
}

/// Comparer for `init_player_entries` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca (não pode rodar duas vezes): roda só ela e confere cada ficha pela regra (a vaga 6
/// ou 1 pela nota de goleiro e o bit 0x20 do +5 apagado).
pub(crate) unsafe extern "thiscall" fn comparar_init_player_entries(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_PLAYER_ENTRIES)(this);
    let base = read_u32_as_usize(this + 0x651d);
    if base == 0 {
        return;
    }
    let mut erradas = 0usize;
    let mut primeira = None;
    for k in 0..read_i32(G_PLAYER_DATA_COUNT).max(0) as usize {
        let f = read_u32_as_usize(G_PLAYER_DATA_TABLE) + 0x46 * k;
        let e = base.wrapping_add((read_i32(f) as usize).wrapping_mul(6));
        let vaga = if *((f + 0xf) as *const i8) >= 0xf { 6 } else { 1 };
        if *(e as *const u8) != vaga || *((e + 5) as *const u8) & 0x20 != 0 {
            erradas += 1;
            primeira.get_or_insert(k);
        }
    }
    PLACAR.registrar(erradas == 0, || format!("init_player_entries: {} fichas fora da regra (a primeira {:?})", erradas, primeira));
}
