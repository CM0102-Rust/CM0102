use super::*;

// The comparers of the routines in ../comp/squad_manager.rs.
// [pt] Os comparadores das rotinas de ../comp/squad_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `process_euro_registration`.
///
/// [pt] Lado a lado: o ano das inscrições (+0), a marca +0x6d de todas as pessoas com contrato (a tabela inteira) e o
/// [0xae1080] da 0x85aae0. Com algum clube das três copas com técnico humano (a notícia não roda duas vezes), só a
/// original.
pub(crate) unsafe extern "thiscall" fn compare_process_euro_registration(this: usize, step: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_PROCESS_EURO_REGISTRATION);
    let mut has_human = false;
    for &chave in EUROPEAN_REGISTRATION_CUPS.iter() {
        let id = read_u32(chave);
        let comp = read_u32(read_u32(G_COMP_OBJECT_TABLE) as usize + 4 * id as usize) as usize;
        if comp != 0 {
            for_each_comp_club(comp, step, |c| {
                if c != 0 && read_u32(c + club::CONTINENTAL_COMP_ID) == id && exe::news_club_has_human_manager(read_u32(G_NEWS) as usize, c) != 0 {
                    has_human = true;
                }
            });
        }
    }
    if has_human {
        return o(this, step);
    }
    let n = (read_u32(G_STAFF_COUNT) as i32).max(0) as usize;
    let reg = [(this, 2usize), (read_u32(G_STAFF_TABLE) as usize, n * staff::SIZE), (G_CONTRACT_LOOKUP_RESULT, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, step), || process_euro_registration(this, step));
    SCOREBOARD.record(igual, || {
        let base = read_u32(G_STAFF_TABLE) as usize;
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| {
            let a0 = a.0;
            a.1.iter().zip(b.1.iter()).enumerate().filter(|(_, (x, y))| x != y).map(move |(i, (x, y))| (a0 + i, *x, *y))
        }).take(12).map(|(e, x, y)| if e >= base && e < base + n * staff::SIZE { format!("pessoa {} +{:x}: {}/{}", (e - base) / 0x6e, (e - base) % 0x6e, x, y) } else { format!("{:x}: {}/{}", e, x, y) }).collect();
        format!("process_euro_registration(etapa {}): {}", step as u8, d.join(", "))
    });
}

/// Comparer for `get_squad_number`.
///
/// [pt] A devolução das duas (a rotina só lê; as chamadas de dentro são buscas).
pub(crate) unsafe extern "thiscall" fn compare_get_squad_number(this: usize, p: usize, c: usize) -> i8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i8>(IDX_GET_SQUAD_NUMBER);
    let r = o(this, p, c);
    let n = get_squad_number(this, p, c);
    SCOREBOARD.record(r == n, || format!("get_squad_number(pessoa {:x}, clube {:x}): original {} rust {}", p, c, r, n));
    r
}

/// Comparer for `club_uses_squad_numbers`.
///
/// [pt] A devolução das duas (a rotina só lê); sem clube, só a original (a caixa não roda duas vezes).
pub(crate) unsafe extern "thiscall" fn compare_club_uses_squad_numbers(this: usize, c: usize, other: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CLUB_USES_SQUAD_NUMBERS);
    if c == 0 {
        return o(this, c, other);
    }
    let r = o(this, c, other);
    let n = club_uses_squad_numbers(this, c, other);
    SCOREBOARD.record(r == n, || format!("club_uses_squad_numbers(clube {:x}, outra {:x}): original {} rust {}", c, other, r, n));
    r
}

/// Comparer for `set_squad_number`.
///
/// [pt] Lado a lado: o +0x3a dos dois contratos da pessoa (achados antes; as buscas só leem). Número fora da faixa (a caixa),
/// sem pessoa ou sem clube: só a original.
pub(crate) unsafe extern "thiscall" fn compare_set_squad_number(this: usize, p: usize, c: usize, n: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_SET_SQUAD_NUMBER);
    if p == 0 || c == 0 || !(-50..=50).contains(&(n as u8 as i8)) {
        return o(this, p, c, n);
    }
    let mut reg = Vec::new();
    for k in [exe::contract_manager_get_contract(read_u32(G_CONTRACT_MANAGER) as usize, p), exe::contract_manager_get_loan_contract(read_u32(G_CONTRACT_MANAGER) as usize, p)] {
        if k != 0 {
            reg.push((k + 0x3a, 1));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, p, c, n), || set_squad_number(this, p, c, n));
    SCOREBOARD.record(igual, || format!("set_squad_number(pessoa {:x}, clube {:x}, {}): original {:?} rust {:?}", p, c, n as u8 as i8, mo.0, mr.0));
}

/// Comparer for `allocate_squad_number` (comparison mode: see the header of motor.rs): side by side on `+0x3a` of the
/// person's two contracts (the only one the routine numbers) and on the box mark.
///
/// When no position on the player sheet is good, the original routine reads the 4-byte list of preferred numbers with
/// no value on the stack (left over from the caller): the copy uses the empty list (a value the game produces when the
/// leftover is 0xff) and the comparer leaves only the original routine.
///
/// [pt] Lado a lado: o +0x3a dos dois contratos da pessoa (a única que a rotina numera) e a marca da caixa. Quando nenhuma
/// posição da ficha é boa, a original lê a lista de preferidos de 4 bytes sem valor na pilha (sobra de quem chamou):
/// a cópia usa a lista vazia (um valor que o jogo produz quando a sobra é 0xff) e o comparador deixa só a original.
pub(crate) unsafe extern "thiscall" fn compare_allocate_squad_number(this: usize, c: usize, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ALLOCATE_SQUAD_NUMBER);
    if p == 0 || read_u32(p + staff::PLAYER) == 0 || preferred_squad_numbers(read_u32(p + staff::PLAYER) as usize).is_none() {
        return o(this, c, p);
    }
    let reg = squad_number_regions(&[p]);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, c, p), || allocate_squad_number(this, c, p));
    SCOREBOARD.record(igual, || format!("allocate_squad_number(clube {:x}, pessoa {:x}): original {:?} rust {:?}", c, p, mo.0, mr.0));
}

/// Comparer for `assign_club_squad_numbers`.
///
/// [pt] Lado a lado: o +0x3a dos contratos de quem está no elenco do clube e no do clube ligado (as únicas pessoas que a
/// rotina e a 0x85a470 numeram) e a marca da caixa.
pub(crate) unsafe extern "thiscall" fn compare_assign_club_squad_numbers(this: usize, c: usize, keep: u32) {
    let c = Club(c);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ASSIGN_CLUB_SQUAD_NUMBERS);
    let mut ps: Vec<usize> = (0..50).map(|k| read_u32(c.addr() + club::SQUAD + 4 * k) as usize).collect();
    let l = Club(exe::club_get_linked_b_team(c.addr(), (0) as *mut u32, 1));
    if !l.is_null() {
        ps.extend((0..50).map(|k| read_u32(l.addr() + club::SQUAD + 4 * k) as usize));
    }
    let reg = squad_number_regions(&ps);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, c.addr(), keep), || assign_club_squad_numbers(this, c.addr(), keep));
    SCOREBOARD.record(igual, || {
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).take(12).map(|(a, b)| format!("{:x}: {:?}/{:?}", a.0, a.1, b.1)).collect();
        format!("assign_club_squad_numbers(clube {:x}, manter {}): {}", c.addr(), keep, d.join(", "))
    });
}

/// Comparer for `init_new_game`.
///
/// [pt] Só a original (ela numera as ligas pela 0x859e20, que manda notícia ao técnico humano: não roda duas vezes); o plano
/// da cópia (as ligas, achadas antes, sem numerar) tem de ser o que a original fez (gravado pelo comparador da
/// 0x859e20) e o +0 zerado.
pub(crate) unsafe extern "thiscall" fn compare_init_new_game(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_NEW_GAME);
    let mut plan = Vec::new();
    for_each_numbered_league(|c| plan.push(c));
    SQUAD_NUMBERS_PLAN = Some(Vec::new());
    o(this);
    let done = SQUAD_NUMBERS_PLAN.take().unwrap_or_default();
    let zerado = *(this as *const u16) == 0;
    SCOREBOARD.record(done == plan && zerado, || format!("init_new_game: plano {:x?} original {:x?} ano zerado {}", plan, done, zerado));
}

/// Comparer for `assign_league_squad_numbers`.
///
/// Side by side: +0x3a of the contracts (the club one and the loan one) of everyone in the squad of each league club
/// and of the linked club, and the error box mark. If some club has a human manager without `automatico` (the request
/// news item cannot run twice), only the original runs. The club list comes from the same method +0x58 (freed
/// afterwards).
///
/// [pt] Lado a lado: o +0x3a dos contratos (o do clube e o de empréstimo) de quem está no elenco de cada clube da liga e
/// no do clube ligado, e a marca da caixa. Com algum clube de técnico humano sem `automatico` (a notícia do pedido não
/// roda duas vezes), só a original. A lista dos clubes vem do mesmo método +0x58 (solta depois).
pub(crate) unsafe extern "thiscall" fn compare_assign_league_squad_numbers(this: usize, league: usize, initial: u32, automatico: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_ASSIGN_LEAGUE_SQUAD_NUMBERS);
    if let Some(v) = (*core::ptr::addr_of_mut!(SQUAD_NUMBERS_PLAN)).as_mut() {
        v.push(league);
    }
    let clubs: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(read_u32(read_u32(league) as usize + 0x58) as usize);
    let mut list = 0usize;
    let n = clubs(league, &mut list, 0) as i16 as i32;
    let cs: Vec<usize> = (0..n.max(0) as usize).map(|i| read_u32(list + 4 * i) as usize).collect();
    if list != 0 {
        exe::crt_free(list);
    }
    if automatico == 0 && cs.iter().any(|&c| exe::news_club_has_human_manager(read_u32(G_NEWS) as usize, c) != 0) {
        return o(this, league, initial, automatico);
    }
    let mut reg = vec![(0xb67a34usize, 4usize)];
    for &c in &cs {
        let l = exe::club_get_linked_b_team(c, (0) as *mut u32, 1);
        for q in [c, l] {
            if q == 0 {
                continue;
            }
            for k in 0..50 {
                let p = read_u32(q + club::SQUAD + 4 * k) as usize;
                if p == 0 {
                    continue;
                }
                for ct in [exe::contract_manager_get_contract(read_u32(G_CONTRACT_MANAGER) as usize, p), exe::contract_manager_get_loan_contract(read_u32(G_CONTRACT_MANAGER) as usize, p)] {
                    if ct != 0 && !reg.contains(&(ct + 0x3a, 1)) {
                        reg.push((ct + 0x3a, 1));
                    }
                }
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, league, initial, automatico), || assign_league_squad_numbers(this, league, initial, automatico));
    SCOREBOARD.record(igual, || {
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).take(12).map(|(a, b)| format!("{:x}: {:?}/{:?}", a.0, a.1, b.1)).collect();
        format!("assign_league_squad_numbers(liga {:x}, {} {}): {}", league, initial, automatico, d.join(", "))
    });
}

/// Comparer for `select_euro_registered_players`.
///
/// [pt] Lado a lado: a marca de inscrição (+0x6d) de quem está no elenco (a mesma 0x541f90), o contrato achado [0xae1080]
/// e a devolução.
pub(crate) unsafe extern "thiscall" fn compare_select_euro_registered_players(this: usize, c: usize, step: u32, mark: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u8>(IDX_SELECT_EURO_REGISTERED_PLAYERS);
    let mut sq = [0usize; 50];
    exe::database_collect_club_squad(c, sq.as_mut_ptr(), 0);
    let mut reg = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    for &p in sq.iter() {
        if p != 0 && !reg.contains(&(p + staff::SQUAD_SELECTED_FOR, 1)) {
            reg.push((p + staff::SQUAD_SELECTED_FOR, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, c, step, mark), || select_euro_registered_players(this, c, step, mark));
    SCOREBOARD.record(igual && r == n, || {
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).take(12).map(|(a, b)| format!("{:x}: {:?}/{:?}", a.0, a.1, b.1)).collect();
        format!("select_euro_registered_players(clube {:x}, etapa {}, marca {}): original {} rust {} {}", c, step as u8, mark, r, n, d.join(", "))
    });
    r
}

/// Comparer for `squad_manager_save`.
///
/// [pt] Comparador da 0x859cc0: só a original.
pub(crate) unsafe extern "thiscall" fn compare_squad_manager_save(this: usize, save: usize) -> u32 {
    // simplification: writes to the save container (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SQUAD_MANAGER_SAVE)(this, save)
}
