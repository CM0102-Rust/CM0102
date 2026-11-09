use super::*;

/// Is the club run by a human manager who decides for it? The manager (`pessoa`, or the club's manager +0xcf) must be
/// the club's and one of the 16 human managers; unless `sem_tatica`, managers letting the computer decide (+0x9ace or
/// +0x9a01) are excluded; `forcar` or `qualquer` then means yes, else the national-team or reserve-team control flag
/// decides. If not and the club is a second national team, repeats with its nation's main national team. Returns 1 or
/// 0.
///
/// [pt] { 0x600a50 } (human_manager.cpp): o clube e dirigido por um tecnico humano que decide por ele?
/// O tecnico (`pessoa`, ou o do clube em +0xcf) tem de ser o do clube e um dos 16 humanos. Sem
/// `sem_tatica`: ficam de fora os que tem 0x9ace (0x5fe040) ou +0x9a01 no registro. Com `forcar` ou
/// `qualquer`: sim. Senao, selecao: 0x5fd6b0; clube: 0x5fd480. Se nao, e o clube e do segundo
/// conjunto de selecoes (0x5409b0), repete com a selecao principal da nacao dele (0x53b3d0),
/// ja sem `qualquer`. Clube nulo: a original (caixa de erro, zera 0xb67a34, 0).
pub(crate) unsafe extern "C" fn human_club(club: usize, without_tactic: u32, any: u8, person: usize, force: u32) -> u32 {
    let h = *(G_HUMAN_MANAGER as *const usize);
    let (mut club, mut any, mut person) = (club, any, person);
    loop {
        if club == 0 {
            return original_fn::<unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32>(IDX_HUMAN_CLUB)(0, without_tactic, any, person, force);
        }
        if person == 0 {
            person = *((club + club::MANAGER) as *const usize);
        }
        let manager = *((club + club::MANAGER) as *const usize);
        if manager != 0 && person == manager && *(person as *const i32) >= (*(G_STAFF_COUNT as *const i32)).wrapping_sub(16) {
            let tactic = without_tactic == 0
                && (exe::human_manager_is_computer_controlled(h, person) != 0 || matches!(human_manager_record_checked(h, person), Ok(r) if *((r + 0x9a01) as *const u32) != 0));
            if !tactic {
                if force != 0 || any != 0 {
                    return 1;
                }
                let yes = if exe::database_is_national_team(club) != 0 { exe::human_manager_controls_all_national_teams(h, person, core::ptr::null_mut()) as u32 } else { exe::human_manager_controls_reserve_team(h, person) };
                if yes != 0 {
                    return 1;
                }
            }
        }
        if exe::club_is_national_team(club) == 0 {
            return 0;
        }
        any = 0;
        club = exe::database_get_national_team(*((club + club::NATION) as *const usize));
    }
}

/// The person (or the current human manager) and his human manager record; None in the original's error cases.
///
/// [pt] A pessoa (ou, sem ela, o tecnico humano atual) e o registro dela; `None` nos casos de erro do
/// original (caixa de mensagem), que ficam com ele.
pub(crate) unsafe fn person_and_human_record(this: usize, person: usize) -> Option<(usize, usize)> {
    let p = if person != 0 { person } else { *((0xb74c26 + 768 * *(0xb77c7a as *const i16) as isize) as *const usize) };
    if p == 0 {
        return None;
    }
    human_manager_record_checked(this, p).ok().map(|r| (p, r))
}

/// Returns the +0x9ace field (computer-controlled flag) of the human manager record for the person. Without a person or
/// out of range, falls back to the original (error box, returns 0).
///
/// [pt] { 0x5fe040 } (human_manager.cpp): campo +0x9ace do registro do tecnico humano da pessoa.
/// Sem pessoa nenhuma ou fora da faixa: a original (caixa de erro, zera 0xb67a34, 0).
pub(crate) unsafe extern "thiscall" fn manager_9ace(this: usize, person: usize) -> u32 {
    match person_and_human_record(this, person) {
        Some((_, r)) => *((r + 0x9ace) as *const u32),
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MANAGER_9ACE)(this, person),
    }
}

/// Human manager: tells whether byte +0x9a06 of the human manager record of `pessoa` is set [inference: an on/off
/// option]. If the person's club (+0x24) belongs to a nation whose +0x88 is below 400, returns false and sets `*saida`
/// = 1; `*saida` otherwise starts at 0. Invalid person: falls back to the original (error box).
///
/// [pt] { 0x5fd6b0 } (human_manager.cpp): byte +0x9a06 do registro do tecnico humano e diferente de
/// zero? Se a pessoa tem clube (+0x24) de nacao com +0x88 abaixo de 400: falso, e `saida` = 1.
/// `saida` (se houver) comeca em 0. Erros como em `manager_9ace`.
pub(crate) unsafe extern "thiscall" fn manager_9a06(this: usize, person: usize, out: *mut u32) -> u8 {
    if !out.is_null() {
        *out = 0;
    }
    match person_and_human_record(this, person) {
        Some((p, r)) => {
            let club = Club(*((p + 0x24) as *const usize));
            if !club.is_null() && *((club.nation().addr() + 0x88) as *const i32) < 400 {
                if !out.is_null() {
                    *out = 1;
                }
                return 0;
            }
            (*((r + 0x9a06) as *const u8) != 0) as u8
        }
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8>(IDX_MANAGER_9A06)(this, person, out),
    }
}

/// The human manager record of the person (`human_manager.cpp`).
///
/// The 16 human managers are the last 16 persons (index = id − number of persons in `0xae2364` + 16), 42,735-byte
/// records starting at `+0` of the object, count at `+4`. Without a person, it uses the current manager's (table
/// `0xb74c26`, 768 bytes per manager, index in the word `0xb77c7a`). `None`: no person at all, or index out of range
/// (the bool says which).
///
/// [pt] Registro do tecnico humano da pessoa (human_manager.cpp): os 16 tecnicos humanos sao as 16
/// ultimas pessoas (indice = id - quantidade de pessoas em 0xae2364 + 16), registros de 42.735
/// bytes a partir de +0 do objeto, quantidade em +4. Sem pessoa, usa a do tecnico atual (tabela
/// 0xb74c26, 768 bytes por tecnico, indice na palavra 0xb77c7a). `None`: sem pessoa nenhuma, ou
/// indice fora da faixa (o bool diz qual).
pub(crate) unsafe fn human_manager_record_checked(this: usize, person: usize) -> Result<usize, bool> {
    let mut person = person;
    if person == 0 {
        let current = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        person = *((0xb74c26 + 768 * current) as *const usize);
        if person == 0 {
            return Err(true);
        }
    }
    let i = (*(person as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(16);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Err(false);
    }
    Ok((*(this as *const usize)).wrapping_add((42735 * i) as usize))
}

/// Returns the dword at +0x9a01 of the human manager record of `pessoa` (0 out of range) [inference: a human manager
/// setting]. With no person at all, clears the mouse-button global (the original shows an error box) and returns 0.
///
/// [pt] { 0x5fd210 } (human_manager.cpp): campo +0x9a01 do registro do tecnico humano da pessoa (0 fora
/// da faixa). Sem pessoa nenhuma: o original mostra uma caixa de erro e zera 0xb67a34.
pub(crate) unsafe extern "thiscall" fn manager_9a01(this: usize, person: usize) -> u32 {
    match human_manager_record_checked(this, person) {
        Ok(r) => *((r + 0x9a01) as *const u32),
        Err(sem_pessoa) => {
            if sem_pessoa {
                *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
            }
            0
        }
    }
}

/// Is the byte +0x9a05 of the person's human-manager record set? Returns 1 or 0; with no valid person, clears 0xb67a34
/// and returns 0 (the original also shows an error box). Meaning of the flag unknown.
///
/// [pt] { 0x5fd480 } (human_manager.cpp): byte +0x9a05 do registro do tecnico humano da pessoa e diferente
/// de zero? Sem pessoa nenhuma ou fora da faixa: o original mostra uma caixa de erro e zera 0xb67a34.
pub(crate) unsafe extern "thiscall" fn manager_9a05(this: usize, person: usize) -> u32 {
    match human_manager_record_checked(this, person) {
        Ok(r) => (*((r + 0x9a05) as *const u8) != 0) as u32,
        Err(_) => {
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
            0
        }
    }
}

/// Converts a tactical cell code (high byte = row 1/2/4/8..., low byte = column) into its bit in the formation mask,
/// using a table copied from the game; unknown codes give 0.
///
/// [pt] { 0x5b5a60 } (formation.cpp): bit da mascara de formacao pelo codigo da casa tatica (byte alto:
/// linha 1, 2, 4, 8; byte baixo: coluna). Tabela tirada do proprio jogo; 0x802 e 0x804 dao o mesmo
/// bit no original. Codigo desconhecido: 0.
pub(crate) unsafe extern "stdcall" fn square_bit(code: i16) -> u32 {
    match code {
        0x1 => 0x1, 0x82 => 0x2, 0x84 => 0x40, 0x88 => 0x800, 0x90 => 0x10000, 0xa0 => 0x200000, 0xc0 => 0x4000000,
        0x102 => 0x4, 0x104 => 0x80, 0x108 => 0x1000, 0x110 => 0x20000, 0x120 => 0x400000, 0x140 => 0x8000000,
        0x202 => 0x8, 0x204 => 0x100, 0x208 => 0x2000, 0x210 => 0x40000, 0x220 => 0x800000, 0x240 => 0x10000000,
        0x402 => 0x10, 0x404 => 0x200, 0x408 => 0x4000, 0x410 => 0x80000, 0x420 => 0x1000000, 0x440 => 0x20000000,
        0x802 | 0x804 => 0x400, 0x808 => 0x8000, 0x810 => 0x100000, 0x820 => 0x2000000, 0x840 => 0x40000000,
        _ => 0,
    }
}

/// Formation: splits a position mask `m` into its zone (first set bit among 1, 2, 4 ... 0x40, written to `setor`) and
/// its side (0x80, 0x100, 0x200 (also for bit 1), 0x400, 0x800, written to `lado`). Returns 1, or 0 if either part is
/// missing (clearing the mouse-button global; the original shows an error box).
///
/// [pt] { 0x5b73a0 } (formation.cpp): separa a mascara em setor (o primeiro de 1, 2, 4 ... 0x40, em
/// `setor`) e lado (0x80, 0x100, 0x200 (tambem com o bit 1), 0x400, 0x800, nessa ordem, em `lado`).
/// Sem uma das partes: o original mostra uma caixa de erro, zera 0xb67a34 e devolve 0.
pub(crate) unsafe extern "C" fn split_mask(m: u32, setor: *mut u16, side: *mut u16) -> u32 {
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
    *side = v;
    1
}

/// The given manager or, if null, the current human ([0xb74c26 + 0x300 x slot]).
///
/// [pt] O técnico pedido ou, nulo, o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
pub(crate) unsafe fn manager_or_current_human(manager: usize) -> usize {
    if manager != 0 {
        return manager;
    }
    read_usize((read_i16(0xb77c7a) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize)
}

/// Index of the human manager record (id - [0xae2364] + 16) if within 0..count-1, else None (original error box).
///
/// [pt] O índice do registro do técnico humano no gerenciador `this` (id − [0xae2364] + 16), se estiver entre 0 e a conta
/// ([this+4]) − 1; fora disso a original mostra a caixa "Error".
pub(crate) unsafe fn human_manager_human_manager_index(this: usize, manager: usize) -> Option<usize> {
    let k = read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if k < 0 || k > read_i32(this + 4).wrapping_sub(1) {
        return None;
    }
    Some(k as usize)
}

/// Human manager record k (0xa6ef bytes each from [this]).
///
/// [pt] O registro do técnico humano `k` (0xa6ef bytes cada, a partir de [this]).
pub(crate) unsafe fn human_manager_record_at(this: usize, k: usize) -> usize {
    read_usize(this).wrapping_add(k.wrapping_mul(0xa6ef))
}

/// Returns the tactic block (`0x1ecd` bytes) of the active slot of the human manager's record.
///
/// `HUMAN_MANAGER::get_active_tactic(tecnico)` (manager), called by the team selection (`0x6db500`), the formation
/// (`0x6e0230`), the match day (`0x6b4360`), the club tactic (`0x896660`) and the screens. The tactic block of the
/// active slot (signed byte at `+0x9a69` of the record) of the manager (null: the current human). No manager or out of
/// range: the original routine runs ("Error" box, 0).
/// Original: `HUMAN_MANAGER::get_active_tactic` (0x5fbd00, human_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x5fbd00 } (thiscall, ret 4, human_manager.cpp): `HUMAN_MANAGER::get_active_tactic(tecnico)`, pela escalação
/// (0x6db500), pela formação (0x6e0230), pelo dia de jogo (0x6b4360), pela tática do clube (0x896660) e pelas telas. O
/// bloco de tática (0x1ecd bytes) da vaga ativa (byte com sinal em +0x9a69 do registro) do técnico (nulo: o humano da
/// vez). Sem técnico ou fora da conta: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn get_active_tactic(this: usize, manager: usize) -> usize {
    let t = manager_or_current_human(manager);
    let Some(k) = (if t != 0 { human_manager_human_manager_index(this, t) } else { None }) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_ACTIVE_TACTIC)(this, manager); // message box: original only
    };
    let slot = read_u8(human_manager_record_at(this, k) + 0x9a69) as i8 as i32;
    read_usize(this).wrapping_add(slot.wrapping_mul(0x1ecd) as usize).wrapping_add(k.wrapping_mul(0xa6ef))
}

/// Selects the human manager's active tactic slot: 4 for the second national team, 3 with reserve in a national team, 2
/// in a national team, 1 with reserve, 0 for the first team.
///
/// `HUMAN_MANAGER::select_tactic_slot(reserva, clube, tecnico)` (reserve, club, manager), by the same callers as
/// `0x5fbd00`. Null manager: the current human (with none, nothing); null club: the manager's (`+0x39`). The active
/// slot (`+0x9a69` of the record): 4 in the second national team (`0x5409b0`), 3 with `reserva` in a national team
/// (`0x53b900`), 2 in a national team, 1 with `reserva`, 0 in the first team. Out of range: the original routine runs
/// ("Error" box).
/// Original: `HUMAN_MANAGER::select_tactic_slot` (0x5fbe20, human_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5fbe20 } (thiscall, ret 0xc, human_manager.cpp): `HUMAN_MANAGER::select_tactic_slot(reserva, clube, tecnico)`,
/// pelos mesmos chamadores da 0x5fbd00. Técnico nulo: o humano da vez (sem nenhum, nada); clube nulo: o do técnico
/// (+0x39). A vaga ativa (+0x9a69 do registro): 4 na segunda seleção (0x5409b0), 3 com `reserva` numa seleção
/// (0x53b900), 2 numa seleção, 1 com `reserva`, 0 no time principal. Fora da conta: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn select_tactic_slot(this: usize, reserve: u32, club: usize, manager: usize) {
    let t = Staff(manager_or_current_human(manager));
    if t.is_null() {
        return;
    }
    let Some(k) = human_manager_human_manager_index(this, t.addr()) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize)>(IDX_SELECT_TACTIC_SLOT)(this, reserve, club, manager); // message box: original only
    };
    let club = if club != 0 { club } else { t.club_job() };
    let slot: u8 = if club != 0 && exe::club_is_national_team(club) != 0 {
        4
    } else if reserve != 0 && club != 0 && exe::database_is_national_team(club) != 0 {
        3
    } else if club != 0 && exe::database_is_national_team(club) != 0 {
        2 // with `reserva` and outside the national team the original asks again (0x53b900 twice)
    } else if reserve != 0 {
        1
    } else {
        0
    };
    *((human_manager_record_at(this, k) + 0x9a69) as *mut u8) = slot;
}

/// The watched match (79 bytes at [this] + 0x9ad4) of human manager i.
///
/// [pt] A partida assistida do técnico humano `i` (79 bytes em [this] + 0x9ad4, 0xa6ef bytes por humano).
pub(crate) unsafe fn human_manager_watched_match_record(this: usize, i: usize) -> usize {
    read_usize(this).wrapping_add(0x9ad4).wrapping_add(i.wrapping_mul(0xa6ef))
}

/// Clears (79 bytes) the watched fixture of each active human manager who may no longer watch it. Returns 1.
///
/// `HUMAN_MANAGER::clear_invalid_watched_fixtures()`: for each of the 16 human managers (the last 16 of `[0xae23bc]`)
/// that is active (`+0x5f` ≠ 0) and can no longer watch his watched fixture (`0x5fe700`), it zeroes its 79 bytes.
/// Original: `HUMAN_MANAGER::clear_invalid_watched_fixtures` (0x5fe8d0, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fe8d0 } (thiscall, human_manager.cpp): `HUMAN_MANAGER::clear_invalid_watched_fixtures()`: para cada um dos 16
/// técnicos humanos (os últimos 16 de [0xae23bc]) ativo (+0x5f ≠ 0) que não pode mais ver a partida assistida dele
/// (0x5fe700), zera os 79 bytes dela. Devolve 1.
pub(crate) unsafe extern "thiscall" fn clear_invalid_watched_fixtures(this: usize) -> u32 {
    for i in 0..0x10usize {
        let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(G_STAFF_COUNT).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(staff::SIZE)));
        if p.classification() == 0 {
            continue;
        }
        let f = human_manager_watched_match_record(this, i);
        if exe::human_manager_can_watch_fixture(this, f, p) == 0 {
            core::ptr::write_bytes(f as *mut u8, 0, 79); // the match is removed
        }
    }
    1
}

/// People in the scouting requests (0x11-byte entries at [this+0x5c]) of club id.
///
/// [pt] Os pedidos de observação (0x11 bytes em [this+0x5c], [this+0x64] deles) do clube `id` (+0): as pessoas (+4).
pub(crate) unsafe fn scouting_targets_of_club(this: usize, id: u32) -> Vec<usize> {
    let mut v = Vec::new();
    let mut k = 0i32;
    while k < read_usize(this + 0x64) as i32 {
        let e = read_usize(this + 0x5c).wrapping_add(k as usize * 0x11);
        if read_usize(e) as u32 == id { // the club’s request
            v.push(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(e + 4).wrapping_mul(staff::SIZE)));
        }
        k += 1;
    }
    v
}

/// Tells whether any of the active human managers chose to watch the fixture.
///
/// 1 if any of the 16 active human managers (110-byte records in `[0xae23bc]`, from `[0xae2364]` − 16, `+0x5f` not
/// zero) chose to watch `partida` (`0x5fe290`); otherwise 0. Null fixture: the original routine runs (error box and 0).
/// Original: `HUMAN_MANAGER::is_fixture_watched` (0x5fe420, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fe420 } (thiscall, ret 4, human_manager.cpp): 1 se algum dos 16 técnicos humanos ativos
/// (registros de 110 bytes em [0xae23bc], a partir de [0xae2364] − 16, +0x5f não nulo) escolheu
/// assistir a `partida` (0x5fe290); senão 0. Partida nula: a original (caixa de erro e 0).
pub(crate) unsafe extern "thiscall" fn is_fixture_watched(this: usize, partida: usize) -> u32 {
    if partida == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_FIXTURE_WATCHED)(this, partida);
    }
    for k in 0..16i32 {
        let i = k.wrapping_add(*(G_STAFF_COUNT as *const i32)).wrapping_sub(0x10);
        let h = Staff((*(G_STAFF_TABLE as *const usize)).wrapping_add((i as isize * 110) as usize));
        if h.classification() != 0 && exe::human_manager_is_watching_fixture(this, partida, h) != 0 {
            return 1;
        }
    }
    0 // nobody watches
}

/// Index of human manager (or current one) for `has_nickname`: None without human, Some(None) out of range.
///
/// [pt] O índice (0..[this+4]) do técnico humano `humano` (ou o da vez, [0xb74c26 + 0x300 × [0xb77c7a]])
/// para a 0x600740: None se não há humano; Some(None) fora da faixa (caso de erro da original).
pub(crate) unsafe fn human_index_for_nickname(this: usize, human: usize) -> Option<Option<usize>> {
    let h = if human != 0 {
        human
    } else {
        *((0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize) as *const usize)
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

/// Tells whether the human manager (or the current one) gave the person a nickname, by binary search in his list.
///
/// 1 if human manager `humano` (0 = the current one) gave a nickname to the person with id [`pessoa`]: binary search
/// (`0x946464` with `0x601a10`, items of `0x1e` bytes) in list `+0xa6e3` (count at `+0xa6eb`) of his record (`0xa6ef`
/// bytes in `[this]`). Null person or human, or empty list: 0. Human out of range: the original routine runs (error box
/// and 0).
/// Original: `HUMAN_MANAGER::has_nickname` (0x600740, human_manager.cpp, thiscall).
///
/// [pt] { 0x600740 } (thiscall, ret 8, human_manager.cpp): 1 se o técnico humano `humano` (0 = o da vez)
/// deu apelido à pessoa de id [`pessoa`]: busca binária (0x946464 com 0x601a10, itens de 0x1e bytes)
/// na lista +0xa6e3 (quantos em +0xa6eb) do registro dele (0xa6ef bytes em [this]). Pessoa ou humano
/// nulos, ou lista vazia: 0. Humano fora da faixa: a original (caixa de erro e 0).
pub(crate) unsafe extern "thiscall" fn has_nickname(this: usize, person: usize, human: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    let i = match human_index_for_nickname(this, human) {
        None => return 0,
        Some(None) => return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_HAS_NICKNAME)(this, person, human),
        Some(Some(i)) => i,
    };
    let r = (*(this as *const usize)).wrapping_add(i.wrapping_mul(0xa6ef));
    let list = core::ptr::read_unaligned((r + 0xa6e3) as *const usize);
    // the key: the id and a 0 byte (0x601a10 only compares the id)
    let key = [*(person as *const u32), 0u32];
    let found = if list == 0 { 0 } else { (exe::crt_bsearch((key.as_ptr()) as usize, list, core::ptr::read_unaligned((r + 0xa6eb) as *const u32), 0x1e, 0x601a10) != 0) as u32 };
    found // apelido
}

/// The 42735-byte record of the human manager `pessoa`, or of the current human when null; None when the original
/// routine shows its error box.
///
/// With a null `pessoa`, it is the network player of the current human's slot, [0xb74c26 + 0x300 × slot]. The humans
/// are the last 16 persons: index = id − ([0xae2364] − 16), valid from 0 to [manager+4] − 1.
///
/// [pt] O registro de 42735 bytes do técnico humano `pessoa` (nula: o jogador de rede da vaga do humano da
/// vez, [0xb74c26 + 0x300 × vaga]). Os humanos são as 16 últimas pessoas: índice = id − ([0xae2364] − 16),
/// válido de 0 a [gerente+4] − 1. `None` quando a original dá a caixa de erro.
pub(crate) unsafe fn human_manager_record(manager: usize, person: usize) -> Option<usize> {
    let person = if person != 0 { person } else { *((0xb74c26 + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize) };
    if person == 0 {
        return None;
    }
    let d = (*(person as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
    if d < 0 || d > *((manager + 4) as *const i32) - 1 {
        return None;
    }
    Some((*(manager as *const usize) as isize + d as isize * 42735) as usize)
}

/// the clock stamp stored when the human manager went on holiday
/// Original: `HUMAN_MANAGER::get_holiday_stamp` (0x5fdc30, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fdc30 } (thiscall, ret 4, human_manager.cpp): a hora (relógio em ms) guardada quando o técnico
/// humano saiu de férias (+0x9a65 do registro). Fora da faixa: a original (caixa de erro, 0).
pub(crate) unsafe extern "thiscall" fn get_holiday_stamp(manager: usize, person: usize) -> u32 {
    match human_manager_record(manager, person) {
        Some(r) => core::ptr::read_unaligned((r + 0x9a65) as *const u32),
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_GET_HOLIDAY_STAMP)(manager, person),
    }
}

/// the address of the human manager's saved line-up (16 pairs)
/// Original: `HUMAN_MANAGER::get_saved_lineup` (0x5fed50, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fed50 } (thiscall, ret 4, human_manager.cpp): o endereço da escalação guardada do técnico
/// humano (16 pares jogador e posição, +0x9b28 do registro). Fora da faixa: a original (caixa de erro, 0).
pub(crate) unsafe extern "thiscall" fn get_saved_lineup(manager: usize, person: usize) -> usize {
    match human_manager_record(manager, person) {
        Some(r) => r + 0x9b28,
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_SAVED_LINEUP)(manager, person),
    }
}

/// the club's assistant or, without one, the highest-reputation staff member
/// Original: `human_manager_get_stand_in_manager` (0x600e90, human_manager.cpp, cdecl).
///
/// [pt] { 0x600e90 } (cdecl, human_manager.cpp): quem responde pelo técnico do clube: o auxiliar (+0xd3)
/// se houver; senão, dos cinco da comissão (+0x19f) com dados (+0x69), o de maior reputação (+0xa,
/// ficando o primeiro no empate); 0 se nenhum.
pub(crate) unsafe extern "C" fn human_manager_get_stand_in_manager(club: usize) -> usize {
    let club = Club(club);
    let auxiliar = core::ptr::read_unaligned((club.addr() + club::ASSISTANT_MANAGER) as *const usize);
    if auxiliar != 0 {
        return auxiliar;
    }
    let injury_word_0c = |p: usize| core::ptr::read_unaligned((core::ptr::read_unaligned((p + staff::NON_PLAYER) as *const usize) + 0xa) as *const i16);
    let mut best = 0usize;
    for k in 0..5 {
        let c = Staff(core::ptr::read_unaligned((club.addr() + club::COACHES + k * 4) as *const usize));
        if c.is_null() || core::ptr::read_unaligned((c.addr() + staff::NON_PLAYER) as *const usize) == 0 {
            continue;
        }
        if best == 0 || injury_word_0c(c.addr()) > injury_word_0c(best) {
            best = c.addr();
        }
    }
    best
}

/// Tells whether the fixture is the one the human manager chose to watch.
///
/// Whether `partida` is the one the human chose to watch: his record (`+0x9ad4..`) with the same competition (`+0x3f`),
/// dates (`+0x2a`, `+0x28`), club (`+0`), `+0x34` and both sides (`+0xc`, `+0x10`). Null fixture or human out of range:
/// the original routine runs (error box and 0).
/// Original: `HUMAN_MANAGER::is_watching_fixture` (0x5fe290, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fe290 } (thiscall, ret 8, human_manager.cpp): se a `partida` é a que o humano escolheu
/// assistir: o registro dele (+0x9ad4..) com a mesma competição (+0x3f), datas (+0x2a, +0x28), clube
/// (+0), +0x34 e os dois lados (+0xc, +0x10). Partida nula ou humano fora da faixa: a original (caixa
/// de erro e 0).
pub(crate) unsafe extern "thiscall" fn is_watching_fixture(this: usize, partida: usize, human: usize) -> u32 {
    let r = match formation_watched_match_record(this, human) {
        Err(()) => return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_WATCHING_FIXTURE)(this, partida, human),
        Ok(_) if partida == 0 => return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_WATCHING_FIXTURE)(this, partida, human),
        Ok(0) => return 0,
        Ok(r) => r,
    };
    let same = read_u8(partida + fixture::TIME_SLICE) == read_u8(r + 0x9b13)
        && read_u16(partida + fixture::DAY) == read_u16(r + 0x9afe)
        && read_u16(partida + fixture::YEAR) == read_u16(r + 0x9afc)
        && read_u32(partida + 0) == read_u32(r + 0x9ad4)
        && read_u16(partida + fixture::ROUND_NUMBER) == read_u16(r + 0x9b08)
        && read_u32(partida + fixture::HOME_CLUB_ID) == read_u32(r + 0x9ae0)
        && read_u32(partida + fixture::AWAY_CLUB_ID) == read_u32(r + 0x9ae4);
    if same { 1 } else { 0 } // another match
}

/// Tells whether the human may watch the fixture: 1 when the fixture has a negative `+0x43` and neither it nor the
/// fixtures of that day (in each key-nation group) have a club of the human; otherwise 0.
///
/// `HUMAN_MANAGER::can_watch_fixture(partida, humano)` (fixture, human): the human (or, if null, the one of the current
/// screen `[0xb74c26 + 0x300 × [0xb77c7a]]`) can watch the fixture if it has a negative `+0x43`, neither of the two
/// clubs (`+0x1c`, `+0x20`, with flags `+0x40`, `+0x41`) is his (`0x600a50`) and, in each group of the key nations
/// (`[0xb64738]`), no fixture in the list of its day (date `+0x28`/`+0x2a`, time slot `+0x3f`; `0x5ae6a0`) has a club
/// of his.
///
/// Null fixture: the "Error" box (`0x707`) and 0. Called by the clean-up of watched fixtures (`0x5fe8d0`) and by the
/// choice of the fixture (`0x5fe500`).
/// Original: `HUMAN_MANAGER::can_watch_fixture` (0x5fe700, human_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x5fe700 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::can_watch_fixture(partida, humano)`: o humano
/// (ou, nulo, o da tela atual [0xb74c26 + 0x300 × [0xb77c7a]]) pode ver a partida se ela tem +0x43 negativo, nenhum dos
/// dois clubes (+0x1c, +0x20, com as marcas +0x40, +0x41) é dele (0x600a50) e, em cada grupo das nações-chave
/// ([0xb64738]), nenhuma partida da lista do dia dela (data +0x28/+0x2a, fatia +0x3f; 0x5ae6a0) tem um clube dele: 1.
/// Senão 0. Partida nula: a caixa "Error" (0x707) e 0. Pela limpeza das partidas assistidas (0x5fe8d0) e pela escolha
/// da partida (0x5fe500).
pub(crate) unsafe extern "thiscall" fn can_watch_fixture(this: usize, partida: usize, human: usize) -> u32 {
    if partida == 0 {
        show_error_at(0x9cf11c, 0x707);
        return 0;
    }
    let mut human = human;
    if human == 0 {
        human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize).wrapping_mul(0x300)));
        if human == 0 {
            return 0;
        }
    }
    if read_u8(partida + fixture::HOME_GOALS) as i8 > -1 {
        return 0;
    }
    let dele = |p: usize| manager_manager_is_human_club(read_usize(p + 0x1c), read_u8(p + 0x40), human) != 0
        || manager_manager_is_human_club(read_usize(p + 0x20), read_u8(p + 0x41), human) != 0;
    if dele(partida) {
        return 0;
    }
    let list_of: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_FIX_MAN_GET_FIXTURE_LIST_FOR_DATE);
    let mut group = 0u8;
    while (group as i8) < read_u8(G_KEY_NATION_ACTIVE_COUNT) as i8 {
        let mut d = [0u8; 8];
        let data = exe::new_date(d.as_mut_ptr() as usize, read_i16(partida + fixture::DAY) as u16 as u32, read_i16(partida + fixture::YEAR) as u16 as u32);
        // simplification: the original pushes the group as a whole word (the upper bytes are stack leftover); 0x5ae6a0
        // passes the type (char) on and 0x5abf10 reads only the byte.
        let list = list_of(data, read_u8(partida + fixture::TIME_SLICE) as u32, group as u32);
        if list != 0 {
            write_u32(list + 0xa, read_usize(list + 2) as u32);
            let mut p = exe::llist_get_next(list);
            while p != 0 {
                if dele(p) {
                    return 0;
                }
                p = exe::llist_get_next(list);
            }
        }
        group = group.wrapping_add(1);
    }
    1
}

/// Removes the person's nickname from every human manager: in each one, finds the nickname (30 bytes), pulls the next
/// ones up, blanks the last one and lowers the count; then sorts the last manager's array.
///
/// `HUMAN_MANAGER::remove_nickname_from_all(id)`: in each non-null human manager (`[this]`, `[this+4]` records of
/// `0xa6ef` bytes; counter in a signed byte), with no nickname array (`+0xa6e3`) it exits for good; otherwise it
/// searches (bsearch with `0x601a10`, which reads only the id at `+0`; the key has byte `+4` zeroed and the rest
/// undefined, the port uses zero) for the nickname of person `*id` among the `[+0xa6eb]` 30-byte ones; if found: pulls
/// the following ones up one place, the last one gets id −1 and `+4` = 0 and the count goes down.
///
/// At the end it re-sorts (qsort) only the array of the last manager seen (with no managers, `id` stands in for the
/// manager, as in the original).
/// Original: `HUMAN_MANAGER::remove_nickname_from_all` (0x5fff30, human_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x5fff30 } (thiscall, ret 4, human_manager.cpp): `HUMAN_MANAGER::remove_nickname_from_all(id)`: em cada técnico
/// humano ([this], [this+4] registros de 0xa6ef bytes; contador em byte com sinal) não nulo, sem vetor de apelidos
/// (+0xa6e3) sai de vez; senão busca (bsearch com 0x601a10, que lê só o id em +0; a chave tem o byte +4 zerado e o resto
/// sem valor, a cópia usa zero) o apelido da pessoa `*id` entre os [+0xa6eb] de 30 bytes; achou: puxa os seguintes um
/// lugar, o último fica com id −1 e +4 = 0 e a contagem desce. No fim reordena (qsort) só o vetor do último técnico
/// visto (sem técnicos, o `id` faz as vezes do técnico, como no original).
pub(crate) unsafe extern "thiscall" fn remove_nickname_from_all(this: usize, id: usize) {
    if id == 0 {
        return;
    }
    let mut last = id;
    if read_u32(this + 4) as i32 > 0 {
        let mut c = 0i8;
        loop {
            let g = (read_u32(this) as usize).wrapping_add((c as i32 as usize).wrapping_mul(0xa6ef));
            last = g;
            if g != 0 {
                if read_u32(g + 0xa6e3) == 0 {
                    return;
                }
                let mut key = [0u8; 0x1e];
                key[..4].copy_from_slice(&read_u32(id).to_le_bytes());
                let array = read_u32(g + 0xa6e3) as usize;
                let s = crt_bsearch(key.as_ptr() as usize, array, read_u32(g + 0xa6eb), 0x1e, 0x601a10);
                if s != 0 {
                    let mut k = ((s.wrapping_sub(array) as u32 as u64 * 0x88888889) >> 36) as i32;
                    while k < read_u32(g + 0xa6eb) as i32 - 1 {
                        let r = (read_u32(g + 0xa6e3) as usize).wrapping_add(k as usize * 30);
                        core::ptr::copy((r + 30) as *const u8, r as *mut u8, 30); // o seguinte um lugar acima
                        k += 1;
                    }
                    let n = read_u32(g + 0xa6eb) as usize;
                    let end = (read_u32(g + 0xa6e3) as usize).wrapping_add(n.wrapping_mul(30));
                    *((end - 0x1a) as *mut u8) = 0;
                    core::ptr::write_unaligned((end - 0x1e) as *mut u32, u32::MAX);
                    core::ptr::write_unaligned((g + 0xa6eb) as *mut u32, (n as u32).wrapping_sub(1));
                }
            }
            c = c.wrapping_add(1);
            if c as i32 >= read_u32(this + 4) as i32 {
                break;
            }
        }
    }
    media_crt_qsort(read_u32(last + 0xa6e3) as usize, read_u32(last + 0xa6eb) as i32, 0x1e, 0x601a10);
}

/// Tells whether the current human manager runs the club (or, for second national teams, the nation's main team).
///
/// Does the current human manager run `clube` (or, for second national teams, the nation's main national team,
/// `0x53b3d0`)? He must be the club's manager (`+0xcf`); without `qualquer` (any), he must not have flags `+0x9ace`
/// (`0x5fe040`) or `+0x9a01` in the human record `[0xb63c98]`; and then it is enough that `forcar` (force) is set, that
/// the club is a national team (`0x53b900`) or flag `+0x9a05` (`0x5fd480`). Null club: the original routine runs (error
/// box and 0).
/// Original: `human_runs_club` (0x600c90, human_manager.cpp, cdecl).
///
/// [pt] { 0x600c90 } (cdecl, human_manager.cpp): o técnico humano da vez manda no `clube` (ou, nas
/// segundas seleções, na seleção principal da nação, 0x53b3d0)? Precisa ser o técnico do clube
/// (+0xcf); sem `qualquer`, não pode ter as marcas +0x9ace (0x5fe040) nem +0x9a01 no registro de
/// humanos [0xb63c98]; e então basta `forcar`, o clube ser seleção (0x53b900) ou a marca +0x9a05
/// (0x5fd480). Clube nulo: a original (caixa de erro e 0).
pub(crate) unsafe extern "C" fn human_runs_club(club: usize, any: u32, force: i8) -> u32 {
    let mut c = club;
    loop {
        if c == 0 {
            return original_fn::<unsafe extern "C" fn(usize, u32, i8) -> u32>(IDX_HUMAN_RUNS_CLUB)(0, any, force);
        }
        let team = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        let human = core::ptr::read_unaligned((0xb74c26 + team * 0x300) as usize as *const usize);
        let manager = core::ptr::read_unaligned((c + club::MANAGER) as *const usize);
        'proximo: {
            if manager == 0 || human != manager {
                break 'proximo;
            }
            let humanos = *(G_HUMAN_MANAGER as *const usize);
            if any == 0 {
                if exe::human_manager_is_computer_controlled(humanos, human) != 0 {
                    break 'proximo;
                }
                let i = (*(human as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
                if i >= 0
                    && i <= (*((humanos + 4) as *const i32)).wrapping_sub(1)
                    && core::ptr::read_unaligned((*(humanos as *const usize) + i as usize * 0xa6ef + 0x9a01) as *const u32) != 0
                {
                    break 'proximo;
                }
            }
            if force != 0 || exe::database_is_national_team(c) != 0 || exe::human_manager_controls_reserve_team(humanos, human) != 0 {
                crate::comparison::branch("human runs the club");
                return 1;
            }
        }
        if exe::club_is_national_team(c) == 0 {
            return 0;
        }
        c = exe::database_get_national_team(core::ptr::read_unaligned((c + club::NATION) as *const usize));
    }
}

/// Stores `valor` at `+0x9a65` of the human manager record (`0xa6ef` bytes). Returns 1.
///
/// Writes `valor` (value) at `+0x9a65` of the record of human manager `tecnico` (without one, the current one at
/// `[0xb74c26 + 0x300 × [0xb77c7a]]`) among the humans `this` (`[this]`, `0xa6ef` bytes each; the index is id −
/// `[0xae2364]` + 16, up to `[this+4]` − 1). No manager or index out of range: the original routine runs (error box).
/// Original: `set_human_field` (0x5fdd30, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fdd30 } (thiscall, ret 8, human_manager.cpp): grava `valor` em +0x9a65 do registro do técnico
/// humano `tecnico` (sem ele, o da vez em [0xb74c26 + 0x300 × [0xb77c7a]]) entre os humanos `this`
/// ([this], 0xa6ef bytes cada; o índice é o id − [0xae2364] + 16, até [this+4] − 1). Devolve 1.
/// Sem técnico ou índice fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn set_human_field(t: usize, value: u32, manager: *const i32) -> u32 {
    let mut p = manager;
    if p.is_null() {
        let team = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + team * 0x300) as usize as *const *const i32);
    }
    if p.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(IDX_SET_HUMAN_FIELD)(t, value, manager);
    }
    let i = (*p).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((t + 4) as *const i32)).wrapping_sub(1) {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(IDX_SET_HUMAN_FIELD)(t, value, manager);
    }
    crate::comparison::branch("human data saved");
    core::ptr::write_unaligned((*(t as *const usize) + i as usize * 0xa6ef + 0x9a65) as *mut u32, value);
    1
}

/// Turns the human manager's holiday on or off, keeping the count of humans on holiday; when turning it on, answers
/// pending news.
///
/// `HUMAN_MANAGER::set_on_holiday(férias, técnico)` (holiday, manager): with no manager, the current human (`[0xb74c26
/// + 0x300 × [0xb77c7a]]`); none: the box of line `0x41c`. The manager's index (id − `[0xae2364]` + 16) outside
/// 0..`[this+4]`: the box of line `0x428`. The holiday flag (`+0x9a01` of the `0xa6ef`-byte record in `[this]`) becomes
/// `férias`, with the count of humans on holiday (`+0xc`) going up when turned on and down when turned off; when turned
/// on, the manager's pending news with the club (`+0x39`) and with the national team (`+0x24`) are answered
/// (`0x7874b0`); with the day in `[0xb67a50]`, the day's interest (`0x6b5040`).
/// Original: `set_on_holiday` (0x5fd2f0, human_manager.cpp, thiscall).
///
/// [pt] { 0x5fd2f0 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::set_on_holiday(férias, técnico)`: sem técnico, o
/// humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); nenhum: a caixa da linha 0x41c. O índice do técnico (id − [0xae2364]
/// + 16) fora de 0..[this+4]: a caixa da linha 0x428. A marca de férias (+0x9a01 do registro de 0xa6ef bytes em [this])
/// passa a `férias`, com a conta dos humanos de férias (+0xc) subindo ao ligar e descendo ao desligar; ao ligar, as
/// notícias pendentes do técnico com o clube (+0x39) e com a seleção (+0x24) são respondidas (0x7874b0); com o dia em
/// [0xb67a50], o interesse do dia (0x6b5040).
pub(crate) unsafe extern "thiscall" fn set_on_holiday(this: usize, holiday: u32, manager: usize) {
    let mut t = manager;
    if t == 0 {
        t = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * (*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize));
        if t == 0 {
            show_error_at(0x9cf11c, 0x41c);
            return;
        }
    }
    let i = (read_usize(t) as i32).wrapping_sub(read_usize(G_STAFF_COUNT) as i32).wrapping_add(0x10);
    if i < 0 || i > (read_usize(this + 4) as i32).wrapping_sub(1) {
        show_error_at(0x9cf11c, 0x428);
        return;
    }
    let mark = read_usize(this).wrapping_add((i as usize).wrapping_mul(0xa6ef)).wrapping_add(0x9a01);
    let count = (this + 0xc) as *mut i32;
    let before = read_usize(mark);
    if holiday != 0 {
        if before == 0 {
            *count = (*count).wrapping_add(1);
        }
    } else if before != 0 {
        *count = (*count).wrapping_sub(1);
    }
    core::ptr::write_unaligned(mark as *mut u32, holiday);
    crate::comparison::branch("cpy_13 human on holiday");
    if holiday != 0 {
        if read_usize(t + staff::CLUB_JOB) != 0 {
            exe::news_auto_respond_all(read_usize(G_NEWS), t, read_usize(t + staff::CLUB_JOB));
        }
        if read_usize(t + staff::NATIONAL_JOB) != 0 {
            exe::news_auto_respond_all(read_usize(G_NEWS), t, read_usize(t + staff::NATIONAL_JOB));
        }
    }
    let day = read_usize(G_MATCH_DAY);
    if day != 0 {
        exe::match_day_refresh_human_interest_flags(day);
    }
}

