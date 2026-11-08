use super::*;

/// Staff id -> person record (0x6e bytes in G_STAFF_TABLE).
///
/// [pt] O registro da pessoa `id` (tabela [0xae23bc], 0x6e bytes).
pub(crate) unsafe fn training_manager_staff_by_id(id: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((id as isize * 0x6e) as usize)
}

/// Resets a 9-byte coaching slot to empty (id -1, ratings 0).
///
/// [pt] A vaga de 9 bytes da comissão (+0 id do treinador, +4..+8 as notas das cinco categorias) volta a vazia: −1, 0, 0.
pub(crate) unsafe fn clear_coach_slot(e: usize) {
    core::ptr::write_unaligned(e as *mut i32, -1);
    core::ptr::write_unaligned((e + 4) as *mut u32, 0);
    *((e + 8) as *mut u8) = 0;
}

/// The rating divisors by the coach's number of categories (1 to 5), floats built on the stack by the original: 1.0;
/// 1.2; 1.4; 1.6; 2.0.
///
/// [pt] Os divisores da nota pela quantidade de categorias do treinador (1 a 5), floats montados na pilha pela original:
/// 1,0; 1,2; 1,4; 1,6; 2,0.
pub(crate) const RATING_DIVISORS: [u32; 5] = [0x3f80_0000, 0x3f99_999a, 0x3fb3_3333, 0x3fcc_cccd, 0x4000_0000];

/// Non-player attribute rating (0x544410) for the attribute byte at f + k.
///
/// [pt] A chance de conhecer (0x544410, cdecl: a ficha de não jogador e o byte do atributo; a original empurra o
/// registrador com lixo em cima: a chamada lê o byte).
pub(crate) unsafe fn non_player_attribute_rating(f: usize, k: usize) -> i32 {
    exe::non_player_attribute_rating(f, read_i8(f + k)) as i8 as i32 // the original routine sign-extends al (movsx)
}

/// `TRAINING_MANAGER::update_coach_assignments(club, manager)`, called by the coach hirings and departures (0x4ce4e0,
/// 0x4cf720, 0x8c9720, the regen 0x7aaeb0) and by the club change (0x8b2470): updates the six training staff slots of
/// the human manager and recomputes each category rating.
///
/// Without a manager and with a club without a human (0x600a50 with 1, 1, 0, 0), or a national side club (0x53b900):
/// nothing. A null manager: the club's (+0xcf). The human's staff (0x8b4cd0, six 9-byte slots): (1) the slot whose
/// coach has no non-player sheet (+0x69) or is neither the assistant (0x53cb80) nor one of the 5 coaches (0x53cc30) is
/// emptied; (2) the assistant and the coaches with a sheet that are not in the slots go into the first empty one with
/// all five categories (0x01010101 and 1); (3) each flagged category gets its rating: 0 (goalkeepers, sheet +0x10)
/// (chance − sheet +0x12 + 21) / 2; 1 (tactics, +0x21 and +0x10) (chance + chance + +0x12) / 3; 2 (+0x10) (chance +
/// +0xe + +0x12) / 3; 3 (+0x10) (chance + +0x12) / 2; 4 (+0x11) the chance (divisions truncated toward zero; the chance
/// from 0x544410), the signed byte divided by the divisor for the number of flagged categories (1.0 to 2.0, float),
/// truncated and clamped to 1..20.
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
pub(crate) unsafe extern "thiscall" fn update_coach_assignments(this: usize, club: usize, tecnico: usize) {
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if tecnico == 0 && human(club, 1, 1, 0, 0) == 0 {
        return;
    }
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    let manager = if tecnico == 0 { read_usize(club + club::MANAGER) } else { tecnico };
    let staff: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_COACH_ASSIGNMENTS);
    let t = staff(this, manager);
    let aux = exe::database_get_club_assistant(club);
    let mut tr = [0usize; 5];
    exe::database_get_club_coaches(club, tr.as_mut_ptr());
    let candidate = |k: i32| if k < 0 { aux } else { tr[k as usize] };
    for v in 0..6 {
        let e = t + 9 * v;
        let id = read_i32(e);
        if id == -1 {
            continue;
        }
        if read_usize(training_manager_staff_by_id(id) + 0x69) == 0 {
            clear_coach_slot(e);
            continue;
        }
        if !(-1..5).any(|k| {
            let p = candidate(k);
            p != 0 && read_i32(p) == id
        }) {
            clear_coach_slot(e);
        }
    }
    for k in -1..5 {
        let p = candidate(k);
        if p == 0 || read_usize(p + staff::NON_PLAYER) == 0 {
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
        let f = read_usize(training_manager_staff_by_id(id) + 0x69);
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
                0 => (non_player_attribute_rating(f, 0x10) - b(0x12) + 0x15) / 2, // cqf7: the goalkeeping rating
                1 => {
                    let a = non_player_attribute_rating(f, 0x21);
                    (a + non_player_attribute_rating(f, 0x10) + b(0x12)) / 3 // cqf7: the tactics rating
                }
                2 => (non_player_attribute_rating(f, 0x10) + b(0xe) + b(0x12)) / 3,
                3 => (non_player_attribute_rating(f, 0x10) + b(0x12)) / 2,
                _ => non_player_attribute_rating(f, 0x11),
            };
            // the low byte, signed, as float divided by the float of the quantity (x87 double precision)
            let q = ftol(x as i8 as f64 / f32::from_bits(RATING_DIVISORS[n - 1]) as f64) as i8; // cqf7: the quantity divisor
            crate::comparison::branch("cqf_7 nota do treinador");
            *((e + 4 + j) as *mut i8) = q.clamp(1, 20);
        }
    }
}

/// Training schedule `vaga` (0..9, else 1) of human manager `tec`; outside the 16 humans shows the error box and
/// returns 0.
///
/// [pt] O programa `vaga` (0..9; fora disso o 1) do técnico humano `tec`: this + (vaga + 7 + 10 × ([0xae2364] − id − 1)) ×
/// 0x9b; fora dos 16 humanos, a caixa "Error" (linha 0x68f) e 0 (a original lê então o byte +0x15 do nulo).
pub(crate) unsafe fn human_training_schedule(this: usize, manager: usize, slot: i8) -> usize {
    let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    let v = if (0..10).contains(&slot) { slot as i32 } else { 1 };
    this.wrapping_add(((v + 7 + k * 10) * 0x9b) as usize)
}

/// 6-byte training entry of player data `f` ([this+0x651d] + 6 x id).
///
/// [pt] A entrada de 6 bytes do treino da ficha `f` (+0 a vaga do programa).
pub(crate) unsafe fn training_entry_of_player_data(this: usize, f: usize) -> usize {
    read_usize(this + 0x651d).wrapping_add((read_i32(f) as isize * 6) as usize)
}

/// `TRAINING_MANAGER::reassign_squad_schedules(manager, old, new)`, when the human manager changes club (0x690890 and
/// 0x68f9c0/0x69ebc0, manager_manager.cpp): reassigns the training schedules of the old and new squads.
///
/// The old club's squad (0x541f90, 50): each valid person (id below [0xae2364]) gets load 50 (0x62c660) and slot 6
/// (goalkeeper with 15 or more, sheet +0xf) or 1 in the training entry; without a sheet, the box (line 0x34c). With a
/// new club: of the manager's ten schedules, the last one of type 6 (+0x15) is the goalkeepers' and the last one of
/// type 1 the general one; each player of the new squad with a sheet goes to the goalkeepers' schedule (goalkeeper with
/// 15 or more, if there is one), otherwise the general one (if there is one), otherwise slot 1, with the schedule's
/// load (0x8b4f60) on the fitness sheet; finally the training staff (0x8b2880 with the new club and the manager).
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
pub(crate) unsafe extern "thiscall" fn reassign_squad_schedules(this: usize, manager: usize, old: usize, new: usize) {
    let (mut gol, mut geral) = (-1i8, -1i8);
    if old != 0 {
        let mut v = [0usize; 50];
        exe::database_collect_club_squad(old, v.as_mut_ptr(), 0);
        for &p in &v {
            if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            exe::injury_set_training_load(read_usize(G_INJURY), p, 0x32);
            let f = read_usize(p + staff::PLAYER);
            if f == 0 {
                show_error_at(0xa8ccd0, 0x34c);
                continue;
            }
            *(training_entry_of_player_data(this, f) as *mut u8) = if read_i8(f + player_sheet::GOALKEEPER) >= 0xf { 6 } else { 1 };
        }
    }
    if new == 0 {
        return;
    }
    for vaga in 0..10i8 {
        if *((human_training_schedule(this, manager, vaga) + 0x15) as *const u8) == 6 {
            gol = vaga;
        } else if *((human_training_schedule(this, manager, vaga) + 0x15) as *const u8) == 1 {
            geral = vaga;
        }
    }
    let mut v = [0usize; 50];
    exe::database_collect_club_squad(new, v.as_mut_ptr(), 0);
    for &p in &v {
        if p == 0 {
            continue;
        }
        let f = read_usize(p + staff::PLAYER);
        if f == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let e = training_entry_of_player_data(this, f) as *mut i8;
        *e = if read_i8(f + player_sheet::GOALKEEPER) >= 0xf && gol != -1 {
            gol // cqf7: the goalkeepers programme
        } else if geral != -1 {
            geral
        } else {
            1
        };
        crate::comparison::branch("cqf_7 programa do elenco novo");
        let c = exe::training_schedule_get_training_load(human_training_schedule(this, manager, *e));
        exe::injury_set_training_load(read_usize(G_INJURY), p, c as u32);
    }
    let staff: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRAINING_MANAGER_UPDATE_COACH_ASSIGNMENTS);
    staff(this, new, manager);
}

/// the default names of the 167 training schedules
/// Original: `(training_manager.cpp, 0x8b2230)` (0x8b2230, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b2230 } (thiscall, ret 0, training_manager.cpp): os nomes padrão dos 167 programas de treino (0x8b50f0 em cada
/// um, de 0x9b em 0x9b bytes), pela criação dos programas padrão (0x8b3930).
pub(crate) unsafe extern "thiscall" fn training_manager_cpp_0x8b2230(this: usize) {
    for k in 0..0xa7 {
        exe::training_schedule_set_default_name(this + k * 0x9b);
    }
}

/// 6-byte training entry of a person ([this+0x651d] + 6 x id of player data +0x61).
///
/// [pt] A entrada de 6 bytes do treino do jogador: [this+0x651d] + 6 × id da ficha (+0x61).
pub(crate) unsafe fn training_entry_of_staff(this: usize, p: usize) -> usize {
    read_usize(this + 0x651d).wrapping_add((read_i32(read_usize(p + staff::PLAYER)) as isize * 6) as usize)
}

/// Person -> contract (get_contract), stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa (CONTRACT_MANAGER 0x4cf5a0, thiscall no gerente [0xae1050]), guardado em [0xae1080] como a
/// original faz a cada consulta.
pub(crate) unsafe fn training_manager_get_contract_stored(p: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
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
        read_usize(G_CLUB_TABLE).wrapping_add((read_i32(c + 4) as isize * 0x245) as usize)
    }
}

/// Manager's id is among the last 16 staff ids (the human managers).
///
/// [pt] O técnico humano é um dos 16 últimos ids de pessoa ([0xae2364] − 16 para cima).
pub(crate) unsafe fn is_human_manager_id(manager: usize) -> bool {
    read_i32(manager) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10)
}

/// `TRAINING_MANAGER::update_player_training(person, skip)`, called by the player initialisation for the match
/// (0x6eaea0, match_man.cpp, with 0: every player of every match), by a new injury (0x62d250) and by the screens:
/// brings the player's training weeks up to date (up to 30).
///
/// A null person (line 0x18a), without the sheet (0x195) or without a schedule (0x8b39f0 null, 0x1a3): the "Error" box;
/// an id from [0xae2364] on: nothing. A player of a human manager's club without flag 0x20 in +5 of the training entry:
/// the flag, the schedule slot (the goalkeepers' one, type 6, for a goalkeeper with 15 or more; otherwise the general
/// one, type 1; otherwise 1) and its load (0x8b4f60) on the fitness sheet (0x62c660). Weeks = training week (+0x688d) −
/// the entry's (+1); none and without `skip`: nothing. With the flag and the club no longer with a human manager:
/// clears the flag, load 50, slot 6 or 1 and the position training stops (0x8b4450 with 0).
///
/// Up to 30 weeks: brings the entry up to date; each category (0..4) has the weight level × 2 + 60 + (coach factor −
/// 100) / 5 (0..200) and, per week, progress rolls: up 1 with weight above 100, draw(210 − weight) < 20, value below
/// (+0x58) / 3 and draw(|value| × 2 + 14) < 10; otherwise down 1 with weight below 110, draw(weight + 60) < 20, value
/// above −7 and above (+0x5b) / 3 − 6 and the same last draw (goalkeeper with 19 or more: only with draw(4), or draw(2)
/// in category 4, at zero).
///
/// The position training items (0x8b4b20, 8-byte items: +4 the mask, +6 the rating, +7 the rate) advance per week (or
/// once, with `skip` in the mask): with a positive rate, draw(rate) < 5 gives rating + 2 (up to 20) and the player may
/// complain (club contract 0x4cf5f0 without 0x8000 in +0x45: by a rate above 50 or an aptitude 0x8b4770 below 12, with
/// +0x5b below 15, draw(+0x5b) < 10 and no injury 0x62c400: flag 0x8000 (0x867700), −5 on +0x44 (−100..100) and
/// 0x5885c0); with a negative rate, draw(−rate) < 5 gives rating − 2 and, below 1, the item ends (rate 0, rating 1). It
/// applies the progress (0x8b2be0) and, with finished items, removes them from the list (the last one into the gap) and
/// re-sorts (qsort 0x945333 with 0x8b4e00).
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
pub(crate) unsafe extern "thiscall" fn update_player_training(this: usize, p: usize, skip: u32) {
    if p == 0 {
        show_error_at(0xa8ccd0, 0x18a);
        return;
    }
    if read_i32(p) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    if read_usize(p + staff::PLAYER) == 0 {
        show_error_at(0xa8ccd0, 0x195);
        return;
    }
    let mut d = [0i8; 8]; // the progress of the five categories (the original clears a dword and a byte)
    let e = training_entry_of_staff(this, p);
    let prog = exe::training_manager_get_player_schedule(this, p);
    if prog == 0 {
        show_error_at(0xa8ccd0, 0x1a3);
        return;
    }
    let mark = (e + 5) as *mut u8;
    if *mark & 0x20 == 0 {
        let c = training_manager_get_contract_stored(p);
        if c != 0 && training_manager_club_of_contract(c) != 0 && read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf) != 0
            && is_human_manager_id(read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf)) {
            let manager = read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf);
            *mark |= 0x20;
            let (mut goleiro, mut geral) = (-1i8, -1i8);
            for k in 0..10i8 {
                if *((human_training_schedule(this, manager, k) + 0x15) as *const u8) == 6 {
                    goleiro = k;
                } else if *((human_training_schedule(this, manager, k) + 0x15) as *const u8) == 1 {
                    geral = k;
                }
            }
            let slot = if read_i8(read_usize(p + staff::PLAYER) + 0xf) >= 0xf && goleiro != -1 {
                goleiro
            } else if geral != -1 {
                geral
            } else {
                1
            };
            *(e as *mut i8) = slot;
            let v = exe::training_schedule_get_training_load(human_training_schedule(this, manager, slot));
            crate::comparison::branch("cqd_8 programa do humano");
            exe::injury_set_training_load(read_usize(G_INJURY), p, v as u32);
        }
    }
    let mut weeks = read_u16(this + 0x688d).wrapping_sub(read_u16(e + 1));
    let skip = skip as u16;
    if weeks == 0 && skip == 0 {
        return;
    }
    if *mark & 0x20 != 0 {
        let c = training_manager_get_contract_stored(p);
        let ainda = c != 0 && training_manager_club_of_contract(c) != 0 && read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf) != 0
            && is_human_manager_id(read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf));
        if !ainda {
            crate::comparison::branch("cqd_8 saiu do clube humano");
            *mark &= 0xdf;
            exe::injury_set_training_load(read_usize(G_INJURY), p, 0x32);
            let f = read_usize(p + staff::PLAYER);
            *(training_entry_of_staff(this, p) as *mut u8) = if read_i8(f + player_sheet::GOALKEEPER) >= 0xf { 6 } else { 1 };
            let mut q = 0usize;
            let n = exe::training_manager_find_position_training(this, p, &mut q);
            let mut k = 0i8;
            if n > 0 {
                loop {
                    exe::training_manager_set_position_training_rate(this, q.wrapping_add((k as isize * 8) as usize), 0);
                    k = k.wrapping_add(1);
                    if k as i32 >= n {
                        break;
                    }
                }
            }
        }
    }
    if weeks > 0x1e {
        weeks = 0x1e;
    }
    core::ptr::write_unaligned((e + 1) as *mut u16, read_u16(this + 0x688d));
    let mut q = 0usize;
    let npos = exe::training_manager_find_position_training(this, p, &mut q);
    if *mark & 0x10 != 0 && npos == 0 {
        *mark &= 0xef;
    }
    for k in 0..5usize {
        // simplification: the original pushes the category in a dword whose upper bytes are stack leftovers; 0x8b4f50 and
        // 0x8b41d0 leem o byte
        let lv = exe::training_schedule_get_category_level(prog, k as u32) as u8 as i8;
        let base = (lv as i16).wrapping_mul(2).wrapping_add(0x3c); // cqd8: the level weight
        let club = training_manager_club_of_contract(training_manager_get_contract_stored(p));
        let fa = exe::training_manager_get_coaching_factor(this, club, k as u32) as u16 as i16 as i32;
        let x = base.wrapping_add(((fa - 100) / 5) as i16);
        let weight: i16 = if x < 0 { 0 } else if x > 200 { 200 } else { x };
        let floor = ((read_i8(p + staff::PROFESSIONALISM) as i32 / 3) as i8).wrapping_sub(6);
        let ceiling = (read_i8(p + staff::DETERMINATION) as i32 / 3) as i8;
        let middle = training_nibble(e, k);
        for _ in 0..weeks {
            let v = middle.wrapping_add(d[k]);
            if read_i8(read_usize(p + staff::PLAYER) + 0xf) > 0x12 {
                let r = if k == 4 { friendly_random(2) } else { friendly_random(4) };
                if r != 0 {
                    continue; // goalkeeper: the category barely moves
                }
            }
            if weight > 0x64 {
                if friendly_random(0xd2 - weight as i32) < 0x14 && v < ceiling { // cqd8: the rise random draw
                    if friendly_random((v as i32).abs() * 2 + 0xe) < 10 {
                        crate::comparison::branch("cqd_8 progresso sobe");
                        d[k] = d[k].wrapping_add(1); // cqd8: o progresso sobe
                    }
                    continue;
                }
            }
            if weight >= 0x6e {
                continue;
            }
            if friendly_random(weight as i32 + 0x3c) >= 0x14 || v <= -7 || v <= floor {
                continue;
            }
            if friendly_random((v as i32).abs() * 2 + 0xe) < 10 {
                crate::comparison::branch("cqd_8 progresso desce");
                d[k] = d[k].wrapping_sub(1); // cqd8: o progresso desce
            }
        }
    }
    let mut voltas = weeks;
    let mut contract_of_club = 0usize;
    if skip != 0 || voltas != 0 {
        if skip != 0 {
            voltas = 1;
        }
        let sheet: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        contract_of_club = sheet(read_usize(G_CONTRACT_MANAGER), p, read_usize(p + staff::CLUB_JOB) as u32);
    }
    let mut acabou = false;
    if npos > 0 {
        let reclamar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
        let mut k = 0i8;
        let mut it = q + 7; // the item’s fee (+7); the rating at +6, the mask at +4
        loop {
            for _ in 0..voltas {
                let taxa = read_i8(it);
                if (skip != 0 && read_u16(it - 3) == skip) || taxa > 0 {
                    if friendly_random(read_i8(it) as i32) < 5 {
                        let n = read_i8(it - 1).wrapping_add(2);
                        *((it - 1) as *mut i8) = if n > 0x14 { 0x14 } else { n };
                    }
                    let c = contract_of_club;
                    if c != 0 && read_usize(c + 0x45) & 0x8000 == 0 {
                        // simplification: the original pushes the mask in ecx with the leftover upper 16 bits; 0x8b4770
                        // reads the word
                        let apt = exe::training_manager_get_position_aptitude(this, p, read_u16(it - 3) as u32) as u8 as i8;
                        let t = read_i8(it);
                        let quer = (t > 0x32 && friendly_random(100 - t as i32) < 10) || (apt < 0xc && friendly_random(apt as i32) < 2);
                        let a = read_i8(p + staff::PROFESSIONALISM);
                        if quer && a < 0xf && friendly_random(a as i32) < 10 && read_usize(G_INJURY) != 0
                            && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 0 {
                            crate::comparison::branch("cqd_8 reclama do treino de posicao");
                            reclamar(c, 0x8000, 0);
                            let m = (c + 0x44) as *mut i8;
                            let v = *m as i32 - 5;
                            *m = if v > 100 { 100 } else if v < -100 { -100 } else { (*m).wrapping_sub(5) };
                            exe::news_post_player_unhappy_with_training(p);
                            contract_of_club = 0;
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
    exe::training_manager_apply_training_progress(this, p, d.as_ptr());
    if acabou {
        crate::comparison::branch("cqd_8 treino de posicao acabou");
        let n0 = read_i32(this + 0x6525);
        let mut i = 0i32;
        if n0 > 0 {
            loop {
                let base = read_usize(this + 0x6521);
                let n = read_i32(this + 0x6525);
                let item = base.wrapping_add((i as isize * 8) as usize);
                if *((item + 7) as *const u8) == 0 {
                    let last = base.wrapping_add((n as isize * 8 - 8) as usize);
                    core::ptr::write_unaligned(item as *mut u32, read_usize(last) as u32);
                    core::ptr::write_unaligned((item + 4) as *mut u32, read_usize(last + 4) as u32);
                    core::ptr::write_unaligned((this + 0x6525) as *mut i32, read_i32(this + 0x6525) - 1);
                }
                i += 1;
                if i >= read_i32(this + 0x6525) {
                    break;
                }
            }
        }
        if n0 != read_i32(this + 0x6525) {
            exe::crt_qsort(read_usize(this + 0x6521), read_usize(this + 0x6525) as u32, 8, 0x8b4e00);
        }
    }
}

/// `TRAINING_MANAGER::get_player_schedule(person)`, called by the people's daily update (game_daily_staff_update
/// 0x5ce2d0), by 0x5d6aa0 and by update_player_training (0x8b1750): returns the player's training schedule.
///
/// A null person or one without the sheet: the "Error" box (line 0x6ad) and 0. At a human manager's club (contract
/// 0x4cf5a0, kept in [0xae1080]; manager +0xcf among the last 16 ids): injured (0x62c400 = 2), the human's schedule 1,
/// this + 0x4d8 + 0x60e × ([0xae2364] − id − 1); otherwise the one of the training entry's slot (+0, outside 0..9 slot
/// 1); outside the 16, the box (line 0x68f) and 0. Otherwise: injured, schedule 0 (this); otherwise this + 0x9b × slot
/// (signed).
/// Original: `TRAINING_MANAGER::get_player_schedule` (0x8b39f0, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b39f0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::get_player_schedule(pessoa)`, pelo diário
/// das pessoas (game_daily_staff_update 0x5ce2d0), pela 0x5d6aa0 e pela update_player_training (0x8b1750). Pessoa nula
/// ou sem a ficha: a caixa "Error" (linha 0x6ad) e 0. No clube de técnico humano (contrato 0x4cf5a0, guardado em
/// [0xae1080]; técnico +0xcf entre os 16 últimos ids): lesionado (0x62c400 = 2) o programa 1 do humano, this + 0x4d8 +
/// 0x60e × ([0xae2364] − id − 1); senão o da vaga da entrada do treino (+0, fora de 0..9 a 1); fora dos 16, a caixa
/// (linha 0x68f) e 0. Senão: lesionado, o programa 0 (this); senão this + 0x9b × vaga (com sinal).
pub(crate) unsafe extern "thiscall" fn get_player_schedule(this: usize, p: usize) -> usize {
    if p == 0 || read_usize(p + staff::PLAYER) == 0 {
        show_error_at(0xa8ccd0, 0x6ad);
        return 0;
    }
    let c = training_manager_get_contract_stored(p);
    if c != 0 && training_manager_club_of_contract(c) != 0 {
        let manager = read_usize(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf);
        if manager != 0 && is_human_manager_id(manager) {
            if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
                let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
                if !(0..0x10).contains(&k) {
                    show_error_at(0xa8ccd0, 0x68f);
                    return 0;
                }
                return this.wrapping_add(0x4d8 + 0x60e * k as usize);
            }
            return human_training_schedule(this, manager, *(training_entry_of_staff(this, p) as *const i8));
        }
    }
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
        return this;
    }
    this.wrapping_add((*(training_entry_of_staff(this, p) as *const i8) as isize * 0x9b) as usize)
}

/// `TRAINING_SCHEDULE::get_category_level(category)`: returns the level of the schedule's category (+4 + category,
/// signed), in al; the upper bytes of eax keep the sign-extended category (movsx), as in the original. Called by
/// update_player_training (0x8b1750, the five categories of each update).
/// Original: `TRAINING_SCHEDULE::get_category_level` (0x8b4f50, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b4f50 } (thiscall, ret 4, training_manager.cpp): `TRAINING_SCHEDULE::get_category_level(categoria)`: o nível da
/// categoria (+4 + categoria, com sinal) do programa, no al; os bytes de cima do eax ficam com a categoria estendida
/// (movsx), como na original. Pela update_player_training (0x8b1750, as cinco categorias de cada atualização).
pub(crate) unsafe extern "thiscall" fn get_category_level(this: usize, category: u32) -> u32 {
    let k = category as u8 as i8 as i32;
    let v = *((this as isize + k as isize + 4) as *const u8);
    (k as u32 & 0xffff_ff00) | v as u32
}

/// 6-byte training entry of a person ([this+0x651d] + 6 x id of player data +0x61).
///
/// [pt] A entrada de 6 bytes do treino do jogador: [this+0x651d] + 6 × id da ficha (+0x61).
pub(crate) unsafe fn player_training_entry(this: usize, p: usize) -> usize {
    read_usize(this + 0x651d).wrapping_add((*(read_usize(p + staff::PLAYER) as *const i32) as isize * 6) as usize)
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
    exe::training_manager_apply_training_progress(this, p, d.as_ptr());
}

/// `TRAINING_MANAGER::remove_or_restore_training_progress(person, restore)`: takes the progress stored in the training
/// entry off the attributes or puts it back, around the player attribute normalisations (0x538bc0 and 0x538ec0: on
/// ageing in the people's daily update 0x5ce2d0, in the young regen 0x7abac0 and on load), which take the progress off
/// before and put it back after.
///
/// A null person or one without the sheet: the "Error" box (line 0x621). Taking off (`restore` = 0): keeps the five
/// progress nibbles of the training entry in 0xde1f3c..0xde1f40 (category 3, 2, 4, 0, 1) and applies the negated
/// progress (0x8b2be0); restoring: applies the kept one.
/// Original: `TRAINING_MANAGER::remove_or_restore_training_progress` (0x8b3640, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b3640 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::remove_or_restore_training_progress(pessoa,
/// devolver)`, pelas normalizações dos atributos do jogador (0x538bc0 e 0x538ec0: no envelhecimento do diário das
/// pessoas 0x5ce2d0, no regen jovem 0x7abac0 e na carga), que tiram o progresso antes e o devolvem depois. Pessoa nula
/// ou sem a ficha: a caixa "Error" (linha 0x621). Tirar (`devolver` = 0): guarda os cinco meios bytes do progresso da
/// entrada do treino em 0xde1f3c..0xde1f40 (categoria 3, 2, 4, 0, 1) e aplica o progresso negado (0x8b2be0); devolver:
/// aplica o guardado.
pub(crate) unsafe extern "thiscall" fn remove_or_restore_training_progress(this: usize, p: usize, devolver: u32) {
    if p == 0 || read_usize(p + staff::PLAYER) == 0 {
        show_error_at(0xa8ccd0, 0x621);
        return;
    }
    let e = player_training_entry(this, p);
    let mut d = [0i8; 8];
    let g = |a: usize| *(a as *const i8);
    if devolver != 0 {
        crate::comparison::branch("cqd_9 devolver");
        d[0] = g(G_TRAINING_PROGRESS_SAVED_DELTA_0); // cqd9: returns category 0
        d[1] = g(G_TRAINING_PROGRESS_SAVED_DELTA_1);
        d[2] = g(G_TRAINING_PROGRESS_SAVED_DELTA_2);
        d[3] = g(G_TRAINING_PROGRESS_SAVED_DELTA_3);
        d[4] = g(G_TRAINING_PROGRESS_SAVED_DELTA_4); // cqd9: returns category 4
    } else {
        crate::comparison::branch("cqd_9 tirar");
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

/// `TRAINING_MANAGER::get_training_attribute_change(person, attribute)`, called by the training screen (0x8b8184): how
/// much the training progress adds to an attribute: the attribute (0x5431e0) minus the same attribute without the
/// training progress (it applies the negated progress, reads, and reapplies the progress), in al. A null person or one
/// without the sheet: the "Error" box (line 0x657) and 0.
/// Original: `TRAINING_MANAGER::get_training_attribute_change` (0x8b37c0, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b37c0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_training_attribute_change(pessoa,
/// atributo)`, pela tela do treino (0x8b8184): o atributo (0x5431e0) menos o mesmo atributo sem o progresso do treino
/// (aplica o progresso negado, lê e reaplica o progresso), no al. Pessoa nula ou sem a ficha: a caixa "Error" (linha
/// 0x657) e 0.
pub(crate) unsafe extern "thiscall" fn get_training_attribute_change(this: usize, p: usize, attribute: u32) -> u8 {
    if p == 0 || read_usize(p + staff::PLAYER) == 0 {
        show_error_at(0xa8ccd0, 0x657);
        return 0;
    }
    let e = player_training_entry(this, p);
    let before = (exe::staff_get_player_attribute(p, attribute) as u8);
    let mut d = [0i8; 8];
    for k in 0..5 {
        d[k] = training_nibble(e, k).wrapping_neg();
    }
    training_manager_apply_training_progress(this, p, &d);
    let sem = (exe::staff_get_player_attribute(p, attribute) as u8);
    let mut v = [0i8; 8];
    for k in 0..5 {
        v[k] = d[k].wrapping_neg();
    }
    training_manager_apply_training_progress(this, p, &v);
    before.wrapping_sub(sem)
}

/// Position masks accepted by 0x8b4770 (others show the error box).
///
/// [pt] As máscaras de posição que a 0x8b4770 aceita (as outras dão a caixa).
pub(crate) fn is_valid_position_mask(m: i32) -> bool {
    matches!(m, 1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x40 | 0x80 | 0x200 | 0x800)
}

/// `TRAINING_MANAGER::get_position_aptitude(person, mask)` (`this` not used), called by update_player_training
/// (0x8b1750, the position training complaint) and by set_position_training_rate (0x8b4450): the player's aptitude to
/// learn a position or side: the average of the adaptability (person's +0x56) and the attributes the position needs
/// (ratings 1 to 20 through 0x544380 over the sheet +0x61).
///
/// Goalkeeper (1) +0x39, +0x2a and +0x35, over 4; defence (2, 4) +0x3e, +0x39 and +0x32, over 4; midfield and attack
/// (8, 0x10, 0x20) +0x37, +0x32, +0x40 and +0x3f, over 5; 0x40 +0x33, +0x27, +0x40 and +0x36, over 5; 0x80 +0x30, over
/// 2; 0x200 only the adaptability (in al, the upper bytes with the mask); 0x800 +0x3b, over 2 (truncated divisions).
/// Any other mask: the "Error" box (line 0x945) and 0.
/// Original: `TRAINING_MANAGER::get_position_aptitude` (0x8b4770, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b4770 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_position_aptitude(pessoa, mascara)`
/// (this não usado), pela update_player_training (0x8b1750, a reclamação do treino de posição) e pela
/// set_position_training_rate (0x8b4450): a média da adaptabilidade (+0x56 da pessoa) com os atributos que a posição
/// pede (notas de 1 a 20 pela 0x544380 sobre a ficha +0x61): goleiro (1) +0x39, +0x2a e +0x35, por 4; defesa (2, 4)
/// +0x3e, +0x39 e +0x32, por 4; meio e frente (8, 0x10, 0x20) +0x37, +0x32, +0x40 e +0x3f, por 5; 0x40 +0x33, +0x27,
/// +0x40 e +0x36, por 5; 0x80 +0x30, por 2; 0x200 só a adaptabilidade (no al, os bytes de cima com a máscara); 0x800
/// +0x3b, por 2 (divisões truncadas). Outra máscara: a caixa "Error" (linha 0x945) e 0.
pub(crate) unsafe extern "thiscall" fn get_position_aptitude(this: usize, p: usize, mask: u32) -> u32 {
    let _ = this;
    let m = mask as u16 as i16 as i32;
    if !is_valid_position_mask(m) {
        show_error_at(0xa8ccd0, 0x945);
        return 0;
    }
    let f = read_usize(p + staff::PLAYER);
    let rating_of: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_PLAYER_ATTRIBUTE_RATING);
    // simplification: the original pushes the attribute byte in a register with leftover upper bytes; 0x544380 reads
    // o byte
    let rating = |o: usize| rating_of(f, *((f + o) as *const u8) as u32) as u8 as i8 as i32;
    let b = |o: usize| read_i8(f + o) as i32;
    let adapt = read_i8(p + staff::ADAPTABILITY) as i32;
    let r = match m {
        1 => {
            let x = rating(0x39);
            let y = rating(0x2a);
            (b(0x35) + x + y + adapt) / 4
        }
        2 | 4 => {
            let x = rating(0x3e);
            let y = rating(0x39);
            let z = rating(0x32);
            (x + y + z + adapt) / 4
        }
        8 | 0x10 | 0x20 => {
            let x = rating(0x37);
            let y = rating(0x32);
            (x + y + adapt + b(0x40) + b(0x3f)) / 5
        }
        0x40 => {
            let x = rating(0x33);
            let y = rating(0x27);
            (x + y + adapt + b(0x40) + b(0x36)) / 5
        }
        0x80 => (adapt + b(0x30)) / 2,
        0x200 => return (m as u32 & 0xffff_ff00) | *((p + staff::ADAPTABILITY) as *const u8) as u32,
        _ => (adapt + b(0x3b)) / 2,
    };
    r as u32
}

/// Writes the five levels (+4..+8) of a training schedule.
///
/// [pt] Os cinco níveis (+4..+8) de um programa de treino.
pub(crate) unsafe fn set_schedule_levels(this: usize, n: [u8; 5]) {
    for (k, v) in n.iter().enumerate() {
        *((this + 4 + k) as *mut u8) = *v;
    }
}

/// `TRAINING_SCHEDULE::set_type(slot, type)`: stores the schedule's slot and type and its five category levels from the
/// type's mix.
///
/// The slot (signed, as a dword) at +0, the type at +0x15 and the five category levels (+4..+8) by the type's mix (jump
/// table 0x8b4f08): 1 general (25 in the first four), 2 to 5 one category at 50 and the others at 10, 6 goalkeepers (50
/// in the fifth, 0 in the third), 0 and 7 no training (all 0); outside 0..=7 (the −1 of the human's first slot) only
/// the slot and the type. Called by set_default_schedules (0x8b3d40, 167 times at the start of the game) and by the
/// training screens.
/// Original: `TRAINING_SCHEDULE::set_type` (0x8b4e50, training_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8b4e50 } (thiscall, ret 8, training_manager.cpp): `TRAINING_SCHEDULE::set_type(vaga, tipo)`: a vaga (com sinal,
/// em dword) em +0, o tipo em +0x15 e os cinco níveis das categorias (+4..+8) pela mistura do tipo (tabela de saltos
/// 0x8b4f08): 1 geral (25 nas quatro primeiras), 2 a 5 uma categoria com 50 e as outras com 10, 6 goleiros (50 na quinta,
/// 0 na terceira), 0 e 7 sem treino (tudo 0); fora de 0..=7 (o −1 da primeira vaga do humano) só a vaga e o tipo. Pela
/// set_default_schedules (0x8b3d40, 167 vezes no começo do jogo) e pelas telas do treino.
pub(crate) unsafe extern "thiscall" fn set_type(this: usize, slot: i8, kind: i8) {
    *(this as *mut i32) = slot as i32;
    *((this + 0x15) as *mut i8) = kind;
    match kind {
        1 => {
            crate::comparison::branch("cpu_8 geral");
            set_schedule_levels(this, [0x19, 0x19, 0x19, 0x19, 0]); // cpu8: o treino geral
        }
        2 => set_schedule_levels(this, [0x32, 0xa, 0xa, 0xa, 0]),
        3 => set_schedule_levels(this, [0xa, 0xa, 0x32, 0xa, 0]),
        4 => set_schedule_levels(this, [0xa, 0xa, 0xa, 0x32, 0]),
        5 => set_schedule_levels(this, [0xa, 0x32, 0xa, 0xa, 0]),
        6 => {
            crate::comparison::branch("cpu_8 goleiros");
            set_schedule_levels(this, [0xa, 0xa, 0, 0xa, 0x32]); // cpu8: the goalkeeper training
        }
        0 | 7 => set_schedule_levels(this, [0; 5]),
        _ => {}
    }
}

/// Training schedule slot of human manager `pessoa` as 0x8b3d40 computes it; outside the 16 humans shows the box and
/// returns 0.
///
/// [pt] O programa `vaga` (0..=9) do técnico humano `pessoa` como a 0x8b3d40 o calcula sem chamar a 0x8b3930 (vagas 0 a
/// 5): this + 0x43d + 0x9b × vaga + 0x60e × humano; fora dos 16 humanos, a caixa (linha 0x68f) e 0.
pub(crate) unsafe fn human_schedule_slot(this: usize, person: usize, slot: usize) -> usize {
    let v = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(person)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    this + 0x43d + 0x9b * slot + 0x60e * v as usize
}

/// `TRAINING_MANAGER::set_default_schedules()`, called by the constructor (0x8b0ec0) in a new game (without
/// training.dat): sets up the computer's seven schedules and the ten schedules of each of the 16 human managers, then
/// gives the default names.
///
/// The computer's seven schedules (0x9b bytes each from this; type k in slot k: the computer trains through slot 1,
/// general, and slot 6, goalkeepers, according to the player's training entry) and, for each of the 16 human managers
/// (person [0xae2364] − h − 1, 0x6e-byte record in [0xae23bc]), the ten schedules: slots 0 to 5 computed here (types
/// −1, 0, 1, 2, 3, 4), 6 to 9 through 0x8b3930 (types 5, 6, 7, 7). At the end the default names (0x8b2230). Called by
/// address (0x8b4e50 and 0x8b3930 in this batch).
/// Original: `TRAINING_MANAGER::set_default_schedules` (0x8b3d40, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b3d40 } (thiscall, ret 0, training_manager.cpp): `TRAINING_MANAGER::set_default_schedules()`, pelo construtor
/// (0x8b0ec0) num jogo novo (sem training.dat): os sete programas do computador (0x9b bytes cada a partir de this; o
/// tipo k na vaga k: o computador treina pela vaga 1, geral, e pela 6, goleiros, conforme a ficha de treino do
/// jogador) e, para cada um dos 16 técnicos humanos (pessoa [0xae2364] − h − 1, registro de 0x6e bytes em
/// [0xae23bc]), os dez programas: vagas 0 a 5 calculadas aqui (tipos −1, 0, 1, 2, 3, 4), 6 a 9 pela 0x8b3930 (tipos
/// 5, 6, 7, 7). No fim os nomes padrão (0x8b2230). Chamadas pelo endereço (0x8b4e50 e 0x8b3930 nesta leva).
pub(crate) unsafe extern "thiscall" fn set_default_schedules(this: usize) {
    exe::training_schedule_set_type(this, 0, 0);
    crate::comparison::branch("cpu_8 programas padrao");
    exe::training_schedule_set_type(this + 0x9b, 1, 1); // cpu8: the general computer programme
    for k in 2..7i8 {
        exe::training_schedule_set_type(this + 0x9b * k as usize, k, k);
    }
    for h in 0..0x10i32 {
        let id = read_i32(G_STAFF_COUNT).wrapping_sub(h).wrapping_sub(1);
        let person = (*(G_STAFF_TABLE as *const usize)).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        for (vaga, t) in [(0usize, -1i8), (1, 0), (2, 1), (3, 2), (4, 3), (5, 4)] {
            exe::training_schedule_set_type(human_schedule_slot(this, person, vaga), vaga as i8, t);
        }
        // simplification: the original pushes slots 8 and 9 in a single dword with only the low byte written; both calls read the byte.
        for (vaga, t) in [(6i8, 5i8), (7, 6), (8, 7), (9, 7)] {
            exe::training_schedule_set_type(exe::training_manager_get_manager_schedule(this, person, vaga), vaga, t);
        }
    }
    exe::training_manager_set_default_schedule_names(this);
}

/// `TRAINING_MANAGER::get_manager_schedule(manager, slot)`: returns the human manager's schedule for the slot (0..=9;
/// outside that, 1): this + 0x9b × (slot + 10 × human + 7), human = [0xae2364] − id − 1. Outside the 16 human managers:
/// the box (line 0x68f) and 0. Called by 0x8b3d40 (slots 6 to 9) and by the screens.
/// Original: `TRAINING_MANAGER::get_manager_schedule` (0x8b3930, training_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8b3930 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_manager_schedule(tecnico, vaga)`: o
/// programa da vaga (0..=9; fora disso, 1) do técnico humano: this + 0x9b × (vaga + 10 × humano + 7), humano =
/// [0xae2364] − id − 1. Fora dos 16 humanos: a caixa (linha 0x68f) e 0. Pela 0x8b3d40 (vagas 6 a 9) e pelas telas.
pub(crate) unsafe extern "thiscall" fn get_manager_schedule(this: usize, manager: usize, slot: i8) -> usize {
    let v = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    let slot = if (0..0xa).contains(&slot) { slot } else { 1 };
    this.wrapping_add(0x9b * (slot as i32 + 10 * v + 7) as usize)
}

/// Divisor of the coaches' average by number of coaches (1..6): 1, 1.45, 1.7, 1.9, 2.05, 2.2.
///
/// [pt] Os seis divisores da média dos treinadores (floats imediatos da 0x8b41d0: 1; 1,45; 1,7; 1,9; 2,05; 2,2), pela
/// quantidade de treinadores (1 a 6).
pub(crate) fn coach_average_divisor(n: i8) -> f64 {
    const T: [u32; 6] = [0x3f800000, 0x3fb9999a, 0x3fd9999a, 0x3ff33333, 0x40033333, 0x400ccccd];
    f32::from_bits(T[(n - 1) as usize]) as f64
}

/// `TRAINING_MANAGER::get_coaching_factor(club, category)`, called by TRAINING_MANAGER::update_player_training
/// (0x8b1750, five times per player in each training update; the value goes in as (factor − 100) / 5 into the weight of
/// the progress roll): the coaching factor of a training category.
///
/// A null club: 0. Without a manager (+0xcf) or with a computer manager (id below [0xae2364] − 16): 100 (eax carries
/// the upper 16 bits of the manager pointer, as in the original). A human manager: his staff (0x8b4cd0, six 9-byte
/// slots: id −1 is an empty slot, the category rating at +4 + category); with n coaches with a rating, the sum (16
/// bits) divided by the nth divisor and truncated (_ftol 0x945640), × 40/7 (0x970400) and truncated; below 50, 50. Hot
/// on the computer branch: no counter there.
/// Original: `TRAINING_MANAGER::get_coaching_factor` (0x8b41d0, training_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8b41d0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_coaching_factor(clube, categoria)`,
/// pela TRAINING_MANAGER::update_player_training (0x8b1750, cinco vezes por jogador em cada atualização do treino; o
/// valor entra como (fator − 100) / 5 no peso do sorteio do progresso). Clube nulo: 0. Sem técnico (+0xcf) ou com técnico
/// do computador (id abaixo de [0xae2364] − 16): 100 (o eax leva os 16 bits altos do ponteiro do técnico, como na
/// original). Técnico humano: a comissão dele (0x8b4cd0, seis vagas de 9 bytes: id −1 é vaga vazia, a nota da categoria
/// em +4 + categoria); com n treinadores de nota, a soma (16 bits) dividida pelo n-ésimo divisor e truncada (_ftol
/// 0x945640), × 40/7 (0x970400) e truncada; abaixo de 50, 50. Quente no ramo do computador: sem contador ali.
pub(crate) unsafe extern "thiscall" fn get_coaching_factor(this: usize, club: usize, category: i8) -> u32 {
    let _ = this;
    if club == 0 {
        return 0;
    }
    let t = read_usize(club + club::MANAGER);
    if t == 0 {
        crate::comparison::branch("cpt_8 sem tecnico");
        return 100; // cpt8: sem técnico
    }
    if read_i32(t) < read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        return (t as u32 & 0xffff_0000) | 100; // cpt8: técnico do computador
    }
    let s = exe::training_manager_get_coach_assignments(this, t);
    let (mut n, mut soma) = (0i8, 0i16);
    for k in 0..6usize {
        let e = s + 9 * k;
        if read_i32(e) == -1 {
            continue;
        }
        let rating = *((e as isize + 4 + category as isize) as *const i8);
        if rating != 0 {
            n += 1;
            soma = soma.wrapping_add(rating as i16);
        }
    }
    if n != 0 {
        crate::comparison::branch("cpt_8 treinadores");
        soma = ftol(soma as f64 / coach_average_divisor(n)) as i16; // cpt8: the average of the coaches
    }
    let r = ftol(soma as f64 * read_f64_at(0x970400)); // 0x970400 = 5.714285714
    if (r as i16) < 0x32 {
        return 0x32;
    }
    r as u32
}

/// `TRAINING_MANAGER::get_coach_assignments(manager)`: returns the training staff table (six 9-byte slots) of one of
/// the 16 human managers, at this + 0x652d + 54 × ([0xae2364] − id − 1).
///
/// A null manager: the "Error" box (line 0xa20) and 0; outside the 16 human managers: the box (line 0xa2a) and 0.
/// Original: `TRAINING_MANAGER::get_coach_assignments` (0x8b4cd0, training_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8b4cd0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::get_coach_assignments(tecnico)`: a comissão
/// de treino (seis vagas de 9 bytes) de um dos 16 técnicos humanos, em this + 0x652d + 54 × ([0xae2364] − id − 1).
/// Técnico nulo: a caixa "Error" (linha 0xa20) e 0; fora dos 16 humanos: a caixa (linha 0xa2a) e 0.
pub(crate) unsafe extern "thiscall" fn get_coach_assignments(this: usize, manager: usize) -> usize {
    if manager == 0 {
        show_error_at(0xa8ccd0, 0xa20);
        return 0;
    }
    let v = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0xa2a);
        return 0;
    }
    this + 0x652d + 54 * v as usize
}

/// `TRAINING_SCHEDULE::get_training_load()`: the schedule's load from its five category levels (+4..+8, signed) with
/// fixed weights, up to 100.
///
/// trunc(+8 × 0.5) − trunc(+4 × −1.3) − trunc(+7 × −0.5) − trunc(+6 × −0.5) − trunc(+5 × −0.5) (constants 0x968968,
/// 0x96f878, 0x969540; _ftol 0x945640), in 16 bits; from 100 up, 100. Called by update_player_training (0x8b1750, the
/// human club's player), by set_player_schedule (0x8b2250) and by reassign_squad_schedules (0x8b2470); it goes to the
/// fitness sheet (Injury::set_training_load 0x62c660).
/// Original: `TRAINING_SCHEDULE::get_training_load` (0x8b4f60, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b4f60 } (thiscall, ret 0, training_manager.cpp): `TRAINING_SCHEDULE::get_training_load()`: a carga do
/// programa pelos cinco níveis (+4..+8, com sinal): trunc(+8 × 0,5) − trunc(+4 × −1,3) − trunc(+7 × −0,5) −
/// trunc(+6 × −0,5) − trunc(+5 × −0,5) (constantes 0x968968, 0x96f878, 0x969540; _ftol 0x945640), em 16 bits; de 100
/// para cima, 100. Pela update_player_training (0x8b1750, o jogador do clube humano), pela set_player_schedule
/// (0x8b2250) e pela reassign_squad_schedules (0x8b2470); vai para a ficha de forma (Injury::set_training_load 0x62c660).
pub(crate) unsafe extern "thiscall" fn get_training_load(this: usize) -> i32 {
    let level = |k: usize| *((this + k) as *const i8) as f64;
    let a = ftol(level(4) * read_f64_at(0x96f878)) as i16; // 0x96f878 = -1.3
    let mut s = ftol(level(8) * read_f64_at(0x968968)) as i16; // 0x968968 = 0.5
    s = s.wrapping_sub(a);
    s = s.wrapping_sub(ftol(level(7) * read_f64_at(0x969540)) as i16); // 0x969540 = -0.5
    s = s.wrapping_sub(ftol(level(6) * read_f64_at(0x969540)) as i16); // 0x969540 = -0.5
    s = s.wrapping_sub(ftol(level(5) * read_f64_at(0x969540)) as i16); // 0x969540 = -0.5
    if s >= 0x64 {
        return 0x64;
    }
    s as i32
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

/// `TRAINING_MANAGER::apply_training_progress(person, d)`, called by update_player_training (0x8b1750, each player on
/// each training update) and by 0x8b3640 and 0x8b37c0: adds each category's progress to the player's stored nibble,
/// moving the category attributes, then handles the trained position or side.
///
/// A null person: the "Error" box (line 0x4b3); without the sheet (+0x61): the box (line 0x4ba); an id from [0xae2364]
/// on: nothing. With the player's 6-byte entry (this +0x651d + 6 × sheet id), each category with progress `d[k]` other
/// than zero moves its step (progress_step over the stored nibble) on its sheet attributes (those from 1 to 20 by the
/// step, those from −125 to 125 by six times the step; every two steps the attributes move) and stores the nibble plus
/// `d[k]`: 3 (+4 high), 0 (+3 low), 2 (+4 low), 1 (+3 high) and 4 (+5 low).
///
/// Then the position training (0x8b4b20): with the learning entry at 15 or more, the position or side of the mask goes
/// to 20 (learnt); below that, it goes back to 1 (training stopped) if more than one position (+0xf..+0x15) or side
/// (+0x17..+0x19) at 15 or more is still left. Hot: no counter.
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
pub(crate) unsafe extern "thiscall" fn apply_training_progress(this: usize, person: usize, d: usize) {
    if person == 0 {
        show_error_at(0xa8ccd0, 0x4b3);
        return;
    }
    let f = read_usize(person + staff::PLAYER);
    if f == 0 {
        show_error_at(0xa8ccd0, 0x4ba);
        return;
    }
    if read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let e = read_usize(this + 0x651d) + 6 * read_i32(f) as usize;
    // category 3 (the high nibble of +4)
    let k = read_i8(d + 3);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 4) >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            add_clamped_1_20(f + player_sheet::CORNERS, dd);
            add_clamped_125(f + player_sheet::CROSSING, seis);
            add_clamped_125(f + player_sheet::DRIBBLING, seis);
            add_clamped_1_20(f + player_sheet::FREE_KICKS, dd);
            add_clamped_125(f + player_sheet::HEADING, seis);
            add_clamped_125(f + player_sheet::PASSING, seis);
            add_clamped_125(f + player_sheet::TACKLING, seis);
            add_clamped_1_20(f + player_sheet::TECHNIQUE, dd);
            add_clamped_125(f + player_sheet::THROW_INS, seis);
        }
        add_high_nibble(e + 4, k); // cpt9: stores the progress of category 3
    }
    // category 0 (the low nibble of +3)
    let k = read_i8(d);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 3) << 4 >> 4) as i32);
        if dd != 0 {
            add_clamped_1_20(f + player_sheet::ACCELERATION, dd); // cpt9: the first attribute of category 0
            add_clamped_1_20(f + player_sheet::AGILITY, dd);
            add_clamped_1_20(f + player_sheet::JUMPING, dd);
            add_clamped_1_20(f + player_sheet::NATURAL_FITNESS, dd);
            add_clamped_1_20(f + player_sheet::PLAYER_PACE, dd);
            add_clamped_125(f + player_sheet::REFLEXES, dd * 6);
            add_clamped_1_20(f + player_sheet::STAMINA, dd);
            add_clamped_1_20(f + player_sheet::STRENGTH, dd);
        }
        add_low_nibble(e + 3, k);
    }
    // category 2 (the low nibble of +4)
    let k = read_i8(d + 2);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 4) << 4 >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            add_clamped_125(f + player_sheet::FINISHING, seis);
            add_clamped_125(f + player_sheet::LONG_SHOTS, seis);
            add_clamped_125(f + player_sheet::PENALTIES, seis);
        }
        add_low_nibble(e + 4, k);
    }
    // category 1 (the high nibble of +3)
    let k = read_i8(d + 1);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 3) >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6; // cpt9: category 1, six times the step
            add_clamped_125(f + player_sheet::DECISIONS, seis);
            add_clamped_125(f + player_sheet::MARKING, seis);
            add_clamped_125(f + player_sheet::MOVEMENT, seis);
            add_clamped_1_20(f + player_sheet::TEAMWORK, dd);
            add_clamped_125(f + player_sheet::POSITIONING, seis);
        }
        add_high_nibble(e + 3, k);
    }
    // category 4 (the low nibble of +5)
    let k = read_i8(d + 4);
    if k != 0 {
        let dd = progress_step(k, (read_i8(e + 5) << 4 >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            add_clamped_125(f + player_sheet::HANDLING, seis);
            add_clamped_125(f + player_sheet::ONE_ON_ONES, seis);
        }
        add_low_nibble(e + 5, k);
    }
    // position training
    let mut p = 0usize;
    let n = exe::training_manager_find_position_training(this, person, &mut p);
    if n <= 0 {
        return;
    }
    let de15 = |o: usize| (read_i8(f + o) >= 0xf) as i8;
    let mut positions = de15(player_sheet::GOALKEEPER) + de15(player_sheet::SWEEPER) + de15(player_sheet::DEFENDER) + de15(player_sheet::DEFENSIVE_MIDFIELDER) + de15(player_sheet::MIDFIELDER) + de15(player_sheet::ATTACKING_MIDFIELDER) + de15(player_sheet::ATTACKER);
    let mut sides = de15(player_sheet::LEFT_SIDE) + de15(player_sheet::CENTRAL) + de15(player_sheet::RIGHT_SIDE);
    for j in 0..n as usize {
        let item = p + 8 * j;
        let field = match *((item + 4) as *const i16) as i32 {
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
            *((f + field) as *mut u8) = 0x14;
        } else if field == 0x18 || field == 0x19 || field == 0x17 {
            if sides > 1 {
                *((f + field) as *mut u8) = 1;
                sides -= 1;
            }
        } else if positions > 1 {
            *((f + field) as *mut u8) = 1;
            positions -= 1;
        }
    }
}

/// `TRAINING_MANAGER::find_position_training(person, &first)`: finds the player's first entry in the sorted position
/// training list and returns how many are his.
///
/// A null person, the "Error" box (line 0x995); a null output, the box (line 0x99c); without the sheet (+0x61), the box
/// (line 0x9a3): 0. Without flag 0x10 in +5 of the player's training entry (this +0x651d + 6 × sheet id), the position
/// training mark: 0. Otherwise the binary search (0x946464, order 0x8b4e00) of the person's id in the position training
/// list (this +0x6521, count +0x6525, 8-byte items); not found: 0; found: steps back to the person's first item, stores
/// it in the output and returns how many consecutive ones are hers.
/// Original: `TRAINING_MANAGER::find_position_training` (0x8b4b20, training_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8b4b20 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::find_position_training(pessoa, &primeira)`:
/// pessoa nula, a caixa "Error" (linha 0x995); saída nula, a caixa (linha 0x99c); sem a ficha (+0x61), a caixa (linha
/// 0x9a3): 0. Sem a marca 0x10 em +5 da entrada de treino do jogador (this +0x651d + 6 × id da ficha): 0. Senão a busca
/// binária (0x946464, ordem 0x8b4e00) do id da pessoa na lista do treino de posição (this +0x6521, conta +0x6525, itens
/// de 8 bytes); não achou: 0; achou: recua até o primeiro item da pessoa, grava-o na saída e devolve quantos seguidos são
/// dela.
pub(crate) unsafe extern "thiscall" fn find_position_training(this: usize, person: usize, out: *mut usize) -> i32 {
    if person == 0 {
        show_error_at(0xa8ccd0, 0x995);
        return 0;
    }
    if out.is_null() {
        show_error_at(0xa8ccd0, 0x99c);
        return 0;
    }
    let f = read_usize(person + staff::PLAYER);
    if f == 0 {
        show_error_at(0xa8ccd0, 0x9a3);
        return 0;
    }
    if *((read_usize(this + 0x651d) + 6 * read_i32(f) as usize + 5) as *const u8) & 0x10 == 0 {
        return 0;
    }
    let key = read_i32(person);
    let search: unsafe extern "C" fn(*const i32, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let base = read_usize(this + 0x6521);
    let mut a = search(&key, base, read_usize(this + 0x6525), 8, 0x8b4e00);
    if a == 0 {
        return 0;
    }
    while a > base {
        if read_i32(a - 8) != key {
            break;
        }
        a -= 8;
    }
    *out = a;
    let last = read_usize(this + 0x6521) + read_usize(this + 0x6525) * 8 - 8;
    let mut n = 0i32;
    while a <= last && read_i32(a) == key {
        a += 8;
        n += 1;
    }
    n
}

/// `TRAINING_MANAGER::has_fast_position_training(person)`, called by the people's daily update (game_daily_staff_update
/// 0x5ce2d0): returns 1 if some item of the person's position training (0x8b4b20) has a learning rate (+7) above 50;
/// otherwise 0.
/// Original: `TRAINING_MANAGER::has_fast_position_training` (0x8b4db0, training_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8b4db0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::has_fast_position_training(pessoa)`, pelo
/// diário das pessoas (game_daily_staff_update 0x5ce2d0): 1 se algum item do treino de posição da pessoa (0x8b4b20) tem
/// aprendizado (+7) acima de 50; senão 0.
pub(crate) unsafe extern "thiscall" fn has_fast_position_training(this: usize, person: usize) -> u32 {
    let mut p = 0usize;
    let n = exe::training_manager_find_position_training(this, person, &mut p);
    for j in 0..n.max(0) as usize {
        if read_i8(p + 8 * j + 7) > 0x32 {
            return 1;
        }
    }
    0
}

/// `TRAINING_MANAGER::advance_week_counter(&day)`, every day from the day loop (game_main_day_loop 0x5cccd0, with the
/// day of the year of the game date 0xae2c90): on a day that is a multiple of 7 (signed remainder), the training week
/// (+0x688d, 16 bits) goes up by 1; update_player_training (0x8b1750) catches up each player with the missing weeks.
/// Returns day / 7 (the caller does not read it).
/// Original: `TRAINING_MANAGER::advance_week_counter` (0x8b1730, training_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8b1730 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::advance_week_counter(&dia)`, todo dia pelo
/// laço do dia (game_main_day_loop 0x5cccd0, com o dia do ano da data do jogo 0xae2c90): no dia múltiplo de 7 (resto
/// com sinal), a semana do treino (+0x688d, 16 bits) sobe 1; a update_player_training (0x8b1750) põe em dia as semanas
/// que faltam de cada jogador. Devolve dia / 7 (o chamador não lê).
pub(crate) unsafe extern "thiscall" fn advance_week_counter(this: usize, day: *const i16) -> i32 {
    let v = *day as i32;
    let rest = v % 7; // cpt10: the 7-day week
    if rest == 0 { // cpt10: o sétimo dia
        crate::comparison::branch("cpt_10 semana");
        let s = (this + 0x688d) as *mut u16;
        *s = (*s).wrapping_add(1); // cpt10: mais uma semana
    }
    v / 7
}

// simplification: the reserve loses the item (does not run twice): original only; the copy takes the branch.

/// `TRAINING_MANAGER::init_player_entries()`, called by the constructor (0x8b0ec0) in a new game: allocates the 6-byte
/// training entry of every player sheet and puts goalkeepers on schedule 6 and the others on 1.
///
/// Allocates (calloc 0x945501) the 6-byte training entries, one per player sheet ([0xae236c] sheets of 0x46 bytes in
/// [0xae23c4], the sheet by its id +0), at this + 0x651d; without memory, the box (line 0x6e3) and it returns. In each
/// entry it clears bit 0x20 of +5 and sets the schedule slot (+0): 6 (goalkeepers) with a goalkeeping rating (+0xf,
/// signed) of 15 or more, otherwise 1 (general). Each player's first training update (0x8b1750, which only afterwards
/// redoes the slot of the computer's players) and all of the human club's training use this slot.
/// Original: `TRAINING_MANAGER::init_player_entries` (0x8b3c50, training_manager.cpp, thiscall).
///
/// [pt] { 0x8b3c50 } (thiscall, ret 0, training_manager.cpp): `TRAINING_MANAGER::init_player_entries()`, pelo construtor
/// (0x8b0ec0) num jogo novo: aloca (calloc 0x945501) as fichas de treino de 6 bytes, uma por ficha de jogador
/// ([0xae236c] fichas de 0x46 bytes em [0xae23c4], a ficha pelo id +0), em this + 0x651d; sem memória, a caixa (linha
/// 0x6e3) e volta. Em cada ficha apaga o bit 0x20 do +5 e põe a vaga do programa (+0): 6 (goleiros) com a nota de
/// goleiro (+0xf, com sinal) de 15 para cima, senão 1 (geral). A primeira atualização do treino de cada jogador
/// (0x8b1750, que só depois refaz a vaga dos jogadores do computador) e todo o treino do clube humano usam essa vaga.
pub(crate) unsafe extern "thiscall" fn init_player_entries(this: usize) {
    let p = exe::crt_calloc(read_usize(G_PLAYER_DATA_COUNT) as u32, 6);
    *((this + 0x651d) as *mut u32) = p as u32;
    if p == 0 {
        show_error_at(0xa8ccd0, 0x6e3);
        return;
    }
    let mut f = read_usize(G_PLAYER_DATA_TABLE);
    let mut i = 0i32;
    while i < read_i32(G_PLAYER_DATA_COUNT) {
        let sheet = || read_usize(this + 0x651d).wrapping_add((read_i32(f) as usize).wrapping_mul(6));
        *((sheet() + 5) as *mut u8) &= 0xdf;
        let goal = *((f + player_sheet::GOALKEEPER) as *const i8);
        if goal >= 0xf { // cpu9: goalkeeper from 15 up
            crate::comparison::branch("cpu_9 goleiro");
            *(sheet() as *mut u8) = 6; // cpu9: the goalkeepers programme
        } else {
            crate::comparison::branch("cpu_9 linha");
            *(sheet() as *mut u8) = 1; // cpu9: o programa geral
        }
        i += 1;
        f += 0x46;
    }
}

