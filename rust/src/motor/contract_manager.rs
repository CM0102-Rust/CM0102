use super::*;

/// Finds the person's contract with a club in the contract list (`this`, 80-byte records): the person has up to two
/// contract indices in the staff state table (79 bytes per person, +0 and +4) and the one whose club id (+4) matches
/// wins; if neither matches, retries with the linked B team when the club is the A team. Returns the contract record or
/// 0.
///
/// [pt] { 0x4cf5f0 } (ret 8): o contrato (registros de 80 bytes em [this]) da pessoa com o clube. A
/// pessoa tem ate dois indices (tabela 0xae2d0c, 79 bytes por pessoa, +0 e +4); vale o registro
/// cujo +4 e o id do clube. Sem nenhum, tenta o clube parceiro (0x540a50) quando ele e o time A.
pub(crate) unsafe extern "thiscall" fn contract_with_club(this: usize, person: usize, club: usize) -> usize {
    if person == 0 || club == 0 {
        return 0;
    }
    let id = *(person as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let e = (*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(79));
    let i0 = *(e as *const i32);
    if i0 < 0 {
        return 0;
    }
    let base = *(this as *const usize);
    let r0 = base.wrapping_add((i0 as usize).wrapping_mul(80));
    if r0 == 0 {
        return 0;
    }
    let id_de = |q: usize| *((q + 4) as *const u32);
    let id_club = *(club as *const u32);
    if id_de(r0) == id_club {
        return r0;
    }
    let i1 = *((e + 4) as *const i32);
    let mut r1 = 0usize;
    if i1 >= 0 {
        r1 = base.wrapping_add((i1 as usize).wrapping_mul(80));
        if r1 != 0 && id_de(r1) == id_club {
            return r1;
        }
    }
    let mut target = club;
    let mut eh_a = person as u32; // the original reuses the argument’s slot
    let pc = exe::club_get_linked_b_team(club, &mut eh_a, 1);
    if pc != 0 && eh_a == 0 {
        target = pc;
    }
    let id_target = *(target as *const u32);
    if id_de(r0) == id_target {
        return r0;
    }
    if r1 != 0 && id_de(r1) == id_target {
        return r1;
    }
    0
}

/// Returns the low nibble of byte +0x4f of the contract between person `pessoa` and club `clube` (found by the contract
/// manager at [0xae1050]) minus 10 [inference: a contract type/level]. Null club or no contract: 0; national team (id
/// >= clubs - 2 x nations) or invalid person: the club pointer with its low byte cleared.
///
/// [pt] { 0x53ce90 }: (byte +0x4f do contrato da pessoa com o clube (0x4cf5f0 sobre [0xae1050]) & 0xf)
/// - 10, para clube que nao e selecao (id abaixo de clubes - 2 x nacoes) e pessoa valida. Senao o
/// byte baixo zerado (o resto do registro fica como estava: o clube, ou 0).
pub(crate) unsafe extern "C" fn contract_level(person: usize, club: usize) -> u32 {
    if club == 0 {
        return 0;
    }
    let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(2 * *(G_NATION_COUNT as *const i32));
    if *(club as *const i32) >= limit || *(person as *const i32) >= *(G_STAFF_COUNT as *const i32) {
        return club as u32 & !0xff;
    }
    let r = Contract(exe::contract_manager_get_contract_for_club(*(G_CONTRACT_MANAGER as *const usize), person, club));
    if r.is_null() {
        return 0;
    }
    ((r.squad_status() & 0xf) as u32).wrapping_sub(10)
}

/// Address of the 79-byte staff state record of person id in G_STAFF_STATE_TABLE (+0 = main contract index).
///
/// [pt] O registro de 79 bytes da pessoa `id` em [0xae2d0c] (+0 o índice do contrato no vetor).
pub(crate) unsafe fn contract_manager_staff_state_record(id: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(0x4f))
}

/// Copies a 0x50-byte contract record field by field from o to d like the original (dates via the date assign operator,
/// bit fields merged).
///
/// [pt] A cópia do contrato de 0x50 bytes de `o` para `d`, campo a campo como a original (as datas +0x25 e +0x2d pela
/// 0x417c10; os campos de bits de +0x35 e o nibble de +0x4f, mesclados em passos, acabam iguais à origem).
pub(crate) unsafe fn copy_contract(d: Contract, o: usize) {
    let o = Contract(o);
    for k in (0..0x1c).step_by(4) {
        write_usize(d.addr() + k, read_usize(o.addr() + k));
    }
    for k in 0x1c..0x21 {
        write_u8(d.addr() + k, read_u8(o.addr() + k));
    }
    d.set_release_fee(o.release_fee());
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(d.addr() + contract::DATE_STARTED, (tmp.as_mut_ptr()) as usize, o.addr() + contract::DATE_STARTED);
    exe::date_operator_3d(d.addr() + contract::CONTRACT_EXPIRES, (tmp.as_mut_ptr()) as usize, o.addr() + contract::CONTRACT_EXPIRES);
    let s = o.contract_type();
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ d.contract_type()) & 0x3f) ^ d.contract_type();
    let al = ((((s << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl;
    d.set_contract_type(((al ^ s) & 0x7f) ^ s);
    d.set_previous_wage(o.previous_wage());
    d.set_squad_number(o.squad_number());
    d.set_club_appearances(((o.club_appearances() as u16)) as i16);
    for k in [0x3dusize, 0x3e, 0x3f] {
        write_u8(d.addr() + k, read_u8(o.addr() + k));
    }
    d.set_flags(((o.flags() as usize)) as u32);
    d.set_morale(((o.morale() as u8)) as i8);
    d.set_grievances(((o.grievances() as usize)) as u32);
    d.set_leaving_on_bosman(o.leaving_on_bosman());
    d.set_transfer_arranged_for(o.transfer_arranged_for());
    d.set_transfer_status(o.transfer_status());
    let s = o.squad_status();
    let a = ((d.squad_status() ^ s) & 0xf) ^ d.squad_status();
    d.set_squad_status(((a ^ s) & 0xf) ^ s);
}

/// Sets the contract type byte (+0x3d) from the job nibble (+0x4f >> 4): 1→75, 2→50, 3→40, 4/5/6/8→0, others untouched.
///
/// [pt] O tipo do contrato (+0x3d) pelo cargo (nibble alto de +0x4f): 1 → 75, 2 → 50, 3 → 40, 4, 5, 6 e 8 → 0; os outros
/// não mexem (a tabela de saltos da original).
pub(crate) unsafe fn set_contract_type_from_job(c: Contract) {
    match c.squad_status() >> 4 {
        1 => c.set_happiness((0x4b) as i8),
        2 => c.set_happiness((0x32) as i8),
        3 => c.set_happiness((0x28) as i8),
        4 | 5 | 6 | 8 => c.set_happiness((0) as i8),
        _ => {}
    }
}

/// `CONTRACT_MANAGER::sign_contract(contract, job)` (no SEH), called by the transfer completion (`exe::transfer_manager_move_player_to_club`), by the
/// accepted offer (`complete_accepted_contract_offer`) and by the staff screen: puts the person in the contract club, handles loans, and stores
/// the contract with its value, payroll and club change.
///
/// A person without a club (+0x39) outside a loan (+0x35 = 5): added to the contract club (`database_add_staff_to_club` with today); on
/// failure: 0. A new job outside a loan with no room at the current club (`database_club_has_room_for_job`): the cancelled-offer news
/// (`exe::contract_manager_post_offer_cancelled_no_space_news`) and 0. A human current club (`exe::human_manager_is_human_club` with 1, 1): the scouts' lists (`sync_scout_lists`) and `update_coach_assignments`. On a loan: a
/// squad above 50, the cancelled-trial news (`exe::contract_manager_post_trial_cancelled_news`) and 0; otherwise `exe::virtual_staff_remove_virtual_players`, added with job 11, the end by the
/// length (`exe::date_days_since` and `exe::date_add_days` from today), job 8 in the high nibble of +0x4f, the start today and `staff_set_squad_flags`. The end
/// is at least 4 weeks after the start (`exe::date_add_weeks`); the end also goes to the person's +0x46.
///
/// The place in the array (+0 pointer, +4 capacity, +8 count): the index from the 79-byte record; with a contract at
/// the same club, the old terms of +0x3a..+0x4f stay (+0x49 0, +0x4a −1) and, in the nation of [0x9cf434] for a non-EU
/// player (`database_is_staff_eu_national`), 100 days (`exe::date_sub_weeks`) after the person's date +0x3e, the second nationality (+0x1e) becomes that
/// nation's (with the 1st from [0x9cf260] or on a draw of 3); at another club, the type by the job, +0x49/+0x4a,
/// `reset_morale_state` and +0x3b zeroed. Without a record: the first free slot (+0 = −1) gets the contract (type by the job,
/// `reset_morale_state`), the record, the payroll (`update_wage_bill`) and 1; otherwise at the end (grows by 100, realloc `exe::crt_realloc` or malloc
/// `exe::crt_malloc`; without memory, box 0x1637).
///
/// Then: the value (+0xc) in the person's +0x4e, the morale (+0x14, `staff_adjust_morale`) in type 2 with a player sheet, the
/// payroll (`update_wage_bill`), and with a new job the departure from the current club (`database_remove_staff_from_club`) and the arrival at the
/// contract club (`database_add_staff_to_club`); on failure: the box (0x1672, 0x167a) and 0.
/// Original: `CONTRACT_MANAGER::sign_contract` (0x4cf720, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cf720 } (thiscall, ret 8, contract_manager.cpp, sem SEH): `CONTRACT_MANAGER::sign_contract(contrato, cargo)`,
/// pela conclusão da transferência (0x8c9720), pela proposta aceita (0x8c38b0) e pela tela do pessoal. Pessoa sem clube
/// (+0x39) fora do empréstimo (+0x35 = 5): inclui no clube do contrato (0x53bd10 com hoje); falhou: 0. Cargo novo fora do
/// empréstimo sem vaga no clube atual (0x53b9a0): a notícia da proposta cancelada (0x4d64e0) e 0. Clube atual humano
/// (0x600a50 com 1, 1): as listas dos olheiros (0x7f7270) e 0x8b2880. No empréstimo: elenco acima de 50, a notícia do
/// teste cancelado (0x4dade0) e 0; senão 0x90d860, inclui com o cargo 11, o fim pela duração (0x54cdb0 e 0x54c5b0 a
/// partir de hoje), o cargo 8 no nibble alto de +0x4f, o início hoje e 0x540980. O fim no mínimo 4 semanas depois do
/// início (0x54c770); o fim também em +0x46 da pessoa. O lugar no vetor (+0 ponteiro, +4 capacidade, +8 conta): o
/// índice do registro de 79 bytes; com contrato no mesmo clube, os termos antigos de +0x3a..+0x4f ficam (+0x49 0, +0x4a
/// −1) e, na nação de [0x9cf434] para quem não é comunitário (0x53d740), daqui a 100 dias (0x54cab0) depois da data +0x3e
/// da pessoa, a segunda nacionalidade (+0x1e) vira a dessa nação (com a 1ª de [0x9cf260] ou num sorteio de 3); noutro
/// clube, o tipo pelo cargo, +0x49/+0x4a, 0x866ee0 e +0x3b zerado. Sem registro: a primeira vaga livre (+0 = −1) recebe
/// o contrato (tipo pelo cargo, 0x866ee0), o registro, a folha (0x598370) e 1; senão no fim (cresce 100, realloc
/// 0x945667 ou malloc 0x944e46; sem memória, a caixa 0x1637). Depois: o valor (+0xc) em +0x4e da pessoa, a moral (+0x14,
/// 0x544710) no tipo 2 com ficha, a folha (0x598370), e com cargo novo a saída do clube atual (0x53c190) e a entrada no
/// do contrato (0x53bd10); falhou: a caixa (0x1672, 0x167a) e 0.
pub(crate) unsafe extern "thiscall" fn sign_contract(this: usize, contract: usize, job: u32) -> u32 {
    let olheiros: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS);
    let morale: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let c = Contract(contract);
    let club = Club(transfer_manager_club_by_id(c.club()));
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(c.addr()).wrapping_mul(staff::SIZE)));
    let loan = || c.contract_type() & 0x3f == 5;
    if person.club_job().is_null() && !loan() && exe::database_add_staff_to_club(club, person, job, G_CURRENT_DATE, 1, 0) == 0 {
        return 0;
    }
    if job as u8 != person.job_for_club() && !loan() && exe::database_club_has_room_for_job(person.club_job(), job, person) == 0 {
        exe::contract_manager_post_offer_cancelled_no_space_news(this, person, person.club_job());
        return 0;
    }
    if !person.club_job().is_null() && exe::human_manager_is_human_club(person.club_job(), 1, (1) as u32, (0) as u32, 0) != 0 {
        olheiros(read_usize(G_SCOUT_MANAGER), read_usize(person.club_job().addr() + 0xcf));
        exe::training_manager_update_coach_assignments(read_usize(G_TRAINING_MANAGER), person.club_job(), 0);
    }
    let mut tmp = [0u8; 8];
    let mut tmp2 = [0u8; 8];
    if loan() {
        crate::comparison::branch("cqg_1 loan");
        if ((exe::database_count_club_squad(club, 0, 0) as i8) as i32) + 1 > 0x32 {
            exe::contract_manager_post_trial_cancelled_news(this, person, club);
            return 0;
        }
        exe::virtual_staff_remove_virtual_players(read_usize(G_VIRTUAL_STAFF), transfer_manager_club_by_id(c.club()), 0);
        exe::database_add_staff_to_club(transfer_manager_club_by_id(c.club()), person, 0xb, G_CURRENT_DATE, 1, 0);
        let n = exe::date_days_since(c.addr() + contract::CONTRACT_EXPIRES, c.date_started() as u32, read_usize(c.addr() + contract::DATE_STARTED + 4) as u32);
        let end = exe::date_add_days(G_CURRENT_DATE, (tmp2.as_mut_ptr()) as usize, (n) as u32); // cqg1: the end by the duration
        exe::date_operator_3d(c.addr() + contract::CONTRACT_EXPIRES, (tmp.as_mut_ptr()) as usize, end);
        c.set_squad_status((c.squad_status() & 0xf) | 0x80);
        exe::date_operator_3d(c.addr() + contract::DATE_STARTED, (tmp.as_mut_ptr()) as usize, G_CURRENT_DATE);
        exe::staff_set_squad_flags(person, 1);
    }
    // the end at least 4 weeks after the start
    let (ini_dia, ini_ano) = ((c.date_started() as i16), c.date_started_year());
    let end_year = (c.contract_expires_year() as u16);
    if ini_ano > end_year || (ini_ano == end_year && !(ini_dia < c.contract_expires())) {
        crate::comparison::branch("cqg_1 ends in 4 weeks");
        let p = exe::date_add_weeks(c.addr() + contract::DATE_STARTED, (tmp2.as_mut_ptr()) as *mut [u8; 8], 4); // cqg1: as 4 semanas
        exe::date_operator_3d(c.addr() + contract::CONTRACT_EXPIRES, (tmp.as_mut_ptr()) as usize, p);
    }
    exe::date_operator_3d(person.addr() + staff::DATE_EXPIRES_CLUB, (tmp.as_mut_ptr()) as usize, c.addr() + contract::CONTRACT_EXPIRES);
    let idx = read_usize(contract_manager_staff_state_record(read_usize(c.addr()))) as i32;
    if idx > -1 {
        let off = (idx as usize).wrapping_mul(0x50);
        let r = read_usize(this).wrapping_add(off);
        if read_usize(r + 4) == c.club() { // cqg1: the same club
            crate::comparison::branch("cqg_1 same club");
            let r = || read_usize(this).wrapping_add(off);
            c.set_squad_number(read_u8(r() + 0x3a));
            for k in [0x3dusize, 0x3e, 0x3f, 0x44] {
                write_u8(c.addr() + k, read_u8(r() + k));
            }
            c.set_grievances((read_usize(r() + 0x45)) as u32);
            c.set_transfer_status(read_u8(r() + 0x4e));
            c.set_leaving_on_bosman(0);
            c.set_transfer_arranged_for(u32::MAX as usize);
            let a = c.squad_status();
            c.set_squad_status(((read_u8(r() + 0x4f) ^ a) & 0xf) ^ a);
            let a = c.contract_type();
            c.set_contract_type(((((read_u8(r() + 0x35) << 1) as i8 >> 1) as u8 ^ a) & 0x40) ^ a);
            c.set_club_appearances((read_u16(r() + 0x3b)) as i16);
            let nation = club.nation().addr();
            if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_PORTUGAL) && exe::database_is_staff_eu_national(person, 0) == 0 {
                let p = exe::date_sub_weeks(G_CURRENT_DATE, (tmp2.as_mut_ptr()) as *mut [u8; 8], 0x64);
                let (dia, ano) = (read_i16(p), read_u16(p + cm_date::YEAR));
                let after = ano > person.date_joined_club_year() || (ano == person.date_joined_club_year() && dia > (person.date_joined_club() as i16));
                if after {
                    let first = person.nation();
                    if (!first.is_null() && read_usize(first.addr()) == read_usize(KEY_NATION_BRAZIL)) || exe::utils_random(3) == 0 {
                        person.set_second_nation(Nation(read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_PORTUGAL).wrapping_mul(nation::SIZE))));
                    }
                }
            }
        } else {
            set_contract_type_from_job(c);
            c.set_transfer_arranged_for(u32::MAX as usize);
            c.set_leaving_on_bosman(0);
            exe::staff_contract_reset_morale_state(c, person, transfer_manager_club_by_id(c.club()));
            c.set_club_appearances((0) as i16);
        }
        copy_contract(Contract(read_usize(this).wrapping_add(off)), c.addr());
    } else {
        let count = read_usize(this + contract_manager_fields::CONTRACT_COUNT) as i32;
        let mut i = 0i32;
        while i < count {
            let r = read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
            if read_usize(r) as i32 == -1 { // cqg1: the free place
                crate::comparison::branch("cqg_1 free slot");
                set_contract_type_from_job(c);
                c.set_leaving_on_bosman(0);
                c.set_transfer_arranged_for(u32::MAX as usize);
                exe::staff_contract_reset_morale_state(c, person, transfer_manager_club_by_id(c.club()));
                copy_contract(Contract(r), c.addr());
                write_usize(contract_manager_staff_state_record(read_usize(r)), i as u32 as usize);
                let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(c.club()));
                if f != 0 {
                    exe::finance_update_wage_bill(f);
                }
                return 1;
            }
            i += 1;
        }
        if count >= read_usize(this + contract_manager_fields::CONTRACT_CAPACITY) as i32 {
            let cap = (read_usize(this + contract_manager_fields::CONTRACT_CAPACITY) as u32).wrapping_add(0x64);
            write_usize(this + contract_manager_fields::CONTRACT_CAPACITY, cap as usize);
            let p = if read_usize(this) == 0 { exe::crt_malloc(cap.wrapping_mul(0x50)) } else { exe::crt_realloc(read_usize(this), cap.wrapping_mul(0x50)) };
            write_usize(this, p);
            if p == 0 {
                show_error_at(0x9a8190, 0x1637);
            }
        }
        set_contract_type_from_job(c);
        c.set_leaving_on_bosman(0);
        c.set_transfer_arranged_for(u32::MAX as usize);
        exe::staff_contract_reset_morale_state(c, person, transfer_manager_club_by_id(c.club()));
        c.set_club_appearances((0) as i16);
        write_usize(contract_manager_staff_state_record(read_usize(c.addr())), read_usize(this + contract_manager_fields::CONTRACT_COUNT));
        let n = read_usize(this + contract_manager_fields::CONTRACT_COUNT);
        let r = read_usize(this).wrapping_add(n.wrapping_mul(0x50));
        write_usize(this + contract_manager_fields::CONTRACT_COUNT, n.wrapping_add(1));
        copy_contract(Contract(r), c.addr());
    }
    crate::comparison::branch("cqg_1 value on the person");
    person.set_wage((c.wage() as usize)); // cqg1: the contract value in the person
    if c.contract_type() & 0x3f == 2 && person.player() != 0 {
        morale(person.addr(), 0x14, 0);
    }
    let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(c.club()));
    if f != 0 {
        exe::finance_update_wage_bill(f);
    }
    if job as u8 == person.job_for_club() {
        return 1;
    }
    if exe::database_remove_staff_from_club(person.club_job(), person, 0, 1, 0) == 0 {
        show_error_at(0x9a8190, 0x1672);
        return 0;
    }
    if exe::database_add_staff_to_club(transfer_manager_club_by_id(c.club()), person, job, person.addr() + staff::DATE_JOINED_CLUB, 1, 0) != 0 {
        return 1;
    }
    show_error_at(0x9a8190, 0x167a);
    0
}

/// Multiplies the three contract bonuses (+0x10, +0x14, +0x18) that are not -1 by f, truncated.
///
/// [pt] Os três bônus do contrato (+0x10, +0x14, +0x18): os que não são −1 (> −1) × `f`, truncados.
pub(crate) unsafe fn scale_contract_bonuses(c: Contract, f: f64) {
    for k in 0..3 {
        let a = c.addr() + contract::GOAL_BONUS + 4 * k;
        let v = read_i32(a);
        if v > -1 {
            write_u32(a, ftol(v as f64 * f) as u32);
        }
    }
}

/// `contract_manager_offer_contract(person, club, out, slot, factor, mode)` (SEH frame for the local STAFF_CONTRACT
/// without a destructor; x87): the contract the club offers: the desired contract × factor, the minimum release fee,
/// the cap, the end date by age, the total by age and the value capped; returns the terms or makes the offer
/// (`add_contract_offer`).
///
/// Without a club, the club of the person's contract (the 79-byte record in [0xae2d0c] → the 0x50-byte contract in
/// [[0xae1050]], kept in [0xae1080]); none: 0. A national side (`exe::database_is_national_team`), a flagged regen (bits 0x30 of +0xb of the
/// record), the club itself outside types 11..15 (`exe::database_is_player_job` of +0x3d) with the club's list at 2 or without staff and
/// manageable (`club_can_be_managed`), a human club (`exe::human_manager_is_human_club`) or a future transfer (`get_future_transfer`): 0. The desired contract (0x85fc50
/// into the local, with the status `get_squad_status_for_club`; a 3 with ability (+0xb of the sheet) above 5250 becomes 2); a return equal
/// to [0x9689a0]: 0. The wage (+8) and the value (+0xc) × factor. With a signing-on fee (+0x1d): the minimum release
/// fee (+0x21) by the value of the slot's offer (`exe::transfer_offer_get_weighted_value`: in bands of 50 thousand, 1 million and 5 million) or 50
/// thousand without an offer (100 thousand below the 1st band); without a signing-on fee, with +0x1f or +0x1c, 50
/// thousand.
///
/// The cap: the club's maximum wage (`get_max_wage`, with the job from the high nibble of +0x4f) × (factor + 1) × 0.5 for
/// status 11, otherwise the staff one (`get_max_staff_wage`). The current contract with the club (`exe::contract_manager_get_contract_for_club`). A player (+0x61): the
/// age in days (`exe::date_days_since`) × k minus the sheet's weight (type 0x14 fixed, otherwise +0x3c, +0x34 and the person's
/// +0x5b): above [0x968df8] (or a current contract with nibble 6) the end next year (value × [0x9689c0]; bonus ×
/// [0x968928]); above [0x968df0], in 2 years (bonus × [0x968de8]); above [0x968de0] with the club's reputation above
/// the sheet's (+0xd) × [0x968978], the year by age (bonus × [0x968940]). A value above 10 thousand and an age above
/// [0x968dd8]: the total (weeks to the end × value) against ability and age (pow 0x945ff0): reduces the value for each
/// 52 weeks fewer or lowers the cap to total / weeks.
///
/// A current contract that ends later: 0; an injury (`exe::injury_get_player_injury`) above 70 days with [0xb63d28]: 0. With [0xae233c] = 1,
/// at most 5 years. A value above the cap × [0x968dc0]: 0; above the cap: the wage and the bonuses × value / max(cap,
/// 1) and the value = cap. The grievances cleared (`clear_all_grievances`). With `out`: copies the contract and 1; otherwise, in mode
/// 2, or with a current contract in mode 1 that ends before today + 150 days, the offer (`add_contract_offer`); 1.
/// Original: `contract_manager_offer_contract` (0x4cda70, contract_manager.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x4cda70 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH do STAFF_CONTRACT local sem destrutor; x87):
/// `contract_manager_offer_contract(pessoa, clube, saida, vaga, fator, modo)`: o contrato que o clube oferece. Sem
/// clube, o do contrato da pessoa (o registro de 79 bytes em [0xae2d0c] → o contrato de 0x50 bytes em [[0xae1050]],
/// guardado em [0xae1080]); nenhum: 0. Seleção (0x53b900), regen marcado (bits 0x30 do +0xb do registro), o próprio clube
/// fora dos tipos 11..15 (0x53b620 de +0x3d) com a lista do clube em 2 ou sem comissão e dirigível (0x544820), clube
/// humano (0x600a50) ou transferência futura (0x8cca10): 0. O contrato desejado (0x85fc50 no local, com o estatuto
/// 0x849f00, o 3 de habilidade (+0xb da ficha) acima de 5250 vira 2); a devolução igual a [0x9689a0]: 0. O salário (+8)
/// e o valor (+0xc) × fator. Com luvas (+0x1d): o mínimo da multa (+0x21) pelo valor da oferta da vaga (0x8e7b50: em
/// faixas de 50 mil, 1 milhão e 5 milhões) ou 50 mil sem oferta (100 mil abaixo da 1ª faixa); sem luvas, com +0x1f ou
/// +0x1c, 50 mil. O teto: o salário máximo do clube (0x5968d0, com o cargo do nibble alto de +0x4f) × (fator + 1) × 0,5
/// no estatuto 11, senão o do pessoal (0x597db0). O contrato atual com o clube (0x4cf5f0). Jogador (+0x61): a idade em
/// dias (0x54cdb0) × k menos o peso da ficha (o tipo 0x14 fixo, senão +0x3c, +0x34 e o +0x5b da pessoa): acima de
/// [0x968df8] (ou contrato atual com o nibble 6) o fim no ano que vem (valor × [0x9689c0]; bônus × [0x968928]); acima de
/// [0x968df0], daqui a 2 anos (bônus × [0x968de8]); acima de [0x968de0] com a reputação do clube acima da da ficha (+0xd)
/// × [0x968978], o ano pela idade (bônus × [0x968940]). Valor acima de 10 mil e idade acima de [0x968dd8]: o total
/// (semanas até o fim × valor) contra a habilidade e a idade (pow 0x945ff0): reduz o valor a cada 52 semanas a menos ou
/// baixa o teto para total / semanas. Contrato atual que acaba depois: 0; lesão (0x62ed70) acima de 70 dias com o
/// [0xb63d28]: 0. Com [0xae233c] = 1, no máximo 5 anos. Valor acima do teto × [0x968dc0]: 0; acima do teto: o salário e
/// os bônus × valor / max(teto, 1) e o valor = teto. As queixas limpas (0x867520). Com `saida`: copia o contrato e 1;
/// senão, no modo 2, ou com contrato atual no modo 1 que acaba antes de hoje + 150 dias, a proposta (0x4cc490); 1.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn contract_manager_offer_contract(this: usize, person: usize, club_arg: usize, out: usize, slot: i32, factor: f64, mode: u32) -> u32 {
    let out = Contract(out);
    let person = Staff(person);
    let injury: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);

    // the local STAFF_CONTRACT (0x50 bytes; the original has it at [esp+0x38])
    let mut local = [0u32; 0x14];
    let c = local.as_mut_ptr() as usize;
    exe::new_staff_contract(c);
    let mut club = club_arg;
    if club == 0 {
        let k = if person.is_null() {
            0
        } else if (read_usize(person.addr()) as i32) < read_i32(G_STAFF_COUNT) {
            let idx = read_i32(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person.addr()).wrapping_mul(0x4f)));
            if idx < 0 { 0 } else { (idx as usize).wrapping_mul(0x50).wrapping_add(read_usize(read_usize(G_CONTRACT_MANAGER))) }
        } else {
            0
        };
        write_u32(G_CONTRACT_LOOKUP_RESULT, k as u32);
        if k == 0 {
            return 0;
        }
        club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + 4).wrapping_mul(club::SIZE));
        if club == 0 {
            return 0;
        }
    }
    if exe::database_is_national_team(club) != 0 {
        return 0;
    }
    if read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person.addr()).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return 0;
    }
    if exe::database_is_player_job(person.job_for_club() as i8) == 0 && club == person.club_job().addr() {
        let list = read_usize(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856) + 0xdc);
        if read_u8(list.wrapping_add(read_usize(club).wrapping_mul(9)) + 4) == 2 || (read_usize(club + club::MANAGER) == 0 && exe::club_can_be_managed(club) != 0) {
            return 0;
        }
    }
    if exe::human_manager_is_human_club(club, 0, (1) as u8 as u32, 0 as u32, 0) != 0 {
        return 0;
    }
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), person, 0 as u32) != 0 {
        return 0;
    }
    write_u32(c + contract::CLUB, read_usize(club) as u32);
    let mut offer = 0usize;
    if slot != -1 {
        let p = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), slot);
        if p != 0 {
            offer = read_usize(p);
        }
    }
    let mut job: u8 = 0;
    let mut est = exe::shortlist_manager_get_squad_status_for_club(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), person, club, (&mut job) as *mut u8 as usize);
    if est == 3 && read_i16(person.player() + 0xb) > 0x1482 {
        est = 2;
    }
    // simplification: the status and the job go in dwords with the high bytes of stack leftovers in the original; 0x85fc50 and
    // 0x597db0 leem o byte
    let r = exe::staff_contract_set_to_desired_contract(c, person, club, slot, est as u32, job as u32, 0, 0);
    if r as f64 == read_f64(rdata::F64_0) {
        return 0;
    }
    write_u32(c + contract::UNKNOWN, ftol(read_i32(c + contract::UNKNOWN) as f64 * factor) as u32);
    write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * factor) as u32);
    if read_u8(c + contract::MINIMUM_FEE_RC) != 0 {
        let mut floor: i32 = 0;
        if offer != 0 {
            let v = exe::transfer_offer_get_weighted_value(offer, 0, 0, 0, 0, 1);
            if v > 0x4c4b40 {
                floor = ftol((v - 0x4c4b40) as f64 * read_f64(rdata::F64_1_25) + read_f64(rdata::F64_8425000));
            } else if v > 0xf4240 {
                floor = ftol((v - 0xf4240) as f64 * read_f64(rdata::F64_1_5) + read_f64(rdata::F64_2425000));
            } else if v > 0xc350 {
                floor = ftol((v - 0xc350) as f64 * read_f64(rdata::F64_2_5) + read_f64(rdata::F64_100000));
            } else if read_i32(c + contract::RELEASE_FEE) < 0x186a0 {
                floor = 0x186a0;
            }
        } else if read_i32(c + contract::RELEASE_FEE) < 0xc350 {
            floor = 0xc350;
        }
        if read_i32(c + contract::RELEASE_FEE) < floor {
            write_u32(c + contract::RELEASE_FEE, floor as u32);
        }
    } else if (read_u8(c + contract::RELEGATION_RC) != 0 || read_u8(c + contract::NON_PROMOTION_RC) != 0) && read_i32(c + contract::RELEASE_FEE) < 0xc350 {
        write_u32(c + contract::RELEASE_FEE, 0xc350);
    }
    let mut ceiling: f64 = if job == 0xb {
        let nibble = (read_u8(c + contract::SQUAD_STATUS) >> 4) as u32;
        let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let m = exe::finance_get_max_wage(f, person, nibble);
        (factor + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5) * m as f64 // cqf3: the cap of status 11;
    } else {
        let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        exe::finance_get_max_staff_wage(f, job as i8) as f64
    };
    let current = Contract(exe::contract_manager_get_contract_for_club(this, person, club));
    let mut year = read_i16(G_CURRENT_DATE_YEAR) as u16;
    let sheet = person.player();
    if sheet != 0 {
        let days = exe::date_days_since(G_CURRENT_DATE, read_usize(person.addr() + staff::DATE_OF_BIRTH) as u32, person.date_of_birth_leap_year() as u32);
        let mut x = days as f64 * read_f64(rdata::F64_0_002739726027);
        if read_u8(sheet + player_sheet::GOALKEEPER) == 0x14 {
            x -= read_f64(rdata::F64_2);
        } else {
            let s = read_u8(sheet + player_sheet::STAMINA) as i8 as i32 + read_u8(sheet + player_sheet::NATURAL_FITNESS) as i8 as i32 + (person.professionalism() as u8) as i8 as i32;
            x -= (s as f64 * read_f64(rdata::F64_0_3333333333) - read_f64(rdata::F64_15)) * read_f64(rdata::F64_0_4);
        }
        let end = |c: usize| core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32;
        if x > read_f64(rdata::F64_34) || (!current.is_null() && current.squad_status() & 0xf0 == 0x60) {
            if end(c) > year as u32 + 1 {
                crate::comparison::branch("cqf_3 ends next year");
                write_u16(c + 0x2f, year.wrapping_add(1)); // cqf3: the end next year
                write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * read_f64(rdata::F64_1_1)) as u32);
            }
            scale_contract_bonuses(Contract(c), read_f64(rdata::F64_1_25));
        } else if x > read_f64(rdata::F64_31) {
            if end(c) > year as u32 + 2 {
                write_u16(c + 0x2f, year.wrapping_add(2));
                write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * read_f64(rdata::F64_1_1)) as u32);
            }
            scale_contract_bonuses(Contract(c), read_f64(rdata::F64_1_3));
        } else if x > read_f64(rdata::F64_29) && read_i16(club + club::REPUTATION) as f64 > read_i16(sheet + player_sheet::WORLD_REPUTATION) as f64 * read_f64(rdata::F64_1_2) {
            let k = ftol(x + read_f64(rdata::F64_0_5)) as i8;
            if end(c) as i32 > (year as i32 - k as i32) + 0x22 {
                write_u16(c + 0x2f, year.wrapping_sub(k as i16 as u16).wrapping_add(0x22));
                write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * read_f64(rdata::F64_1_1)) as u32);
            }
            scale_contract_bonuses(Contract(c), read_f64(rdata::F64_1_5));
        }
        if read_i32(c + contract::WAGE) > 0x2710 && x > read_f64(rdata::F64_30) {
            let days_end = exe::date_days_since(c + contract::CONTRACT_EXPIRES, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
            let mut weeks = days_end / 7;
            let mut total = weeks.wrapping_mul(read_i32(c + contract::WAGE));
            let mut a = game_pow(read_i16(sheet + player_sheet::CURRENT_REPUTATION) as f64, read_f64(rdata::F64_2)) * read_f64(rdata::F64_0_0001333333333);
            if !(a < read_f64(rdata::F64_8000)) {
                a = 8000.0;
            }
            let mut b = game_pow(x - read_f64(rdata::F64_30), read_f64(rdata::F64_2));
            if !(b > read_f64(rdata::F64_1)) {
                b = 1.0;
            }
            let limit = ftol(game_pow(a, read_f64(rdata::F64_2)) / b);
            let reduces = (limit as f64 * read_f64(rdata::F64_1_5)) < total as f64;
            let mut done = false;
            if reduces {
                while weeks > 0x68 {
                    weeks -= 0x34;
                    write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * read_f64(rdata::F64_1_1)) as u32);
                    total = weeks.wrapping_mul(read_i32(c + contract::WAGE));
                    if limit >= total {
                        done = true;
                        break;
                    }
                }
            }
            if !done && total > limit {
                let q = limit / weeks;
                if (q as f64) < ceiling {
                    crate::comparison::branch("cqf_3 cap by total");
                    ceiling = q as f64; // cqf3: the cap by the total
                }
            }
        }
        year = read_i16(G_CURRENT_DATE_YEAR) as u16;
    }
    if !current.is_null() {
        if (current.contract_expires_year() as u16) >= core::ptr::read_unaligned((c + 0x2f) as *const u16) {
            return 0;
        }
        if read_usize(G_INJURY) != 0 {
            let l = injury(read_usize(G_INJURY), person.addr());
            if l != 0 && read_i16(injury(read_usize(G_INJURY), person.addr()) + 0x2c) > 0x46 {
                return 0;
            }
            year = read_i16(G_CURRENT_DATE_YEAR) as u16;
        }
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32 > year as u32 + 5 {
        write_u16(c + 0x2f, year.wrapping_add(5));
    }
    let w = read_i32(c + contract::WAGE) as f64;
    if w > ceiling * read_f64(rdata::F64_1_4) {
        return 0;
    }
    if w > ceiling {
        crate::comparison::branch("cqf_3 above the cap");
        let den = if read_f64(rdata::F64_1) > ceiling { read_f64(rdata::F64_1) } else { ceiling };
        let ratio = w / den;
        write_u32(c + contract::UNKNOWN, ftol(read_i32(c + contract::UNKNOWN) as f64 * ratio) as u32);
        scale_contract_bonuses(Contract(c), ratio);
        write_u32(c + contract::WAGE, ftol(ceiling) as u32); // cqf3: the value at the cap
    }
    exe::staff_contract_clear_all_grievances(c, 0);
    if !out.is_null() {
        for k in (0..0x20).step_by(4) {
            write_u32(out.addr() + k, read_usize(c + k) as u32);
        }
        out.set_manager_job_rc(read_u8(c + contract::MANAGER_JOB_RC));
        out.set_release_fee((read_usize(c + contract::RELEASE_FEE) as u32) as usize);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(out.addr() + contract::DATE_STARTED, (tmp.as_mut_ptr()) as *mut u8 as usize, c + contract::DATE_STARTED);
        exe::date_operator_3d(out.addr() + contract::CONTRACT_EXPIRES, (tmp.as_mut_ptr()) as *mut u8 as usize, c + contract::CONTRACT_EXPIRES);
        // +0x35 by the original’s bit fields: 6 bits, 1 bit and 1 bit
        let cl = read_u8(c + contract::CONTRACT_TYPE);
        let dl = out.contract_type();
        let al = ((((cl << 2) as i8 >> 2) as u8 ^ dl) & 0x3f) ^ dl;
        let d2 = (((((cl << 1) as i8 >> 1) as u8 ^ al) & 0xc0) ^ cl ^ al) & 0x7f;
        out.set_previous_wage((read_usize(c + contract::PREVIOUS_WAGE) as u32) as usize);
        out.set_contract_type(d2 ^ cl);
        out.set_squad_number(read_u8(c + 0x3a));
        out.set_club_appearances((read_i16(c + 0x3b) as u16) as i16);
        for k in [0x3dusize, 0x3e, 0x3f] {
            write_u8(out.addr() + k, read_u8(c + k));
        }
        out.set_flags(read_usize(c + 0x40) as u32);
        out.set_morale((read_u8(c + 0x44)) as i8);
        out.set_grievances(read_usize(c + 0x45) as u32);
        out.set_leaving_on_bosman(read_u8(c + contract::LEAVING_ON_BOSMAN));
        out.set_transfer_arranged_for((read_usize(c + contract::TRANSFER_ARRANGED_FOR) as u32) as usize);
        out.set_transfer_status(read_u8(c + contract::TRANSFER_STATUS));
        out.set_squad_status(read_u8(c + contract::SQUAD_STATUS));
        return 1;
    }
    let mode = mode as u8;
    let enviar = if mode == 2 {
        true
    } else if current.is_null() || mode != 1 {
        false
    } else {
        let mut d = [0u8; 8];
        let p = exe::date_add_days(G_CONTRACT_DATE_PLUS_365_DAYS, (d.as_mut_ptr()) as *mut u8 as usize, 0x96);
        let (dia, ano_d) = (read_i16(p), core::ptr::read_unaligned((p + cm_date::YEAR) as *const u16));
        let ano_a = (current.contract_expires_year() as u16);
        ano_d < ano_a || (ano_d == ano_a && dia < current.contract_expires())
    };
    if enviar {
        // simplification: the role (+0x3d) goes in a dword with high bytes left over from edx in the original; 0x4cc490 reads the byte
        exe::contract_manager_add_contract_offer(this, (c) as usize as *mut u8, (slot) as i32 as usize, (u32::MAX) as u32 as usize, person.job_for_club() as u32, 0, 1, 0);
    }
    1
}

/// Adds delta to a signed byte (contract happiness +0x3d or morale +0x44) clamped to -100..100 like the original.
///
/// [pt] Soma `delta` ao read_i8(felicidade +0x3d + (moral +0x44 do contrato)) entre −100 e 100, como a original: a soma em 32
/// bits decide o teto e o piso; dentro deles, a soma do byte.
pub(crate) unsafe fn add_clamped_100(a: usize, delta: i32) {
    let v = read_i8(a);
    let s = v as i32 + delta;
    *(a as *mut i8) = if s > 100 { 100 } else if s < -100 { -100 } else { v.wrapping_add(delta as i8) };
}

/// Returns (this side's club, other club) of a match from +0x1c home / +0x20 away.
///
/// [pt] Os dois clubes do jogo (+0x1c mandante, +0x20 visitante): o do lado e o outro.
pub(crate) unsafe fn match_clubs_by_side(game: Fixture, side: usize) -> (usize, usize) {
    if side == 0 { (game.home_club().addr(), game.away_club().addr()) } else { (game.away_club().addr(), game.home_club().addr()) }
}

/// True when match byte +0x3a = 1 and the other club's reputation (+0x80) is below max(own rep × constant, own rep +
/// 1500): that side is skipped.
///
/// [pt] Com o byte +0x3a do jogo = 1, o lado fica de fora quando a reputação do outro clube (+0x80) fica abaixo do maior
/// entre a dele × [0x968928] e a dele + 1500 (x87 em dupla, como a original).
pub(crate) unsafe fn skip_side_by_reputation(game: Fixture, side: usize) -> bool {
    if (game.category() as i8) != 1 {
        return false;
    }
    let (meu, outro) = match_clubs_by_side(game, side);
    let r = read_i16(meu + club::REPUTATION) as i32;
    let x = r as f64 * read_f64(rdata::F64_1_25);
    let y = (r + 0x5dc) as f64;
    let m = if x > y { x } else { y };
    m > read_i16(outro + club::REPUTATION) as f64
}

/// Person (0x6e-byte record in G_STAFF_TABLE) of a match summary slot, or None when the original skips the slot (id -1
/// or out of range).
///
/// [pt] A pessoa da vaga do resumo (id em +0 e +4 diferente de −1; registro de 0x6e bytes em [0xae23bc]), ou None quando a
/// original pula a vaga. A pessoa nula (base 0) passa, como na original.
pub(crate) unsafe fn staff_of_summary_slot(slot: usize) -> Option<usize> {
    let id = read_i32(slot);
    if id == -1 || read_i32(slot + 4) == -1 {
        return None;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
        return None;
    }
    Some(p)
}

/// The three contract bonuses paid for a summary slot (Finance::pay_bonuses `0x599960`).
///
/// `+0x21` × `+0x10`, `+0x27` × `+0x14` and, with the summary word = 1 and `+0x22`, `+0x27` × `+0x18` (the original
/// routine uses `+0x27` again); each only when the byte and the value are positive.
///
/// No bonus in a friendly (`0x5c6710`), in a match of competition [`0x9cf684`] or for contract types 5 and 6.
///
/// [pt] Os três bônus do contrato pela vaga (Finance::pay_bonuses 0x599960): +0x21 × +0x10, +0x27 × +0x14 e, com a palavra
/// do resumo = 1 e +0x22, +0x27 × +0x18 (a original usa o +0x27 de novo); cada um só com o byte e o valor positivos.
/// Sem bônus no amistoso (0x5c6710), no jogo da competição [0x9cf684] e nos contratos de tipo 5 e 6.
pub(crate) unsafe fn match_bonuses_for_slot(game: Fixture, contract: Contract, slot: usize, word: i16) -> [Option<i32>; 3] {
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let comp = game.competition();
    if !comp.is_null() && (friendly(read_usize(G_FRIENDLY), comp.addr()) != 0 || read_usize(game.addr()) == read_usize(KEY_CLUB_COMP_RESERVE)) {
        return [None; 3];
    }
    let kind = contract.contract_type() & 0x3f;
    if kind == 6 || kind == 5 {
        return [None; 3];
    }
    let g = *((slot + match_record_row::GOALS) as *const u8) as i32;
    let a = *((slot + match_record_row::ASSISTS) as *const u8) as i32;
    let um = (g != 0 && contract.goal_bonus() > 0).then(|| g.wrapping_mul(contract.goal_bonus()));
    let two = (a != 0 && contract.assist_bonus() > 0).then(|| a.wrapping_mul(contract.assist_bonus()));
    let three = (word == 1 && read_i8(slot + 0x22) != 0 && contract.clean_sheet_bonus() > 0).then(|| a.wrapping_mul(contract.clean_sheet_bonus()));
    [um, two, three]
}

/// `CONTRACT_MANAGER::update_after_match(match, summary)`, called by the closing of the day's match (`close_day_match`): after
/// each match, contract bonuses, happiness by minutes played, morale by a draw and by the result, and the development
/// and happiness of those left out.
///
/// With the match's +0x40 and +0x41, the winner by the score (+0x47/+0x48; with +0x47 = −1, +0x43/+0x44) and a copy of
/// the 50 squad slots (+0xd7) of each club. First, per side (club B of the partner `exe::club_get_linked_b_team` and the side skipped by
/// reputation are left out), the 20 slots of the summary (+0x190 + side × 0x625, 0x49 bytes): outside national sides,
/// the contract with the club (`exe::contract_manager_get_contract_for_club`) pays the bonuses (match_bonuses_for_slot); the minutes (+0x30 for starters, 90
/// without a value; 90 − +0x32 for substitutes) change the happiness (+0x3d, minutes × 10 / 90, held at 50 below 50)
/// or, for national sides, the +9 of the 79-byte record; outside national sides and friendlies, the substitutes and the
/// starters with little playing time draw (2 × draw(30)) against the minutes: the happiness drops 4, +0x3e counts and
/// the morale drops (`contract_manager_lower_match_morale`); otherwise +0x3f counts (with 5 or more, outside the human club, squad mark 1 in
/// `staff_set_squad_flags`) and the morale rises (`contract_manager_raise_match_morale`). The winner (and the loser) draws (21 − +0x57) below 4 for +3 (−3) on
/// morale +0x44; the person leaves the squad copy.
///
/// Then, those left in the copy (with a player sheet, a contract, without suspension `is_player_suspended`, injury `get_player_injury_status`,
/// national duty `get_international_duty_club` or loan) develop (`exe::update_player_reputation_after_match` with the average rating [0xb74350] − 1), +0x3e counts and, from 8
/// (14 with a partner), outside the human club and the partner, they leave the first team (`get_squad_placement` 2 or 4 → `toggle_squad_placement`);
/// the happiness drops 8 and the morale drops.
/// Original: `CONTRACT_MANAGER::update_after_match` (0x4ca6d0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4ca6d0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::update_after_match(jogo, resumo)`, pelo
/// fechamento da partida do dia (0x6b5f00). Com +0x40 e +0x41 do jogo, o vencedor pelo placar (+0x47/+0x48; com +0x47 =
/// −1, +0x43/+0x44) e uma cópia dos 50 do elenco (+0xd7) de cada clube. Primeiro, por lado (clube B do parceiro
/// 0x540a50 e o lado pulado pela reputação ficam de fora), as 20 vagas do resumo (+0x190 + lado × 0x625, 0x49 bytes):
/// fora das seleções, o contrato com o clube (0x4cf5f0) paga os bônus (match_bonuses_for_slot); os minutos (+0x30 dos titulares,
/// 90 sem valor; 90 − +0x32 dos reservas) mudam a felicidade (+0x3d, minutos × 10 / 90, presa em 50 abaixo de 50) ou,
/// nas seleções, o +9 do registro de 79 bytes; fora das seleções e dos amistosos, os reservas e os titulares de pouco
/// jogo sorteiam (2 × exe::utils_random(30)) contra os minutos: a felicidade cai 4, +0x3e conta e a moral cai (0x4cb250); senão
/// +0x3f conta (com 5 ou mais, fora do clube humano, a marca de elenco 1 em 0x540980) e a moral sobe (0x4cb0a0). O
/// vencedor (e o perdedor) sorteia(21 − +0x57) abaixo de 4 para +3 (−3) na moral +0x44; a pessoa sai da cópia do
/// elenco. Depois, quem ficou na cópia (com ficha, contrato, sem suspensão 0x55d270, lesão 0x62c400, seleção 0x7757d0
/// nem empréstimo) evolui (0x6d26b0 com a nota média [0xb74350] − 1), +0x3e conta e, a partir de 8 (14 com parceiro),
/// fora do humano e do parceiro, sai do principal (0x4cbb60 2 ou 4 → 0x4cbd20); a felicidade cai 8 e a moral cai.
pub(crate) unsafe extern "thiscall" fn update_after_match(this: usize, game: usize, summary: usize) {
    let game = Fixture(game);
    if (game.home_side_mark() as i8) == 0 || game.away_side_mark() == 0 {
        return;
    }
    // the original sets ecx = this before 0x4cb0a0 (stdcall in the Rust copy): thiscall with 3 on the stack serves both
    let (c47, c48) = (game.home_goals_extra_time(), game.away_goals_extra_time());
    let (a, b) = if c47 == -1 { (game.home_goals(), game.away_goals()) } else { (c47, c48) };
    let won = [(a > b) as u32, (a < b) as u32];
    let mut list = [[0usize; 50]; 2];
    for (lado, l) in list.iter_mut().enumerate() {
        let c = match_clubs_by_side(game, lado).0;
        core::ptr::copy_nonoverlapping((c + 0xd7) as *const usize, l.as_mut_ptr(), 50);
    }
    let mut eh_a = 0u32;
    for lado in 0..2usize {
        let club = match_clubs_by_side(game, lado).0;
        if exe::club_get_linked_b_team(club, &mut eh_a, 1) != 0 && eh_a == 0 {
            continue;
        }
        if skip_side_by_reputation(game, lado) {
            continue;
        }
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let base = summary + lado * 0x625;
        for k in 0..20usize {
            let slot = base + match_stats_record::HOME_LINEUP + k * 0x49;
            let Some(person) = staff_of_summary_slot(slot) else { continue };
            let contract = if exe::database_is_national_team(club) != 0 {
                0
            } else {
                let c = exe::contract_manager_get_contract_for_club(this, person, club);
                if c == 0 {
                    continue;
                }
                for v in match_bonuses_for_slot(game, Contract(c), slot, read_i16(base + 0x154 + 2 * k)).into_iter().flatten() {
                    exe::finance_pay_bonuses(fin, v);
                }
                c
            };
            let minutos: i8 = if k < 11 {
                let v = read_i8(slot + match_record_row::MINUTE_OFF);
                if v > -1 { v } else { 0x5a }
            } else {
                let v = read_i8(slot + match_record_row::MINUTE_ON);
                if v <= -1 {
                    0
                } else {
                    let e = 0x5a - v as i32;
                    if e <= 0 { 0 } else { e as i8 }
                }
            };
            if k < 11 || read_i8(slot + match_record_row::MINUTE_ON) != -1 {
                let q = ((minutos as i32 * 10) / 90) as i8;
                if exe::database_is_national_team(club) != 0 {
                    let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)) + 9;
                    *(r as *mut i8) = read_i8(r).wrapping_add(q);
                    if read_i8(r) > 100 {
                        *(r as *mut i8) = 100;
                    }
                } else {
                    let al = read_i8(contract + contract::HAPPINESS);
                    let s = al as i32 + q as i32;
                    *((contract + contract::HAPPINESS) as *mut i8) = if al > 0x32 {
                        if s > 100 { 100 } else if s < -100 { -100 } else { al.wrapping_add(q) }
                    } else if s > 0x32 {
                        0x32
                    } else {
                        let v = al.wrapping_add(q);
                        if v > 100 { 100 } else if v < -100 { -100 } else { v }
                    };
                }
            }
            if exe::database_is_national_team(club) == 0 && (read_i32(game.addr()) < 0 || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), game.competition()) == 0) {
                let testa = k >= 11 || !(minutos > 0x1e || read_i8(slot + 0x1f) != 0 || read_i8(slot + 0x20) < 0x46);
                let mal = testa && {
                    crate::comparison::branch("cqa_8 tests the morale");
                    let m = read_i8(contract + contract::MORALE) as i32;
                    let r1 = exe::utils_random(0x1e); // cqa8: the first morale random draw
                    let v = r1 as f64 + (read_f64(rdata::F64_45) - m as f64 * read_f64(rdata::F64_0_303030303));
                    let r2 = exe::utils_random(0x1e);
                    v - r2 as f64 > minutos as f64
                };
                let p = contract as *mut u8;
                if mal {
                    *p.add(0x3f) = 0;
                    if *p.add(0x3e) != 100 {
                        *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
                    }
                    add_clamped_100(contract + contract::HAPPINESS, -4);
                    exe::contract_manager_lower_match_morale(this, person, club, contract);
                } else {
                    *p.add(0x3e) = 0;
                    if *p.add(0x3f) != 100 {
                        *p.add(0x3f) = (*p.add(0x3f)).wrapping_add(1);
                    }
                    if read_i8(contract + contract::MATCHES_LEFT_OUT) >= 5 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
                        exe::staff_set_squad_flags(person, 1);
                    }
                    exe::contract_manager_raise_match_morale(this, person, club, contract);
                }
            }
            if won[lado] != 0 {
                crate::comparison::branch("cqa_8 winner morale");
                let r = exe::utils_random(0x15 - read_i8(person + staff::AMBITION) as i32); // cqa8: the winner random draw
                if r < 4 && contract != 0 {
                    add_clamped_100(contract + contract::MORALE, 3);
                }
            } else if won[1 - lado] != 0 {
                let r = exe::utils_random(0x15 - read_i8(person + staff::AMBITION) as i32);
                if r < 4 && contract != 0 {
                    add_clamped_100(contract + contract::MORALE, -3);
                }
            }
            if let Some(x) = list[lado].iter_mut().find(|x| **x == person) {
                *x = 0;
            }
        }
    }
    for lado in 0..2usize {
        let (club, outro) = match_clubs_by_side(game, lado);
        let limit: i8 = if exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) != 0 { 14 } else { 8 };
        if skip_side_by_reputation(game, lado) {
            continue;
        }
        for i in 0..50usize {
            let person = list[lado][i];
            if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) || read_usize(person + staff::PLAYER) == 0 || exe::database_is_national_team(club) != 0 {
                continue;
            }
            let contract = exe::contract_manager_get_contract_for_club(this, person, club);
            if contract == 0 {
                continue;
            }
            if read_i32(game.addr()) != -2 && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), person, game.competition(), 1, 0) != 0 {
                continue;
            }
            if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) != 0 || exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), person, G_CURRENT_DATE, club, 0, 0) != 0 {
                continue;
            }
            // on loan: the person’s second contract (+4 of the 79-byte record)
            let idx = read_i32(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)) + 4);
            if idx >= 0 && (idx as usize).wrapping_mul(0x50).wrapping_add(read_usize(this)) != 0 {
                continue;
            }
            let m = read_usize(G_PLAYER_STATS);
            let x = core::ptr::read_unaligned((m + 0x18) as *const f64);
            let c = read_f64(rdata::F64_0);
            // fcomp + C3: equal or unordered goes to the constant
            let rating = if !(x < c || x > c) { read_f64(rdata::F64_6_8) } else { core::ptr::read_unaligned((m + 0x10) as *const f64) / x };
            crate::comparison::branch("cqa_8 develops who was left out");
            exe::update_player_reputation_after_match(0, person, club, outro, game, (rating - read_f64(rdata::F64_0_05)) as f32, 0, 0); // cqa8: the progress of those left out;
            let p = contract as *mut u8;
            *p.add(0x3f) = 0;
            if *p.add(0x3e) != 100 {
                *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
            }
            if read_i8(contract + contract::UNHAPPY_FLAG) >= limit && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) == 0 {
                let l = exe::contract_manager_get_squad_placement(this, person, club);
                if l == 2 || l == 4 {
                    exe::contract_manager_toggle_squad_placement(this, person, club, 0);
                }
            }
            add_clamped_100(contract + contract::HAPPINESS, -8);
            exe::contract_manager_lower_match_morale(this, person, club, contract);
        }
    }
}

/// the morale of the player record (+0x45), for the screens
/// Original: `contract_manager_get_player_morale` (0x4cc2d0, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4cc2d0 } (stdcall, ret 4, contract_manager.cpp): `contract_manager_get_player_morale(pessoa)`: o byte +0x45 da
/// ficha (+0x61), pelas telas. O eax leva os bytes altos da pessoa, como na original.
pub(crate) unsafe extern "stdcall" fn contract_manager_get_player_morale(person: usize) -> u32 {
    let person = Staff(person);
    (person.addr() as u32 & 0xffff_ff00) | *((person.player() + 0x45) as *const u8) as u32
}

/// Copies the zero-terminated text at a (up to 0x400 bytes) into a Vec, for the text comparers.
///
/// [pt] O texto terminado em zero em `a` (até 0x400 bytes), para os comparadores dos textos.
pub(crate) unsafe fn c_string_bytes(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// `contract_manager_write_morale_text(person, out, _)` (stdcall, ret 0xc): writes the morale word into a text, for the
/// screens.
///
/// The morale (+0x45 of the sheet): below 4 "Very Low", 8 "Low", 12 "Ok", 15 "Good", 18 "Very Good", otherwise
/// "Superb", written to the output (0x66f4e0 with 0xad9c64).
/// Original: `contract_manager_write_morale_text` (0x4cc2e0, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4cc2e0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_write_morale_text(pessoa, saída, _)`: a
/// palavra da moral (+0x45 da ficha: abaixo de 4 "Very Low", 8 "Low", 12 "Ok", 15 "Good", 18 "Very Good", senão
/// "Superb") na saída (0x66f4e0 com 0xad9c64), pelas telas.
pub(crate) unsafe extern "stdcall" fn contract_manager_write_morale_text(person: usize, out: usize, _x: u32) {
    let person = Staff(person);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let m = read_i8(person.player() + 0x45);
    let f = if m < 4 {
        0x9a8320usize
    } else if m < 8 {
        0x9a82fc
    } else if m < 0xc {
        0x9a82dc
    } else if m < 0xf {
        0x9a82b8
    } else if m < 0x12 {
        0x9a8290
    } else {
        0x9a826c
    };
    format(out, f, 0xad9c64usize);
}

/// Main contract record (0x50 bytes in the contract manager) of a person via G_STAFF_STATE_TABLE, or 0 when none/out of
/// range.
///
/// [pt] O contrato principal da pessoa (índice +0 do registro de 79 bytes em [0xae2d0c], registros de 0x50 bytes em
/// [[0xae1050]]), ou 0: sem pessoa, fora das pessoas ([0xae2364]) ou sem índice.
pub(crate) unsafe fn main_contract_of_staff(person: Staff) -> usize {
    if person.is_null() || read_usize(person.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person.addr()).wrapping_mul(0x4f))) as i32;
    if i < 0 { 0 } else { (i as usize).wrapping_mul(0x50).wrapping_add(read_usize(read_usize(G_CONTRACT_MANAGER))) }
}

/// Caps value w at the contract release fee (+0x21) when the release clause is active, then truncates (_ftol).
///
/// [pt] O mínimo com a cláusula de rescisão (+0x21 do contrato, se 0x867b50 a diz ativa) e o truncamento (_ftol).
pub(crate) unsafe fn cap_value_at_release_fee(contract: Contract, w: f64) -> i32 {
    if !contract.is_null() && exe::staff_contract_has_active_release_clause(contract) != 0 {
        let cl = (contract.release_fee() as i32) as f64;
        return ftol(if !(w >= cl) { w } else { cl });
    }
    ftol(w)
}

/// `CONTRACT_MANAGER::calc_player_value(person, club, mode, ability, _)`: the player value by peak age and age, record
/// ratings, ability as seen by the club manager, club nation, contract days left and release clause.
///
/// The main contract goes to [0xae1080] and the contract with his club (`exe::contract_manager_get_contract_for_club`, into the incoming `this`). Without a
/// player sheet (+0x61): the compensation (`transfer_manager_calc_compensation_fee` with 1) by the contract club, or 0. The base and the peak age by
/// the sheet's positions (+0x15/+0x14/+0x13/+0x12 ≥ 15: 13, 11, 10, 9 million; goalkeeper +0xf = 20: 6 million, 33;
/// otherwise 8 million; peak 28 or 29). The age factor (peak − age, with the correction above [0x9688a0]) × the sheet's
/// ratings (+0x28, +0x27, +0x43, +0x36, +0x2f, outside goalkeepers) and +0x2d²; the reputation (+9) limits the factor.
/// The ability: without a club, the current one (+0xb); with a club, the one seen by the manager (`perceived_ability` if `ability`
/// = −1, with the seen reputation `database_get_staff_reputation_for_viewer`) × 50 + the reputation + up to 5000 × factor, × [0x968f08], between 1 and
/// 12000; the potential (+0xd) above 5000 (and 8000) before the peak pulls it up.
///
/// Value = ability² × base × the table 0x9a7f30 (by age, with experience +0x22/+0x1a) × the club's nation (+0x53: +0x7e
/// and +0x85 or the division +0x69; without a club, draw(3) + 1), with a floor. `mode`: the nation [0x9cf434] and the
/// clause. Otherwise: the contract days (with the Bosman rule `player_club_uses_bosman`) and the nations [0x9cf434], [0x9cf300],
/// [0x9cf484]; at the end the active release clause (`has_active_release_clause`) limits it.
/// Original: `CONTRACT_MANAGER::calc_player_value` (0x4d0c60, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d0c60 } (thiscall, ret 0x14, contract_manager.cpp): `CONTRACT_MANAGER::calc_player_value(pessoa, clube, modo,
/// habilidade, _)`. O contrato principal vai para [0xae1080] e o contrato com o clube dele (0x4cf5f0, no this de
/// entrada). Sem ficha (+0x61): a compensação (0x8d0780 com 1) pelo clube do contrato, ou 0. A base e a idade do auge
/// pelas posições da ficha (+0x15/+0x14/+0x13/+0x12 ≥ 15: 13, 11, 10, 9 milhões; goleiro +0xf = 20: 6 milhões, 33; senão
/// 8 milhões; auge 28 ou 29). O fator da idade (auge − idade, com a correção acima de [0x9688a0]) × as notas da ficha
/// (+0x28, +0x27, +0x43, +0x36, +0x2f, fora dos goleiros) e +0x2d²; a reputação (+9) limita o fator. A habilidade: sem
/// clube, a atual (+0xb); com clube, a vista pelo técnico (0x6e86d0 se `habilidade` = −1, com a reputação vista 0x5407e0)
/// × 50 + a reputação + até 5000 × fator, × [0x968f08], entre 1 e 12000; o potencial (+0xd) acima de 5000 (e 8000) antes
/// do auge a puxa. Valor = habilidade² × base × a tabela 0x9a7f30 (pela idade, com a experiência +0x22/+0x1a) × a nação
/// do clube (+0x53: +0x7e e +0x85 ou a divisão +0x69; sem clube, exe::utils_random(3) + 1), com piso. `modo`: a nação [0x9cf434]
/// e a cláusula. Senão: os dias de contrato (com a lei Bosman 0x8d0a70) e as nações [0x9cf434], [0x9cf300], [0x9cf484];
/// no fim a cláusula de rescisão ativa (0x867b50) limita.
pub(crate) unsafe extern "thiscall" fn calc_player_value(this: usize, person: usize, club: usize, mode: u32, ability: u32, _x: u32) -> i32 {
    let person = Staff(person);
    let club = Club(club);
    let c1080 = Contract(main_contract_of_staff(person));
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c1080.addr();
    let transfer_manager_contract_club = if c1080.is_null() { 0 } else { friendly_club_by_id(c1080.club()) };
    let contract = Contract(exe::contract_manager_get_contract_for_club(this, person, transfer_manager_contract_club));
    let sheet = person.player();
    if sheet == 0 {
        if transfer_manager_contract_club == 0 {
            return 0;
        }
        // the original sets ecx = [0xde1f54] (the Rust copy is stdcall): thiscall with 3 on the stack works for both
        return exe::transfer_manager_calc_compensation_fee(read_usize(G_TRANSFER_MANAGER), person, transfer_manager_contract_club, 1);
    }
    let (base, auge): (f64, i8) = if read_i8(sheet + player_sheet::ATTACKER) >= 0xf {
        (13_000_000.0, 0x1c) // cqa9: the base of those who play up front
    } else if read_i8(sheet + player_sheet::ATTACKING_MIDFIELDER) >= 0xf {
        (11_000_000.0, 0x1d)
    } else if read_i8(sheet + player_sheet::MIDFIELDER) >= 0xf {
        (10_000_000.0, 0x1d)
    } else if read_i8(sheet + player_sheet::DEFENSIVE_MIDFIELDER) >= 0xf {
        (9_000_000.0, 0x1d)
    } else if read_i8(sheet + player_sheet::GOALKEEPER) == 0x14 {
        (6_000_000.0, 0x21)
    } else {
        (8_000_000.0, 0x1d)
    };
    let age = (person.age() as i8);
    let dif = auge as i32 - age as i32;
    let mut t = dif as f64;
    if t > read_f64(rdata::F64_5) {
        t = read_f64(rdata::F64_8) - t * read_f32(rdata::F32_0_5) as f64;
    }
    let a = ftol(t).wrapping_abs();
    let mut f = t * a as f64 * read_f32(rdata::F32_0_3333333433) as f64 * read_f64(rdata::F64_0_08333333333) + read_f64(rdata::F64_1);
    if read_i8(sheet + player_sheet::GOALKEEPER) < 0xf {
        let rating = |o: usize, c: usize| (read_i8(sheet + o) as i32 - 0xf) as f64 * read_f64(c) + read_f64(rdata::F64_1);
        let fa = rating(0x28, 0x968ef8);
        let fb = read_i8(sheet + player_sheet::FINISHING) as f64 * read_f64(rdata::F64_0_005) + read_f64(rdata::F64_1);
        let fc = read_i8(sheet + player_sheet::VISION) as f64 * read_f64(rdata::F64_0_003333333333) + read_f64(rdata::F64_1);
        let fd = rating(0x36, 0x968ef8);
        let fe = rating(0x2f, 0x968ef8);
        f = fa * (fb * f) * fc * fd * fe;
    }
    let q = read_i8(sheet + player_sheet::INJURY_PRONENESS) as i32;
    let factor = ((read_f64(rdata::F64_1_33) - (q * q) as f64 * read_f64(rdata::F64_0_05) * read_f64(rdata::F64_0_034)) * f) as f32;
    let mut r = read_i16(sheet + player_sheet::HOME_REPUTATION) as f64 * read_f64(rdata::F64_0_0003333333333) + read_f64(rdata::F64_0_17);
    if !(r >= read_f64(rdata::F64_1_1)) {
        r = read_f64(rdata::F64_1_1);
    }
    let x: f32 = if !(factor as f64 >= read_f64(rdata::F64_0_01)) {
        read_f64(rdata::F64_0_01) as f32
    } else if !(r >= factor as f64) {
        r as f32
    } else {
        factor
    };
    let mut v: u32 = if club.is_null() {
        read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32 as u32
    } else {
        let manager = club.manager().addr();
        let rep = (exe::database_get_staff_reputation_for_viewer(person, manager, 0) as u16) as i16 as i32;
        let h = if ability as u16 == 0xffff {
            exe::match_man_judge_player_ability(manager, person, club, 1, rep as i16) as u16
        } else {
            ability as u16
        };
        // only the ax of h × 50 counts (movsx afterwards); negative becomes 0
        let h50 = (h.wrapping_mul(50) as i16).max(0);
        let ceiling = if (rep as u32) < 0x1388 { rep as u32 } else { 0x1388 };
        let s = (h50 as i32).wrapping_add(rep) as u32;
        let y = ftol((s as f64 + ceiling as f64 * x as f64) * read_f32(rdata::F32_0_3333333433) as f64) as u32;
        if y <= 1 { 1 } else if y > 0x2ee0 { 0x2ee0 } else { y }
    };
    let pot = read_i16(sheet + player_sheet::WORLD_REPUTATION);
    if pot > 0x1388 && (age as i32) < auge as i32 + 4 {
        let c = (v >> 1).wrapping_add(pot as i32 as u32).wrapping_sub(0x9c4);
        if c > v {
            v = c;
        }
        if pot > 0x1f40 && age <= auge {
            let z = pot as f64 * read_f64(rdata::F64_2_25) + (v >> 1) as f64 - read_f64(rdata::F64_12500);
            let e = v as f64;
            v = ftol(if z > e { z } else { e }) as u32;
        }
    }
    let vv = v.wrapping_mul(v) as f64 * base * read_f64(rdata::F64_1ENEG_08);
    let mut age_ef = age;
    let exp = person.int_apps();
    if exp != 0 && !person.nation().is_null() {
        let n = (read_i16(person.nation().addr() + 0x8e) as i32).wrapping_mul(exp as i32) / 18;
        let n = n as i16 as i32;
        let m = if n < dif { n } else { dif };
        let sum = if m < 0 { 0 } else if n >= dif { dif } else { n };
        age_ef = age_ef.wrapping_add(sum as i8);
    }
    let mut i = auge.wrapping_sub(age_ef).wrapping_add(10); // cqa9: the table index by age
    if i < 0 {
        i = 0;
    } else if i >= 0x14 {
        i = 0x13;
    }
    let table = *((0x9a7f30 + i as usize) as *const i8) as i32;
    let mut w = table as f64 * vv * read_f64(rdata::F64_0_01);
    let c890 = read_f64(rdata::F64_1);
    let k = if club.is_null() {
        (exe::utils_random(3) + 1) as f64
    } else {
        let div = Competition(club.division().addr());
        let nat = club.nation().addr();
        if nat == 0 {
            c890
        } else {
            let mut t = read_i8(nat + nation::STATE_OF_DEVELOPMENT) as f64;
            if !(t >= c890) || t > read_f64(rdata::F64_3) {
                t = read_f64(rdata::F64_3);
            }
            if !div.is_null() {
                t = (t + c890) * read_f64(rdata::F64_0_5);
            }
            match (club.professional_status() as i8) {
                2 => t = t + t,
                3 => t *= read_f64(rdata::F64_7_5),
                _ => {}
            }
            let n = if !div.is_null() { div.reputation() as i32 } else { read_i8(nat + nation::LEAGUE_STANDARD) as i32 };
            let e = n.wrapping_mul(n).wrapping_mul(25).wrapping_add(club.reputation() as i32);
            e as f64 / (t * read_f64(rdata::F64_1000))
        }
    };
    let dd = read_f64(rdata::F64_20) - k;
    let mut l = c890 - dd * dd * read_f64(rdata::F64_0_2) * read_f64(rdata::F64_0_02);
    if !(l >= read_f64(rdata::F64_0_1)) {
        l = read_f64(rdata::F64_0_1);
    } else if l > read_f64(rdata::F64_1_1) {
        l = read_f64(rdata::F64_1_1);
    }
    w = l * w;
    let k2 = k * k;
    if !(k2 * read_f64(rdata::F64_65000) >= w) {
        w = w * read_f64(rdata::F64_0_25) + k2 * read_f64(rdata::F64_32500);
    }
    let floor = k * read_f64(rdata::F64_5000);
    if !(w >= floor) {
        w = floor;
    }
    let nation_and = |g: usize| -> bool {
        let nat = club.nation().addr();
        !club.is_null() && nat != 0 && read_usize(nat) == read_usize(g)
    };
    // the nation [0x9cf434]: below [0x968e90] the curve with a floor, otherwise × [0x9689d8]
    let curva = |w: f64, floor: f64| -> f64 {
        if !(w >= read_f64(rdata::F64_6000000)) {
            let v = (w * read_f64(rdata::F64_1_666666667ENEG_07) * read_f64(rdata::F64_0_6) + read_f64(rdata::F64_0_3)) * w;
            if !(read_f64(rdata::F64_5000) > v) { v } else { floor }
        } else {
            w * read_f64(rdata::F64_0_9)
        }
    };
    if mode as u8 != 0 {
        if nation_and(KEY_NATION_PORTUGAL) {
            w = curva(w, 5000.0);
        }
        return cap_value_at_release_fee(contract, w);
    }
    let mut nations = true; // 0x4d1480 (the other nations) or 0x4d14ad (the [0x9cf434] one before)
    if contract.is_null() {
        w = read_f64(rdata::F64_0);
    } else {
        if exe::transfer_manager_player_club_uses_bosman(read_usize(G_TRANSFER_MANAGER), person) != 0 {
            let year = |a: usize| core::ptr::read_unaligned(a as *const u16) as i32;
            let mut days = (year(contract.addr() + contract::CONTRACT_EXPIRES_YEAR) - year(G_CURRENT_DATE_YEAR)).wrapping_mul(365) + contract.contract_expires() as i32 - read_i16(G_CURRENT_DATE) as i32;
            if days < 1 {
                return 0;
            }
            let mut w2 = w;
            if club.addr() == transfer_manager_contract_club && contract.happiness() > 0 {
                if contract.contract_type() & 0x3f == 2 {
                    days = days + 7 * contract.happiness() as i32 + 0xc8;
                } else {
                    w2 = w * read_f64(rdata::F64_0_5);
                }
            }
            let x = days as f64 * read_f64(rdata::F64_0_001428571429);
            if days < 0x15e { // cqa9: o contrato curto
                w = w2 * x;
            } else {
                let mut dv = read_f64(rdata::F64_2_5) - x;
                if !(dv >= c890) {
                    dv = read_f32(rdata::F32_1) as f64;
                }
                if dv > read_f64(rdata::F64_2) {
                    dv = read_f32(rdata::F32_2) as f64;
                }
                w = w2 / dv;
            }
        }
        nations = false;
    }
    if !nations {
        // 0x4d14ad: zero (or unordered) value goes to the other nations; with no club, to the end
        if !(w < read_f64(rdata::F64_0) || w > read_f64(rdata::F64_0)) {
            nations = true;
        } else if club.is_null() {
            return cap_value_at_release_fee(contract, w);
        } else if nation_and(KEY_NATION_PORTUGAL) {
            w = curva(w, read_f64(rdata::F64_5000));
            return cap_value_at_release_fee(contract, w);
        } else {
            nations = true;
        }
    }
    if nations && !club.is_null() {
        if nation_and(KEY_NATION_FRANCE) {
            w *= read_f64(rdata::F64_0_8);
        } else if nation_and(KEY_NATION_SOUTH_KOREA) {
            w *= read_f64(rdata::F64_0_25);
            let c = read_f64(rdata::F64_125000);
            if w > c {
                w = ftol((w - c) * read_f64(rdata::F64_0_5) + c) as f64;
            }
            let c = read_f64(rdata::F64_250000);
            if w > c {
                w = ftol((w - c) * read_f64(rdata::F64_0_5) + c) as f64;
            }
        }
    }
    cap_value_at_release_fee(contract, w)
}

/// Memory regions the calc_player_value comparer watches: RNG state, date statics and the contract lookup global
/// 0xae1080.
///
/// [pt] As estáticas das datas (DATE_OP_STATICS) e o contrato guardado em [0xae1080]: o que a original e as chamadas dela
/// gravam, com o estado do sorteador (na pilha: a rotina é quente).
pub(crate) fn player_value_regions() -> [(usize, usize); 11] {
    let (e, c) = (crate::rng::STATE_REGIONS, DATE_OP_STATICS);
    [e[0], e[1], (G_CONTRACT_LOOKUP_RESULT, 4), c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]
}

/// `contract_manager_write_morale_comparison_text(a, b, out, _)` (stdcall, ret 0x10): writes the morale difference of
/// two players as a word, for the comparison screen.
///
/// The difference of the morales (+0x45 of the sheets, a − b, as a byte) becomes "Superior" (down to −14 and from 13),
/// "Better" (−13 to −8 and 7 to 12), "Marginal" (−7 to −4 and 3 to 6) or "Same" (−3 to 2) in the output (0x66f4e0 with
/// 0xad9c64); returns the difference (the game reads al).
/// Original: `contract_manager_write_morale_comparison_text` (0x4cc3a0, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4cc3a0 } (stdcall, ret 0x10, contract_manager.cpp): `contract_manager_write_morale_comparison_text(a, b, saída,
/// _)`: a diferença das morais (+0x45 das fichas, a − b, em byte) vira "Superior" (até −14 e desde 13), "Better" (−13 a
/// −8 e 7 a 12), "Marginal" (−7 a −4 e 3 a 6) ou "Same" (−3 a 2) na saída (0x66f4e0 com 0xad9c64); devolve a diferença
/// (o jogo lê o al), pela tela de comparação.
pub(crate) unsafe extern "stdcall" fn contract_manager_write_morale_comparison_text(a: usize, b: usize, out: usize, _x: u32) -> u8 {
    let b = Staff(b);
    let a = Staff(a);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let dif = read_i8(a.player() + 0x45).wrapping_sub(read_i8(b.player() + 0x45));
    let f = if dif < -13 {
        0x9a8348usize
    } else if dif < -7 {
        0x9a83bc
    } else if dif < -3 {
        0x9a8394
    } else if dif < 3 {
        0x9a8370
    } else if dif < 7 {
        0x9a8394
    } else if dif < 13 {
        0x9a83bc
    } else {
        0x9a8348
    };
    format(out, f, 0xad9c64usize);
    dif as u8
}

/// `CONTRACT_MANAGER::get_squad_flags_for_club(person, club)`, called by the club screens: returns the squad flags the
/// club screen shows.
///
/// The person's squad flags (+0x5e without the national side bits, & 0xf3), unless she is on loan (second contract, +4
/// of the 79-byte record) to a club other than `club` or its partner (`exe::club_get_linked_b_team`): with a partner, 1; otherwise 2 with
/// bit 0x40 of +0x35 of the main contract (the reserve bit), 1 without. Without a club, the "Error" box (the original).
/// The game reads al.
/// Original: `CONTRACT_MANAGER::get_squad_flags_for_club` (0x4cc130, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cc130 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_flags_for_club(pessoa, clube)`,
/// pelas telas do clube: as marcas de elenco da pessoa (+0x5e sem os bits de seleção, & 0xf3), a não ser que ela esteja
/// emprestada (segundo contrato, +4 do registro de 79 bytes) a outro clube que não o `clube` nem o parceiro dele
/// (0x540a50): com parceiro, 1; senão 2 com o bit 0x40 de +0x35 do contrato principal, 1 sem. Sem clube, a caixa
/// "Error" (a original). O jogo lê o al.
pub(crate) unsafe extern "thiscall" fn get_squad_flags_for_club(this: usize, person: usize, club: usize) -> u8 {
    let person = Staff(person);
    if club == 0 || person.is_null() {
        // simplification: without a club the error box; without a person the original reads [0x5e] and crashes: both stay with the original
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB)(this, person.addr(), club);
    }
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let general = || person.playing_squad() & 0xf3;
    if read_usize(person.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return general();
    }
    let reg = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person.addr()).wrapping_mul(0x4f));
    let i = read_usize(reg + 4) as i32;
    if i < 0 {
        return general();
    }
    let c = Contract((i as usize).wrapping_mul(0x50).wrapping_add(read_usize(this)));
    if c.is_null() || c.club() == read_usize(club) {
        return general();
    }
    let p = partner(club, core::ptr::null_mut(), 1);
    if p != 0 && c.club() == read_usize(p) {
        return general();
    }
    if partner(club, core::ptr::null_mut(), 1) != 0 {
        return 1;
    }
    let i0 = read_usize(reg) as i32;
    if i0 < 0 {
        // the original reads [0x35] of the null contract and crashes
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB)(this, person.addr(), club);
    }
    let c0 = Contract((i0 as usize).wrapping_mul(0x50).wrapping_add(read_usize(this)));
    (c0.contract_type() & 0x40 != 0) as u8 + 1
}

/// Calls STAFF_CONTRACT::calc_contract_value (contract, person, offer) → value.
///
/// [pt] O valor de um contrato para a pessoa (STAFF_CONTRACT::calc_contract_value 0x866830: thiscall(contrato, pessoa,
/// oferta), ret 8).
pub(crate) unsafe fn calc_contract_value(contract: usize, person: Staff, offer: usize) -> i32 {
    exe::staff_contract_calc_contract_value(contract, person, offer)
}

/// Writes a person's id into news slot 2 and the +0x33 keys of +4/+8/+0xc into slots 3-5.
///
/// [pt] Os dados da pessoa numa notícia: o id na vaga 2 e as três chaves (+0x33 de +4, +8 e +0xc) nas vagas 3 a 5.
pub(crate) unsafe fn set_news_staff_slots(n: usize, p: Staff) {
    exe::newsitem_set_slot(n, 2, read_usize(p.addr()) as u32);
    exe::newsitem_set_slot(n, 3, read_usize(p.first_name() + 0x33) as u32);
    exe::newsitem_set_slot(n, 4, read_usize(p.second_name() + 0x33) as u32);
    exe::newsitem_set_slot(n, 5, read_usize(p.common_name() + 0x33) as u32);
}

/// `CONTRACT_MANAGER::post_wants_new_contract_news(contract)` (SEH frame for the local news item), called by the daily
/// staff pass (0x5ce2d0): posts news item 0xbc2, that the player wants a new contract, to the club manager, unless he
/// is transfer listed or in an offer.
///
/// Without a contract: the "Error" box (line 0x2d91). A person on the transfer list (`person_in_transfer_list`), with offers (`get_offers_for_player`)
/// or offered in exchange (`get_exchange_offers_for_player`): nothing. Otherwise news item 0xbc2 to the manager of the contract club
/// (`send_to_club_manager`): the club, the person, the squad status (+0x4f >> 4), 0, the contract end (+0x2d signed day, +0x2f year),
/// the wage (+0xc) and, in slot 11, 1 if the adaptability (+0x59) is below 10, +0x5b below 5 and the sheet's reputation
/// (+0xd) above 4750.
/// Original: `post_wants_new_contract_news` (0x4d9f80, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d9f80 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_wants_new_contract_news(contrato)`, pelo passe diário do pessoal (0x5ce2d0). Sem contrato: a caixa "Error"
/// (linha 0x2d91). Pessoa na lista de transferências (0x8c1870), com ofertas (0x8c5cf0) ou oferecida em troca (0x8c5e40):
/// nada. Senão a notícia 0xbc2 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, a situação no elenco (+0x4f
/// >> 4), 0, o fim do contrato (+0x2d dia com sinal, +0x2f ano), o salário (+0xc) e, na vaga 11, 1 se a adaptabilidade
/// (+0x59) for menor que 10, a +0x5b menor que 5 e a reputação da ficha (+0xd) passar de 4750.
pub(crate) unsafe extern "thiscall" fn post_wants_new_contract_news(this: usize, contract: usize) {
    let contract = Contract(contract);
    let _ = this;
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x2d91);
    }
    let club = friendly_club_by_id(contract.club());
    let p = Staff(staff_records_staff_by_id(read_usize(contract.addr())));
    let tm = read_usize(G_TRANSFER_MANAGER);
    if exe::transfer_manager_is_queued_for_free_transfer(tm, p) != 0 || exe::transfer_manager_get_offers_for_player(tm, p, 0, 0) != 0 || exe::transfer_manager_get_exchange_offers_for_player(tm, p, 0) != 0 {
        return;
    }
    crate::comparison::branch("cpw_1 new contract news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbc2, 0);
    exe::newsitem_set_slot(n, 0, read_usize(club) as u32);
    set_news_staff_slots(n, p);
    exe::newsitem_set_slot(n, 6, (contract.squad_status() >> 4) as u32);
    exe::newsitem_set_slot(n, 7, 0);
    exe::newsitem_set_slot(n, 8, contract.contract_expires() as i32 as u32);
    exe::newsitem_set_slot(n, 9, contract.contract_expires_year() as u16 as u32);
    exe::newsitem_set_slot(n, 10, (contract.wage() as usize) as u32);
    let sheet = p.player();
    let reputado = ((p.loyalty() as u8) as i8) < 0xa && ((p.professionalism() as u8) as i8) < 5 && sheet != 0 && read_i16(sheet + player_sheet::WORLD_REPUTATION) > 0x128e;
    exe::newsitem_set_slot(n, 11, reputado as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.

/// `CONTRACT_MANAGER::post_transfer_request_withdrawn_news(contract, reason)` (SEH frame for the local news item):
/// posts news item 0xbc0 of a withdrawn transfer request, with the reason, to the club manager.
///
/// Without a contract: the "Error" box (line 0x2ba6). Otherwise news item 0xbc0 to the manager of the contract club
/// (`send_to_club_manager`): the club, the person, the reason and, in slot 7, 1 if the contract's +0x3f is above 1.
/// Original: `post_transfer_request_withdrawn_news` (0x4d9460, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d9460 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_transfer_request_withdrawn_news(contrato, motivo)`. Sem contrato: a caixa "Error" (linha 0x2ba6). Senão a
/// notícia 0xbc0 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, o motivo e, na vaga 7, 1 se +0x3f do
/// contrato passar de 1.
pub(crate) unsafe extern "thiscall" fn post_transfer_request_withdrawn_news(this: usize, contract: usize, reason: u32) {
    let contract = Contract(contract);
    let _ = this;
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x2ba6);
    }
    crate::comparison::branch("cpw_1 request withdrawn");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbc0, 0);
    let club = friendly_club_by_id(contract.club());
    let p = staff_records_staff_by_id(read_usize(contract.addr()));
    exe::newsitem_set_slot(n, 0, read_usize(club) as u32);
    set_news_staff_slots(n, Staff(p));
    exe::newsitem_set_slot(n, 6, reason);
    exe::newsitem_set_slot(n, 7, ((contract.matches_left_out() as i8) > 1) as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.

/// Main contract of a person through CONTRACT_MANAGER::get_contract (`exe::contract_manager_get_contract`).
///
/// [pt] O contrato principal da pessoa (0x4cf5a0).
pub(crate) unsafe fn main_contract_via_manager(p: usize) -> usize {
    exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p)
}

/// Calls Shortlist_Manager::staff_interest_in_joining_club → float interest of the person in joining the club.
///
/// [pt] O interesse da pessoa em ir para o clube (Shortlist_Manager::staff_interest_in_joining_club 0x846920: thiscall(lista,
/// pessoa, clube, salário, modo, situação, motivos), ret 0x18, float no st0).
pub(crate) unsafe fn staff_interest_in_club(list: usize, person: usize, club: Club, salary: u32, mode: u32, status: u32) -> f64 {
    exe::shortlist_manager_staff_interest_in_joining_club(list, person, club, salary, mode, status, (0) as u32)
}

/// The comparer's dry run: the copy runs without the loan offer (`make_loan_offer`) or the list update (`update_staff_status`), which
/// allocate and touch the lists; it returns −2 where it would make the offer and −3 where it would update the list.
///
/// [pt] O ensaio do comparador: a cópia roda sem a oferta de empréstimo (0x8bed60) nem a atualização da lista (0x843e10),
/// que alocam e mexem nas listas; devolve −2 onde faria a oferta e −3 onde atualizaria a lista.
pub(crate) static mut LOAN_OFFER_DRY_RUN: bool = false;

/// True when 8-byte date a (day i16, year u16) is before date b, compared as the original does.
///
/// [pt] A data (dia i16, ano u16) de 8 bytes em `a`, como a original a compara (o ano sem sinal, o dia com sinal).
pub(crate) unsafe fn contract_manager_date_is_before(a: usize, b: usize) -> bool {
    let (aa, ab) = (read_i16(a + cm_date::YEAR) as u16, read_i16(b + cm_date::YEAR) as u16);
    aa < ab || (aa == ab && read_i16(a) < read_i16(b))
}

/// Calls `nation_allows_free_transfers` (cdecl): whether the nation is active / allows free transfers.
///
/// [pt] A nação está ativa (0x544670, cdecl)?
pub(crate) unsafe fn contract_manager_nation_allows_free_transfers(nation: usize) -> bool {
    exe::nation_allows_free_transfers(nation) != 0
}

/// `CONTRACT_MANAGER::post_contract_expiry_news(contract, period, bosman, notify_interested)` (SEH frame for the local
/// news item and list), called by the contracts pass: posts news item 0xbbd of an expiring contract to the club manager
/// and, if asked, to interested clubs of active nations.
///
/// Without a contract: the "Error" box (line 0x24f0). Otherwise news item 0xbbd to the club's manager (`send_to_club_manager`): the
/// club, the person, the period (signed), `bosman` and, in slot 8, whether the person has a player sheet (+0x61). With
/// `notify_interested` (cancelled if the club's nation, +0x53, is not active, `nation_allows_free_transfers`): the clubs interested in the
/// player (`collect_interested_clubs` into a local LLIST, with 1, 1, 0) from another active nation also get it.
/// Original: `post_contract_expiry_news` (0x4d6790, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d6790 } (thiscall, ret 0x10, contract_manager.cpp, quadro SEH da notícia e da lista locais): `CONTRACT_MANAGER::
/// post_contract_expiry_news(contrato, período, bosman, avisar_interessados)`, pelo passe dos contratos. Sem contrato: a
/// caixa "Error" (linha 0x24f0). Senão a notícia 0xbbd ao técnico do clube (0x7861e0): o clube, a pessoa, o período (com
/// sinal), `bosman` e, na vaga 8, se a pessoa tem ficha (+0x61). Com `avisar_interessados` (anulado se a nação do clube,
/// +0x53, não estiver ativa, 0x544670): os clubes interessados no jogador (0x842180 numa LLIST local, com 1, 1, 0) de
/// outra nação ativa também a recebem.
pub(crate) unsafe extern "thiscall" fn post_contract_expiry_news(this: usize, contract: usize, period: u32, bosman: u32, warn: u32) {
    let contract = Contract(contract);
    let _ = this;
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x24f0);
    }
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbbd, 0);
    let club = Club(friendly_club_by_id(contract.club()));
    let p = Staff(staff_records_staff_by_id(read_usize(contract.addr())));
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    set_news_staff_slots(n, p);
    exe::newsitem_set_slot(n, 6, period as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 7, bosman);
    exe::newsitem_set_slot(n, 8, (p.player() != 0) as u32);
    let nacao = club.nation().addr();
    let warn = if nacao != 0 && !contract_manager_nation_allows_free_transfers(nacao) { 0 } else { warn };
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    if warn == 0 {
        return;
    }
    crate::comparison::branch("cpw_3 notify interested parties");
    let mut list = [0u8; 0xe];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    exe::shortlist_manager_collect_interested_clubs(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let mut x = exe::llist_get_next(l);
    while x != 0 {
        let id = read_usize(x + 4);
        if id != read_usize(club.addr()) {
            let nation = read_usize(friendly_club_by_id(id) + 0x53);
            if nation != 0 && contract_manager_nation_allows_free_transfers(nation) {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, friendly_club_by_id(read_usize(x + 4)));
            }
        }
        x = exe::llist_get_next(l);
    }
    exe::drop_llist(l);
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// `CONTRACT_MANAGER::post_bosman_departure_news(contract, destination)` (SEH frame for the local news item): posts
/// news item 0xbbe of a Bosman departure.
///
/// Without a contract: the "Error" box (line 0x2611). Otherwise news item 0xbbe to the club's manager (`send_to_club_manager`): the
/// club, the person and the destination club (−1 without one).
/// Original: `post_bosman_departure_news` (0x4d6d70, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d6d70 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_bosman_departure_news(contrato, destino)`. Sem contrato: a caixa "Error" (linha 0x2611). Senão a notícia 0xbbe ao
/// técnico do clube (0x7861e0): o clube, a pessoa e o clube de destino (−1 sem ele).
pub(crate) unsafe extern "thiscall" fn post_bosman_departure_news(this: usize, contract: usize, target: usize) {
    let contract = Contract(contract);
    let _ = this;
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x2611);
    }
    crate::comparison::branch("cpw_3 departure under the Bosman ruling");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbbe, 0);
    let club = friendly_club_by_id(contract.club());
    let p = staff_records_staff_by_id(read_usize(contract.addr()));
    exe::newsitem_set_slot(n, 0, read_usize(club) as u32);
    set_news_staff_slots(n, Staff(p));
    exe::newsitem_set_slot(n, 6, if target != 0 { read_usize(target) as u32 } else { 0xffff_ffff });
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.

/// Address of the person's contract offer counter (byte +8 of the 79-byte staff state record).
///
/// [pt] O contador de ofertas de contrato da pessoa (byte +8 do registro de 79 bytes em [0xae2d0c]).
pub(crate) unsafe fn contract_offer_counter(person: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)).wrapping_add(8)
}

/// Assigns date origem to destino with the game's date operator (`exe::date_operator_3d`).
///
/// [pt] A atribuição de data (0x417c10: thiscall(destino, temporária, origem)).
pub(crate) unsafe fn contract_manager_copy_date(target: usize, source: usize) {
    let mut t = [0u32; 2];
    exe::date_operator_3d(target, (&mut t) as *mut [u32; 2] as usize, source);
}

/// Finds the contract offer of person id from club in list l without moving the cursor; 0 if none.
///
/// [pt] A oferta de contrato da pessoa (id em +0) pelo clube (id em +4) na lista `l` (nós: +0 o item, +6 o próximo), sem
/// mexer no cursor; nenhuma: 0.
pub(crate) unsafe fn find_contract_offer(l: usize, id: usize, club: usize) -> usize {
    let mut no = read_usize(l + llist::HEAD);
    let mut n = 0;
    while no != 0 && n < 100000 {
        let x = read_usize(no);
        if read_usize(x) == id && read_usize(x + 4) == club {
            return x;
        }
        no = read_usize(no + llist_node::NEXT);
        n += 1;
    }
    0
}

/// The comparer's dry run: the copy does not create or append to the list (`exe::crt_operator_new`, `exe::new_llist`, 0x68d0d0) and does not
/// call what comes after (`note_terms_offered_to_main_player`, `note_terms_offered_to_exchange_player`, `schedule_offer_decision`), which allocate, draw random numbers and change the offers.
///
/// [pt] O ensaio do comparador: a cópia não cria nem acrescenta à lista (0x944cf1, 0x68d020, 0x68d0d0) e não chama o que vem
/// depois (0x8c1f80, 0x8c2060, 0x4cd230), que alocam, sorteiam e mexem nas ofertas.
pub(crate) static mut CONTRACT_OFFER_DRY_RUN: bool = false;

/// `CONTRACT_MANAGER::add_contract_offer(terms, slot, value, flag, a5, decide, agreed)` (SEH frame without a
/// destructor), called by the loan offer (`make_loan_offer`), the stored offers (`resubmit_held_offers`), the lists (`make_club_bids`), the
/// negotiation (`contract_manager_offer_contract`) and the screen: updates the same club's offer to the person or adds a new one to the list;
/// marks the terms as offered on the transfer offer and, if asked, schedules the decision.
///
/// Computes today + 14 (`exe::date_add_days`, discarded) and builds a local 0x6f-byte record (contract `construct_staff_contract`, date `exe::date_date_54a110`).
/// With counted offers (+8 of the person's 79-byte record in [0xae2d0c]) and a list (+0x34), the offer from the same
/// club is updated: the terms (the inlined operator= `contract_manager_operator`), +0x50 = 2 if `agreed` (otherwise 0), +0x51 = today,
/// +0x59 the slot, +0x5d the value, +0x61 the flag, +0x62 = 1 outside a type 6 contract. Otherwise the local record
/// gets the same (+0x63 = −1, the rest left as the original leaves it) and goes into the list (created when missing;
/// 0x68d0d0), and the offer count goes up. With a slot, the offer (`offer_slot`) of the same player marks the terms as
/// offered (`note_terms_offered_to_main_player`), otherwise the exchange player (`note_terms_offered_to_exchange_player`); with `decide`, it schedules the decision (`schedule_offer_decision`).
/// Original: `add_contract_offer` (0x4cc490, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cc490 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH sem destrutor): `CONTRACT_MANAGER::
/// add_contract_offer(termos, vaga, valor, marca, a5, decidir, acertado)`, pela oferta de empréstimo (0x8bed60), pelas
/// ofertas guardadas (0x4cf4a0), pelas listas (0x84a020), pela negociação (0x4cda70) e pela tela. Calcula hoje + 14 (0x54c5b0,
/// descartado) e monta um registro local de 0x6f bytes (contrato 0x85e410, data 0x54a110). Com ofertas contadas (+8 do
/// registro de 79 bytes da pessoa em [0xae2d0c]) e lista (+0x34), a oferta do mesmo clube é atualizada: os termos (o
/// operator= 0x4cc810 embutido), +0x50 = 2 se `acertado` (senão 0), +0x51 = hoje, +0x59 a vaga, +0x5d o valor, +0x61 a
/// marca, +0x62 = 1 fora do contrato de tipo 6. Senão o registro local recebe o mesmo (+0x63 = −1, o resto fica como a
/// original o deixa) e entra na lista (criada se faltar; 0x68d0d0), e o contador sobe. Com vaga, a oferta (0x8c61e0) do
/// mesmo jogador marca os termos oferecidos (0x8c1f80), senão o jogador da troca (0x8c2060); com `decidir`, agenda a
/// decisão (0x4cd230).
pub(crate) unsafe extern "thiscall" fn add_contract_offer(this: usize, terms: usize, slot: i32, value: u32, mark: u32, a5: u32, decides: u32, acertado: u32) {
    let terms = Contract(terms);
    let _ = a5;
    let mut quinzena = [0u32; 2];
    exe::date_add_days(G_CURRENT_DATE, (&mut quinzena) as *mut [u32; 2] as usize, 0xe);
    // simplification: bytes +0x67..+0x6e of the local record the original does not fill (stack leftover); the copy uses zero
    let mut record = [0u8; 0x6f];
    let r = record.as_mut_ptr() as usize;
    exe::new_staff_contract(r);
    exe::date_date_54a110(r + contract_offer::OFFER_DATE);
    let id = read_usize(terms.addr());
    let mut item = 0;
    if read_u8(contract_offer_counter(terms.addr())) != 0 && read_usize(this + contract_manager_fields::CONTRACT_OFFERS) != 0 {
        let club = terms.club();
        let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
        core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
        let mut x = exe::llist_get_next(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
        while x != 0 {
            if read_usize(x) == id && read_usize(x + 4) == club {
                item = x;
                break;
            }
            x = exe::llist_get_next(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
        }
    }
    let tipo6 = terms.contract_type() & 0x3f == 6;
    if item != 0 {
        crate::comparison::branch("cpw_4 offer updated");
        // simplification: the original inlines operator= (0x4cc810) here; the copy calls its Rust version, the same writes
        contract_manager_operator(item, terms.addr());
        *((item + contract_offer::STATE) as *mut u8) = if acertado != 0 { 2 } else { 0 };
        contract_manager_copy_date(item + contract_offer::OFFER_DATE, G_CURRENT_DATE);
        write_u32(item + contract_offer::TRANSFER_OFFER_SLOT, slot as u32);
        write_u32(item + contract_offer::TRANSFER_OFFER_SERIAL, value);
        *((item + contract_offer::MARK) as *mut u8) = mark as u8;
        *((item + contract_offer::DECISION_VALID) as *mut u8) = !tipo6 as u8;
    } else {
        exe::staff_contract_operator_3d(r, terms);
        *((r + contract_offer::STATE) as *mut u8) = 0;
        contract_manager_copy_date(r + contract_offer::OFFER_DATE, G_CURRENT_DATE);
        write_u32(r + contract_offer::TRANSFER_OFFER_SERIAL, value);
        write_u32(r + contract_offer::TRANSFER_OFFER_SLOT, slot as u32);
        *((r + contract_offer::MARK) as *mut u8) = mark as u8;
        write_u32(r + contract_offer::RESERVED_RECORD, 0xffff_ffff);
        *((r + contract_offer::DECISION_VALID) as *mut u8) = !tipo6 as u8;
        if CONTRACT_OFFER_DRY_RUN {
            core::ptr::copy_nonoverlapping(r as *const u8, core::ptr::addr_of_mut!(DRY_RUN_CONTRACT_OFFER_RECORD) as *mut u8, 0x6f);
        } else {
            if read_usize(this + contract_manager_fields::CONTRACT_OFFERS) == 0 {
                let p = exe::crt_operator_new(0xe);
                let l = if p != 0 { exe::new_llist(p) } else { 0 };
                write_u32(this + contract_manager_fields::CONTRACT_OFFERS, l as u32);
            }
            (exe::llist_add_tail(read_usize(this + contract_manager_fields::CONTRACT_OFFERS), r, 0x6f) as u32);
        }
        crate::comparison::branch("cpw_4 new offer");
        let c = contract_offer_counter(terms.addr()) as *mut u8;
        *c = (*c).wrapping_add(1); // cpw4: the offer counter of the person
    }
    if CONTRACT_OFFER_DRY_RUN {
        return;
    }
    if slot != -1 {
        let tm = read_usize(G_TRANSFER_MANAGER);
        let v = exe::transfer_manager_get_offer_slot(tm, slot);
        if read_usize(read_usize(v) + 0xc) == read_usize(terms.addr()) {
            crate::comparison::branch("cpw_4 terms to the player");
            exe::transfer_manager_note_terms_offered_to_main_player(read_usize(G_TRANSFER_MANAGER), slot); // cpw4: the terms offered to the player
        } else {
            exe::transfer_manager_note_terms_offered_to_exchange_player(read_usize(G_TRANSFER_MANAGER), slot, read_usize(terms.addr()) as u32);
        }
    }
    if decides != 0 {
        crate::comparison::branch("cpw_4 schedule decision");
        exe::contract_manager_schedule_offer_decision(this, read_usize(terms.addr()) as u32, (slot) as i32 as u32); // cpw4: the scheduled decision
    }
}

/// O registro que a cópia em ensaio acrescentaria à lista.
pub(crate) static mut DRY_RUN_CONTRACT_OFFER_RECORD: [u8; 0x6f] = [0; 0x6f];

/// `CONTRACT_MANAGER::remove_contract_offer_for_transfer(person, slot)`, called by the withdrawn and undone offers
/// (`reject_transfer_offer`, `withdraw_transfer_offer`, `exe::transfer_manager_cancel_club_offers`, `hold_offers_at_window_close`, 0x8d0f70, `exe::transfer_manager_cancel_offers_for_released_player`): removes the person's contract offer linked to the
/// transfer slot and lowers the offer count.
///
/// In the list of contract offers (+0x34; cursor restarted), the person's one with the slot (+0x59). When found: with
/// +0x63 flagged, the original `add_spare_non_player_record` of [0xb7434c] with +0x63, +0x67 and +0x6b; if agreed (+0x50 = 2), it clears the
/// agreed-contract flag (`clear_agreed_contract_flag`); removes it from the list (`manager_manager_remove`) and the person's offer count (+8 of the 79-byte
/// record) goes down.
/// Original: `remove_contract_offer_for_transfer` (0x4ccb30, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4ccb30 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer_for_transfer(pessoa,
/// vaga)`, pelas ofertas retiradas e desfeitas (0x8bf680, 0x8bfcb0, 0x8c4950, 0x8c4e10, 0x8d0f70, 0x8df9b0): na lista das
/// ofertas de contrato (+0x34; cursor recomeçado) a da pessoa com a vaga (+0x59). Achada: com +0x63 marcado, a original
/// 0x7a81a0 de [0xb7434c] com +0x63, +0x67 e +0x6b; acertada (+0x50 = 2), desliga a marca de contrato acertado (0x8c5430);
/// tira da lista (0x68d1f0) e o contador de ofertas da pessoa (+8 do registro de 79 bytes) desce.
pub(crate) unsafe extern "thiscall" fn remove_contract_offer_for_transfer(this: usize, person: usize, slot: i32) {
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let mut x = exe::llist_get_next(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    while x != 0 {
        if read_usize(x) == read_usize(person) && read_usize(x + contract_offer::TRANSFER_OFFER_SLOT) as i32 == slot {
            break;
        }
        x = exe::llist_get_next(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    }
    if x == 0 {
        return;
    }
    crate::comparison::branch("cpw_4 offer removed by the slot");
    if read_usize(x + contract_offer::RESERVED_RECORD) as i32 != -1 {
        exe::player_regen_add_spare_non_player_record(read_usize(G_PLAYER_REGEN), (read_usize(x + contract_offer::RESERVED_RECORD) as u32) as u32 as usize, (read_usize(x + contract_offer::RESERVED_RECORD_ARG1) as u32) as u32 as usize, (read_usize(x + contract_offer::RESERVED_RECORD_ARG2) as u32) as u32 as usize);
    }
    if read_u8(x + contract_offer::STATE) == 2 {
        exe::transfer_manager_clear_agreed_contract_flag(read_usize(G_TRANSFER_MANAGER), (person) as usize as *const u32);
    }
    exe::llist_remove(read_usize(this + contract_manager_fields::CONTRACT_OFFERS), x);
    let c = contract_offer_counter(person) as *mut u8;
    *c = (*c).wrapping_sub(1);
}

/// The lowest wage the person asks of the club (stdcall, ret 8; SEH frame without a destructor), for the contract
/// screen: the lower of the desired wages for squad status 1 and 2 at the club, at most 10 million.
///
/// In a local contract (`construct_staff_contract`), the desired one (0x85fc50 with −1, status 1 and then 2, 0xb), at most 10 million.
/// Original: `contract_manager_lowest_wage_demand` (0x4ce450, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4ce450 } (stdcall, ret 8, contract_manager.cpp, quadro SEH sem destrutor): o menor salário que a pessoa pede ao
/// clube: num contrato local (0x85e410), o desejado (0x85fc50 com −1, a situação 1 e depois 2, 0xb), no máximo 10 milhões.
/// Pela tela do contrato.
pub(crate) unsafe extern "stdcall" fn contract_manager_lowest_wage_demand(person: usize, club: usize) -> i32 {
    let mut local = [0u8; 0x50];
    let c = local.as_mut_ptr() as usize;
    exe::new_staff_contract(c);
    let mut smaller = 0x989680;
    let a = exe::staff_contract_set_to_desired_contract(c, person, club, -1, 1, 0xb, 0, 0);
    if a < smaller {
        smaller = a;
    }
    let b = exe::staff_contract_set_to_desired_contract(c, person, club, -1, 2, 0xb, 0, 0);
    if b < smaller { b } else { smaller }
}

/// `CONTRACT_MANAGER::handle_transfer_request(contract)` (SEH frame for the local news item): handles a player's
/// transfer request.
///
/// A null contract: the "Error" box 0x2999. A person without a player sheet (+0x61): nothing. A human club (`news_club_has_human_manager`):
/// the request news item (0xbbf), built on the stack (by the original). At a computer club, outside squad status 1
/// (+0x4f & 0xf0 = 0x10): if the value (`calc_player_value`) × 1.5 beats the asking price (`exe::contract_manager_calc_asking_price`), the club lists him (+0x4e:
/// clears bit 2 and sets 8; morale +12 in `staff_adjust_morale`; the club lists `add_player_to_club_shortlists`; `post_transfer_listed_news`). Otherwise a draw by the request
/// strength (+0x44) and the temperament (+0x59) against another of 50 gives the 0x10000 grievance (`exe::staff_contract_apply_event_flags`); the
/// morale drops 12 and the request strength grows by temperament × 5 + 56, held at ±100.
/// Original: `CONTRACT_MANAGER::handle_transfer_request` (0x4d8610, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4d8610 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// handle_transfer_request(contrato)`: contrato nulo: a caixa "Error" 0x2999. Pessoa sem ficha de jogador (+0x61): nada.
/// Clube humano (0x7875e0): a notícia do pedido (0xbbf), montada na pilha. No clube do computador, fora do status de
/// elenco 1 (+0x4f & 0xf0 = 0x10): se o valor (0x4d0c60) × 1,5 passa do preço pedido (0x4d1600), o clube o põe na lista
/// (+0x4e: tira o bit 2 e liga o 8; moral +12 em 0x544710; as listas dos clubes 0x842c00; 0x4d71b0). Senão um sorteio
/// pela força do pedido (+0x44) e pelo ânimo (+0x59) contra outro de 50 dá a queixa 0x10000 (0x867700); a moral cai 12
/// e a força do pedido sobe ânimo × 5 + 56, presa em ±100.
pub(crate) unsafe extern "thiscall" fn handle_transfer_request(this: usize, contract: usize) {
    let contract = Contract(contract);
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x2999);
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add(contract.club().wrapping_mul(club::SIZE));
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(contract.addr()).wrapping_mul(staff::SIZE)));
    if person.player() == 0 {
        return;
    }
    if exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
        // simplification: the human club receives the news item built on the stack (0x77bbe0 and the delivery are switched off): that
        // case stays with the original (which repeats the 0x7875e0 query, with no effect).
        // the record serves the proof of CMRTE’s ”Deixar infeliz” (make unhappy) (31/10/2026; deixar_infeliz.py): rare call
        linha(&format!("pedido de transferencia no clube humano: pessoa {}", read_usize(contract.addr())));
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HANDLE_TRANSFER_REQUEST)(this, contract.addr());
    }
    if contract.squad_status() & 0xf0 != 0x10 {
        let price: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
        let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
        let a = price(this, club, 0, person.addr(), 0, 1, 1);
        let v = value(this, person.addr(), club, 0, -1, 0);
        // the 1.5 is the double 0x968940
        if (v as f64) * 1.5 > a as f64 { // cps1: the value exceeds the price
            crate::comparison::branch("cps_1 listed by club");
            let m = (contract.addr() + contract::TRANSFER_STATUS) as *mut u8;
            if *m & 2 != 0 {
                *m &= 0xfd;
            }
            *m |= 8;
            exe::staff_adjust_morale(person, 0xc, 0);
            exe::shortlist_manager_add_player_to_club_shortlists(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), person, 1);
            exe::contract_manager_post_transfer_listed_news(this, person, club);
            return;
        }
    }
    let r = exe::utils_random((contract.morale() as u8) as i8 as i32 + ((person.loyalty() as u8) as i8 as i32 + 5) * 20);
    if r < exe::utils_random(0x32) { // cps1: the complaint random draw
        crate::comparison::branch("cps_1 request complaint");
        exe::staff_contract_apply_event_flags(contract, 0x10000, (0) as u32);
    }
    crate::comparison::branch("cps_1 request at computer club");
    exe::staff_adjust_morale(person, 0xffff_fff4, 0); // cps1: a moral cai 12
    // reread after the calls, as in the original (the 8-bit `imul` keeps the low byte)
    let al = (person.loyalty() as u8).wrapping_mul(5).wrapping_add(0x38);
    let cl = (contract.morale() as u8);
    let s = cl as i8 as i32 + al as i8 as i32;
    let m = (contract.addr() + contract::MORALE) as *mut u8;
    *m = if s > 100 {
        100
    } else if s < -100 {
        0x9c
    } else {
        cl.wrapping_add(al)
    };
}

/// 1 for jobs 0, 6 to 0xd and 0xf (those holding a contract manager contract)
/// Original: `is_contract_staff_job` (0x4cbb20, contract_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x4cbb20 } (stdcall, ret 4, contract_manager.cpp): `is_contract_staff_job(cargo)`: 1 para os cargos (byte) 0, 6 a
/// 0xd e 0xf, os que têm contrato pelo gerente de contratos; senão 0.
pub(crate) unsafe extern "stdcall" fn is_contract_staff_job(job: u32) -> u32 {
    matches!(job as u8, 0 | 6..=0xd | 0xf) as u32
}

/// 0 when the job (+0x3d) is a board job (1 to 4) at a club (+0x39); otherwise 1
/// Original: `staff_is_not_board_member` (0x4cb960, contract_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x4cb960 } (stdcall, ret 4, contract_manager.cpp): `staff_is_not_board_member(pessoa)`: 0 se o cargo (+0x3d) é de
/// diretoria (1 a 4) num clube (+0x39); senão 1.
pub(crate) unsafe extern "stdcall" fn staff_is_not_board_member(person: usize) -> u32 {
    let person = Staff(person);
    let job = person.job_for_club();
    if (1..=4).contains(&job) && !person.club_job().is_null() {
        return 0;
    }
    1
}

// simplification: draws (0x90cfc0), adds to the list (0x842c00, 0x4d71b0), changes morale and complaints (which may call the request for
// new): original only; the copy takes the branches.

/// Calls one of the game's date comparison functions f (>, <, ==) on date a with date b by value.
///
/// [pt] As comparações de data do jogo (thiscall na data `a`, a outra por valor): 0x456740 `>`, 0x456770 `<`, 0x4a7f20 `==`.
pub(crate) unsafe fn call_date_compare(f: usize, a: usize, b: usize) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(f);
    g(a, read_u32(b), read_u32(b + 4))
}

/// `compare_decision_dates(a, b)` (cdecl): the qsort order of the decision dates (14-byte entries: +0 person, +4 date,
/// +0xc time slice of the day, +0xd type).
///
/// `a`'s date earlier (`exe::date_operator_3c`): −1; later (`exe::date_operator_3e`): 1; on the same day, `a`'s slice minus `b`'s (signed). Hot (the
/// qsort): no counter.
/// Original: `compare_decision_dates` (0x4dc910, contract_manager.cpp, cdecl).
///
/// [pt] { 0x4dc910 } (cdecl, contract_manager.cpp): `compare_decision_dates(a, b)`: a ordem do qsort das datas de decisão
/// (entradas de 14 bytes: +0 pessoa, +4 data, +0xc fatia do dia, +0xd tipo): a data de `a` antes (0x456770): −1;
/// depois (0x456740): 1; no mesmo dia, a fatia de `a` menos a de `b` (com sinal). Quente (o qsort): sem contador.
pub(crate) unsafe extern "C" fn compare_decision_dates(a: usize, b: usize) -> i32 {
    if call_date_compare(0x456770, a + decision_date::DATE, b + decision_date::DATE) != 0 {
        return -1; // a before b
    }
    if call_date_compare(0x456740, a + decision_date::DATE, b + decision_date::DATE) != 0 {
        return 1;
    }
    read_u8(a + decision_date::SLICE) as i8 as i32 - read_u8(b + decision_date::SLICE) as i8 as i32
}

/// consumes the decision dates due by the given date and slice: each one becomes −1 and, for type 3, the person decides
/// on the contract offers; turned off (DESLIGADAS) from 2026-10-30 to 2026-10-31, when the test criterion changed and the list was emptied: the detour alone (without running the copy) made the career
/// in original mode leave the reference be54a138 (bisection of the 39 routines of batch 252): the detour changes the
/// stack residue a later original reads; the copy stays in the code and does not run
/// Original: `CONTRACT_MANAGER::process_due_decision_dates` (0x4cd9c0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cd9c0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::process_due_decision_dates(data, fatia)`:
/// do índice +0x18 até a conta +0x14 do vetor das datas de decisão (+0xc, entradas de 14 bytes, ordenadas pela
/// 0x4dc910): para na primeira data depois de `data` (0x456740) ou, no mesmo dia (0x4a7f20), com a fatia da entrada
/// maior que `fatia`. Cada entrada vencida com pessoa (+0 ≠ −1) fica com −1 e, sendo do tipo 3 (+0xd), a pessoa decide
/// as ofertas de contrato (0x4ce4e0). O índice +0x18 sobe a cada entrada (relido depois da decisão).
pub(crate) unsafe extern "thiscall" fn process_due_decision_dates(this: usize, data: usize, slice: u32) {
    let n = |c: usize| read_u32(c) as i32;
    if n(this + contract_manager_fields::DECISION_NEXT_INDEX) >= n(this + contract_manager_fields::DECISION_COUNT) {
        return;
    }
    loop {
        let e = (read_u32(this + contract_manager_fields::DECISION_DATES) as usize).wrapping_add((n(this + contract_manager_fields::DECISION_NEXT_INDEX) as usize).wrapping_mul(14));
        if call_date_compare(0x456740, e + decision_date::DATE, data) != 0 {
            return; // has not won yet
        }
        if call_date_compare(0x4a7f20, e + decision_date::DATE, data) != 0 && read_u8(e + decision_date::SLICE) as i8 > slice as u8 as i8 {
            return; // hoje, numa fatia depois desta
        }
        let id = n(e);
        if id != -1 {
            let person = (read_u32(G_STAFF_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            core::ptr::write_unaligned(e as *mut i32, -1);
            if read_u8(e + decision_date::KIND) == 3 {
                crate::comparison::branch("cpp_1 decide offers");
                exe::contract_manager_resolve_contract_offers(this, person); // the person decides the offers
            }
        }
        let i = n(this + contract_manager_fields::DECISION_NEXT_INDEX) + 1;
        core::ptr::write_unaligned((this + contract_manager_fields::DECISION_NEXT_INDEX) as *mut i32, i);
        if i >= n(this + contract_manager_fields::DECISION_COUNT) {
            return;
        }
    }
}

/// `CONTRACT_MANAGER::get_current_contract_offers(club, out)`: copies the club's open contract offers into a new array
/// of 111 bytes per offer and returns how many.
///
/// It walks the offers list (+0x34; cursor reset, `next_list_item`) and copies each offer of the club (+4 = id) outside states
/// 5 and 6 (+0x35 & 0x3f) into a new vector of 111 bytes per offer, which grows by 20 at a time (realloc `exe::crt_realloc` with
/// the capacity as a signed byte). Stores the vector in `out` and returns how many (byte). After the realloc the
/// original checks the list (+0x34), not the vector: only with a null list does it go to the "cannot allocate memory"
/// box (line 0x64c) and return 0 with a null output. The caller (0x598a06, the transfer budget) frees the vector.
/// Original: `CONTRACT_MANAGER::get_current_contract_offers` (0x4ca300, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4ca300 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_current_contract_offers(clube, saida)`:
/// percorre a lista das ofertas (+0x34; cursor reposto, 0x68d1b0) e copia cada oferta do clube (+4 = id) fora dos
/// estados 5 e 6 (+0x35 & 0x3f) para um vetor novo de 111 bytes por oferta, que cresce de 20 em 20 (realloc 0x945667
/// com a capacidade em byte com sinal). Grava o vetor em `saida` e devolve quantas (byte). A original confere a lista
/// (+0x34), não o vetor, depois do realloc: só com a lista nula vai à caixa "cannot allocate memory" (linha 0x64c) e
/// devolve 0 com a saída nula. Quem a chama (0x598a06, o orçamento de transferências) libera o vetor.
pub(crate) unsafe extern "thiscall" fn get_current_contract_offers(this: usize, club: usize, out: usize) -> u8 {
    let mut n: i8 = 0;
    let mut cap: i8 = 0;
    let mut array: usize = 0;
    let list = read_u32(this + contract_manager_fields::CONTRACT_OFFERS) as usize;
    if list != 0 {
        core::ptr::write_unaligned((list + llist::CURSOR) as *mut u32, read_u32(list + llist::HEAD));
        let mut item = exe::llist_get_next(list);
        while item != 0 {
            let state = read_u8(item + contract_offer::CONTRACT_TYPE) & 0x3f;
            if read_u32(item + contract_offer::CLUB_ID) == read_u32(club) && state != 5 && state != 6 {
                if cap == n {
                    cap = cap.wrapping_add(20);
                    array = exe::crt_realloc(array, (cap as i32 * 111) as u32);
                    if read_u32(this + contract_manager_fields::CONTRACT_OFFERS) == 0 {
                        show_cannot_create_space_box(0x64c, 0x9a8220); // "cannot allocate memory"
                        core::ptr::write_unaligned(out as *mut u32, 0);
                        return 0;
                    }
                }
                crate::comparison::branch("cpp_1 club offer");
                core::ptr::copy(item as *const u8, array.wrapping_add((n as i32 * 111) as usize) as *mut u8, 111);
                n = n.wrapping_add(1);
            }
            item = exe::llist_get_next(read_u32(this + contract_manager_fields::CONTRACT_OFFERS) as usize);
        }
    }
    core::ptr::write_unaligned(out as *mut u32, array as u32);
    n as u8
}

/// Volatile byte read (so a missing main contract crashes like the original).
pub(crate) unsafe fn read_byte_volatile(a: usize) -> u8 {
    // simplification: volatile read because the original reads [0 + 0x35] when the main contract is missing (crashes like it)
    core::ptr::read_volatile(a as *const u8)
}

/// Address of the person's 79-byte staff state record (+0 main contract index, +4 loan contract index; -1 none).
///
/// [pt] O registro de 79 bytes da pessoa em [0xae2d0c] (+0 o contrato principal, +4 o de empréstimo; −1 sem).
pub(crate) unsafe fn staff_state_entry(person: Staff) -> usize {
    (read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(person.addr()) as usize).wrapping_mul(0x4f))
}

/// Entry k (0x50 bytes) of the contract array starting at vetor.
///
/// [pt] A entrada `k` (0x50 bytes) do vetor de contratos que começa em `vetor`.
pub(crate) fn contract_at_index(array: u32, k: i32) -> usize {
    (array as usize).wrapping_add((k as usize).wrapping_mul(0x50))
}

/// Club record (0x245 bytes) of id in G_CLUB_TABLE.
///
/// [pt] O clube de id `id` (registro de 0x245 bytes em [0xae23b4]).
pub(crate) unsafe fn contract_manager_club_by_id(id: u32) -> usize {
    (read_u32(G_CLUB_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(club::SIZE))
}

/// Main contract of the person in the global CONTRACT_MANAGER (0 if none/out of range), also stored in the lookup
/// global 0xae1080.
///
/// [pt] O contrato principal da pessoa no CONTRACT_MANAGER global ([[0xae1050]]), como as originais o refazem: pessoa fora
/// da faixa (id ≥ [0xae2364]) ou sem contrato principal: 0. Grava em [0xae1080] (a ficha global).
pub(crate) unsafe fn main_contract_lookup(person: Staff) -> usize {
    let c = if read_u32(person.addr()) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_u32(staff_state_entry(person)) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), k) }
    };
    write_u32(G_CONTRACT_LOOKUP_RESULT, c as u32);
    c
}

/// Shows the contract_manager.cpp "Error" message box for source line `linha`, as the original routine does; the game
/// continues.
///
/// The text is "v%s %s.%s %d" at 0xae24d0 (`exe::crt_strtok` splits the path 0x9a8190, 0x90d130 the version; the original does
/// not pop its arguments, which are the rest of the format), then the box `exe::gui_show_message_box` and [0xb67a34] = 0.
///
/// [pt] A caixa "Error" do contract_manager.cpp na `linha`: "v%s %s.%s %d" em 0xae24d0 (0x944cff separa o caminho 0x9a8190,
/// 0x90d130 a versão; a original não desempilha os argumentos dela, que são o resto do formato), a caixa 0x5e8290 e
/// [0xb67a34] = 0. O jogo segue.
pub(crate) unsafe fn show_contract_error_box(row: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), row);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

// The database routines all three call (by address; the ported ones go through the hook).
/// Calls the club's linked A/B team lookup (`exe::club_get_linked_b_team`); sets whether the club is the A team.
pub(crate) unsafe fn contract_manager_linked_b_team(club: usize, eh_a: *mut u32) -> usize {
    exe::club_get_linked_b_team(club, eh_a, 1)
}
/// Calls the exe's Staff set-squad-flags routine for the person.
pub(crate) unsafe fn set_squad_flags(person: Staff, squad: u32) {
    exe::staff_set_squad_flags(person, squad);
}
/// Calls Tactics remove_player_from_selection on G_TACTICS for the club and person.
pub(crate) unsafe fn remove_from_tactics_selection(club: usize, person: Staff, a: u32, b: u32) {
    exe::tactics_remove_player_from_selection(read_u32(G_TACTICS) as usize, club, person, a, b);
}
/// Calls PlayerStats archive_player_stats on G_PLAYER_STATS for the person and club.
pub(crate) unsafe fn contract_manager_archive_player_stats(person: Staff, a4: u32, club: i32) {
    exe::playerstats_archive_player_stats(read_u32(G_PLAYER_STATS) as usize, person, -1, 0, a4, club, 0);
}
/// Calls Database remove_staff_from_club (club, person, date, flags).
pub(crate) unsafe fn contract_manager_remove_staff_from_club(club: usize, person: Staff, data: usize, disable: u32, como_jogador: u32) {
    exe::database_remove_staff_from_club(club, person, data, disable, como_jogador);
}
/// Calls Database add_staff_to_club (club, person, job, date, flags).
pub(crate) unsafe fn add_staff_to_club(club: usize, person: Staff, job: u32, data: usize, reduzir: u32, write: u32) {
    exe::database_add_staff_to_club(club, person, job, data, reduzir, write);
}
/// Calls Database club_has_reserve_squad for the club.
pub(crate) unsafe fn club_has_reserve_squad(club: usize) -> u32 {
    exe::database_club_has_reserve_squad(club)
}
/// Recomputes the club's wage bill (Finance::update_wage_bill on its entry in G_FINANCE_TABLE).
///
/// [pt] Refaz a folha de salários (0x598370) das finanças do clube (0x5a0590 em [0xae2a38]; a original chama mesmo sem).
pub(crate) unsafe fn update_club_wage_bill(club: usize) {
    exe::finance_update_wage_bill(exe::financetable_get_club_finance(read_u32(G_FINANCE_TABLE) as usize, club));
}
/// Assigns date origem to destino with the date operator (`exe::date_operator_3d`).
pub(crate) unsafe fn contract_manager_assign_date(target: usize, source: usize) {
    let mut ret = [0u8; 8];
    exe::date_operator_3d(target, (&mut ret) as *mut [u8; 8] as usize, source);
}

/// `CONTRACT_MANAGER::sign_loan_contract(contract)` (SEH frame with only a local date, no destructor): moves the person
/// to the loan club, marks the reserve bit in the main contract and stores the loan contract in her slot; recalculates
/// the wage bills.
///
/// The person (+0) of the new loan contract and her club by the main contract (the record [0xae1080]); without it,
/// nothing. Keeps the person's date +0x3e, takes her off the lists (`archive_player_stats` with the new club +4), checks the main
/// contract in the array itself (without it, the box of line 0x16a9 and nothing more) and sets in it bit 0x40 of +0x35
/// (in the reserves): with a reserve team (`database_club_has_reserve_squad`), by the squad mark +0x5e & 1; with a partner club (`exe::club_get_linked_b_team`), if
/// the current club (+0x39) is not the owner; otherwise clears it. Notifies the new club (`exe::virtual_staff_remove_virtual_players`), takes the person
/// out of the current club's tactics (`remove_player_from_selection` with 0 and 1), puts her into the new club as a player (`database_add_staff_to_club`, job 0xb
/// with today's date) and restores the date +0x3e.
///
/// The contract goes into the person's loan slot (+4 of the 79-byte record): the one she already has; otherwise the
/// first free one (−1) of the array, which takes the index (and then the original leaves without redoing the wage
/// bills); otherwise at the end (full: +100 slots, realloc `exe::crt_realloc` or malloc `exe::crt_malloc`; without memory, the box of line
/// 0x1708 and it goes on), through STAFF_CONTRACT::operator= `contract_manager_operator`. Finally it redoes the wage bills of the owning
/// club and the new one.
/// Original: `CONTRACT_MANAGER::sign_loan_contract` (0x4d01e0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4d01e0 } (thiscall, ret 4, contract_manager.cpp, quadro SEH só com uma data local, sem destrutor):
/// `CONTRACT_MANAGER::sign_loan_contract(contrato)`: a pessoa (+0) do contrato de empréstimo novo e o clube dela pelo
/// contrato principal (a ficha [0xae1080]); sem ele, nada. Guarda a data +0x3e da pessoa, tira-a das listas (0x7c5fe0
/// com o clube novo +4), confere o contrato principal no próprio vetor (sem ele, caixa da linha 0x16a9 e nada mais) e
/// marca nele o bit 0x40 de +0x35 (no reserva): com time reserva (0x542740), pela marca de elenco +0x5e & 1; com clube
/// parceiro (0x540a50), se o clube atual (+0x39) não é o dono; senão apaga. Avisa o clube novo (0x90d860), tira a pessoa
/// das táticas do clube atual (0x896c60 com 0 e 1), põe no clube novo como jogador (0x53bd10, cargo 0xb com a data de
/// hoje) e devolve a data +0x3e. O contrato vai para a vaga de empréstimo da pessoa (+4 do registro de 79 bytes): na
/// que ela já tem; senão na primeira livre (−1) do vetor, que fica com o índice (e aí a original sai sem refazer as
/// folhas); senão no fim (cheio: +100 vagas, realloc 0x945667 ou malloc 0x944e46; sem memória, caixa da linha 0x1708
/// e segue), pelo STAFF_CONTRACT::operator= 0x4cc810. Por fim refaz as folhas de salários do clube dono e do novo.
pub(crate) unsafe extern "thiscall" fn sign_loan_contract(this: usize, contract: usize) {
    let contract = Contract(contract);
    let mut stored = [0u8; 8];
    exe::date_date_54a110((&mut stored) as *mut [u8; 8] as usize);
    let person = Staff((read_u32(G_STAFF_TABLE) as usize).wrapping_add((read_u32(contract.addr()) as usize).wrapping_mul(staff::SIZE)));
    let sheet = Contract(if person.is_null() { write_u32(G_CONTRACT_LOOKUP_RESULT, 0); 0 } else { main_contract_lookup(person) });
    if sheet.is_null() {
        return;
    }
    let owner = contract_manager_club_by_id((sheet.club() as u32));
    if owner == 0 {
        return;
    }
    contract_manager_assign_date(stored.as_mut_ptr() as usize, person.addr() + staff::DATE_JOINED_CLUB);
    let new = contract_manager_club_by_id((contract.club() as u32));
    contract_manager_archive_player_stats(person, 1, read_u32(new) as i32);
    let main = Contract(if !person.is_null() && (read_u32(person.addr()) as i32) < read_u32(G_STAFF_COUNT) as i32 && (read_u32(staff_state_entry(person)) as i32) >= 0 {
        contract_at_index(read_u32(this), read_u32(staff_state_entry(person)) as i32)
    } else {
        0
    });
    if main.is_null() {
        show_contract_error_box(0x16a9);
        return;
    }
    crate::comparison::branch("cpp_2 sign loan");
    let m = main.addr() + contract::CONTRACT_TYPE;
    let reserve = if club_has_reserve_squad(owner) != 0 {
        read_byte_volatile(person.addr() + staff::PLAYING_SQUAD) & 1 == 0
    } else if contract_manager_linked_b_team(owner, core::ptr::null_mut()) != 0 {
        owner as u32 != (person.club_job().addr() as u32)
    } else {
        false
    };
    let cl = read_byte_volatile(m);
    *(m as *mut u8) = if reserve { cl | 0x40 } else { cl & 0xbf };
    exe::virtual_staff_remove_virtual_players(read_u32(G_VIRTUAL_STAFF) as usize, new, 0);
    remove_from_tactics_selection((person.club_job().addr() as u32) as usize, person, 0, 1);
    remove_from_tactics_selection((person.club_job().addr() as u32) as usize, person, 1, 1);
    add_staff_to_club(new, person, 0xb, G_CURRENT_DATE, 1, 0); // the player at the new club
    contract_manager_assign_date(person.addr() + staff::DATE_JOINED_CLUB, stored.as_ptr() as usize);
    let emp = read_u32(staff_state_entry(person) + 4) as i32;
    if emp > -1 {
        // simplification: the original copies inline, same as STAFF_CONTRACT::operator= (0x4cc810, ported)
        contract_manager_operator(contract_at_index(read_u32(this), emp), contract.addr());
    } else {
        let n = read_u32(this + contract_manager_fields::CONTRACT_COUNT) as i32;
        let array = read_u32(this);
        let mut i = 0i32;
        while i < n {
            if read_u32(contract_at_index(array, i)) as i32 == -1 {
                contract_manager_operator(contract_at_index(array, i), contract.addr());
                write_u32(staff_state_entry(person) + 4, i as u32);
                return; // the original exits without rebuilding the sheets
            }
            i += 1;
        }
        let cap = read_u32(this + contract_manager_fields::CONTRACT_CAPACITY) as i32;
        if n >= cap {
            let cap = cap + 100;
            write_u32(this + contract_manager_fields::CONTRACT_CAPACITY, cap as u32);
            let new_array = if array != 0 {
                exe::crt_realloc(array as usize, (cap * 0x50) as u32)
            } else {
                exe::crt_malloc((cap * 0x50) as u32)
            };
            write_u32(this, new_array as u32);
            if new_array == 0 {
                show_contract_error_box(0x1708);
            }
        }
        let n = read_u32(this + contract_manager_fields::CONTRACT_COUNT);
        write_u32(staff_state_entry(person) + 4, n);
        let target = contract_at_index(read_u32(this), n as i32);
        write_u32(this + contract_manager_fields::CONTRACT_COUNT, n + 1);
        exe::staff_contract_operator_3d(target, contract);
    }
    update_club_wage_bill(owner);
    update_club_wage_bill(new);
}

/// `CONTRACT_MANAGER::end_loan(person)`: takes the person out of the loan club, frees the loan contract, recalculates
/// the wage bills and returns her to the parent club with the squad mark.
///
/// The loan club (+0x39; null: the box of line 0x1729), or its A team when it is the B team (`exe::club_get_linked_b_team`). With the main
/// contract in the array itself (without it: the box of line 0x1735): the record [0xae1080] gives the owning club;
/// takes the person off the lists (`archive_player_stats` with the owner), the disputes (`restore_held_bans`), the loan club's tactics (`remove_player_from_selection`
/// with 0 and 1); frees the loan contract (−1 in the entry and in slot +4); removes her from the loan club's squad
/// (`database_remove_staff_from_club`); redoes the wage bills of the loan club and the owner; removes her from the loan club's shortlist
/// (`exe::shortlist_manager_find_club_shortlist_entry`, `update_staff_status` with 4). The person goes back to the owner (+0x39) and to the squad marks (`staff_set_squad_flags`): with a
/// reserve team (`database_club_has_reserve_squad`), 2 if the main contract has bit 0x40 of +0x35, otherwise 1; otherwise 1 and, with a partner
/// club, she stays at the partner if she is in its 50 slots. Status +0x6d 0x10 becomes 4.
/// Original: `CONTRACT_MANAGER::end_loan` (0x4d07e0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4d07e0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::end_loan(pessoa)`: o clube do empréstimo
/// (+0x39; nulo: caixa da linha 0x1729), ou o time A dele quando é o B (0x540a50). Com o contrato principal no próprio
/// vetor (sem ele: caixa da linha 0x1735): a ficha [0xae1080] dá o clube dono; tira a pessoa das listas (0x7c5fe0 com o
/// dono), das disputas (0x556570), das táticas do clube do empréstimo (0x896c60 com 0 e 1); libera o contrato de
/// empréstimo (−1 na entrada e na vaga +4); tira do elenco do clube do empréstimo (0x53c190); refaz as folhas do clube
/// do empréstimo e do dono; tira da lista de observação do clube do empréstimo (0x849ce0, 0x843e10 com 4). A pessoa
/// volta ao dono (+0x39) e às marcas de elenco (0x540980): com time reserva (0x542740), 2 se o contrato principal tem o
/// bit 0x40 de +0x35, senão 1; senão 1 e, com clube parceiro, fica no parceiro se está nas 50 vagas dele. A situação
/// +0x6d 0x10 vira 4.
pub(crate) unsafe extern "thiscall" fn end_loan(this: usize, person: usize) {
    let person = Staff(person);
    let mut club = (person.club_job().addr() as u32) as usize;
    if club == 0 {
        show_contract_error_box(0x1729);
        return;
    }
    let mut eh_a = 0u32; // read only when 0x540a50 writes it
    if contract_manager_linked_b_team(club, &mut eh_a) != 0 && eh_a == 0 {
        club = contract_manager_linked_b_team(club, core::ptr::null_mut()); // o time A
    }
    let main = Contract(if !person.is_null() && (read_u32(person.addr()) as i32) < read_u32(G_STAFF_COUNT) as i32 && (read_u32(staff_state_entry(person)) as i32) >= 0 {
        contract_at_index(read_u32(this), read_u32(staff_state_entry(person)) as i32)
    } else {
        0
    });
    if main.is_null() {
        show_contract_error_box(0x1735);
        return;
    }
    crate::comparison::branch("cpp_2 end loan");
    let sheet = Contract(contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), read_u32(staff_state_entry(person)) as i32));
    write_u32(G_CONTRACT_LOOKUP_RESULT, sheet.addr() as u32);
    let owner = if sheet.is_null() { 0 } else { contract_manager_club_by_id((sheet.club() as u32)) };
    contract_manager_archive_player_stats(person, 0, read_u32(owner) as i32);
    exe::discipline_restore_held_bans(read_u32(G_DISCIPLINE) as usize, person);
    remove_from_tactics_selection((person.club_job().addr() as u32) as usize, person, 0, 1);
    remove_from_tactics_selection((person.club_job().addr() as u32) as usize, person, 1, 1);
    let emp = read_u32(staff_state_entry(person) + 4) as i32;
    if emp != -1 {
        write_u32(contract_at_index(read_u32(this), emp), u32::MAX);
        write_u32(staff_state_entry(person) + 4, u32::MAX);
    }
    contract_manager_remove_staff_from_club((person.club_job().addr() as u32) as usize, person, 0, 1, 0);
    update_club_wage_bill(club);
    let f = main_contract_lookup(person);
    update_club_wage_bill(if f == 0 { 0 } else { contract_manager_club_by_id(read_u32(f + contract::CLUB)) });
    let list = read_u32(read_u32(G_TRANSFER_MANAGER) as usize + 0x856) as usize;
    if exe::shortlist_manager_find_club_shortlist_entry(list, club, person, 0, 0) != 0 {
        (exe::shortlist_manager_update_staff_status(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + 0x856) as usize, person, club, 4, 0) as usize);
    }
    let f = main_contract_lookup(person);
    let volta = if f == 0 { 0 } else { contract_manager_club_by_id(read_u32(f + contract::CLUB)) };
    person.set_club_job(Club((volta as u32) as usize)); // the person returns to the owner
    if club_has_reserve_squad(volta) != 0 {
        set_squad_flags(person, if read_byte_volatile(main.addr() + contract::CONTRACT_TYPE) & 0x40 != 0 { 2 } else { 1 });
    } else {
        let partner = Club(contract_manager_linked_b_team((person.club_job().addr() as u32) as usize, core::ptr::null_mut()));
        set_squad_flags(person, 1);
        if !partner.is_null() {
            for k in 0..0x32 {
                if (partner.squad(k) as u32) as usize == person.addr() {
                    person.set_club_job(Club((partner.addr() as u32) as usize));
                }
            }
        }
    }
    if read_byte_volatile(person.addr() + staff::SQUAD_SELECTED_FOR) == 0x10 {
        person.set_squad_selected_for(4);
    }
}

/// `CONTRACT_MANAGER::toggle_squad_placement(person, club, out)`: moves the person between the club's first team and
/// reserves or between the club and its partner (A/B side), taking the loan contract into account; 0 with no room at
/// the destination.
///
/// By the place (`get_squad_placement`): 0 nothing. 1 and 2 (first team / reserves at the club): on loan to another club (loan
/// contract +4 with another club), it only clears (1) or sets (2) bit 0x40 of +0x35 of the main contract; otherwise
/// squad mark 1 or 2 (`staff_set_squad_flags`) and the club's tactics (`remove_player_from_selection` with 1 or 0). 3 (B team) and the others (A team): the
/// destination club is the club (3) or the partner (`exe::club_get_linked_b_team`), stored in `out`; with no room for the job (+0x3d,
/// `database_club_has_room_for_job`) it returns 0. On loan to this club: she leaves the other's lists, squad and tactics and enters the
/// destination with the job; with another club's loan contract: toggles bit 0x40 and moves to the destination as a
/// player (0xb); otherwise as the loaned one, without storing. Returns 1 (also for a null club). Without a main
/// contract the original reads [0x35] and crashes.
/// Original: `CONTRACT_MANAGER::toggle_squad_placement` (0x4cbd20, contract_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x4cbd20 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::toggle_squad_placement(pessoa, clube,
/// saida)`: pelo lugar (0x4cbb60): 0 nada. 1 e 2 (principal / reserva no clube): emprestada a outro clube (contrato de
/// empréstimo +4 com outro clube), só apaga (1) ou marca (2) o bit 0x40 de +0x35 do contrato principal; senão a marca de
/// elenco 1 ou 2 (0x540980) e as táticas do clube (0x896c60 com 1 ou 0). 3 (time B) e os outros (time A): o clube de
/// destino é o clube (3) ou o parceiro (0x540a50), gravado em `saida`; sem vaga para o cargo (+0x3d, 0x53b9a0) devolve
/// 0. Emprestada a este clube: sai das listas, do elenco e das táticas do outro e entra no destino com o cargo; com
/// contrato de empréstimo de outro clube: troca o bit 0x40 e passa ao destino como jogador (0xb); senão como o
/// emprestado, sem gravar. Devolve 1 (clube nulo também). Sem contrato principal a original lê [0x35] e cai.
pub(crate) unsafe extern "thiscall" fn toggle_squad_placement(this: usize, person: usize, club: usize, out: usize) -> u32 {
    let person = Staff(person);
    if club == 0 {
        return 1;
    }
    if out != 0 {
        write_u32(out, 0);
    }
    let l = exe::contract_manager_get_squad_placement(this, person, club);
    if l == 0 {
        return 1;
    }
    let in_band = |p: usize| p != 0 && (read_u32(p) as i32) < read_u32(G_STAFF_COUNT) as i32;
    // the loan contract (in the global CONTRACT_MANAGER) and the main one (in the array itself; 0 if none)
    let loan = |p: usize| {
        let k = read_u32(staff_state_entry(Staff(p)) + 4) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), k) }
    };
    let main = |p: usize| {
        let k = read_u32(staff_state_entry(Staff(p))) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(this), k) }
    };
    if l == 1 || l == 2 {
        if in_band(person.addr()) {
            let e = loan(person.addr());
            if e != 0 && read_u32(e + contract::CLUB) != read_u32(club) {
                let m = main(person.addr()) + 0x35;
                let cl = read_byte_volatile(m);
                *(m as *mut u8) = if l == 1 { cl & 0xbf } else { cl | 0x40 };
                return 1;
            }
        }
        crate::comparison::branch("cpp_2 change at the club");
        set_squad_flags(person, l as u32); // 1 principal, 2 reserva
        remove_from_tactics_selection(club, person, if l == 1 { 1 } else { 0 }, 1);
        return 1;
    }
    let (origem, destino) = if l == 3 {
        (contract_manager_linked_b_team(club, core::ptr::null_mut()), club)
    } else {
        (club, contract_manager_linked_b_team(club, core::ptr::null_mut()))
    };
    if out != 0 {
        write_u32(out, destino as u32);
    }
    // simplification: the original pushes the job in a register with garbage in the high bytes; 0x53b9a0 reads only the byte
    if exe::database_club_has_room_for_job(destino, read_byte_volatile(person.addr() + staff::JOB_FOR_CLUB) as u32, person) == 0 {
        return 0;
    }
    if in_band(person.addr()) {
        let e = loan(person.addr());
        if e != 0 && read_u32(e + contract::CLUB) == read_u32(club) {
            // loaned to this club: passes from the other one to the destination with the role
            contract_manager_archive_player_stats(person, 0, -1);
            contract_manager_remove_staff_from_club(origem, person, 0, 1, 1);
            remove_from_tactics_selection(origem, person, 1, 1);
            remove_from_tactics_selection(origem, person, 0, 1);
            // simplification: the job goes with the high bytes of eax from 0x896c60; 0x53bd10 reads only the byte
            add_staff_to_club(destino, person, read_byte_volatile(person.addr() + staff::JOB_FOR_CLUB) as u32, 0, 1, 1);
            return 1;
        }
        if in_band(person.addr()) && e != 0 {
            let m = main(person.addr()) + 0x35;
            let cl = read_byte_volatile(m);
            *(m as *mut u8) = if cl & 0x40 == 0 { cl | 0x40 } else { cl & 0xbf };
            crate::comparison::branch("cpp_2 change loaned");
            contract_manager_remove_staff_from_club(origem, person, 0, 0, 1);
            add_staff_to_club(destino, person, 0xb, 0, 0, 1);
            return 1;
        }
    }
    crate::comparison::branch("cpp_2 change team");
    contract_manager_archive_player_stats(person, 0, -1);
    contract_manager_remove_staff_from_club(origem, person, 0, 1, 0);
    remove_from_tactics_selection(origem, person, 1, 1);
    remove_from_tactics_selection(origem, person, 0, 1);
    // simplification: the role goes with the high bytes of edx left by 0x896c60; 0x53bd10 reads only the byte
    add_staff_to_club(destino, person, read_byte_volatile(person.addr() + staff::JOB_FOR_CLUB) as u32, 0, 1, 0);
    1
}

/// Volatile byte read (so a missing contract club crashes like the original).
pub(crate) unsafe fn read_u8_volatile(a: usize) -> u8 {
    // simplification: volatile because the original reads [0 + 0x53] when the contract’s club is missing (crashes like it)
    core::ptr::read_volatile(a as *const u8)
}

/// Club reputation as seen by a manager: weighted by the nation's league standard (or × 0.65 without nation), truncated
/// and capped above 0x203a.
///
/// [pt] A reputação "vista" de um clube para o técnico: com nação (+0x53), o peso dela (+0x85) × reputação (+0x80) × 0,05
/// (0x967888); sem, a reputação × 0,65 (0x96a5d0); truncadas (16 bits) e, acima de 0x203a, 0x203a − (excesso × −1,1
/// truncado).
pub(crate) unsafe fn perceived_club_reputation(club: Club) -> i16 {
    let r = club.reputation() as i32;
    let nation = Nation((club.nation().addr() as u32) as usize);
    let v = if !nation.is_null() {
        ftol((read_u8_volatile(nation.addr() + nation::LEAGUE_STANDARD) as i8 as i32 * r) as f64 * read_f64(rdata::F64_0_05))
    } else {
        ftol(r as f64 * read_f64(rdata::F64_0_65))
    } as i16;
    if v > 0x203a {
        (0x203a - ftol((v as i32 - 0x203a) as f64 * read_f64(rdata::F64_NEG_1_1))) as i16
    } else {
        v
    }
}

/// `staff_can_take_job(job, person)` (stdcall, ret 8): tells whether the person fits the job (6 to 15) by age, current
/// job and the player and non-player records; a null person: 0.
///
/// Through the jump table 0x4cbaf4 (job − 6, 0..9; outside: 0); 7, 12 and 14: 0; a null person: 0 (the non-player regen
/// 0x7ab9f2 always asks with null). 6: aged 30 (+0x18) or more, a non-player (+0x69), above 36, or with +0x57 below 10
/// and the player's +0x2f 12 or more: 1; otherwise, a non-player in job 8, 6 or 5. 8: aged 30, a non-player or above 36
/// or (+0x2f ≥ 12 and +0x57 < 10) outside jobs 5 and 10: 1; otherwise as for 6. 9: a non-player in job 9. 10: a
/// non-player in job 10 or with +0x28 = 10. 11: a player (+0x61). 13 and 15: both player and non-player, or above 34,
/// or aged 30 with +0x2f ≥ 12.
/// Original: `staff_can_take_job` (0x4cb990, contract_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x4cb990 } (stdcall, ret 8, contract_manager.cpp): `staff_can_take_job(cargo, pessoa)`: pela tabela de salto
/// 0x4cbaf4 (cargo − 6, 0..9; fora: 0); 7, 12 e 14: 0; pessoa nula: 0 (o regen de não jogador 0x7ab9f2 pergunta sempre
/// com nula). 6: com 30 anos (+0x18) ou mais, não jogador (+0x69), acima de 36, ou com +0x57 abaixo de 10 e +0x2f do
/// jogador 12 ou mais: 1; senão, não jogador no cargo 8, 6 ou 5. 8: de 30 anos, não jogador ou acima de 36 ou (+0x2f ≥ 12
/// e +0x57 < 10) fora dos cargos 5 e 10: 1; senão como o 6. 9: não jogador no cargo 9. 10: não jogador no cargo 10 ou com
/// +0x28 = 10. 11: jogador (+0x61). 13 e 15: jogador e não jogador, ou acima de 34, ou de 30 anos com +0x2f ≥ 12.
pub(crate) unsafe extern "stdcall" fn staff_can_take_job(job: u32, person: usize) -> u32 {
    let k = (job as u8 as i8 as i32).wrapping_sub(6);
    if k as u32 > 9 || k == 1 || k == 6 || k == 8 {
        return 0;
    }
    if person == 0 {
        crate::comparison::branch("cpp_4 null person");
        return 0; // null person
    }
    let p = Staff(person);
    let age = read_u8_volatile(p.addr() + staff::AGE) as i8;
    let not_player = (p.non_player() as u32);
    let function = read_u8_volatile(p.addr() + staff::JOB_FOR_CLUB);
    let strength = || read_u8_volatile((p.player() as u32) as usize + 0x2f) as i8; // +0x2f of the player
    // the common end (0x4cb9db): non-player in jobs 8, 6 or 5
    let common = || not_player != 0 && (function == 8 || function == 6 || function == 5);
    let r = match k {
        0 => {
            if age >= 0x1e {
                if not_player != 0 || age > 0x24 {
                    return 1;
                }
                if (read_u8_volatile(p.addr() + staff::AMBITION) as i8) < 0xa && strength() >= 0xc {
                    return 1;
                }
            }
            common()
        }
        2 => {
            if age < 0x1e {
                common()
            } else if not_player == 0 && age <= 0x24 && (strength() < 0xc || read_u8_volatile(p.addr() + staff::AMBITION) as i8 >= 0xa) {
                common()
            } else if function == 5 || function == 0xa {
                common()
            } else {
                true
            }
        }
        3 => not_player != 0 && function == 9,
        4 => not_player != 0 && (function == 0xa || read_u8_volatile(p.addr() + staff::JOB_FOR_NATION) == 0xa),
        5 => (p.player() as u32) != 0,
        _ => {
            // 13 e 15
            let player = (p.player() as u32) as usize;
            player != 0 && (not_player != 0 || age > 0x22 || (age >= 0x1e && read_u8_volatile(player + player_sheet::LEADERSHIP) as i8 >= 0xc))
        }
    };
    r as u32
}

/// Days (0..400) from the date (day, unsigned year) to today (G_CURRENT_DATE), 365-day years.
///
/// [pt] Os dias (0..400) desde a data (dia +0, ano +2 sem sinal) até hoje ([0xae2c90]), com o ano de 365 dias.
pub(crate) unsafe fn contract_manager_days_since_date(day: usize, year: usize) -> i32 {
    let today = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16) as i32;
    let a = core::ptr::read_unaligned(year as *const u16) as i32;
    let d = (today - a) * 365 - read_i16(day) as i32 + read_i16(G_CURRENT_DATE) as i32;
    d.clamp(0, 400)
}

/// The morale drop (the original's byte `m`) after a bad match, or None when the routine exits without touching
/// anything (some null, the club's manager or assistant +0xcf/+0xd3, or outside job 0xb).
///
/// x = (rating +0xb − club reputation) + 50 × days at the club (since +0x3e, up to 400) + 417 × age − 31680. With the
/// contract days (since +0x2d) above 400 (never: the cap is 400) and age 24..31: + 250 × +0x57 + 5000; otherwise, with
/// +0x22 and the nation's reputation (+0x1a +0x8e) above 3500 and under 32 years old: + 250 × +0x57 + 25 × +0x22 +
/// 2500; otherwise by the contract band (+0x4f >> 4): 1: + 250 × +0x57 + 5000; 2: + 125 × +0x57 + 2500; 3: + 1000 ×
/// +0x3e; in these cases at most 15000.
///
/// y = 1 − x × (6.667e-4 if x < 0, otherwise 6e-5), between 0.1 and 10; m = ftol((+0x3e + 4) / y + 0.5) (byte). Double
/// precision; the constants come from .rdata.
///
/// [pt] A queda da moral (o byte `m` da original) pela partida ruim, ou None quando a rotina sai sem mexer (algum nulo, o
/// técnico ou o auxiliar do clube +0xcf/+0xd3, ou fora do cargo 0xb). x = (nota +0xb − reputação do clube) + 50 × dias
/// no clube (desde +0x3e, até 400) + 417 × idade − 31680; com os dias do contrato (desde +0x2d) acima de 400 (nunca: o
/// teto é 400) e idade 24..31: + 250 × +0x57 + 5000; senão, com +0x22 e a reputação da nação (+0x1a +0x8e) acima de 3500
/// e menos de 32 anos: + 250 × +0x57 + 25 × +0x22 + 2500; senão pela faixa do contrato (+0x4f >> 4): 1: + 250 × +0x57 +
/// 5000; 2: + 125 × +0x57 + 2500; 3: + 1000 × +0x3e; nesses casos no máximo 15000. y = 1 − x × (6,667e-4 se x < 0, senão
/// 6e-5), entre 0,1 e 10; m = ftol((+0x3e + 4) / y + 0,5) (byte). Dupla precisão; as constantes do .rdata.
pub(crate) unsafe fn match_morale_drop(person: Staff, club: Club, contract: Contract) -> Option<i8> {
    if contract.is_null() || club.is_null() || person.is_null() {
        return None;
    }
    if person.addr() == (club.manager().addr() as u32) as usize || person.addr() == (club.assistant_manager() as u32) as usize || read_u8_volatile(person.addr() + staff::JOB_FOR_CLUB) != 0xb {
        return None;
    }
    let a = contract_manager_days_since_date(person.addr() + staff::DATE_JOINED_CLUB, person.addr() + staff::DATE_JOINED_CLUB_YEAR);
    let b = contract_manager_days_since_date(contract.addr() + contract::CONTRACT_EXPIRES, contract.addr() + contract::CONTRACT_EXPIRES_YEAR);
    let age = read_u8_volatile(person.addr() + staff::AGE) as i8;
    let player = (person.player() as u32) as usize;
    let dif = read_i16(player + player_sheet::CURRENT_REPUTATION) as i32 - club.reputation() as i32;
    let mut x = ((dif as f64 + (a * 50) as f64) + age as f64 * read_f64(rdata::F64_417)) - read_f64(rdata::F64_31680);
    let c57 = read_u8_volatile(person.addr() + staff::AMBITION) as i8 as i32;
    let ceiling = read_f32(rdata::F32_15000) as f64;
    let limitar = |x: f64| if x >= ceiling { ceiling } else { x };
    let al = read_u8_volatile(person.addr() + staff::INT_APPS);
    if b > 0x190 && age > 0x17 && age < 0x20 {
        x = limitar(x + (c57 * 250) as f64 + read_f32(rdata::F32_5000) as f64); // never: b goes up to 400;
    } else if al != 0 && read_i16((person.nation().addr() as u32) as usize + 0x8e) > 0xdac && age < 0x20 {
        x = limitar(x + (c57 * 250 + al as i32 * 25) as f64 + read_f32(rdata::F32_2500) as f64);
    } else {
        match (read_u8_volatile(contract.addr() + contract::SQUAD_STATUS) >> 4) as i8 {
            1 => x = limitar(x + (c57 * 250) as f64 + read_f32(rdata::F32_5000) as f64),
            2 => x = limitar(x + (c57 * 125) as f64 + read_f32(rdata::F32_2500) as f64),
            3 => x = limitar(x + (read_u8_volatile(contract.addr() + contract::UNHAPPY_FLAG) as i8 as i32 * 1000) as f64),
            _ => {}
        }
    }
    let y = if !(x >= read_f64(rdata::F64_0)) { x * read_f64(rdata::F64_0_0006666666667) } else { x * read_f64(rdata::F64_6ENEG_05) };
    let mut y = read_f64(rdata::F64_1) - y;
    if !(y >= read_f64(rdata::F64_0_1)) {
        y = read_f64(rdata::F64_0_1);
    } else if y > read_f64(rdata::F64_10) {
        y = read_f64(rdata::F64_10);
    }
    let d = (read_u8_volatile(contract.addr() + contract::UNHAPPY_FLAG) as i8 as i32 + 4) as f64;
    Some(ftol(d / y + read_f64(rdata::F64_0_5)) as u8 as i8)
}

/// Morale minus m, clamped to -100..100 (byte sum when it fits).
///
/// [pt] A moral (+0x44) menos `m`, entre −100 e 100 (a soma em byte quando cabe).
pub(crate) fn lowered_morale(morale: i8, m: i8) -> i8 {
    let minus = m.wrapping_neg();
    let s = morale as i32 + minus as i32;
    if s > 100 {
        100
    } else if s < -100 {
        -100
    } else {
        morale.wrapping_add(minus)
    }
}

/// Club of the person's main contract (null if none); stores the contract in G_CONTRACT_LOOKUP_RESULT like the
/// original.
///
/// [pt] O contrato principal da pessoa no CONTRACT_MANAGER global e o clube dele, como a original os refaz (a ficha
/// [0xae1080] gravada); sem contrato o clube é nulo.
pub(crate) unsafe fn club_of_main_contract(person: Staff) -> usize {
    let c = Contract(if read_u32(person.addr()) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(person.addr()) as usize).wrapping_mul(0x4f))) as i32;
        if k < 0 { 0 } else { (read_u32(read_u32(G_CONTRACT_MANAGER) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) }
    });
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut u32, c.addr() as u32);
    if c.is_null() { 0 } else { (read_u32(G_CLUB_TABLE) as usize).wrapping_add(((c.club() as u32) as usize).wrapping_mul(club::SIZE)) }
}

/// Whether the transfer window is open for the club of the person's contract.
///
/// The key nation index (`0x669340` of the club's nation; with no nation, the low byte of the club pointer that the
/// original routine reads from the argument slot) gives the two dates of table `0xb63d8a` (72 bytes per nation): true
/// when today is after the first and before the second.
///
/// [pt] A janela de transferências aberta para o clube do contrato da pessoa: o índice da nação-chave (0x669340 da nação do
/// clube; sem nação, o byte baixo do ponteiro do clube que a original lê da vaga do argumento) dá as duas datas da
/// tabela 0xb63d8a (72 bytes por nação): hoje depois da primeira e antes da segunda.
pub(crate) unsafe fn transfer_window_open(person: Staff, club: Club) -> bool {
    let c1 = Club(club_of_main_contract(person));
    let k = if (c1.nation().addr() as u32) != 0 {
        let c2 = Club(club_of_main_contract(person));
        exe::key_nation_index_of_nation((c2.nation().addr() as u32) as usize)
    } else {
        // simplification: the original reads the low byte of the slot of the `clube` argument (the pointer), not an index
        club.addr() as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return false;
    }
    let t = G_LEAGUE_WORLD_CUP_START_DATE + (k as usize) * 0x48;
    let day = read_i16(G_CURRENT_DATE);
    let year = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
    let year_of = |a: usize| core::ptr::read_unaligned(a as *const u16);
    let after = (day > read_i16(t) && year == year_of(t + 2)) || year > year_of(t + 2);
    let before = (day < read_i16(t + 0xf) && year == year_of(t + 0x11)) || year < year_of(t + 0x11);
    after && before
}

/// `contract_manager_lower_match_morale(person, club, contract)`: lowers the player's contract morale after a poor
/// match (by rating, reputation, time at the club, age and contract band) and, with very low morale, may add the
/// wants-to-leave or lack-of-games grievance; injured, on loan or retiring clears the grievances.
///
/// The drop `m` (match_morale_drop). Injured (`get_player_injury_status`), on loan (loan contract), retiring (`record_bits`), under 18 or with
/// +0x49: clears grievances 4 and 0x10 (`exe::staff_contract_clear_grievances`). Otherwise, with m > 5 and morale − m below −30 − +0x59: above 21
/// years old, stronger (+0xb) than the club and above 2250, or with potential (+0xd) above reputation − 1500 and above
/// 500 with draw(+0x57) ≠ 0 and +0x3d negative: in the transfer window of the contract club, grievance 4 (`exe::staff_contract_clear_grievances` and
/// `exe::staff_contract_apply_event_flags` with 4); in the first team (0x4cbb60 = 1), rating above 1250, more than 23 years old and draw(+0x59 + 5) =
/// 0: grievance 0x10; up to 21 years old, in the first team with draw(+0x57) > 5, stronger than the club and above
/// 1250: grievance 0x10. Finally the happiness in the contract (+0x44) drops by m, kept between −100 and 100.
/// Original: `contract_manager_lower_match_morale` (0x4cb250, contract_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x4cb250 } (thiscall, ret 0xc, contract_manager.cpp): `contract_manager_lower_match_morale(pessoa, clube,
/// contrato)`: a queda `m` (match_morale_drop). Machucado (0x62c400), emprestado (contrato de empréstimo), aposentando
/// (0x7a7eb0), menor de 18 ou com +0x49: apaga as queixas 4 e 0x10 (0x8674e0). Senão, com m > 5 e a moral − m abaixo de
/// −30 − +0x59: acima de 21 anos, mais forte (+0xb) que o clube e acima de 2250, ou de potencial (+0xd) acima da
/// reputação − 1500 e de 500 com sorteio(+0x57) ≠ 0 e +0x3d negativo: na janela de transferências do clube do contrato,
/// a queixa 4 (0x8674e0 e 0x867700 com 4); no principal (0x4cbb60 = 1), nota acima de 1250, mais de 23 anos e
/// sorteio(+0x59 + 5) = 0: a queixa 0x10; até 21 anos, no principal com sorteio(+0x57) > 5, mais forte que o clube e
/// acima de 1250: a queixa 0x10. Por fim a felicidade no contrato (+0x44) cai m, entre −100 e 100.
pub(crate) unsafe extern "thiscall" fn contract_manager_lower_match_morale(this: usize, person: usize, club: usize, contract: usize) {
    let contract = Contract(contract);
    let person = Staff(person);
    let club = Club(club);
    let Some(m) = match_morale_drop(person, club, contract) else {
        return;
    };
    let emprestado = || {
        if read_u32(person.addr()) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
            return false;
        }
        let k = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(person.addr()) as usize).wrapping_mul(0x4f)) + 4) as i32;
        k >= 0 && (read_u32(read_u32(G_CONTRACT_MANAGER) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) != 0
    };
    let age = read_u8_volatile(person.addr() + staff::AGE) as i8;
    if exe::injury_get_player_injury_status(read_u32(G_INJURY) as usize, person) != 0 || emprestado() || exe::player_regen_get_retirement_status(read_u32(G_PLAYER_REGEN) as usize, person) != 0
        || age < 0x12 || read_u8_volatile(contract.addr() + contract::LEAVING_ON_BOSMAN) != 0 {
        if contract.grievances() & 4 != 0 {
            exe::staff_contract_clear_grievances(contract, 4);
        }
        if read_u8_volatile(contract.addr() + contract::GRIEVANCES) & 0x10 != 0 {
            exe::staff_contract_clear_grievances(contract, 0x10);
        }
    } else if m > 5 && (read_u8_volatile(contract.addr() + contract::MORALE) as i8 as i32 - m as i32) < -30 - read_u8_volatile(person.addr() + staff::LOYALTY) as i8 as i32 {
        let player = (person.player() as u32) as usize;
        if age > 0x15 {
            let rating = read_i16(player + player_sheet::CURRENT_REPUTATION);
            if rating > club.reputation() && rating > 0x8ca {
                if transfer_window_open(person, club) {
                    crate::comparison::branch("cpp_4 complaint in the window");
                    exe::staff_contract_clear_grievances(contract, 4);
                    exe::staff_contract_apply_event_flags(contract, 4, 0);
                }
            } else {
                let mut done = false;
                if exe::contract_manager_get_squad_placement(this, person, club) == 1 && read_i16(player + player_sheet::CURRENT_REPUTATION) > 0x4e2 && age > 0x17
                    && random(read_u8_volatile(person.addr() + staff::LOYALTY) as i8 as i32 + 5) == 0 {
                    exe::staff_contract_clear_grievances(contract, 4);
                    exe::staff_contract_apply_event_flags(contract, 0x10, 0);
                    done = true;
                }
                if !done {
                    let pot = read_i16(player + player_sheet::WORLD_REPUTATION);
                    if (pot as i32) > club.reputation() as i32 - 0x5dc && pot > 0x1f4
                        && random(read_u8_volatile(person.addr() + staff::AMBITION) as i8 as i32) != 0 && (read_u8_volatile(contract.addr() + contract::HAPPINESS) as i8) < 0
                        && transfer_window_open(person, club) {
                        exe::staff_contract_clear_grievances(contract, 4);
                        exe::staff_contract_apply_event_flags(contract, 4, 0);
                    }
                }
            }
        } else if exe::contract_manager_get_squad_placement(this, person, club) == 1 && random(read_u8_volatile(person.addr() + staff::AMBITION) as i8 as i32) > 5
            && read_i16(player + player_sheet::CURRENT_REPUTATION) > club.reputation() && read_i16(player + player_sheet::CURRENT_REPUTATION) > 0x4e2 {
            exe::staff_contract_clear_grievances(contract, 4);
            exe::staff_contract_apply_event_flags(contract, 0x10, 0);
        }
    }
    let p = (contract.addr() + contract::MORALE) as *mut i8;
    *p = lowered_morale(*p, m); // a moral cai
}

/// Address of the person's 79-byte staff state record (+0 main contract index, -1 none).
///
/// [pt] A vaga do contrato principal da pessoa: o registro de 79 bytes dela em [0xae2d0c] (+0 o índice do contrato, −1 sem).
pub(crate) unsafe fn main_contract_slot(person: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f))
}

/// `CONTRACT_MANAGER::remove_contract(person)`: frees the person's main contract entry, puts −1 in her slot of the
/// 79-byte record and recalculates the wage bill of the contract's club.
///
/// With the main contract (index in the person's 79-byte record), it frees its entry (+0 = −1 in the CONTRACT_MANAGER's
/// 0x50-byte vector), puts −1 in the person's slot and redoes the wage bill (`update_wage_bill`) of the finances (`get_club_finance`) of
/// the contract club (+4). Without a contract: nothing.
/// Original: `CONTRACT_MANAGER::remove_contract` (0x4d0b50, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4d0b50 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract(pessoa)`: com o contrato
/// principal (índice no registro de 79 bytes da pessoa), libera a entrada dele (+0 = −1 no vetor de 0x50 bytes do
/// CONTRACT_MANAGER), põe −1 na vaga da pessoa e refaz a folha de salários (0x598370) das finanças (0x5a0590) do clube
/// do contrato (+4). Sem contrato: nada.
pub(crate) unsafe extern "thiscall" fn remove_contract(this: usize, person: usize) {
    let index = read_i32(main_contract_slot(person));
    if index == -1 {
        return;
    }
    let e = Contract(read_usize(this).wrapping_add((index as usize).wrapping_mul(0x50)));
    let id_club = e.club();
    write_usize(e.addr(), usize::MAX);
    let club = read_usize(G_CLUB_TABLE).wrapping_add(id_club.wrapping_mul(club::SIZE));
    crate::comparison::branch("cpo_2 remove contract");
    write_usize(main_contract_slot(person), usize::MAX); // the person is left without a contract
    let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if f != 0 {
        exe::finance_update_wage_bill(f);
    }
}

/// `CONTRACT_MANAGER::resubmit_held_offers(id)`: resubmits the person's held offers as live offers, removes each one
/// from the list and returns how many.
///
/// It walks the held offers list (+0x38) and, for each one of person `id` (+0): with the transfer (+0x59, `offer_slot` over
/// [0xde1f54]) whose club (+0x24) is the offer's (+0x5d), it sets state 0xf to 8 and resubmits the offer (`add_contract_offer` with
/// the transfer, the club, byte +0x61, 1, 1, 0); without a transfer (+0x59 = −1) it resubmits with −1, −1. It removes
/// the offer from the list (`manager_manager_remove`) and counts it. Byte +0x61 is pushed in the whole register (the upper bits of
/// whatever was in it: the pointer read or −1). Returns how many.
/// Original: `CONTRACT_MANAGER::resubmit_held_offers` (0x4cf4a0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cf4a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::resubmit_held_offers(id)`: percorre a lista
/// das ofertas guardadas (+0x38) e, para cada uma da pessoa `id` (+0): com a transferência (+0x59, 0x8c61e0 em
/// [0xde1f54]) cujo clube (+0x24) é o da oferta (+0x5d), põe o estado 0xf em 8 e reenvia a oferta (0x4cc490 com a
/// transferência, o clube, o byte +0x61, 1, 1, 0); sem transferência (+0x59 = −1) reenvia com −1, −1. Tira a oferta da
/// lista (0x68d1f0) e conta. O byte +0x61 vai empurrado no registrador inteiro (os bits de cima do que estava nele:
/// o ponteiro lido ou −1). Devolve quantas.
pub(crate) unsafe extern "thiscall" fn resubmit_held_offers(this: usize, id: usize) -> u32 {
    let list = read_usize(this + contract_manager_fields::HELD_OFFERS);
    if list == 0 {
        return 0;
    }
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    let mut n = 0u32;
    let mut item = exe::llist_get_next(read_usize(this + contract_manager_fields::HELD_OFFERS));
    while item != 0 {
        if read_usize(item) == id {
            let t = read_usize(item + contract_offer::TRANSFER_OFFER_SLOT);
            let byte = *((item + contract_offer::MARK) as *const u8) as u32;
            if t as i32 != -1 {
                let p = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), (t) as i32);
                if p != 0 {
                    let q = read_usize(p);
                    if q != 0 && read_usize(q + offer::SERIAL) == read_usize(item + contract_offer::TRANSFER_OFFER_SERIAL) {
                        if *((q + offer::STATUS) as *const u8) == 0xf {
                            *((q + offer::STATUS) as *mut u8) = 8;
                        }
                        crate::comparison::branch("cpo_2 resend offer");
                        exe::contract_manager_add_contract_offer(this, (item) as usize as *mut u8, t, read_usize(item + contract_offer::TRANSFER_OFFER_SERIAL), (q as u32 & 0xffff_ff00) | byte, 1, 1, 0); // with the transfer
                    }
                }
            } else {
                exe::contract_manager_add_contract_offer(this, (item) as usize as *mut u8, usize::MAX, usize::MAX, 0xffff_ff00 | byte, 1, 1, 0);
            }
            exe::llist_remove(read_usize(this + contract_manager_fields::HELD_OFFERS), item);
            n += 1;
        }
        item = exe::llist_get_next(read_usize(this + contract_manager_fields::HELD_OFFERS));
    }
    n
}

/// `contract_manager_season_end_month(club)` (stdcall, ret 4; SEH frame without a destructor on the normal path): the
/// (exe::date_get_month(0..11) as u32) of the date ten days after the end of the season of the club's nation.
///
/// The club's season end date (Date() `exe::date_date_54a110` and `key_nation_club_in_season` with 0), plus 10 days (`exe::date_add_days`), and its (exe::date_get_month(`exe::date_get_month`) as u32).
/// Returns the eax of `exe::date_get_month` (the month 0..11).
/// Original: `contract_manager_season_end_month` (0x4d45f0, contract_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x4d45f0 } (stdcall, ret 4, contract_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `contract_manager_season_end_month(clube)`: a data do fim da temporada do clube (Data() 0x54a110 e 0x669bd0 com 0),
/// mais 10 dias (0x54c5b0), e o mês dela (0x54cfb0). Devolve o eax da 0x54cfb0 (o mês 0..11).
pub(crate) unsafe extern "stdcall" fn contract_manager_season_end_month(club: usize) -> u32 {
    let mut d1 = [0u32; 2];
    let mut d2 = [0u32; 2];
    exe::date_date_54a110((d1.as_mut_ptr()) as usize);
    exe::key_nation_club_in_season(club, 0, (d1.as_mut_ptr()) as *mut u32 as usize);
    let p = exe::date_add_days((d1.as_mut_ptr()) as usize, (d2.as_mut_ptr()) as usize, (0xa) as u32); // dez dias depois
    (exe::date_get_month(p) as u32)
}

/// Shows the CONTRACT_MANAGER "cannot create space" box for a line and text; the game continues.
///
/// Text "v%s %s.%s %d (%s)" (version 0x90d130, name and extension of contract_manager.cpp 0x9a8190, `linha`, `texto`)
/// at 0xae24d0; the title "Memory Allocation Error" (0x9a8108) translated (0x66f4e0) at 0xae28bc, the body 0x9a7ff8
/// with the text at 0xae26c4, and the box `exe::gui_show_message_box`. The game continues ([0xb67a34] stays as it is).
///
/// [pt] A caixa "cannot create space" do CONTRACT_MANAGER: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de
/// contract_manager.cpp 0x9a8190, `linha`, `texto`) em 0xae24d0; o título "Memory Allocation Error" (0x9a8108)
/// traduzido (0x66f4e0) em 0xae28bc, o corpo 0x9a7ff8 com o texto em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está).
pub(crate) unsafe fn show_cannot_create_space_box(row: u32, text: usize) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row, text);
    format(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), row, text);
    translate(0xae28bc, 0x9a8108);
    translate(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// `CONTRACT_MANAGER::get_free_staff_contract_space(append_only)`: returns the slot in the contract vector (0x50-byte
/// records at [this], capacity +4, count +8).
///
/// Without `append_only`, the first empty slot (person +0 = −1); otherwise (or with no empty slot) the one at the end:
/// when full it grows by 200 at a time (realloc `exe::crt_realloc`, or malloc `exe::crt_malloc` without a vector; without memory, box
/// 0x175 and −1) and the count goes up. Returns the index. Both callers (the contract creation 0x4c7560 and `add_staff_contract`)
/// pass 1.
/// Original: `CONTRACT_MANAGER::get_free_staff_contract_space` (0x4c79a0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4c79a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::get_free_staff_contract_space(so_no_fim)`:
/// a vaga do vetor de contratos (registros de 0x50 bytes em [this], capacidade +4, conta +8). Sem `so_no_fim`, a
/// primeira vaga vazia (pessoa +0 = −1); senão (ou sem vaga vazia) a do fim: cheio, cresce de 200 em 200 (realloc
/// 0x945667, ou malloc 0x944e46 sem vetor; sem memória, caixa 0x175 e −1) e a conta sobe. Devolve o índice. Os dois
/// chamadores (a criação dos contratos 0x4c7560 e a 0x4c7950) passam 1.
pub(crate) unsafe extern "thiscall" fn get_free_staff_contract_space(this: usize, so_in_end: u32) -> i32 {
    if so_in_end == 0 {
        let n = read_usize(this + contract_manager_fields::CONTRACT_COUNT) as i32;
        let p = read_usize(this);
        let mut i = 0i32;
        while i < n {
            if read_usize(p.wrapping_add((i as usize).wrapping_mul(0x50))) as i32 == -1 {
                return i;
            }
            i += 1;
        }
    }
    if read_usize(this + contract_manager_fields::CONTRACT_COUNT) == read_usize(this + contract_manager_fields::CONTRACT_CAPACITY) {
        let cap = read_usize(this + contract_manager_fields::CONTRACT_CAPACITY).wrapping_add(200);
        let p = read_usize(this);
        write_usize(this + contract_manager_fields::CONTRACT_CAPACITY, cap);
        let q = if p != 0 {
            exe::crt_realloc(p, cap.wrapping_mul(0x50) as u32)
        } else {
            exe::crt_malloc(cap.wrapping_mul(0x50) as u32)
        };
        write_usize(this, q);
        if q == 0 {
            show_cannot_create_space_box(0x175, 0x9a81c4);
            return -1;
        }
    }
    let n = read_usize(this + contract_manager_fields::CONTRACT_COUNT).wrapping_add(1);
    write_usize(this + contract_manager_fields::CONTRACT_COUNT, n); // one more slot
    (n as i32).wrapping_sub(1)
}

/// `CONTRACT_MANAGER::add_staff_contract(person, value)`: takes the appended slot for a new person (hired manager,
/// regen), links the person to it and generates the initial contract.
///
/// The slot at the end (`get_free_staff_contract_space` with 1); without a slot, nothing. Links the person to the slot (index +0 of her
/// 0x4f-byte record in [0xae2d0c]), puts her id in the contract (+0) and generates the initial contract (0x85e960 with
/// the value). Called by the manager hiring (0x68f9c0, MANAGER_MANAGER::daily_update) and by the regen that signs with
/// a club (`player_regen_sign_for_club`). Returns the original's eax (that of 0x85e960, or the negative index); the callers do not read it.
/// Original: `CONTRACT_MANAGER::add_staff_contract` (0x4c7950, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c7950 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::add_staff_contract(pessoa, valor)`: a vaga
/// do fim (0x4c79a0 com 1); sem vaga, nada. Liga a pessoa à vaga (o índice +0 do registro de 0x4f bytes dela em
/// [0xae2d0c]), põe o id dela no contrato (+0) e gera o contrato inicial (0x85e960 com o valor). Pela contratação
/// de técnico (0x68f9c0, MANAGER_MANAGER::daily_update) e pelo regen que assina com um clube (0x7a6f10). Devolve o
/// eax da original (o da 0x85e960, ou o índice negativo); os chamadores não o leem.
pub(crate) unsafe extern "thiscall" fn add_staff_contract(this: usize, person: usize, value: u32) -> u32 {
    let i = exe::contract_manager_get_free_staff_contract_space(this, 1);
    if i < 0 {
        return i as u32;
    }
    crate::comparison::branch("cpl_4 person contract");
    write_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)), i as usize); // the person points to the slot
    let c = read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
    write_usize(c, read_usize(person));
    exe::staff_contract_generate_initial_contract(c, person, value) // the person’s value
}

/// The reminder dates (type 2) that a type-1 decision date gets before it, from the contract morale; returns up to two
/// dates and the count.
///
/// With a player record (+0x61), L = contract morale (+0x44) + 100 (8 bits; the contract via the table [0xae2d0c], as
/// `exe::contract_manager_get_contract` inline: without a contract the original routine reads address 0x44 and crashes, as here); L ≤ 0x50 no
/// reminder; L > 0x96 one reminder L × 90 / 200 days before; and one L / 4 days before (without a player record, L =
/// 100).
///
/// The dates come from Date::sub_days (0x54c8e0), in the original's order.
///
/// [pt] As datas dos lembretes (tipo 2) que a data de decisão de tipo 1 leva antes dela: com o registro de jogador
/// (+0x61), L = moral do contrato (+0x44) + 100 (8 bits; o contrato pela tabela [0xae2d0c], como a 0x4cf5a0
/// inline: sem contrato a original lê o endereço 0x44 e cai, como aqui); L ≤ 0x50 sem lembrete; L > 0x96 um a
/// L × 90 / 200 dias antes; e um a L / 4 dias antes (sem o registro de jogador, L = 100). As datas pela
/// Date::sub_days (0x54c8e0), na ordem da original.
pub(crate) unsafe fn decision_reminder_dates(this: usize, person: Staff, data: usize, kind: u8) -> ([[u8; 8]; 2], usize) {
    let mut v = [[0u8; 8]; 2];
    let mut k = 0usize;
    if kind != 1 {
        return (v, k);
    }
    let mut l: u8 = 100;
    if person.player() != 0 {
        let id = read_usize(person.addr()) as i32;
        let c = if person.is_null() || id >= read_usize(G_STAFF_COUNT) as i32 {
            0
        } else {
            let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(0x4f))) as i32;
            if i < 0 { 0 } else { read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50)) }
        };
        l = (*((c + 0x44) as *const u8)).wrapping_add(100);
        if l <= 0x50 {
            return (v, k);
        }
        if l > 0x96 {
            let mut t = [0u8; 8];
            let r = exe::date_sub_days(data, (t.as_mut_ptr()) as *mut u8 as usize, (l as u32 * 90) / 200);
            v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
            k += 1;
        }
    }
    let mut t = [0u8; 8];
    let r = exe::date_sub_days(data, (t.as_mut_ptr()) as *mut u8 as usize, (l as u32 * 50) / 200);
    v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
    k += 1;
    (v, k)
}

/// `CONTRACT_MANAGER::add_decision_date(person, date, slice, type)`: appends an entry to the 14-byte decision date
/// vector; type 1 first adds type 2 reminders from the contract morale.
///
/// Appends to the 14-byte vector [this+0xc] (capacity +0x10, count +0x14; it grows by 100 at a time when the count
/// passes capacity − 4, realloc `exe::crt_realloc` or malloc `exe::crt_malloc`; without memory, box 0x103e and nothing) the entry (person
/// id +0, date +4, time slice of the day +0xc, type +0xd). Type 1 is preceded by the type 2 reminders
/// (`decision_reminder_dates`), without a slice (+0xc is left as it is). Called by the offers: the person's decision
/// (`schedule_offer_decision`, `exe::contract_manager_resolve_contract_offers`). The date copies (`exe::date_operator_3d`, with no other effect) become 8-byte copies.
/// Original: `CONTRACT_MANAGER::add_decision_date` (0x4cd6d0, contract_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4cd6d0 } (thiscall, ret 0x10, contract_manager.cpp): `CONTRACT_MANAGER::add_decision_date(pessoa, data, fatia,
/// tipo)`: acrescenta ao vetor de 14 bytes [this+0xc] (capacidade +0x10, conta +0x14; cresce de 100 em 100 quando a
/// conta passa da capacidade − 4, realloc 0x945667 ou malloc 0x944e46; sem memória, caixa 0x103e e nada) a entrada
/// (id da pessoa +0, data +4, fatia do dia +0xc, tipo +0xd). O tipo 1 leva antes os lembretes de tipo 2
/// (`decision_reminder_dates`), sem fatia (+0xc fica como está). Pelas propostas: a decisão da pessoa (0x4cd230, 0x4ce4e0).
/// As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
pub(crate) unsafe extern "thiscall" fn add_decision_date(this: usize, person: usize, data: usize, slice: u32, kind: u32) {
    crate::comparison::branch("cpl_4 decision date");
    if read_usize(this + contract_manager_fields::DECISION_COUNT) as i32 > (read_usize(this + contract_manager_fields::DECISION_CAPACITY) as i32).wrapping_sub(4) {
        let cap = read_usize(this + contract_manager_fields::DECISION_CAPACITY).wrapping_add(100);
        write_usize(this + contract_manager_fields::DECISION_CAPACITY, cap);
        let p = read_usize(this + contract_manager_fields::DECISION_DATES);
        let q = if p != 0 {
            exe::crt_realloc(p, cap.wrapping_mul(14) as u32)
        } else {
            exe::crt_malloc(cap.wrapping_mul(14) as u32)
        };
        write_usize(this + contract_manager_fields::DECISION_DATES, q);
        if q == 0 {
            show_cannot_create_space_box(0x103e, 0x9a83e0);
            return;
        }
    }
    let (lembretes, k) = decision_reminder_dates(this, Staff(person), data, kind as u8);
    for d in &lembretes[..k] {
        let n = read_usize(this + contract_manager_fields::DECISION_COUNT);
        let e = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add(n.wrapping_mul(14));
        write_usize(e, read_usize(person));
        core::ptr::write_unaligned((e + decision_date::DATE) as *mut [u8; 8], *d);
        *((e + decision_date::KIND) as *mut u8) = 2;
        write_usize(this + contract_manager_fields::DECISION_COUNT, n.wrapping_add(1));
    }
    let n = read_usize(this + contract_manager_fields::DECISION_COUNT);
    let e = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add(n.wrapping_mul(14));
    write_usize(e, read_usize(person));
    *((e + decision_date::SLICE) as *mut u8) = slice as u8;
    core::ptr::write_unaligned((e + decision_date::DATE) as *mut [u8; 8], core::ptr::read_unaligned(data as *const [u8; 8]));
    *((e + decision_date::KIND) as *mut u8) = kind as u8;
    write_usize(this + contract_manager_fields::DECISION_COUNT, n.wrapping_add(1)); // the decision date
}

// simplification: all three reallocate (and 0x4c7950 generates the contract through 0x85e960); they do not run twice. The comparer builds
// the plan beforehand, runs only the original and checks; Rust mode and the career prove the copies.

/// The person's contract with the club whose relegation clause (+0x1f = 1) is still to trigger, else 0.
///
/// [pt] O contrato (registro de 80 bytes em [this], pelo índice +0 do registro de 79 bytes em [0xae2d0c]) da pessoa `p`
/// com o clube `clube` (+4 = id do clube) e a cláusula de rebaixamento ainda por acionar (+0x1f = 1); senão 0.
pub(crate) unsafe fn contract_with_relegation_clause(this: usize, club: usize, p: usize) -> usize {
    if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(79))) as i32;
    if i < 0 {
        return 0;
    }
    let c = Contract((i as usize).wrapping_mul(80).wrapping_add(read_usize(this)));
    if c.is_null() || c.club() != read_usize(club) || c.relegation_rc() != 1 {
        return 0;
    }
    c.addr()
}

/// The club and its linked B team (a B club or null gives none: the A club handles both).
///
/// [pt] O clube e o parceiro (time A/B, 0x540a50 ativo): o clube B não aciona (o A cuida dos dois). Clube nulo ou B:
/// nenhum.
pub(crate) unsafe fn club_and_b_team(club: usize) -> [usize; 2] {
    if club == 0 {
        return [0, 0];
    }
    let mut eh_a = 0u32;
    let p = exe::club_get_linked_b_team(club, &mut eh_a, 1);
    if p != 0 && eh_a == 0 {
        return [0, 0];
    }
    [club, p]
}

/// `CONTRACT_MANAGER::trigger_relegation_clauses(club, league)`: triggers the relegation clauses at a relegated club.
///
/// At the relegated club and at its partner (A/B team), each of the 50 staff and squad members (+0xd7) with a
/// relegation clause still to be triggered in the contract with the club (+0x1f = 1) goes to 2 and the news is posted
/// (`exe::transfer_manager_post_release_clause_activated_news` over [0xde1f54] with type 3).
/// Original: `CONTRACT_MANAGER::trigger_relegation_clauses` (0x4cd030, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cd030 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::trigger_relegation_clauses(clube,
/// liga)`: no clube rebaixado e no parceiro (time A/B), cada um dos 50 da comissão e elenco (+0xd7) com a cláusula de
/// rebaixamento por acionar no contrato com o clube (+0x1f = 1) passa a 2 e sai a notícia (0x8e3db0 sobre
/// [0xde1f54] com o tipo 3).
pub(crate) unsafe extern "thiscall" fn trigger_relegation_clauses(this: usize, club: usize, league: usize) {
    for c in club_and_b_team(club) {
        if c == 0 {
            continue;
        }
        for k in 0..50 {
            let p = read_usize(c + club::SQUAD + k * 4);
            let contract = Contract(contract_with_relegation_clause(this, c, p));
            if !contract.is_null() {
                crate::comparison::branch("cpk_3 relegation clause");
                contract.set_relegation_rc(2); // the triggered clause
                exe::transfer_manager_post_release_clause_activated_news(read_usize(G_TRANSFER_MANAGER), p, league, 3, contract);
            }
        }
    }
}

/// First contract offer in list l (cursor untouched) from person p (+0) to club c (+4).
///
/// [pt] A primeira oferta de contrato da lista `l` (sem mexer no cursor) da pessoa `p` (+0) ao clube `c` (+4).
pub(crate) unsafe fn first_offer_for_staff_club(l: usize, p: usize, c: usize) -> Option<usize> {
    list_items_capped(l).into_iter().find(|&e| read_usize(e) == p && read_usize(e + contract_offer::CLUB_ID) == c)
}

/// `CONTRACT_MANAGER::remove_contract_offer(person, club)`: removes the person's first offer to the club.
///
/// The first offer in the list (+0x34) of the person to the club (ids): returns the record reservation (+0x63 ≠ −1:
/// `add_spare_non_player_record` over [0xb7434c] with +0x67, +0x6b), clears the agreed-contract flag if it was agreed (+0x50 = 2: `clear_agreed_contract_flag`),
/// leaves the list (`manager_manager_remove`) and the person's pending count (+8 of the 79-byte record in [0xae2d0c]) drops by one.
/// Original: `CONTRACT_MANAGER::remove_contract_offer` (0x4cca70, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cca70 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer(pessoa, clube)`: a
/// primeira oferta da lista (+0x34) da pessoa ao clube (ids): devolve a reserva de registro (+0x63 ≠ −1: 0x7a81a0
/// sobre [0xb7434c] com +0x67, +0x6b), tira a marca de contrato acertado se ela estava acertada (+0x50 = 2: 0x8c5430),
/// sai da lista (0x68d1f0) e as pendentes da pessoa (+8 do registro de 79 bytes em [0xae2d0c]) descem uma.
pub(crate) unsafe extern "thiscall" fn remove_contract_offer(this: usize, person: *const u32, club: *const u32) {
    let (c, p) = (*club as usize, *person as usize);
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let mut e = exe::llist_get_next(l);
    while e != 0 && !(read_usize(e) == p && read_usize(e + contract_offer::CLUB_ID) == c) {
        e = exe::llist_get_next(l);
    }
    if e == 0 {
        return;
    }
    crate::comparison::branch("cpk_3 remove contract offer");
    let reserve = read_usize(e + contract_offer::RESERVED_RECORD);
    if reserve as i32 != -1 {
        exe::player_regen_add_spare_non_player_record(read_usize(G_PLAYER_REGEN), reserve, read_usize(e + contract_offer::RESERVED_RECORD_ARG1), read_usize(e + contract_offer::RESERVED_RECORD_ARG2));
    }
    if *((e + contract_offer::STATE) as *const u8) == 2 {
        exe::transfer_manager_clear_agreed_contract_flag(read_usize(G_TRANSFER_MANAGER), person);
    }
    exe::llist_remove(l, e);
    let n = (read_usize(G_STAFF_STATE_TABLE).wrapping_add((*person as usize).wrapping_mul(79)) + 8) as *mut u8;
    *n = (*n).wrapping_sub(1); // one fewer pending
}

/// Calls LList::get_next on list l.
pub(crate) unsafe fn contract_manager_list_next(l: usize) -> usize {
    exe::llist_get_next(l)
}

/// Calls LList::remove of item e from list l.
pub(crate) unsafe fn list_remove(l: usize, e: usize) {
    exe::llist_remove(l, e);
}

/// Pointer to the stored suspensions counter (+0x1c) of the 29-byte discipline record of id in [this+0x14].
///
/// [pt] O contador de suspensões guardadas (+0x1c) do registro de 29 bytes do id `id` em [this+0x14].
pub(crate) unsafe fn stored_suspension_counter(this: usize, id: usize) -> *mut i8 {
    (read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add(id.wrapping_mul(29)) + 0x1c) as *mut i8
}

/// Pointer to the pending offers byte (+8) of the person's 79-byte staff state record.
///
/// [pt] O byte de ofertas pendentes (+8) do registro de 79 bytes da pessoa em [0xae2d0c].
pub(crate) unsafe fn pending_offers_byte(id: usize) -> *mut u8 {
    (read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(79)) + 8) as *mut u8
}

/// `CONTRACT_MANAGER::remove_pending_offers_for_staff(person)`: removes all of the person's pending offers.
///
/// With the offers list (+0x34) and pending offers in the person's record (+8 of the 79-byte one in [0xae2d0c]): clears
/// the agreed-contract flag (`clear_agreed_contract_flag` over [0xde1f54]) and, from the start of the list, removes each offer of the
/// person (`manager_manager_remove`), decrementing the byte, until it reaches zero.
/// Original: `CONTRACT_MANAGER::remove_pending_offers_for_staff` (0x4ccbf0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4ccbf0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_pending_offers_for_staff(pessoa)`:
/// com a lista das ofertas (+0x34) e ofertas pendentes no registro da pessoa (+8 do de 79 bytes em [0xae2d0c]): tira
/// a marca de contrato acertado (0x8c5430 sobre [0xde1f54]) e, do começo da lista, cada oferta da pessoa (0x68d1f0),
/// descontando o byte, até ele zerar.
pub(crate) unsafe extern "thiscall" fn remove_pending_offers_for_staff(this: usize, person: *const u32) {
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if l == 0 || *pending_offers_byte(*person as usize) == 0 {
        return;
    }
    exe::transfer_manager_clear_agreed_contract_flag(read_usize(G_TRANSFER_MANAGER), person);
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let mut e = contract_manager_list_next(l);
    while e != 0 {
        if read_usize(e) as u32 == *person {
            list_remove(l, e);
            let p = pending_offers_byte(*person as usize);
            *p = (*p).wrapping_sub(1);
            if *p == 0 {
                return; // the last pending one
            }
        }
        e = contract_manager_list_next(l);
    }
}

/// First offer in list l (cursor untouched) matching the transfer offer's person and club and still without decision
/// (+0x62 = 0).
///
/// [pt] A primeira oferta da lista `l` (sem mexer no cursor) da pessoa (+0) e do clube (+4) da oferta de transferência e
/// ainda sem decisão (+0x62 = 0).
pub(crate) unsafe fn offer_to_schedule(l: usize, offer: usize) -> Option<usize> {
    list_items_capped(l).into_iter().find(|&e| read_usize(e) == read_usize(offer + offer::TARGET_STAFF_ID) && read_usize(e + contract_offer::CLUB_ID) == read_usize(offer + offer::BUYER_CLUB_ID) && *((e + contract_offer::DECISION_VALID) as *const u8) == 0)
}

/// 1 when the offered player's ability (+0xb of the player record) / 500 exceeds the offering club's reputation
/// (+0x80), else 0.
///
/// [pt] Habilidade (palavra +0xb do registro de jogador +0x61 da pessoa +0xc) / 500 acima da reputação (+0x80) do clube
/// (+8) da oferta?
pub(crate) unsafe fn ability_above_club_reputation(offer: usize) -> u32 {
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(staff::SIZE)));
    let hab = *((person.player() + 0xb) as *const i16) as i32 / 500;
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + offer::BUYER_CLUB_ID).wrapping_mul(club::SIZE)));
    (hab > club.reputation() as i32) as u32
}

/// `CONTRACT_MANAGER::schedule_decision_for_transfer(offer)`: marks the first still-undecided contract offer of the
/// person and club and schedules the decision.
///
/// The first contract offer in the list (+0x34) of the person (+0xc) and the club (+8) of the transfer offer still
/// without a decision (+0x62) is marked (1) and the person's decision is scheduled (`schedule_offer_decision` with slot +0x20). Returns
/// whether the person's ability / 500 exceeds the club's reputation.
/// Original: `CONTRACT_MANAGER::schedule_decision_for_transfer` (0x4cc930, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cc930 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::schedule_decision_for_transfer(oferta)`:
/// a primeira oferta de contrato da lista (+0x34) da pessoa (+0xc) e do clube (+8) da oferta de transferência ainda
/// sem decisão (+0x62) fica marcada (1) e a decisão da pessoa é agendada (0x4cd230 com a vaga +0x20). Devolve se a
/// habilidade da pessoa / 500 passa a reputação do clube.
pub(crate) unsafe extern "thiscall" fn schedule_decision_for_transfer(this: usize, offer: usize) -> u32 {
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let mut e = contract_manager_list_next(l);
    while e != 0 {
        if read_usize(e) == read_usize(offer + offer::TARGET_STAFF_ID) && read_usize(e + contract_offer::CLUB_ID) == read_usize(offer + offer::BUYER_CLUB_ID) && *((e + contract_offer::DECISION_VALID) as *const u8) == 0 {
            *((e + contract_offer::DECISION_VALID) as *mut u8) = 1; // a oferta vale
            exe::contract_manager_schedule_offer_decision(this, read_usize(offer + offer::TARGET_STAFF_ID) as u32, read_usize(offer + offer::SLOT_INDEX) as u32);
            break;
        }
        e = contract_manager_list_next(l);
    }
    ability_above_club_reputation(offer)
}

/// `CONTRACT_MANAGER::remove_held_offers(person)`: removes from the held offers list (+0x38; cursor +0xa back to the
/// start +2) every item (`manager_manager_remove`) whose +0 is the id `person` (walked with `next_list_item`). Without a list: nothing.
/// Original: `CONTRACT_MANAGER::remove_held_offers` (0x4cf560, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cf560 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_held_offers(pessoa)`: da lista das
/// ofertas guardadas (+0x38; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item cujo +0 é o id `pessoa`
/// (0x68d1b0 percorre). Sem lista: nada.
pub(crate) unsafe extern "thiscall" fn remove_held_offers(this: usize, person: u32) {
    let l = read_usize(this + contract_manager_fields::HELD_OFFERS);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let mut e = exe::llist_get_next(l);
    while e != 0 {
        if read_usize(e) as u32 == person { // the person’s offer
            exe::llist_remove(l, e);
        }
        e = exe::llist_get_next(l);
    }
}

/// Whether a suspension of type `t` (type vector [this], 4 bytes each) applies at the loan club.
///
/// By the type's competition (`+0x17`, −1 = none) being equal to the club's nation (`+0x53`, id at `+0`); with no
/// competition, by `+0x1b` (−1: applies) being equal to [`+0x71`] of the nation. Club with no nation: does not apply.
///
/// [pt] A suspensão do tipo `t` (vetor de tipos [this], 4 bytes cada) vale no clube do empréstimo? Pela competição do tipo
/// (+0x17, −1 = nenhuma) igual à nação do clube (+0x53, id em +0); sem competição, pelo +0x1b (−1: vale) igual ao
/// [+0x71] da nação. Clube sem nação: não vale.
pub(crate) unsafe fn suspension_valid_at_club(this: usize, t: i8, club: Club) -> bool {
    let kind = read_usize(read_usize(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    let nation = Nation(club.nation().addr());
    let comp = read_usize(kind + ruling_body::NATION_ID) as i32;
    if comp != -1 {
        return !nation.is_null() && read_usize(nation.addr()) as i32 == comp;
    }
    let x = read_usize(kind + ruling_body::CONTINENT_ID) as i32;
    if x == -1 {
        return true;
    }
    if nation.is_null() {
        return false;
    }
    let m = nation.continent();
    m != 0 && read_usize(m) as i32 == x // the nation’s +0x71
}

/// Loan club and the 5-byte suspension slots of the player's discipline record that do not apply there (skipping the
/// 4th like the original).
///
/// [pt] O clube do empréstimo (+4 da ficha de contrato, clubes de 0x245 bytes em [0xae23b4]) e o registro de suspensões
/// do jogador (29 bytes em [this+0x14]); as vagas (5 bytes: tipo e o resto) que não valem lá, menos a 4ª (índice 3,
/// que o original pula).
pub(crate) unsafe fn suspensions_to_store(this: usize, sheet: usize, player: usize) -> (usize, Vec<u8>) {
    let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(sheet + 4).wrapping_mul(club::SIZE));
    let reg = read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add(read_usize(player).wrapping_mul(29));
    let mut v = Vec::new();
    for k in 0..4u8 {
        let t = *((reg + 5 * k as usize) as *const i8);
        if k == 3 || t == -1 {
            continue;
        }
        if !suspension_valid_at_club(this, t, Club(club)) {
            v.push(k);
        }
    }
    (reg, v)
}

/// Job a vacancy job code looks for: 6/8/9/10 itself, 13→6, 15→8, others none.
///
/// [pt] O cargo que a vaga `cargo` (byte com sinal) procura: 6, 8, 9 e 10 o mesmo; 13 procura 6 e 15 procura 8 (o byte do
/// argumento é trocado); os outros, nenhum.
pub(crate) fn job_sought_by_slot(job: u32) -> Option<u32> {
    match job as u8 as i8 {
        6 | 8 | 9 | 10 => Some(job),
        13 => Some((job & !0xff) | 6),
        15 => Some((job & !0xff) | 8), // o 15 procura 8
        _ => None,
    }
}

/// First person in G_STAFF_TABLE without club, with the given job, a player record and the club's nation.
///
/// [pt] A primeira pessoa (de [0xae23bc], 0x6e bytes, [0xae2364] delas) sem clube (+0x39), no cargo (+0x3d), com registro
/// (+0x69) e da nação (+0x1a) do clube (+0x53).
pub(crate) unsafe fn first_unemployed_staff(club: Club, job: u8) -> Option<usize> {
    let n = read_usize(G_STAFF_COUNT) as i32;
    let base = read_usize(G_STAFF_TABLE);
    (0..n.max(0) as usize).map(|i| base + i * 0x6e).find(|&p| {
        read_usize(p + staff::CLUB_JOB) == 0 && *((p + staff::JOB_FOR_CLUB) as *const u8) == job && read_usize(p + staff::NON_PLAYER) != 0 && read_usize(p + staff::NATION) == club.nation().addr()
    })
}

/// The person's contract as `get_squad_placement` finds it inline (null, out of range or no index → 0).
///
/// [pt] O contrato da pessoa como a 0x4cbb60 acha por dentro (sem chamar a 0x4cf5a0): pessoa não nula, id abaixo de
/// [0xae2364], índice da tabela [0xae2d0c] (79 bytes por pessoa) não negativo; registro de 80 bytes em [objeto].
pub(crate) unsafe fn squad_contract_of_staff(object: usize, person: Staff) -> usize {
    if person.is_null() {
        return 0;
    }
    let id = read_u32(person.addr()) as i32;
    if id >= read_u32(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(79))) as i32;
    if i < 0 {
        return 0;
    }
    (read_u32(object) as usize).wrapping_add((i as usize).wrapping_mul(80))
}

/// `CONTRACT_MANAGER::get_squad_placement(person, club)`: the person's place at `club`: 0 none, 1 reserves, 2 first
/// team, 3 at the linked B team, 4 at the main club with a B team.
///
/// Only for player jobs (+0x3d other than 12 and `exe::database_is_player_job`). At her own club (+0x39) when it can have reserves
/// (`database_club_has_reserve_squad`): 2 with bit 0 of +0x5e, otherwise 1. The partner club (`exe::club_get_linked_b_team` with 1; the output byte is the person's
/// slot, which keeps her pointer if `exe::club_get_linked_b_team` does not write it): being A with `club` her own, 4; being B with `club`
/// the partner A, 3. Otherwise by the contract (`get_squad_placement` finds it internally in [0xae1050], stored in [0xae1080]): the
/// contract club, which is neither her own nor the partner already seen; with bit 6 of the contract's +0x35 (found
/// again in [this]): at `club` with reserves, 1 with the bit and 2 without; or with the partner (`exe::club_get_linked_b_team`) being `club`
/// (or the contract's being `club`), 3 with the bit and 4 without. The game reads only al.
/// Original: `CONTRACT_MANAGER::get_squad_placement` (0x4cbb60, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cbb60 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_placement(pessoa, clube)`: o
/// lugar da pessoa no `clube`: 0 nenhum, 1 reservas, 2 principal, 3 no time B ligado, 4 no clube principal com time B.
/// Só para os cargos de jogador (+0x3d diferente de 12 e 0x53b620). No próprio clube (+0x39) que pode ter reservas
/// (0x542740): 2 com o bit 0 de +0x5e, senão 1. O clube parceiro (0x540a50 com 1; o byte de saída é a vaga da
/// pessoa, que fica com o ponteiro dela se a 0x540a50 não gravar): sendo A e `clube` o próprio, 4; sendo B e `clube` o
/// parceiro A, 3. Senão pelo contrato (0x4cbb60 acha por dentro em [0xae1050], gravado em [0xae1080]): o clube do
/// contrato, que não é o próprio nem o parceiro já visto; com o bit 6 de +0x35 do contrato (achado de novo em [this]):
/// no `clube` com reservas, 1 com o bit e 2 sem; ou com o parceiro (0x540a50) sendo o `clube` (ou o do contrato o
/// `clube`), 3 com o bit e 4 sem. O jogo lê só o al.
pub(crate) unsafe extern "thiscall" fn get_squad_placement(this: usize, person: usize, club: usize) -> u8 {
    let person = Staff(person);
    let kind = (person.job_for_club() as i8);
    if kind == 12 || exe::database_is_player_job(kind) == 0 {
        return 0;
    }
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let own = (person.club_job().addr() as u32) as usize;
    if own != 0 && own == club && exe::database_is_player_job((person.job_for_club() as i8)) != 0 && exe::database_club_has_reserve_squad(club) != 0 {
        return if person.playing_squad() & 1 != 0 { 2 } else { 1 }; // main or reserves
    }
    let mut slot = person.addr() as u32; // the argument’s slot: 0x540a50 may not write it
    let mut b = 0usize;
    if own != 0 {
        b = partner(own, &mut slot, 1);
        if b != 0 {
            if slot != 0 {
                if club == (person.club_job().addr() as u32) as usize {
                    return 4;
                }
            } else if club == b {
                return 3;
            }
        }
    }
    let c = Contract(squad_contract_of_staff(read_u32(G_CONTRACT_MANAGER) as usize, person));
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c.addr(); // the contract record
    if c.is_null() {
        return 0;
    }
    let i = (c.club() as u32) as usize;
    let other = (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
    if other == 0 || other == (person.club_job().addr() as u32) as usize || other == b {
        return 0;
    }
    let c2 = Contract(squad_contract_of_staff(this, person));
    let mark = (c2.contract_type() << 1) as i8 >> 7; // o bit 6, estendido
    if other == club && exe::database_club_has_reserve_squad(club) != 0 {
        return if mark == 0 { 2 } else { 1 };
    }
    let p = partner(other, &mut slot, 1);
    if p == 0 || (p != club && other != club) {
        return 0;
    }
    if mark == 0 { 4 } else { 3 }
}

/// If the squad placement (`get_squad_placement`) is one of de, toggles it (`toggle_squad_placement`); else 0.
///
/// [pt] A 0x4cbb60 pelo endereço (passa pelo desvio) e a troca 0x4cbd20 (com 0 na saída do novo clube).
pub(crate) unsafe fn toggle_placement_if_in(this: usize, person: usize, club: usize, de: [u8; 2]) -> u32 {
    let l = exe::contract_manager_get_squad_placement(this, person, club);
    if l != de[0] && l != de[1] {
        return 0;
    }
    exe::contract_manager_toggle_squad_placement(this, person, club, 0)
}

/// in the reserves or at the B side (placement 1 or 3), swaps through `toggle_squad_placement`; else 0
/// Original: `CONTRACT_MANAGER::move_to_first_team` (0x4cc0b0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cc0b0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_first_team(pessoa, clube)`: nos
/// reservas (1) ou no time B (3), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
pub(crate) unsafe extern "thiscall" fn move_to_first_team(this: usize, person: usize, club: usize) -> u32 {
    toggle_placement_if_in(this, person, club, [1, 3]) // reserves or B team
}

/// in the first team or at the club with a B side (placement 2 or 4), swaps through `toggle_squad_placement`; else 0
/// Original: `CONTRACT_MANAGER::move_to_reserves` (0x4cc0f0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cc0f0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_reserves(pessoa, clube)`: no
/// principal (2) ou no clube com time B (4), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
pub(crate) unsafe extern "thiscall" fn move_to_reserves(this: usize, person: usize, club: usize) -> u32 {
    toggle_placement_if_in(this, person, club, [2, 4]) // senior team or club with a B team
}

/// Comparer of move_to_first_team / move_to_reserves: side by side outside the toggle placements, original only inside.
///
/// [pt] Comparador das duas trocas: o lugar (0x4cbb60, pelo desvio) decide antes; fora dos lugares da troca, lado a lado
/// pela ficha [0xae1080] (as duas devolvem 0); nos lugares da troca, só a original (a 0x4cbd20 mexe no elenco).
pub(crate) unsafe fn compare_placement_toggle(i: usize, name: &str, this: usize, person: usize, club: usize, de: [u8; 2]) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(i);
    let sheet = read_u32(G_CONTRACT_LOOKUP_RESULT);
    let l = exe::contract_manager_get_squad_placement(this, person, club);
    *(G_CONTRACT_LOOKUP_RESULT as *mut u32) = sheet;
    if l == de[0] || l == de[1] {
        return o(this, person, club);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, person, club), || toggle_placement_if_in(this, person, club, de));
    SCOREBOARD.record(igual && r == n, || format!("{}({:x}, {:x}) lugar {} original={} rust={} {:x?} x {:x?}", name, person, club, l, r, n, mo, mr));
    r
}

/// `STAFF_CONTRACT::operator=`: copies the contract (0x50 bytes) from `source` field by field.
///
/// Four dwords, three dwords (+0x10), five bytes (+0x1c), +0x21, the start (+0x25) and end (+0x2d) dates through
/// `exe::date_operator_3d`, the bit-field byte +0x35 in three steps (6 bits, the 7th, the 8th: at the end it is `source`'s), +0x36,
/// +0x3a, +0x3b, +0x3d, +0x3e, +0x3f, +0x40, +0x44, +0x45, +0x49, +0x4a, +0x4e and +0x4f in two steps (low nibble and
/// then the rest). Returns `this`.
/// Original: `STAFF_CONTRACT::operator=` (0x4cc810, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cc810 } (thiscall, ret 4, contract_manager.cpp): `STAFF_CONTRACT::operator=`: copia o contrato (0x50 bytes) de
/// `origem`: quatro duplas, três duplas (+0x10), cinco bytes (+0x1c), +0x21, as datas de início (+0x25) e do fim
/// (+0x2d) pela 0x417c10, o byte de campos de bits +0x35 em três passos (6 bits, o 7º, o 8º: no fim é o de `origem`),
/// +0x36, +0x3a, +0x3b, +0x3d, +0x3e, +0x3f, +0x40, +0x44, +0x45, +0x49, +0x4a, +0x4e e o +0x4f em dois passos (nibble
/// baixo e depois o resto). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn contract_manager_operator(this: usize, source: usize) -> usize {
    let source = Contract(source);
    let this = Contract(this);
    let d = |o: usize| core::ptr::write_unaligned((this.addr() + o) as *mut u32, read_u32(source.addr() + o));
    let w = |o: usize| core::ptr::write_unaligned((this.addr() + o) as *mut u16, core::ptr::read_unaligned((source.addr() + o) as *const u16));
    let b = |o: usize| *((this.addr() + o) as *mut u8) = read_u8(source.addr() + o);
    for o in [0, 4, 8, 0xc, 0x10, 0x14, 0x18] {
        d(o);
    }
    for o in 0x1c..0x21 {
        b(o); // the five bytes
    }
    d(0x21);
    copy_record_date(this.addr() + contract::DATE_STARTED, source.addr() + contract::DATE_STARTED);
    copy_record_date(this.addr() + contract::CONTRACT_EXPIRES, source.addr() + contract::CONTRACT_EXPIRES);
    let m = (this.addr() + contract::CONTRACT_TYPE) as *mut u8;
    let s = source.contract_type();
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ *m) & 0x3f) ^ *m; // the low 6 bits
    *m = cl;
    let al = ((((source.contract_type() << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl; // o 7º bit
    *m = al;
    let s = source.contract_type();
    *m = ((al ^ s) & 0x7f) ^ s; // o 8º bit
    d(0x36);
    b(0x3a);
    w(0x3b);
    b(0x3d);
    b(0x3e);
    b(0x3f);
    d(0x40);
    b(0x44);
    d(0x45);
    b(0x49);
    d(0x4a);
    b(0x4e);
    let m = (this.addr() + contract::SQUAD_STATUS) as *mut u8;
    let c = *m;
    let al = ((source.squad_status() ^ c) & 0xf) ^ c; // o nibble baixo
    *m = al;
    let s = source.squad_status();
    *m = ((al ^ s) & 0xf) ^ s;
    this.addr()
}

/// `contract_manager_raise_match_morale(person, club, contract)` (stdcall, ret 0xc): raises the contract morale after a
/// match by the rating against the club reputation (up to 10), keeping morale between −100 and 100 and under the
/// grievance cap; clears grievances 4 and 0x10 and checks the request withdrawal.
///
/// With the three and the player record (+0x61): x = (rating +0xb − club reputation +0x80) + 417 × +0x18 − 11680; y = 1
/// + x × (6.6e-5 if x < 0, otherwise 6.6667e-5), between 0.01 and 2; the gain = ftol((+0x3f + 5) / y), at most 10. The
/// cap: 100 (50 with grievance 2), −50 per grievance 1, 8, 0x20, 0x40, 0x80, 0x100 while the cap does not go below −50.
/// The morale (+0x44) adds the gain within −100 and 100 and stays at or under the cap. Without +0x49: with gain > 5 and
/// morale + gain > 0, or happiness (+0x3d) ≥ 75 − gain, it clears grievances 4 and 0x10 (`exe::staff_contract_clear_grievances`). Finally it checks
/// the withdrawal of the request (`check_transfer_request_withdrawal` with 0). Constants from .rdata; x87 in double precision, like the other
/// contract routines.
/// Original: `contract_manager_raise_match_morale` (0x4cb0a0, contract_manager.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x4cb0a0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_raise_match_morale(pessoa, clube,
/// contrato)`: com os três e o registro de jogador (+0x61): x = (nota +0xb − reputação do clube +0x80) + 417 × +0x18
/// − 11680; y = 1 + x × (6,6e-5 se x < 0, senão 6,6667e-5), entre 0,01 e 2; o ganho = ftol((+0x3f + 5) / y), no máximo
/// 10. O teto: 100 (50 com a queixa 2), −50 por queixa 1, 8, 0x20, 0x40, 0x80, 0x100 enquanto o teto não passa de −50
/// para baixo. A moral (+0x44) soma o ganho entre −100 e 100 e fica no teto. Sem +0x49: com ganho > 5 e moral + ganho
/// > 0, ou felicidade (+0x3d) ≥ 75 − ganho, apaga as queixas 4 e 0x10 (0x8674e0). Por fim confere a retirada do pedido
/// (0x867240 com 0). Constantes do .rdata; o x87 em dupla precisão, como as outras do contrato.
pub(crate) unsafe extern "stdcall" fn contract_manager_raise_match_morale(person: usize, club: usize, contract: usize) {
    let contract = Contract(contract);
    let person = Staff(person);
    let club = Club(club);
    if contract.is_null() || club.is_null() || person.is_null() {
        return;
    }
    let player = (person.player() as u32) as usize;
    if player == 0 {
        return;
    }
    let a = core::ptr::read_unaligned((player + player_sheet::CURRENT_REPUTATION) as *const i16) as i32 - club.reputation() as i32;
    let x = (a as f64 + (person.age() as u8) as i8 as f64 * read_f64(rdata::F64_417)) - read_f64(rdata::F64_11680);
    let mut y = (if x < read_f64(rdata::F64_0) { x * read_f64(rdata::F64_6_6ENEG_05) } else { x * read_f64(rdata::F64_6_666666667ENEG_05) }) + read_f64(rdata::F64_1);
    if y < read_f64(rdata::F64_0_01) {
        y = read_f64(rdata::F64_0_01);
    } else if y > read_f64(rdata::F64_2) {
        y = read_f64(rdata::F64_2);
    }
    let happiness = (contract.matches_left_out() as i8 as i32 + 5) as f64; // +0x3f + 5
    let mut gain = ftol(happiness / y) as u8 as i8;
    if gain > 10 {
        gain = 10;
    }
    let queixas = contract.grievances();
    let mut ceiling: i8 = if queixas & 2 != 0 { 50 } else { 100 };
    if queixas & 1 != 0 {
        ceiling = ceiling.wrapping_sub(50);
    }
    for q in [8u32, 0x20, 0x40, 0x80, 0x100] {
        if ceiling < -50 {
            break;
        }
        if queixas & q != 0 {
            ceiling = ceiling.wrapping_sub(50); // menos 50 por queixa
        }
    }
    let morale = (contract.addr() + contract::MORALE) as *mut i8;
    let cl = *morale;
    let s = cl as i32 + gain as i32;
    *morale = if s > 100 { 100 } else if s < -100 { -100 } else { cl.wrapping_add(gain) };
    if *morale > ceiling {
        *morale = ceiling;
    }
    if contract.leaving_on_bosman() == 0 {
        let apaga = if gain > 5 && *morale as i32 + gain as i32 > 0 {
            true
        } else {
            (contract.happiness() as u8) as i8 as i32 >= 75 - gain as i32
        };
        if apaga {
            if queixas & 4 != 0 {
                exe::staff_contract_clear_grievances(contract, 4);
            }
            if (contract.grievances() as u8) & 0x10 != 0 {
                exe::staff_contract_clear_grievances(contract, 0x10);
            }
        }
    }
    exe::staff_contract_check_transfer_request_withdrawal(contract, 0);
}

/// `CONTRACT_MANAGER::remove_talks_records(person, manager, club)`: removes the person's talks (and of the club and
/// manager, when given), moving the last one into the gap, then sorts again.
///
/// In the talks vector ([this+0x28], [this+0x2c] records of 21 bytes), each one of the person (+0 = id) and, when
/// given, of the club (+4) and the manager (+8) is overwritten by the last one (5 dwords and a byte), the count goes
/// down and the same place is looked at again. If any are left, it sorts again (qsort `exe::crt_qsort` with 0x4dc970).
/// Original: `CONTRACT_MANAGER::remove_talks_records` (0x4ca1c0, contract_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x4ca1c0 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::remove_talks_records(pessoa, tecnico,
/// clube)`: no vetor de conversas ([this+0x28], [this+0x2c] registros de 21 bytes), cada um da pessoa (+0 = id) e,
/// quando dados, do clube (+4) e do técnico (+8) recebe por cima o último (5 duplas e um byte), a contagem desce e o
/// mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x4dc970).
pub(crate) unsafe extern "thiscall" fn remove_talks_records(this: usize, person: usize, manager: usize, club: usize) {
    let n = (this + contract_manager_fields::TALKS_COUNT) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = read_u32(this + contract_manager_fields::TALKS_RECORDS) as usize;
            let r = base.wrapping_add(off);
            let home = read_u32(r) == read_u32(person)
                && (club == 0 || read_u32(r + contract_talks::CLUB_ID) == read_u32(club)) // the club
                && (manager == 0 || read_u32(r + contract_talks::MANAGER_ID) == read_u32(manager));
            if home {
                let last = base.wrapping_add((*n as usize).wrapping_mul(21)).wrapping_sub(21);
                core::ptr::copy(last as *const u8, r as *mut u8, 21);
                *n -= 1;
                i -= 1;
                off = off.wrapping_sub(21);
            }
            i += 1;
            off = off.wrapping_add(21);
            if i >= *n {
                break;
            }
        }
    }
    if *n != 0 {
        exe::crt_qsort(read_u32(this + contract_manager_fields::TALKS_RECORDS) as usize, (*n) as u32, 0x15, 0x4dc970);
    }
}

/// Comparer: `exe::staff_contract_clear_grievances` and `check_transfer_request_withdrawal` change the complaints, the links and the transfer (they cannot run twice): only the
/// original runs; the proof is the career and the copy's branch in Rust mode.
///
/// [pt] Comparador: a 0x8674e0 e a 0x867240 mexem nas queixas, nos vínculos e na transferência (não podem rodar duas
/// vezes): só a original; a prova é a carreira e o ramo da cópia no modo Rust.
// simplification: original only (153,222 calls); comparing would require separating morale from the end calls.

/// Address of person *id's 79-byte staff state record (+4 = loan contract index, -1 none).
///
/// [pt] A entrada de 79 bytes da pessoa `*id` na tabela [0xae2d0c] (em +4 a vaga do empréstimo, −1 sem).
pub(crate) unsafe fn loan_state_entry(id: *const i32) -> usize {
    (*(G_STAFF_STATE_TABLE as *const usize) as isize + *id as isize * 79) as usize
}

/// frees the person's loan contract slot, if any
/// Original: `CONTRACT_MANAGER::remove_loan_contract` (0x4d0bd0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d0bd0 } (thiscall, ret 4, contract_manager.cpp): libera a vaga do contrato de empréstimo da
/// pessoa, se ela tiver uma: −1 no começo da vaga (80 bytes cada, em [gerente]) e na entrada da pessoa.
pub(crate) unsafe extern "thiscall" fn remove_loan_contract(manager: usize, id: *const i32) {
    let e = loan_state_entry(id);
    let slot = core::ptr::read_unaligned((e + 4) as *const i32);
    if slot != -1 {
        *((*(manager as *const usize) as isize + slot as isize * 80) as *mut i32) = -1;
        core::ptr::write_unaligned((loan_state_entry(id) + 4) as *mut i32, -1);
    }
}

/// Shortlist entry of club *clube (9 bytes per club in [manager+0xdc]: +0 slots, +4 mark).
///
/// [pt] A lista de observados do clube `*clube` (9 bytes por clube em [gerente+0xdc]: +0 as vagas, +4 a marca).
pub(crate) unsafe fn club_shortlist(manager: usize, club: *const i32) -> usize {
    (*((manager + shortlist_manager_fields::CLUB_SHORTLISTS) as *const usize) as isize + *club as isize * 9) as usize
}

/// Walks the contract manager's offer list (+0x34) from the start until achou accepts an offer; 0 if none.
///
/// [pt] Percorre a lista do gerente de contratos (+0x34, cursor em +0xa voltando ao começo +2, 0x68d1b0)
/// até a oferta que `achou` aceita; 0 sem lista ou sem nenhuma.
pub(crate) unsafe fn find_manager_offer(manager: usize, found: impl Fn(usize) -> bool) -> usize {
    let list = *((manager + contract_manager_fields::CONTRACT_OFFERS) as *const usize);
    if list == 0 {
        return 0;
    }
    core::ptr::write_unaligned((list + llist::CURSOR) as *mut u32, core::ptr::read_unaligned((list + llist::HEAD) as *const u32));
    loop {
        let o = exe::llist_get_next(*((manager + contract_manager_fields::CONTRACT_OFFERS) as *const usize));
        if o == 0 || found(o) {
            return o;
        }
    }
}

/// the person's contract offer for the club, in the contract manager's list
/// Original: `contract_offer` (0x4cc9e0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cc9e0 } (thiscall, ret 8, contract_manager.cpp): a oferta de contrato da pessoa `pessoa`
/// (+0) para o clube `clube` (+4), ou 0.
pub(crate) unsafe extern "thiscall" fn contract_offer(manager: usize, person: i32, club: i32) -> usize {
    find_manager_offer(manager, |o| *(o as *const i32) == person && *((o + contract_offer::CLUB_ID) as *const i32) == club)
}

/// the person's agreed contract offer (state 2)
/// Original: `agreed_contract_offer` (0x4cca30, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cca30 } (thiscall, ret 4, contract_manager.cpp): a oferta acertada (+0x50 = 2) da pessoa, ou 0.
pub(crate) unsafe extern "thiscall" fn agreed_contract_offer(manager: usize, person: i32) -> usize {
    find_manager_offer(manager, |o| *(o as *const i32) == person && *((o + contract_offer::STATE) as *const u8) == 2)
}

/// Memory regions the offer searches touch: the cursor of the manager's offer list.
///
/// [pt] As regiões que as buscas mexem: o cursor da lista do gerente.
pub(crate) unsafe fn offer_list_regions(manager: usize) -> Vec<(usize, usize)> {
    let list = *((manager + contract_manager_fields::CONTRACT_OFFERS) as *const usize);
    if list == 0 { Vec::new() } else { vec![(list, 0xe)] }
}

/// Returns dword +0xc of the contract record (0x50 bytes from [this]) of person `*p`, by the index at +0 of the 79-byte
/// record in [0xae2d0c].
///
/// 0 without a person, with an id outside [0xae2364], a negative index or a record at address 0.
/// Original: `contract_value` (0x4d0c20, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d0c20 } (thiscall, ret 4, contract_manager.cpp): o dword +0xc da ficha de contrato (0x50 bytes
/// a partir de [this]) da pessoa `*p`, pelo índice em +0 do registro de 79 bytes em [0xae2d0c]; 0 sem
/// pessoa, com id fora de [0xae2364], índice negativo ou ficha no endereço 0.
pub(crate) unsafe extern "thiscall" fn contract_value(t: usize, p: *const i32) -> u32 {
    if p.is_null() || *p >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let k = core::ptr::read_unaligned((*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((*p as usize).wrapping_mul(79)) as *const i32);
    if k < 0 {
        return 0;
    }
    let sheet = Contract((*(t as *const usize)).wrapping_add(k as usize * 0x50));
    if sheet.is_null() {
        return 0;
    }
    (sheet.wage() as u32)
}

/// `CONTRACT_MANAGER::schedule_offer_decision(id, slot)` (SEH only for the local date, with an empty destructor): finds
/// or creates the person's type 3 decision entry and sets the day and time slice of the answer: by the offer deadline,
/// or without an offer by today and the contract end; at most 14 days ahead.
///
/// The person's type 3 decision entry (find_type3_decision, with the limit of today + 14; without one, a new one at
/// today + 1 + draw(5), slice 2, through `add_decision_date`). With the offer's slot (`offer_slot`): the answer deadline (`get_offer_response_delay`:
/// days × 3 + slices) against what is left (days until the date × 3 + slices, `exe::date_days_since`); the smaller, at least 1,
/// becomes the date (today + quotient of ÷3) and the slice (+ remainder; passing 2 it goes to the next day); an offer
/// with +0x2f and fewer than 4 days: today + 4 + draw(2). Without a slot (−1): once the date is reached, it becomes
/// today and the slice goes up (up to the day's [0xae2c88]); if the person's contract ([this] + 80 × index from the
/// 79-byte record) ends before today + 180, today + 4 + draw(4). At the end, the date is no later than today + 14, and
/// +0x1c goes up.
/// Original: `CONTRACT_MANAGER::schedule_offer_decision` (0x4cd230, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cd230 } (thiscall, ret 8, contract_manager.cpp, SEH só da data local, de destrutor vazio):
/// `CONTRACT_MANAGER::schedule_offer_decision(id, vaga)`: a entrada de decisão de tipo 3 da pessoa (find_type3_decision, com
/// o limite de hoje + 14; sem ela, uma nova em hoje + 1 + sorteio(5), fatia 2, pela 0x4cd6d0). Com a vaga da oferta
/// (0x8c61e0): o prazo da resposta (0x8cc5b0: dias × 3 + fatias) contra o que falta (dias até a data × 3 + fatias,
/// 0x54cdb0), o menor, no mínimo 1, vira a data (hoje + ÷3) e a fatia (+ resto, passando de 2 vai ao dia seguinte);
/// oferta com +0x2f e menos de 4 dias: hoje + 4 + sorteio(2). Sem vaga (−1): chegada a data, ela vira hoje e a fatia
/// sobe (até a do dia [0xae2c88]); se o contrato da pessoa ([this] + 80 × índice do registro de 79 bytes) acaba antes
/// de hoje + 180, hoje + 4 + sorteio(4). No fim, a data não passa de hoje + 14, e +0x1c sobe.
pub(crate) unsafe extern "thiscall" fn schedule_offer_decision(this: usize, id: i32, slot: i32) {
    let mut limit = [0u8; 8];
    injury_date_add_days(G_CURRENT_DATE, 0xe, &mut limit);
    let mut e = find_type3_decision(this, id, &limit, true);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if e == 0 {
        crate::comparison::branch("cpr_10 new decision");
        let mut d = [0u8; 8];
        injury_date_add_days(G_CURRENT_DATE, (random(5) + 1) as u32, &mut d); // cpr10: the decision in 1 to 5 days
        exe::contract_manager_add_decision_date(this, person, d.as_ptr(), 2, 3);
        let n = read_usize(this + contract_manager_fields::DECISION_COUNT);
        e = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add(n.wrapping_sub(1).wrapping_mul(14));
    }
    let data = e + decision_date::DATE;
    if slot != -1 {
        let of = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), slot);
        // simplification: the original passes the slots of both arguments as output; they start with their low bytes
        let (mut a, mut b) = (id as u8, slot as u8);
        exe::transfer_manager_get_offer_response_delay(read_usize(G_TRANSFER_MANAGER), read_usize(of), &mut a, &mut b);
        let target = a.wrapping_mul(3).wrapping_add(b) as i8;
        let foul = exe::date_days_since(data, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
        let mut f = (*((e + decision_date::SLICE) as *const u8)).wrapping_sub(*(G_CURRENT_DAY_SLICE as *const u8)) as i8;
        let mut dl = foul as u8;
        if f < 0 {
            f = f.wrapping_add(3);
            dl = dl.wrapping_sub(1);
        }
        let current = dl.wrapping_mul(3).wrapping_add(f as u8) as i8;
        let x = if current < target { current.max(1) } else { target.max(1) } as i32;
        set_today_plus_days(data, (x / 3) as u32);
        advance_time_slice(e, ((x % 3) as u8).wrapping_add(*(G_CURRENT_DAY_SLICE as *const u8)) as i8);
        if *((read_usize(of) + 0x2f) as *const u8) != 0 && exe::date_days_since(data, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) < 4 {
            set_today_plus_days(data, (random(2) + 4) as u32);
        }
    } else {
        let (td, ta) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
        let (rd, ra) = (read_i16(data), read_u16(data + cm_date::YEAR));
        if ta > ra || (ta == ra && td >= rd) {
            injury_assign_date(data, G_CURRENT_DATE);
            let f = *((e + decision_date::SLICE) as *const i8);
            if f as i32 <= *(G_CURRENT_DAY_SLICE as *const u8) as i32 {
                advance_time_slice(e, f.wrapping_add(1));
            }
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
        if p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 {
            let x = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(79))) as i32;
            let c = Contract(if x >= 0 { read_usize(this).wrapping_add((x as usize).wrapping_mul(0x50)) } else { 0 });
            if !c.is_null() {
                let mut t = [0u8; 8];
                let m = injury_date_add_days(G_CURRENT_DATE, 0xb4, &mut t);
                let (ma, ca) = (read_u16(m + cm_date::YEAR), (c.contract_expires_year() as u16));
                if ma > ca || (ma == ca && read_i16(m) > c.contract_expires()) {
                    set_today_plus_days(data, (random(4) + 4) as u32);
                }
            }
        }
    }
    let (td, ta) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    let mut t = [0u8; 8];
    let m = exe::date_sub_days(data, (t.as_mut_ptr()) as *mut u8 as usize, 0xe);
    let ma = read_u16(m + cm_date::YEAR);
    if ta < ma || (ta == ma && td < read_i16(m)) {
        set_today_plus_days(data, 0xe);
    }
    core::ptr::write_unaligned((this + contract_manager_fields::DECISIONS_ADDED) as *mut u32, read_usize(this + contract_manager_fields::DECISIONS_ADDED).wrapping_add(1) as u32);
}

/// the person's contract record (0x50 bytes)
/// Original: `contract_record` (0x4cf6d0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cf6d0 } (thiscall, ret 4, contract_manager.cpp): a ficha de contrato (0x50 bytes, a partir de
/// [this]) da pessoa `*p`, pelo índice em +4 do registro de 79 bytes em [0xae2d0c]; 0 sem pessoa, com
/// id fora de [0xae2364] ou índice negativo.
pub(crate) unsafe extern "thiscall" fn contract_record(t: usize, p: *const i32) -> usize {
    if p.is_null() {
        return 0;
    }
    let id = *p;
    if id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let k = core::ptr::read_unaligned((*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(79)).wrapping_add(4) as *const i32);
    if k < 0 {
        return 0;
    }
    (*(t as *const usize)).wrapping_add(k as usize * 0x50)
}

/// Returns the category 1..8 of player `p` in contract `c`.
///
/// Without a player sheet (+0x61), 7; without a contract, 4; a type 5 contract (+0x35 & 0x3f), 8; otherwise by the
/// squad status (+0x4f >> 4, 0..6; above, 7), the happiness (+0x3d, signed), the age (+0x18), the ambition (+0x57) and
/// the sheet's reputation (+0xb, +0xd).
/// Original: `contract_category` (0x4ca470, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4ca470 } (stdcall, ret 8, contract_manager.cpp): a categoria 1..8 do jogador `p` no contrato
/// `c`: sem ficha (+0x61), 7; sem contrato, 4; contrato de tipo 5 (+0x35 & 0x3f), 8; senão pela
/// situação (+0x4f >> 4, 0..6; acima, 7), a satisfação (+0x3d, com sinal), a idade (+0x18), a
/// ambição (+0x57) e a reputação da ficha (+0xb, +0xd).
pub(crate) unsafe extern "stdcall" fn contract_category(p: usize, c: usize) -> u8 {
    let c = Contract(c);
    let p = Staff(p);
    if p.is_null() {
        return 7;
    }
    let sheet = p.player();
    if sheet == 0 {
        return 7;
    }
    if c.is_null() {
        return 4;
    }
    if c.contract_type() & 0x3f == 5 {
        return 8;
    }
    let k = c.squad_status() >> 4;
    let v = c.happiness();
    let age = (p.age() as i8);
    let ambicao = p.ambition();
    let rep = |o: usize| core::ptr::read_unaligned((sheet + o) as *const i16);
    let low = |limit: i8| if v <= limit { 7 } else { 4 };
    match k {
        0 => {
            if age > 0x15 {
                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { low(-30) }
            } else if v > 0 {
                5
            } else if v > -30 || ambicao > 5 {
                6
            } else {
                7
            }
        }
        1 => if v > 0x2d { 1 } else { (v <= 0x19) as u8 + 2 },
        2 => {
            let rating = ambicao as i32 * 25 - p.loyalty() as i32 * 10 + rep(0xd) as i32;
            if v > 0x4b && rating > 0x186a && age > 0x17 && age < 0x22 {
                1
            } else if v > 0x19 {
                2
            } else {
                (v <= 0) as u8 + 3
            }
        }
        3 => if v > 0x37 && age > 0x17 && age < 0x22 { 2 } else { (v <= 0) as u8 + 3 },
        4 => {
            if v > 0x41 && age > 0x17 {
                (age < 0x22) as u8 + 2
            } else if v > -30 {
                4
            } else if age >= 0x18 {
                7
            } else {
                5
            }
        }
        5 => {
            if age > 0x18 {
                if age > 0x22 { 4 } else if v > 0x4b { 2 } else if v > 0 { 3 } else { low(-30) }
            } else if ambicao > 5 && rep(0xb) > 0x4e2 {
                5
            } else {
                6
            }
        }
        6 => {
            if age > 0x17 {
                if v > 0x4b { 1 } else if v > 0x1e { 2 } else if v > 0 { 3 } else { low(-50) }
            } else if rep(0xb) > 0x4e2 && ambicao > 0xf {
                5
            } else {
                6
            }
        }
        _ => 7,
    }
}

/// `CONTRACT_MANAGER::post_transfer_listed_news(player, club)` (SEH frame for the local list), called by the transfer
/// request (`handle_transfer_request`), the player put up for sale (`transfer_list_player`), the news (`exe::newsitem_auto_respond`) and 0x4dc3a0: posts news item 0xbea
/// of a player on the transfer list, with the price and the strongest interested clubs, to the interested human clubs.
///
/// Without a player, a player sheet (+0x61), a club or the staff (+0xcf): nothing. Without a contract with the club
/// (`exe::contract_manager_get_contract_for_club`): nothing. The news item 0xbea (local): 0 the club, 1 the player, 2..4 his +0x33 of +4, +8 and +0xc, 5 the
/// staff, 6..8 its +0x33, 9 the contract +0x45, 10 the high nibble of +0x4f, 11 the positions (exe::formation_get_position_mask(`position_mask`) as i16), 12 a
/// draw of 2 (`exe::utils_random`).
///
/// The price: the value (+0x52) with a human club, when the player is not on the club's list (`staff_is_transfer_listed`) or the contract
/// has nibble 1; otherwise the asking price (`exe::contract_manager_calc_asking_price`) × (draw of 0x29 × [0x967898] + [0x9689b0]) truncated, rounded
/// down to 10 thousand (between 10 thousand and 200 thousand, exclusive), 100 thousand (below 1 million), 250 thousand
/// (below 5 million) or 1 million; 13 the price. From 500 thousand up: the interested clubs (`collect_interested_clubs`), up to 30, the
/// computer ones and the human ones with a budget (0x598800) ≥ price × [0x9689c0]; sorted by reputation (+0x80); 14 =
/// up to 3; 15.. the ids, with 1 (in place 13) if the 2nd repeats the 1st and 2 (in 12) if the 3rd repeats one of them;
/// finally the "off the list" in the next place. The recipients (`exe::transfer_manager_collect_player_news_recipients` with 0x21a, up to 30, with [0xde1f54] and the
/// sheet) and the delivery (`send_to_club_manager`).
/// Original: `CONTRACT_MANAGER::post_transfer_listed_news` (0x4d71b0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4d71b0 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da lista local): `CONTRACT_MANAGER::
/// post_transfer_listed_news(jogador, clube)`, pelo pedido de transferência (0x4d8610), pelo jogador posto à venda
/// (0x842320), pela notícia (0x784100) e por 0x4dc3a0. Sem jogador, sem ficha (+0x61), sem clube ou sem a comissão
/// (+0xcf): nada. Sem contrato com o clube (0x4cf5f0): nada. A notícia 0xbea (local): 0 o clube, 1 o jogador, 2..4 os
/// +0x33 de +4, +8 e +0xc dele, 5 a comissão, 6..8 os +0x33 dela, 9 o contrato +0x45, 10 o nibble alto de +0x4f, 11 a
/// máscara de posições (0x5b7d20), 12 um sorteio de 2 (0x90cfc0). O preço: o valor (+0x52) com clube humano, quando o
/// jogador não está na lista do clube (0x5444a0) ou o contrato tem o nibble 1; senão o pedido (0x4d1600) × (sorteio de
/// 0x29 × [0x967898] + [0x9689b0]) truncado, arredondado para baixo em 10 mil (entre 10 mil e 200 mil, exclusive), 100 mil
/// (abaixo de 1 milhão), 250 mil (abaixo de 5 milhões) ou 1 milhão; 13 o preço. De 500 mil para cima: os interessados
/// (0x842180), até 30, os do computador e os humanos com verba (0x598800) ≥ preço × [0x9689c0]; ordenados pela reputação
/// (+0x80); 14 = até 3; 15.. os ids, com 1 (no lugar 13) se o 2º repete o 1º e 2 (no 12) se o 3º repete um deles; por
/// fim o "fora da lista" no lugar seguinte. Os destinatários (0x8d1da0 com 0x21a, até 30, com [0xde1f54] e ficha) e a
/// entrega (0x7861e0).
pub(crate) unsafe extern "thiscall" fn post_transfer_listed_news(this: usize, player: usize, club: usize) {
    let player = Staff(player);
    let club = Club(club);
    let request: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
    if player.is_null() || player.player() == 0 || club.is_null() || club.manager().addr() == 0 {
        return;
    }
    let c = exe::contract_manager_get_contract_for_club(this, player, club);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    if c != 0 {
        let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        exe::new_newsitem(n, 0xbea, 0);
        exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
        exe::newsitem_set_slot(n, 1, read_usize(player.addr()) as u32);
        exe::newsitem_set_slot(n, 2, read_usize(player.first_name() + 0x33) as u32);
        exe::newsitem_set_slot(n, 3, read_usize(player.second_name() + 0x33) as u32);
        exe::newsitem_set_slot(n, 4, read_usize(player.common_name() + 0x33) as u32);
        let com = club.manager().addr();
        exe::newsitem_set_slot(n, 5, read_usize(com) as u32);
        exe::newsitem_set_slot(n, 6, read_usize(club.manager().first_name() + 0x33) as u32);
        exe::newsitem_set_slot(n, 7, read_usize(club.manager().second_name() + 0x33) as u32);
        exe::newsitem_set_slot(n, 8, read_usize(club.manager().common_name() + 0x33) as u32);
        exe::newsitem_set_slot(n, 9, read_usize(c + 0x45) as u32);
        exe::newsitem_set_slot(n, 10, (read_u8(c + 0x4f) >> 4) as u32);
        exe::newsitem_set_slot(n, 11, (exe::formation_get_position_mask(player.player()) as i16) as i32 as u32);
        crate::comparison::branch("cqf_2 listed player news");
        let s = exe::utils_random(2); // cqf2: the random draw of die 12
        exe::newsitem_set_slot(n, 12, s as u32);
        let away_of_list = (exe::staff_is_transfer_listed(player, club) == 0) as u32;
        let price = if exe::human_manager_is_human_club(club, 0, (1) as u32, (0) as u32, 0) != 0 && (away_of_list == 0 || read_u8(c + 0x4f) & 0xf0 == 0x10) {
            player.value() as i32
        } else {
            let p = request(this, club.addr(), 0, player.addr(), 0, 1, 1);
            let r = exe::utils_random(0x29);
            let v = ftol((r as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_8)) * p as f64); // cqf2: the price of the news;
            if v > 0x2710 && v < 0x30d40 {
                v / 10000 * 10000
            } else if v < 1_000_000 {
                v / 100000 * 100000
            } else if v < 5_000_000 {
                v / 250000 * 250000
            } else {
                v / 1_000_000 * 1_000_000
            }
        };
        exe::newsitem_set_slot(n, 0xd, price as u32);
        if price >= 500_000 {
            crate::comparison::branch("cqf_2 interested in listed player");
            let mut clubs = [0usize; 30];
            let mut q: i8 = 0;
            exe::shortlist_manager_collect_interested_clubs(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), player, l, 1, 1, 0);
            write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
            let mut no = exe::llist_get_next(l);
            while no != 0 && q < 0x1e {
                let id = read_usize(no + 4);
                let can = if exe::human_manager_is_human_club(transfer_manager_club_by_id(id), 0, (1) as u32, (0) as u32, 0) == 0 {
                    true
                } else {
                    let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(read_usize(no + 4)));
                    f != 0 && price as f64 * read_f64(rdata::F64_1_1) <= exe::finance_get_transfer_budget(f, 0, 0, 0, 1, 0) as f64
                };
                if can && read_usize(no + 4) as i32 > -1 {
                    clubs[q as usize] = transfer_manager_club_by_id(read_usize(no + 4));
                    q += 1;
                }
                no = exe::llist_get_next(l);
            }
            let total = q as i32;
            let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16);
            let mut i = 0i32;
            while i < total - 1 {
                let mut j = i + 1;
                while j < total {
                    if rep(clubs[i as usize]) < rep(clubs[j as usize]) {
                        clubs.swap(i as usize, j as usize);
                    }
                    j += 1;
                }
                i += 1;
            }
            let m = if q >= 3 { 3 } else { total };
            exe::newsitem_set_slot(n, 0xe, m as u32);
            let mut place: i8 = 0xf;
            let mut k: i8 = 0;
            let end = loop {
                if k as i32 >= m {
                    break place;
                }
                if k == 1 && clubs[1] == clubs[0] {
                    exe::newsitem_set_slot(n, place - 2, 1);
                    break place;
                }
                if k == 2 && (clubs[2] == clubs[0] || clubs[2] == clubs[1]) {
                    exe::newsitem_set_slot(n, place - 3, 2);
                    break place;
                }
                exe::newsitem_set_slot(n, place, read_usize(clubs[k as usize]) as u32);
                place += 1;
                k += 1;
            };
            // simplification: the place goes in a dword with the high bytes of the price in the original; 0x785790 reads the byte
            exe::newsitem_set_slot(n, end, away_of_list);
            let mut dest = clubs;
            let qd = if read_usize(G_TRANSFER_MANAGER) != 0 && player.player() != 0 {
                recipients(read_usize(G_TRANSFER_MANAGER), player.addr(), 0x21a, dest.as_mut_ptr(), 0x1e) as i8
            } else {
                0
            };
            for &d in dest.iter().take(if qd > 0 { qd as usize } else { 0 }) {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, d);
            }
        }
    }
    exe::drop_llist(l);
}

/// `CONTRACT_MANAGER::schedule_loan_end(person, club)`, only from the transfers screen (0x8fbcc0): the person's loan
/// contract ends tomorrow (type 7) and, with a human manager at the relevant club, the early return.
///
/// Without a person, with an id outside the total or without the loan contract (+4 of the 79-byte record, in this's
/// vector): the box (line 0xe4a). Otherwise the end (+0x2d) becomes tomorrow (`exe::date_add_days` with 1; `exe::date_operator_3d`) and the type
/// (+0x35) becomes 7 (bits 3..5 zeroed). The current contract (+0 of the record, in the vector of [0xae1050]) in
/// [0xae1080]: from `club`, the person's club (+0x39) with a human manager (`news_club_has_human_manager`, stdcall: ecx = [0xb7431c] is not
/// read); from another, the current contract's club (read again twice) with a human manager: the return (`exe::contract_manager_post_loan_recall_news`: push
/// club; push person; mov ecx, this).
/// Original: `CONTRACT_MANAGER::schedule_loan_end` (0x4ccca0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4ccca0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::schedule_loan_end(pessoa, clube)`, só pela
/// tela de transferências (0x8fbcc0). Sem pessoa, id fora do total ou sem o contrato de empréstimo (+4 do registro de 79
/// bytes, no vetor de this): a caixa (linha 0xe4a). Senão o fim (+0x2d) vira amanhã (0x54c5b0 com 1; 0x417c10) e o tipo
/// (+0x35) vira 7 (os bits 3..5 zerados). O contrato atual (+0 do registro, no vetor de [0xae1050]) em [0xae1080]: do
/// `clube`, o clube da pessoa (+0x39) com técnico humano (0x7875e0, stdcall: o ecx = [0xb7431c] não é lido); de outro,
/// o clube do contrato atual (lido de novo duas vezes) com técnico humano: a volta (0x4db0b0: push clube; push pessoa;
/// mov ecx, this).
pub(crate) unsafe extern "thiscall" fn schedule_loan_end(this: usize, person: usize, club: usize) {
    let person = Staff(person);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let c = Contract(if person.is_null() { 0 } else { staff_contract_by_field(person, 4, this) });
    if c.is_null() {
        show_error_box(0x9a8190, 0xe4a);
        return;
    }
    let mut tomorrow = [0u32; 2];
    let p = exe::date_add_days(G_CURRENT_DATE, (&mut tomorrow) as *mut [u32; 2] as usize, 1);
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(c.addr() + contract::CONTRACT_EXPIRES, tmp.as_mut_ptr() as usize, p);
    c.set_contract_type((c.contract_type() & 0xc7) | 7);
    let a = staff_contract_by_field(person, 0, read_usize(G_CONTRACT_MANAGER));
    write_usize(G_CONTRACT_LOOKUP_RESULT, a);
    let current = if a == 0 { 0 } else { transfer_manager_club_by_id(read_usize(a + contract::CLUB)) };
    if club == current {
        let x = person.club_job();
        if x.is_null() || human_manager_record_checked(x.addr()) == 0 {
            return;
        }
    } else {
        let a = staff_contract_by_field(person, 0, read_usize(G_CONTRACT_MANAGER));
        write_usize(G_CONTRACT_LOOKUP_RESULT, a);
        if a == 0 || transfer_manager_club_by_id(read_usize(a + contract::CLUB)) == 0 {
            return;
        }
        let a = staff_contract_by_field(person, 0, read_usize(G_CONTRACT_MANAGER));
        write_usize(G_CONTRACT_LOOKUP_RESULT, a);
        let x = if a == 0 { 0 } else { transfer_manager_club_by_id(read_usize(a + contract::CLUB)) };
        if human_manager_record_checked(x) == 0 {
            return;
        }
    }
    exe::contract_manager_post_loan_recall_news(this, person, club);
}

/// `CONTRACT_MANAGER::post_contract_end_news(person, type, wide)` (SEH frame for the local LLIST and news item), called
/// by the end of a contract, trial or loan (0x4c7ac0): posts news item 0xbb9 to the club, the interested clubs or, for
/// famous players, the nation's managers or everyone.
///
/// Without a contract or without its club: nothing. The news item 0xbb9: the contract club, the person (the id and the
/// three keys), the type, the contract club for type 6 (otherwise −1) and `wide`. Wide, with a player sheet: world
/// reputation (+0xd) above 7000, to all managers (`send_to_all_managers`); national (+0xb) above 7000, to the managers of the
/// person's nation, of the club's and of the current club's (+0x39) (`send_to_nation_managers`) and to the interested clubs (`collect_interested_clubs`)
/// of other nations. Otherwise to the club, to the current club for type 6 and to the interested clubs (`send_to_club_manager`).
/// Original: `post_contract_end_news` (0x4da590, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4da590 } (thiscall, ret 0xc, contract_manager.cpp, quadro SEH da LLIST e da notícia locais): `CONTRACT_MANAGER::
/// post_contract_end_news(pessoa, tipo, amplo)`, pelo fim de contrato, de teste ou de empréstimo (0x4c7ac0). Sem
/// contrato ou sem o clube dele: nada. A notícia 0xbb9: o clube do contrato, a pessoa (o id e as três chaves), o tipo, o
/// clube do contrato no tipo 6 (senão −1) e `amplo`. Ampla, com ficha: reputação mundial (+0xd) acima de 7000, a todos
/// os técnicos (0x786780); nacional (+0xb) acima de 7000, aos técnicos da nação da pessoa, da do clube e da do clube
/// atual (+0x39) (0x7863f0) e aos clubes interessados (0x842180) de outras nações. Senão ao clube, ao clube atual no
/// tipo 6 e aos interessados (0x7861e0).
pub(crate) unsafe extern "thiscall" fn post_contract_end_news(this: usize, p: usize, kind: u32, amplo: u32) {
    let p = Staff(p);
    let c = contract_inline_lookup(p);
    if c == 0 || friendly_club_by_id(read_usize(c + contract::CLUB)) == 0 {
        return;
    }
    let mut list = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    crate::comparison::branch("cpx_4 contract end news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0xbb9, 0);
    let c = contract_inline_lookup(p);
    // the original reads the id at address 0 without a contract: does not happen, it has just found it
    let club = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    exe::newsitem_set_slot(nt, 0, read_usize(club) as u32);
    exe::newsitem_set_slot(nt, 2, read_usize(p.addr()) as u32);
    exe::newsitem_set_slot(nt, 3, read_usize(p.first_name() + 0x33) as u32);
    exe::newsitem_set_slot(nt, 4, read_usize(p.second_name() + 0x33) as u32);
    exe::newsitem_set_slot(nt, 5, read_usize(p.common_name() + 0x33) as u32);
    let kind = kind as u8;
    exe::newsitem_set_slot(nt, 6, kind as i8 as u32);
    let c = contract_inline_lookup(p);
    let club = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    if kind == 6 {
        // the contract’s club from the manager’s own table (+0), without storing it in [0xae1080]; with no contract, the original
        // reads address 4 (does not happen: it has just found it)
        let k = if p.is_null() || read_usize(p.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            -1
        } else {
            read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p.addr()).wrapping_mul(0x4f))) as i32
        };
        let v = if k < 0 { read_usize(4) } else { read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(0x50)) + 4) };
        exe::newsitem_set_slot(nt, 7, v as u32);
    } else {
        exe::newsitem_set_slot(nt, 7, u32::MAX);
    }
    exe::newsitem_set_slot(nt, 8, amplo);
    let sheet = p.player();
    let current = Club(p.club_job().addr());
    let nation = p.nation();
    let jornal = read_usize(G_NEWS);
    let mut default = true;
    if amplo != 0 && sheet != 0 {
        if read_i16(sheet + player_sheet::WORLD_REPUTATION) > 0x1b58 {
            exe::news_send_to_all_managers(jornal, nt);
            default = false;
        } else if read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 0x1b58 {
            default = false;
            exe::news_send_to_nation_managers(jornal, nt, nation);
            if nation != Nation(read_usize(club + club::NATION)) {
                exe::news_send_to_nation_managers(jornal, nt, read_usize(club + club::NATION));
            }
            if !current.is_null() && current.addr() != club {
                let n = current.nation().addr();
                if n != read_usize(club + club::NATION) && n != nation.addr() {
                    exe::news_send_to_nation_managers(jornal, nt, n);
                }
            }
            for id in interested_club_ids(p, l) {
                if id == read_usize(club) || (kind == 6 && id == read_usize(p.club_job().addr())) {
                    continue;
                }
                let other = Club(friendly_club_by_id(id));
                let n = other.nation().addr();
                if n != p.nation().addr() && n != read_usize(p.club_job().addr() + 0x53) && n != read_usize(club + club::NATION) {
                    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, other);
                }
            }
        }
    }
    if default {
        exe::news_send_to_club_manager(jornal, nt, club);
        if kind == 6 {
            exe::news_send_to_club_manager(read_usize(G_NEWS), nt, p.club_job());
        }
        for id in interested_club_ids(p, l) {
            if id == read_usize(club) || (kind == 6 && id == read_usize(p.club_job().addr())) {
                continue;
            }
            exe::news_send_to_club_manager(read_usize(G_NEWS), nt, friendly_club_by_id(id));
        }
    }
    exe::drop_llist(l);
}

