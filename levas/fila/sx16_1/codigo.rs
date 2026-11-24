// --- sx16_1: player_regen_regenerate_as_non_player (0x7ab340, player_regen.cpp) ---

/// Exe routines the non-player regeneration calls by address (Rust hooks among them, called through the exe so the
/// comparison runs their comparers).
pub(crate) mod regen_non_player_fns {
    pub(crate) const REWIND_STAFF_HISTORY: usize = 0x86fb80; // Staff_Records::rewind_staff_history, thiscall(this, person), ret 4
    pub(crate) const REMOVE_NICKNAME_FROM_ALL: usize = 0x5fff30; // HUMAN_MANAGER::remove_nickname_from_all, thiscall(this, person), ret 4
    pub(crate) const REMOVE_TALKS_RECORDS: usize = 0x4ca1c0; // CONTRACT_MANAGER::remove_talks_records, thiscall(this, person, manager, club), ret 0xc
    pub(crate) const STAFF_CAN_TAKE_JOB: usize = 0x4cb990; // staff_can_take_job, stdcall(job, person), ret 8
}

/// The age band of a regenerated non-player by his job (the job byte minus 1 indexes the original's jump table 0x7aba98):
/// (random range, base). Chairman (1) is handled apart; the manager (5), the scout (9) and anything else fall to the
/// default (15, 0x27).
const NON_PLAYER_AGE_BANDS: [(i32, i8); 10] = [
    (0x14, 0x24), // 1 chairman (the attributes by the club, see regen_chairman_attributes)
    (0x19, 0x28), // 2 managing director
    (0x19, 0x28), // 3 general manager
    (0xa, 0x25),  // 4 director of football
    (0xf, 0x27),  // 5 manager (default)
    (0xf, 0x23),  // 6 assistant manager
    (0xf, 0x23),  // 7 reserve team manager
    (0xf, 0x20),  // 8 coach
    (0xf, 0x27),  // 9 scout (default)
    (0x19, 0x23), // 10 physio
];

/// The formation preference of a regenerated non-player from a roll of 0..124: the first band the roll is below.
const NON_PLAYER_FORMATION_BANDS: [i32; 9] = [0x22, 0x28, 0x3c, 0x5a, 0x5e, 0x60, 0x6c, 0x6f, 0x79];

/// The business attributes of a regenerated chairman by his club's reputation (`club::REPUTATION`, signed):
/// - above 6250 (0x186a) with `club::PLC` set: business 10..20, directness, discipline and interference 1..20, man
///   handling (0..99) − (0..99), patience and motivating 1..20, resources 6..20;
/// - above 6250 without it: business 1..20, the same middle, resources 10..20;
/// - above 3250 (0xcb2): business 1..20, the same middle, patience 5..20 in 4 of 5 (else 1..20), motivating 1..20,
///   resources 1..15 in 9 of 10 (else 6..20);
/// - otherwise: business 1..20, the same middle, patience 6..20 in 9 of 10 (else 1..20), motivating (0..99) −
///   (0..99), resources 6..20 in 99 of 100 (else 1..10).
/// The rolls go in this order, one `utils_random` each.
unsafe fn regen_chairman_attributes(np: NonPlayer, club: Club) {
    let r = |n: i32| friendly_random(n) as u8;
    let reputation = club.reputation();
    let top = reputation > 0x186a;
    let plc = top && club.plc() != 0;
    np.set_business(if plc { r(0xb).wrapping_add(0xa) } else { r(0x14).wrapping_add(1) });
    np.set_directness(r(0x14).wrapping_add(1) as i8);
    np.set_discipline(r(0x14).wrapping_add(1));
    np.set_interference(r(0x14).wrapping_add(1) as i8);
    let a = r(0x64);
    np.set_man_handling(a.wrapping_sub(r(0x64)));
    if top {
        covered!("sx16_1 chairman of a big club");
        np.set_patience(r(0x14).wrapping_add(1));
        np.set_motivating(r(0x14).wrapping_add(1));
        np.set_resources((if plc { r(0xf).wrapping_add(6) } else { r(0xb).wrapping_add(0xa) }) as i8);
    } else if reputation > 0xcb2 {
        covered!("sx16_1 chairman of a middle club");
        np.set_patience(if r(5) != 0 { r(0x10).wrapping_add(5) } else { r(0x14).wrapping_add(1) });
        np.set_motivating(r(0x14).wrapping_add(1));
        np.set_resources((if r(0xa) != 0 { r(0xf).wrapping_add(1) } else { r(0xf).wrapping_add(6) }) as i8);
    } else {
        covered!("sx16_1 chairman of a small club");
        np.set_patience(if r(0xa) != 0 { r(0xf).wrapping_add(6) } else { r(0x14).wrapping_add(1) });
        let a = r(0x64);
        np.set_motivating(a.wrapping_sub(r(0x64)));
        np.set_resources((if r(0x64) != 0 { r(0xf).wrapping_add(6) } else { r(0xa).wrapping_add(1) }) as i8);
    }
}

/// The languages of a regenerated person: every language removed (1..=64), then each native language whose points
/// (`player_stats_get_native_language_points`) are nonzero and at least (20 − adaptability) × 10 added back with the
/// fluency (points − that base) / 75, at most 10. The talks records go between the two loops, as in the original.
unsafe fn regen_non_player_languages(p: Staff) {
    let stats = read_usize(G_PLAYER_STATS);
    let base = 0x14i16.wrapping_sub(p.adaptability() as i16).wrapping_mul(10);
    for k in 1..=0x40i8 {
        let v = exe::player_stats_get_native_language_points(p, k);
        if v != 0 && v >= base {
            let q = (v as i32 - base as i32) / 0x4b;
            exe::playerstats_add_language(stats, p, k as u8 as u32, q.min(0xa) as u32, 1);
        }
    }
}

/// The new contract of a regenerated non-player who keeps a club job: when he can take the job
/// (`staff_can_take_job(job_for_club, 0)`), the old contract goes (`CONTRACT_MANAGER::remove_contract`), the wage is 0,
/// the expiry date is day 1..28 of the club's season-end month (`contract_manager_season_end_month`) in the year now +
/// 1..6 (rolled in that order: the year, the month, the day) and, with a contract record, a fresh initial contract
/// (`STAFF_CONTRACT::generate_initial_contract(contract, person, 0)`).
unsafe fn regen_non_player_contract(p: Staff, club: Club) {
    let can_take: unsafe extern "stdcall" fn(u32, usize) -> u32 = core::mem::transmute(regen_non_player_fns::STAFF_CAN_TAKE_JOB);
    if can_take(p.job_for_club() as u32, 0) == 0 {
        return;
    }
    covered!("sx16_1 new contract");
    let manager = read_usize(G_CONTRACT_MANAGER);
    exe::contract_manager_remove_contract(manager, p);
    p.set_wage(0);
    let years = friendly_random(6);
    let month = exe::contract_manager_season_end_month(manager, club);
    let day = (friendly_random(0x1c) as i8).wrapping_add(1) as i16;
    let year = (read_i16(G_CURRENT_DATE_YEAR) as i32 + years + 1) as u16;
    let mut expiry = [0u16; 4];
    let mut scratch = [0u16; 4];
    let date = exe::date_date_549f70(&mut expiry, day as u16 as u32, month as u32, year as u32, u32::MAX);
    exe::date_operator_3d(p.date_expires_club_addr(), scratch.as_mut_ptr() as usize, date);
    let contract = exe::contract_manager_get_contract(manager, p);
    if contract != 0 {
        exe::staff_contract_generate_initial_contract(contract, p, 0);
    }
}

/// Rebuilds a retired staff slot as a new non-player (the regeneration after a retirement, from `process_retirements`).
///
/// The retirement bits (0x30 of the state's `staff_state::STATUS_FLAGS`) cleared; `CLASSIFICATION` 1 (non-player);
/// the staff history rewound (`Staff_Records::rewind_staff_history`); the caps and international goals 0; the low
/// nibble of the state flags and the state bytes +8 and +9 set to 0 and 0x9c (156). New first, second and common names
/// for his nation; his nickname removed from every human (`HUMAN_MANAGER::remove_nickname_from_all`); joined the club
/// today. Every language removed, the talks records removed (`remove_talks_records(person, 0, 0)`) and the native
/// languages added back (`regen_non_player_languages`).
///
/// The non-player sheet (`staff::NON_PLAYER`): current ability 1 + roll(potential × 0.75), raised to potential × 0.5
/// when below; home reputation roll(750) + ability × 25 + 1, world reputation roll(200) + ability × 15 + 1 (a home
/// below the world becomes world + 10 and the world 1); current reputation roll(50) + home. The age by the job
/// (`NON_PLAYER_AGE_BANDS`, the club job, else the national job): a chairman 36..55 with the business attributes by his
/// club (`regen_chairman_attributes`, when he has a club and a sheet). Birth year this year − age, birth day roll(365),
/// the age recomputed (`database_get_staff_age`), the formation preference by a roll of 125
/// (`NON_PLAYER_FORMATION_BANDS`). With a club and a sheet, the new contract (`regen_non_player_contract`). Last, the
/// attributes normalised (`staff_normalise_non_player_attributes`).
///
/// Defects of the original: without a club job or a national job the job byte is the argument slot's low byte, which
/// the language loops left at 0x41, so every jobless person gets the default band (manager's 39..53); the sheet
/// (`NON_PLAYER`) is written without a null test (only the chairman and the contract test it).
/// Original: `player_regen_regenerate_as_non_player` (0x7ab340, player_regen.cpp, thiscall, ret 4; `this` unused).
pub(crate) unsafe extern "thiscall" fn regenerate_as_non_player(_this: usize, person: usize) {
    let p = Staff(person);
    covered!("sx16_1 regenerate as non-player");
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0xcf;
    p.set_classification(1);
    call_thiscall_1(regen_non_player_fns::REWIND_STAFF_HISTORY, read_usize(G_STAFF_RECORDS), p);
    p.set_int_apps(0);
    p.set_int_goals(0);
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0xf0;
    *staff_state_byte(p, 8) = 0; // state byte +8 (not named yet)
    *staff_state_byte(p, 9) = 0x9c; // state byte +9 (not named yet)
    p.set_first_name(exe::database_pick_first_name_for_nation(p.nation()));
    p.set_second_name(exe::database_pick_second_name_for_nation(p.nation()));
    p.set_common_name(exe::database_get_empty_common_name(p.nation()));
    call_thiscall_1(regen_non_player_fns::REMOVE_NICKNAME_FROM_ALL, read_usize(G_HUMAN_MANAGER), p);
    p.set_date_joined_club_year(read_i16(G_CURRENT_DATE_YEAR) as u16);
    write_i16(p.addr() + staff::DATE_JOINED_CLUB, read_i16(G_CURRENT_DATE));
    for k in 1..=0x40i8 {
        exe::playerstats_remove_language(read_usize(G_PLAYER_STATS), p, k);
    }
    let remove_talks: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(regen_non_player_fns::REMOVE_TALKS_RECORDS);
    remove_talks(read_usize(G_CONTRACT_MANAGER), p.addr(), 0, 0);
    regen_non_player_languages(p);

    let np = p.non_player();
    let potential = np.potential_ability() as i16 as f64;
    np.set_current_ability((friendly_random(ftol(potential * read_f64(rdata::F64_0_75))) + 1) as i16);
    let half = potential * read_f32(rdata::F32_0_5) as f64;
    if (np.current_ability() as f64) < half {
        np.set_current_ability(ftol(half) as i16);
    }
    let r = friendly_random(0x2ee) as i16;
    np.set_home_reputation(r.wrapping_add(np.current_ability().wrapping_mul(0x19)).wrapping_add(1)); // sx16_1: home reputation
    let r = friendly_random(0xc8) as i16;
    np.set_world_reputation(r.wrapping_add(np.current_ability().wrapping_mul(0xf)).wrapping_add(1));
    if np.home_reputation() < np.world_reputation() {
        np.set_home_reputation(np.world_reputation().wrapping_add(0xa));
        np.set_world_reputation(1);
    }
    let r = friendly_random(0x32) as i16;
    np.set_current_reputation(r.wrapping_add(np.home_reputation()));

    let club = p.club_job();
    let job = if !club.is_null() {
        p.job_for_club()
    } else if !p.national_job().is_null() {
        p.job_for_nation()
    } else {
        covered!("sx16_1 jobless");
        0x41 // the original reads the argument slot the language loops left at 0x41
    };
    let (range, base) = NON_PLAYER_AGE_BANDS.get((job as i8 as i32 - 1) as usize).copied().unwrap_or((0xf, 0x27));
    let age = (friendly_random(range) as i8).wrapping_add(base); // sx16_1: the age by the job
    if job == job_kind::CHAIRMAN && !club.is_null() && !np.is_null() {
        regen_chairman_attributes(np, club);
    }
    p.set_date_of_birth_year(read_i16(G_CURRENT_DATE_YEAR).wrapping_sub(age as i16) as u16);
    p.set_date_of_birth(friendly_random(0x16d) as i16);
    p.set_age(exe::database_get_staff_age(p, G_CURRENT_DATE as *const u16) as i16);
    let roll = friendly_random(0x7d);
    let formation = NON_PLAYER_FORMATION_BANDS.iter().position(|&b| roll < b).unwrap_or(9);
    np.set_formation(formation as i8);
    if !club.is_null() && !np.is_null() {
        regen_non_player_contract(p, club);
    }
    exe::staff_normalise_non_player_attributes(p);
}

/// Comparer for `regenerate_as_non_player`: it rolls the generator, rewrites the person and calls routines with side
/// effects (contracts, staff history, nicknames), so it cannot run twice; only the original runs. The copy is proven by
/// the Rust careers and the mutations.
pub(crate) unsafe extern "thiscall" fn compare_regenerate_as_non_player(this: usize, person: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, person)
}
