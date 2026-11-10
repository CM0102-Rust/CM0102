use super::*;

// The comparers of the routines in ../player_regen.rs.
// [pt] Os comparadores das rotinas de ../player_regen.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `regenerate_as_youth_player`.
///
/// Only the original runs (simplification: the routine takes the player sheet out of the database, clears lists and
/// histories, allocates languages and the contract and sends a news item; it cannot run twice). The copy runs in Rust
/// mode (counters "cqg_7 ...") and is proved by the mutations and by the two Rust careers.
///
/// [pt] Comparador: só a original (simplification: a rotina tira a ficha do banco, limpa listas e históricos, aloca idiomas e o
/// contrato e manda notícia; não pode rodar duas vezes). A cópia roda no modo Rust (contadores "cqg_7 ...") e é provada
/// pelas mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn compare_regenerate_as_youth_player(this: usize, p: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REGENERATE_AS_YOUTH_PLAYER)(this, p)
}

/// Comparer for `player_setup_find_staff`.
///
/// [pt] Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn compare_player_setup_find_staff(first: usize, common: usize, second: usize, club: usize, in_squad: u32) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32) -> usize>(IDX_PLAYER_SETUP_FIND_STAFF)(first, common, second, club, in_squad);
    let n = player_setup_find_staff(first, common, second, club, in_squad);
    SCOREBOARD.record(r == n, || format!("player_setup_find_staff(clube {:x}, elenco {}) original={:x} rust={:x}", club, in_squad as u8, r, n));
    r
}

/// Comparer for `player_setup_read_quoted_token`.
///
/// [pt] Comparador: lado a lado no destino (até 0x100 bytes) e na devolução.
pub(crate) unsafe extern "C" fn compare_player_setup_read_quoted_token(source: usize, target: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(IDX_PLAYER_SETUP_READ_QUOTED_TOKEN);
    if source == 0 || target == 0 {
        return o(source, target);
    }
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&[(target, 0x100)], || o(source, target), || player_setup_read_quoted_token(source, target));
    SCOREBOARD.record(igual && r == n, || format!("player_setup_read_quoted_token({:x}) original={:x} rust={:x}", source, r, n));
    r
}

/// Comparer for `player_setup_type_from_name`.
///
/// [pt] Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn compare_player_setup_type_from_name(name: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_PLAYER_SETUP_TYPE_FROM_NAME)(name);
    let n = player_setup_type_from_name(name);
    SCOREBOARD.record(r == n, || format!("player_setup_type_from_name({:x}) original={:x} rust={:x}", name, r, n));
    r
}

/// Comparer for `process_retirements`.
pub(crate) unsafe extern "thiscall" fn compare_process_retirements(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_RETIREMENTS)(this)
}

/// Comparer for `player_regen_post_retirement_news`.
pub(crate) unsafe extern "stdcall" fn compare_player_regen_post_retirement_news(p: usize, reason: u32, anunciada: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, u32, u32)>(IDX_PLAYER_REGEN_POST_RETIREMENT_NEWS)(p, reason, anunciada)
}

/// Comparer for `player_regen_post_youth_promotion_news`.
pub(crate) unsafe extern "stdcall" fn compare_player_regen_post_youth_promotion_news(j: usize, c: usize) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize)>(IDX_PLAYER_REGEN_POST_YOUTH_PROMOTION_NEWS)(j, c)
}

/// Comparer for `player_regen_sign_for_club`.
///
/// [pt] Comparador: o contrato novo aloca e o valor sorteia: só a original, conferindo o plano (a vaga, o clube da pessoa, a
/// devolução e o +0x3a do contrato).
pub(crate) unsafe extern "stdcall" fn compare_player_regen_sign_for_club(person: usize, club: usize) -> u8 {
    let person = Staff(person);
    let club = Club(club);
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_PLAYER_REGEN_SIGN_FOR_CLUB);
    let k = first_free_squad_slot(club);
    let r = o(person.addr(), club.addr());
    if k == 0x32 {
        SCOREBOARD.record(r == 0, || format!("player_regen_sign_for_club({:x}, {:x}) sem vaga original={}", person.addr(), club.addr(), r));
        return r;
    }
    let c = exe::contract_manager_get_contract(read_i32(G_CONTRACT_MANAGER) as usize, person);
    let ok = r == 1 && (club.squad(k) as i32) == person.addr() as i32 && (person.club_job().addr() as i32) == club.addr() as i32 && (c == 0 || read_u8(c + 0x3a) == 0);
    SCOREBOARD.record(ok, || format!("player_regen_sign_for_club({:x}, {:x}) vaga {} original={}", person.addr(), club.addr(), k, r));
    r
}

/// Comparer for `schedule_injury_retirement`.
///
/// [pt] Comparador: lado a lado no estado do sorteio, na tabela (ponteiro, conta e capacidade, e as entradas até a
/// capacidade). Com a tabela cheia (conta = capacidade − 1, a 0x7a8510 realoca): só a original.
pub(crate) unsafe extern "thiscall" fn compare_schedule_injury_retirement(this: usize, person: usize, days: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i16)>(IDX_SCHEDULE_INJURY_RETIREMENT);
    let (n, cap) = (read_i32(this + player_regen_fields::RETIREMENT_COUNT), read_i32(this + player_regen_fields::RETIREMENT_CAPACITY));
    if person == 0 || n >= cap.wrapping_sub(1) || cap <= 0 {
        return o(this, person, days);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((this + player_regen_fields::RETIREMENTS, 12));
    reg.push((read_i32(this + player_regen_fields::RETIREMENTS) as usize, cap as usize * 0x1a));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person, days), || crate::rng::without_counting(|| schedule_injury_retirement(this, person, days)));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("schedule_injury_retirement({:x}, {}) diferencas em {:x?}", person, days, dif)
    });
}

/// Comparer for `player_regen_post_retirement_announcement_news`.
///
/// [pt] Comparador: a notícia vai para as caixas dos técnicos: só a original (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "stdcall" fn compare_player_regen_post_retirement_announcement_news(person: usize, reason: i8) {
    original_fn::<unsafe extern "stdcall" fn(usize, i8)>(IDX_PLAYER_REGEN_POST_RETIREMENT_ANNOUNCEMENT_NEWS)(person, reason)
}

/// Comparador: o id da ficha antes; a original acrescenta (pode realocar); confere a conta e o item novo.
pub(crate) unsafe extern "thiscall" fn compare_add_player_data_to_pool(this: usize, person: i32, nation: u32, second: u32) {
    let n = read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT);
    let id = read_usize(read_usize(read_usize(G_STAFF_TABLE).wrapping_add((person as u32).wrapping_mul(0x6e) as usize) + 0x61)) as u32;
    original_fn::<unsafe extern "thiscall" fn(usize, i32, u32, u32)>(IDX_ADD_PLAYER_DATA_TO_POOL)(this, person, nation, second);
    let item = read_usize(this + player_regen_fields::PLAYER_DATA_POOL).wrapping_add(n << 4);
    let g = [read_usize(item) as u32, read_usize(item + regen_pool_item::NATION_ID) as u32, read_usize(item + regen_pool_item::SECOND_NATION_ID) as u32];
    SCOREBOARD.record(read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == n.wrapping_add(1) && g == [id, nation, second], || {
        format!("add_player_data_to_pool({}, {}, {}) conta {} -> {}, item {:?} (id {})", person, nation, second, n, read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT), g, id)
    });
}

/// Comparer for `has_spare_non_player_record`.
///
/// [pt] Comparador: a devolução.
pub(crate) unsafe extern "thiscall" fn compare_has_spare_non_player_record(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HAS_SPARE_NON_PLAYER_RECORD)(this);
    let n = has_spare_non_player_record(this);
    SCOREBOARD.record(r == n, || format!("has_spare_non_player_record({:x}) original={:x} rust={:x}", this, r, n));
    r
}

/// Comparer for `player_regen_check_initial_retirement`.
///
/// [pt] Comparador: lado a lado pelo sorteio, [0xae1080], a saída e a data estática (a cópia sem contar), e a devolução.
pub(crate) unsafe extern "thiscall" fn compare_player_regen_check_initial_retirement(this: usize, person: usize, out: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_PLAYER_REGEN_CHECK_INITIAL_RETIREMENT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&retirement_decision_regions(out), || o(this, person, out),
        || crate::rng::without_counting(|| player_regen_check_initial_retirement(this, person, out)));
    SCOREBOARD.record(igual && r == n, || format!("player_regen_check_initial_retirement(pessoa {:x}) original={} rust={} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparador: como o da `player_regen_check_initial_retirement`.
pub(crate) unsafe extern "thiscall" fn compare_player_regen_check_retirement(this: usize, person: usize, out: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_PLAYER_REGEN_CHECK_RETIREMENT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&retirement_decision_regions(out), || o(this, person, out),
        || crate::rng::without_counting(|| player_regen_check_retirement(this, person, out)));
    SCOREBOARD.record(igual && r == n, || format!("player_regen_check_retirement(pessoa {:x}) original={} rust={} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `build_initial_retirement_list`.
pub(crate) unsafe extern "thiscall" fn compare_build_initial_retirement_list(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_INITIAL_RETIREMENT_LIST)(this)
}

/// Comparer for `find_new_retirements`.
pub(crate) unsafe extern "thiscall" fn compare_find_new_retirements(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FIND_NEW_RETIREMENTS)(this)
}

/// Comparer for `player_regen_retire_from_non_playing_staff`.
///
/// [pt] Comparador: sem rotinas com efeito (job_exit_is_memory_only), lado a lado pela pessoa (0x6e bytes) e pelos dois clubes
/// (0x245 bytes cada); senão (vaga do técnico, cancelamento de ofertas, listas do humano) só a original.
pub(crate) unsafe extern "stdcall" fn compare_player_regen_retire_from_non_playing_staff(person: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_PLAYER_REGEN_RETIRE_FROM_NON_PLAYING_STAFF);
    if !job_exit_is_memory_only(person) {
        return o(person);
    }
    let mut reg = job_exit_regions(Staff(person));
    reg.retain(|r| r.0 != 0);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(person), || player_regen_retire_from_non_playing_staff(person));
    SCOREBOARD.record(igual, || format!("player_regen_retire_from_non_playing_staff(pessoa {:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `player_regen_score_non_player_candidate`.
///
/// [pt] Comparador: lado a lado pelo estado do sorteio (a cópia sem contar), e a nota.
pub(crate) unsafe extern "stdcall" fn compare_player_regen_score_non_player_candidate(person: usize) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> i32>(IDX_PLAYER_REGEN_SCORE_NON_PLAYER_CANDIDATE);
    let reg = crate::rng::STATE_REGIONS.to_vec();
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(person),
        || crate::rng::without_counting(|| player_regen_score_non_player_candidate(person)));
    SCOREBOARD.record(igual && r == n, || format!("player_regen_score_non_player_candidate({:x}) original={} rust={} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `player_regen_compare_retirements`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_player_regen_compare_retirements(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_REGEN_COMPARE_RETIREMENTS)(a, b);
    let n = player_regen_compare_retirements(a, b);
    SCOREBOARD.record(r == n, || format!("player_regen_compare_retirements({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `player_regen_compare_queue_entries`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_player_regen_compare_queue_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_REGEN_COMPARE_QUEUE_ENTRIES)(a, b);
    let n = player_regen_compare_queue_entries(a, b);
    SCOREBOARD.record(r == n, || format!("player_regen_compare_queue_entries({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `retire_from_playing_staff`.
///
/// [pt] Comparador: lado a lado pela pessoa (+0x24..+0x3d), pelas 50 vagas do clube e da seleção e pelo aviso em
/// [0xde1f64]. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha, então a tática fica fora.
pub(crate) unsafe extern "stdcall" fn compare_retire_from_playing_staff(p: usize) {
    let p = Staff(p);
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_RETIRE_FROM_PLAYING_STAFF);
    let mut reg = vec![(p.addr() + staff::NATIONAL_JOB, 0x1a), (0xde1f64, 0x80)];
    for campo in [0x39usize, 0x24] {
        let team = Club(read_usize(p.addr() + campo));
        if !team.is_null() {
            reg.push((team.addr() + club::SQUAD, 200));
        }
    }
    let (antes_clube, antes_cargo) = (p.club_job(), (p.job_for_club() as i8));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(p.addr()), || retire_from_playing_staff(p.addr()));
    SCOREBOARD.record(igual, || format!("retire_from_playing_staff(id {}, clube {:x}, cargo {}) {:x?} x {:x?}", read_usize(p.addr()), antes_clube, antes_cargo, mo, mr));
}

/// Comparer for `pick_best_club_for_player`.
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x7ad220 sorteia), pelo clube devolvido.
pub(crate) unsafe extern "thiscall" fn compare_pick_best_club_for_player(this: usize, player: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PICK_BEST_CLUB_FOR_PLAYER);
    let reg = crate::rng::STATE_REGIONS;
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, player), || crate::rng::without_counting(|| pick_best_club_for_player(this, player)));
    SCOREBOARD.record(igual && r == n, || format!("pick_best_club_for_player(id {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(player), r, n, mo, mr));
    r
}

/// Comparer for `pick_club_from_nation_list`.
///
/// [pt] Comparador: lado a lado a partir do mesmo estado do sorteio, pelo clube devolvido e pelo cursor (+0xa) das listas
/// que podem ser percorridas (a da nação do jovem e a de [0x9cf2e4]).
pub(crate) unsafe extern "thiscall" fn compare_pick_club_from_nation_list(this: usize, player: usize) -> usize {
    let player = Staff(player);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PICK_CLUB_FROM_NATION_LIST);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    for n in [read_usize(player.nation().addr()), read_usize(KEY_NATION_ENGLAND)] {
        let l = Llist((nation_club_list(this, n)).addr());
        if !l.is_null() {
            reg.push((l.addr() + llist::CURSOR, 4));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, player.addr()), || crate::rng::without_counting(|| pick_club_from_nation_list(this, player.addr())));
    SCOREBOARD.record(igual && r == n, || format!("pick_club_from_nation_list(id {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(player.addr()), r, n, mo, mr));
    r
}

/// Comparador: a original acrescenta (pode realocar); confere a conta e o item novo.
pub(crate) unsafe extern "thiscall" fn compare_add_spare_non_player_record(this: usize, id: u32, nation: u32, second: u32) {
    let n = read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(IDX_ADD_SPARE_NON_PLAYER_RECORD)(this, id, nation, second);
    let item = read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS).wrapping_add(n << 4);
    let g = [read_usize(item) as u32, read_usize(item + regen_pool_item::NATION_ID) as u32, read_usize(item + regen_pool_item::SECOND_NATION_ID) as u32];
    SCOREBOARD.record(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) == n.wrapping_add(1) && g == [id, nation, second], || {
        format!("add_spare_non_player_record({}, {}, {}) conta {} -> {}, item {:?}", id, nation, second, n, read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT), g)
    });
}

/// Comparador: o registro copiado antes; a original acrescenta (pode realocar); confere a conta e os campos.
pub(crate) unsafe extern "thiscall" fn compare_add_retirement(this: usize, r: usize) {
    let n = read_usize(this + player_regen_fields::RETIREMENT_COUNT);
    let expected = retirement_entry_fields(r);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_RETIREMENT)(this, r);
    let item = read_usize(this + player_regen_fields::RETIREMENTS).wrapping_add(n.wrapping_mul(26));
    let g = retirement_entry_fields(item);
    SCOREBOARD.record(read_usize(this + player_regen_fields::RETIREMENT_COUNT) == n.wrapping_add(1) && g == expected, || {
        format!("add_retirement conta {} -> {}, item {:x?} (esperava {:x?})", n, read_usize(this + player_regen_fields::RETIREMENT_COUNT), g, expected)
    });
}

/// Comparer for `queue_staff_for_regen`.
///
/// [pt] Comparador: a original sorteia e realoca (não pode rodar duas vezes): o plano antes (o que entra em cada lista);
/// confere as contas e os itens (a nação sorteada só pela faixa).
pub(crate) unsafe extern "thiscall" fn compare_queue_staff_for_regen(this: usize, entry: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_QUEUE_STAFF_FOR_REGEN);
    let id = read_usize(entry) as i32;
    if id < 0 || id > read_i32(G_STAFF_COUNT) {
        return o(this, entry);
    }
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    let (n1, n2, n3) = (read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT), read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT), read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT));
    let datas = read_usize(entry + 4) as u32;
    let np = person.non_player();
    let jp = person.player();
    let nation = person.nation();
    let reserve = (!np.is_null()).then(|| (read_usize(np.addr()) as u32, (!nation.is_null()).then(|| read_usize(nation.addr()) as u32), read_usize(person.addr()) as u32));
    let bench = (!jp.is_null()).then(|| (read_usize(jp.addr()) as u32, read_usize(nation.addr()) as u32, read_usize(person.addr()) as u32));
    o(this, entry);
    let queue = read_usize(this + player_regen_fields::REGEN_QUEUE).wrapping_add(n1.wrapping_mul(12));
    let mut ok = read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT) == n1 + 1 && read_usize(queue) as i32 == id && read_usize(queue + regen_queue_entry::REGEN_DATE) as u32 == datas;
    ok &= read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) == n2 + reserve.is_some() as usize;
    if let Some((rid, rnac, rp)) = reserve {
        let i = read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS).wrapping_add(n2 << 4);
        let gn = read_usize(i + regen_pool_item::NATION_ID) as u32;
        ok &= read_usize(i) as u32 == rid && read_usize(i + regen_pool_item::STAFF_ID) as u32 == rp
            && rnac.map_or(gn < read_i32(G_NATION_COUNT).max(1) as u32, |v| gn == v);
    }
    ok &= read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == n3 + bench.is_some() as usize;
    if let Some((bid, bnac, bp)) = bench {
        let i = read_usize(this + player_regen_fields::PLAYER_DATA_POOL).wrapping_add(n3 << 4);
        ok &= read_usize(i) as u32 == bid && read_usize(i + regen_pool_item::NATION_ID) as u32 == bnac && read_usize(i + regen_pool_item::STAFF_ID) as u32 == bp;
    }
    SCOREBOARD.record(ok, || {
        format!("queue_staff_for_regen({}) contas {} {} {} -> {} {} {}, reserva {:?}, banco {:?}", id, n1, n2, n3,
            read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT), read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT), read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT), reserve, bench)
    });
}

/// Comparer for `remove_retirement`.
pub(crate) unsafe extern "thiscall" fn compare_remove_retirement(this: usize, key: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_RETIREMENT);
    let n = read_i32(this + player_regen_fields::RETIREMENT_COUNT).max(0) as usize;
    let reg = [(this + player_regen_fields::RETIREMENTS, 8), (read_u32(this + player_regen_fields::RETIREMENTS) as usize, n * 26)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, key), || remove_retirement(this, key));
    SCOREBOARD.record(igual, || format!("remove_retirement {:x?} x {:x?}", mo, mr));
}

/// Comparer for `player_regen_check_unemployed_retirement`.
///
/// [pt] Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio e pela saída (26 bytes), a cópia em sem_contar.
pub(crate) unsafe extern "stdcall" fn compare_player_regen_check_unemployed_retirement(person: usize, out: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_PLAYER_REGEN_CHECK_UNEMPLOYED_RETIREMENT);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((out, 0x1a));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, out),
        || crate::rng::without_counting(|| player_regen_check_unemployed_retirement(person, out)));
    SCOREBOARD.record(igual && r == n, || format!("player_regen_check_unemployed_retirement({:x}) original={} rust={} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `regen_queue_entry_assign`.
pub(crate) unsafe extern "thiscall" fn compare_regen_queue_entry_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_REGEN_QUEUE_ENTRY_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0xc)], || o(this, source), || regen_queue_entry_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("regen_queue_entry_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `player_regen_pick_staff_for_non_player_data`.
///
/// [pt] Comparador: a 0x7a7150 sorteia (0x90cfc0) a cada pessoa; lado a lado pelo estado do sorteio, a cópia sem contar.
pub(crate) unsafe extern "thiscall" fn compare_player_regen_pick_staff_for_non_player_data(this: usize, nation: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PLAYER_REGEN_PICK_STAFF_FOR_NON_PLAYER_DATA);
    let reg = crate::rng::STATE_REGIONS.to_vec();
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, nation),
        || crate::rng::without_counting(|| player_regen_pick_staff_for_non_player_data(this, nation)));
    SCOREBOARD.record(igual && r == n, || format!("player_regen_pick_staff_for_non_player_data({:x}) original={:x} rust={:x} {:x?} x {:x?}", nation, r, n, mo, mr));
    r
}

/// Comparer for `remove_spare_non_player_record`.
pub(crate) unsafe extern "thiscall" fn compare_remove_spare_non_player_record(regen: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_REMOVE_SPARE_NON_PLAYER_RECORD);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&list16_regions(regen, 0x1c, 0x20), || o(regen, id), || remove_spare_non_player_record(regen, id));
    SCOREBOARD.record(igual, || format!("remove_spare_non_player_record({}) {:x?} x {:x?}", id, mo, mr));
}

/// Comparer for `remove_player_data_from_pool`.
pub(crate) unsafe extern "thiscall" fn compare_remove_player_data_from_pool(regen: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_REMOVE_PLAYER_DATA_FROM_POOL);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&list16_regions(regen, 0x3c, 0x40), || o(regen, id), || remove_player_data_from_pool(regen, id));
    SCOREBOARD.record(igual, || format!("remove_player_data_from_pool({}) {:x?} x {:x?}", id, mo, mr));
}

/// Comparer for `find_retirement_entry`.
pub(crate) unsafe extern "thiscall" fn compare_find_retirement_entry(regen: usize, id: *const i32, by_injury: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, u32) -> usize>(IDX_FIND_RETIREMENT_ENTRY)(regen, id, by_injury);
    let n = find_retirement_entry(regen, id, by_injury);
    SCOREBOARD.record(r == n, || format!("find_retirement_entry({}, {}) original={:#x} rust={:#x}", *id, by_injury, r, n));
    r
}

/// Comparer for `player_regen_score_club_for_player`.
pub(crate) unsafe extern "stdcall" fn compare_player_regen_score_club_for_player(player: usize, club: usize, squad: i8) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, i8) -> i32>(IDX_PLAYER_REGEN_SCORE_CLUB_FOR_PLAYER);
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS,
        || o(player, club, squad), || crate::rng::without_counting(|| player_regen_score_club_for_player(player, club, squad)));
    SCOREBOARD.record(igual && r == n, || format!("player_regen_score_club_for_player(elenco {}) original={} rust={} sorteio igual={}", squad, r, n, igual));
    r
}

/// Comparer for `retire_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_retire_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RETIRE_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1a)], || o(this, source), || retire_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("retire_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `player_setup_injury_from_name`.
///
/// [pt] Comparador: a devolução (o al; só lê).
pub(crate) unsafe extern "C" fn compare_player_setup_injury_from_name(name: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_PLAYER_SETUP_INJURY_FROM_NAME)(name);
    let n = player_setup_injury_from_name(name);
    SCOREBOARD.record(r as u8 == n as u8, || format!("player_setup_injury_from_name({:x}) original={:x} rust={:x}", name, r as u8, n as u8));
    r
}

/// Comparer for `build_nation_club_lists`.
///
/// [pt] Comparador: a original aloca as listas: o plano antes (os ids dos clubes de cada nação, na ordem); a original roda;
/// confere os itens de cada lista.
pub(crate) unsafe extern "thiscall" fn compare_build_nation_club_lists(this: usize) {
    let n = read_i32(G_NATION_COUNT).max(0) as usize;
    let mut plan: Vec<Vec<usize>> = vec![Vec::new(); n];
    let base = read_usize(G_CLUB_TABLE);
    for j in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let club = base + j * club::SIZE;
        let nation = injury_youth_list_nation(Club(club));
        if nation != 0 && (read_i32(nation) as usize) < n {
            plan[read_i32(nation) as usize].push(read_usize(club));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_NATION_CLUB_LISTS)(this);
    let wrong = (0..n).find(|&k| {
        let l = read_usize(read_usize(this) + 4 * k);
        let ids: Vec<usize> = if l != 0 { list_items_capped(l).into_iter().map(|e| read_usize(e)).collect() } else { Vec::new() };
        l != 0 && ids != plan[k]
    });
    SCOREBOARD.record(wrong.is_none(), || format!("build_nation_club_lists: a lista da nação {:?} difere do plano", wrong));
}

/// Comparer for `process_regen_queue` (comparison mode: see the header of motor.rs): `0x7abac0` (the young regen)
/// allocates the contract and sends news, and the manager reserves recreate persons found along the way, so only the
/// original routine runs, checking the queue plan.
///
/// With no expired entry and no reserves to recreate, both run side by side on the header (nothing changes). With
/// expired entries (the regen may add new entries to the queue), reserves (more than 2) or an invalid id: only the
/// original routine.
///
/// [pt] O comparador: a 0x7abac0 (o regen jovem) aloca o contrato e manda notícia e as reservas de técnico refazem pessoas
/// achadas pelo caminho: só a original, conferindo o plano da fila. Sem entrada vencida e sem reservas a refazer, os dois
/// lado a lado no cabeçalho (nada muda). Com vencidas (o regen pode pôr entradas novas na fila), reservas (mais de 2)
/// ou id inválido: só a original.
pub(crate) unsafe extern "thiscall" fn compare_process_regen_queue(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_REGEN_QUEUE);
    if read_i32(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) > 2 {
        crate::comparison::branch("cpw_10 reserves only in the original");
        return o(this);
    }
    let c0 = read_i32(this + player_regen_fields::REGEN_QUEUE_COUNT);
    let base = read_usize(this + player_regen_fields::REGEN_QUEUE);
    let mut k = 0i32;
    if read_i32(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) != 0 {
        while k < c0 {
            let e = base + k as usize * 0xc;
            if !is_regen_entry_due(e) {
                break;
            }
            let id = read_i32(e);
            if id < 0 || id > read_i32(G_STAFF_COUNT) {
                return o(this); // simplification: the error box only in the original
            }
            k += 1;
        }
    }
    if k == 0 {
        let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x44)], || o(this), || process_regen_queue(this));
        SCOREBOARD.record(igual, || format!("process_regen_queue({:x}) sem vencidas {:x?} x {:x?}", this, mo, mr));
        return;
    }
    // the regen (0x7abac0) may put new entries in the queue (on save load, 4 expired out of 10 and the count dropped only 1)
    crate::comparison::branch("cpw_10 expired only in the original");
    o(this);
}

/// Comparer for `take_spare_non_player_record`.
pub(crate) unsafe extern "thiscall" fn compare_take_spare_non_player_record(this: usize, id: *mut u32, nation: *mut u32, second: *mut u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u32) -> u32>(IDX_TAKE_SPARE_NON_PLAYER_RECORD)(this, id, nation, second)
}

/// Comparer for `record_bits`.
pub(crate) unsafe extern "stdcall" fn compare_record_bits(p: *const i32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(*const i32) -> u8>(IDX_RECORD_BITS)(p);
    let n = record_bits(p);
    SCOREBOARD.record(r == n, || format!("record_bits({:p}) original={} rust={}", p, r, n));
    r
}
