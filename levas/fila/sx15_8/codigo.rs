// ---------- batch %%LEVA%%: the national teams object built for a new game or from the saved game (national_teams.cpp) ----------

/// `FIFA_RANKINGS::FIFA_RANKINGS(save)` (thiscall, ret 4, returns this): the rankings, from the save when one is given.
const FN_FIFA_RANKINGS_CONSTRUCTOR: usize = 0x58cea0;
/// `NationalTeams::load_dat(save)` (thiscall, ret 4): this item's loader, called by the constructor through the exe.
const FN_NATIONALTEAMS_LOAD_DAT: usize = 0x7763b0;
/// `NationalTeams::init_tables()` (thiscall, ret 0): the tables of a new game (Rust since sma_4).
const FN_NATIONALTEAMS_INIT_TABLES: usize = 0x76a840;
/// "national_teams.dat", the national teams item of the saved game.
const NATIONAL_TEAMS_DAT: usize = 0xa63020;
/// The `jne` on the drive type of the constructor's CD check (nops in a no-CD exe such as cm0102-1080).
const NATIONAL_TEAMS_CD_DRIVE_TYPE_JUMP: usize = 0x76a543;
/// The `test eax, eax` on the found file of the constructor's CD check (`xor` in a no-CD exe).
const NATIONAL_TEAMS_CD_FOUND_TEST: usize = 0x76a586;

/// Builds the national teams object for a new game (`save` 0) or from the saved game.
///
/// `NationalTeams::NationalTeams(save)`: the new qualifiers list (`national_teams_fields::NEW_QUALIFIERS`, an inline
/// LLIST) is built; the CD is checked (`find_game_cd`: the first CD drive holding cm0102.exe, else the "Insert CD" box
/// and again); a new game loads the player setup (`player_setup_cfg_load(4, 0)`). The FIFA rankings
/// (`FIFA_RANKING`: `operator new(0xd)` and `FIFA_RANKINGS::FIFA_RANKINGS(save)`, 0 when the allocation fails), the
/// call-up list (`CALL_UP_LIST`) and the club marks (`CLUB_MARKS`) cleared, and the club call-up records
/// (`CLUB_CALL_UP_RECORDS`, `calloc(clubs without the national sides, 6)`; none: the national_teams.cpp box of line
/// 0x2163, and it goes on). With a save, `load_dat(save)`; without one, or when the load returns 0, `init_tables()`.
/// Returns this.
/// Original: `NationalTeams::NationalTeams` (0x76a4e0, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nationalteams_constructor(this: usize, save: usize) -> usize {
    use national_teams_fields as f;
    exe::new_llist(this + f::NEW_QUALIFIERS);
    find_game_cd(NATIONAL_TEAMS_CD_DRIVE_TYPE_JUMP, NATIONAL_TEAMS_CD_FOUND_TEST);
    if save == 0 {
        let player_setup: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
        player_setup(4, 0);
    }
    let rankings = exe::crt_operator_new(0xd);
    let rankings = if rankings != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FIFA_RANKINGS_CONSTRUCTOR);
        construct(rankings, save)
    } else {
        0
    };
    write_usize(this + f::FIFA_RANKING, rankings);
    write_usize(this + f::CALL_UP_LIST, 0);
    write_usize(this + f::CLUB_MARKS, 0);
    let records = exe::crt_calloc(club_call_up_record_count() as u32, 6);
    write_usize(this + f::CLUB_CALL_UP_RECORDS, records);
    if records == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, 0x2163);
    }
    let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_LOAD_DAT);
    if save == 0 || load(this, save) == 0 { // sx15_8: a new game builds the tables
        let init: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NATIONALTEAMS_INIT_TABLES);
        init(this);
    }
    this
}

/// Comparer for `nationalteams_constructor`: it allocates the object's tables and asks for the CD (it cannot run
/// twice), so only the original runs; the copy is proven by the save proof (a new game saved, and the holiday save
/// loaded).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_constructor(this: usize, save: usize) -> usize {
    // ponytail: builds the object once; the save proof checks the copy
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, save)
}

/// One reserve pool read back from the saved game: its size (4 bytes into `this + count_field`), a new table of size
/// dwords (`malloc`, into `this + pool_field`) and the size person ids, each turned into the person (−1: none). False
/// after the national_teams.cpp box of the failing step (`lines`: the size, the allocation, the ids).
unsafe fn load_reserve_pool(this: usize, count_field: usize, pool_field: usize, z: usize, lines: [u32; 3]) -> bool {
    if exe::zread(this + count_field, 4, 1, z) != 1 {
        show_error_at(NATIONAL_TEAMS_CPP, lines[0]);
        return false;
    }
    let pool = exe::crt_malloc(read_u32(this + count_field) << 2);
    write_usize(this + pool_field, pool);
    if pool == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, lines[1]);
        return false;
    }
    let count = read_i32(this + count_field);
    if exe::zread(pool, 4, count as u32, z) != count as u32 {
        show_error_at(NATIONAL_TEAMS_CPP, lines[2]);
        return false;
    }
    for k in 0..count {
        let slot = read_usize(this + pool_field) + k as usize * 4;
        let id = read_i32(slot);
        write_usize(slot, if id != -1 { staff_at(id as isize).addr() } else { 0 }); // sx15_8: id to person
    }
    true
}

/// Reads the national teams object back from the saved game.
///
/// `NationalTeams::load_dat(save)`: no save: 0, no box. The "national_teams.dat" item (`zopen` with 0, 0x16, 4); then
/// each step checked, a failure showing the national_teams.cpp box of its line and returning 0 with the item left
/// open:
/// - the nation records (`NATION_RECORDS`, `malloc(nations × 0x6e)`, none: line 0x1f83), the next competitions
///   (`NEXT_COMPETITIONS`, `malloc(nations × 4)`) and the Olympic marks (`OLYMPIC_NATION_MARKS`, `calloc(1, nations)`)
///   allocated, and the nations' records read (line 0x1f91);
/// - the senior reserve pool (size +0x14, table +0xc; lines 0x1f98, 0x1fa0, 0x1fa8) and the under-21 one (size +0x18,
///   table +0x10; lines 0x1fc0, 0x1fc8, 0x1fd0), person ids turned into persons (`load_reserve_pool`);
/// - the call-up count (2 bytes, line 0x1fe8); with a count, a new list (`operator new(0xe)` and `LLIST::LLIST`, 0 when
///   the allocation fails) into `CALL_UP_LIST` and that many 0x12-byte call-ups appended (line 0x1ffa);
/// - the club call-up records (`CLUB_CALL_UP_RECORDS`, 6 bytes per club without the national sides): when all were
///   read and there are clubs, each record with entries (+5) gets its capacity (+4) = the count and a new vector
///   (+0, `malloc(capacity × 9)`, none: line 0x2013) with that many 9-byte entries (line 0x201a); a short read of
///   the records skips them silently;
/// - the next competitions (4 bytes per nation; a short read fills them all with −1) and the Olympic marks (1 byte per
///   nation, unchecked).
/// The item closed; returns 1.
///
/// Defect of the original: a club record with no entries keeps the vector pointer it had when the game was saved (a
/// pointer of the old session); nothing reads it while the count is 0, but the destructor frees every non-zero
/// pointer. [inference: whether the game ever leaves a non-zero pointer with count 0]
/// Original: `NationalTeams::load_dat` (0x7763b0, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nationalteams_load_dat(this: usize, save: usize) -> u32 {
    use national_teams_fields as f;
    let clubs = club_call_up_record_count();
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, NATIONAL_TEAMS_DAT, 0, 0x16, 4);
    let fail = |line: u32| {
        show_error_at(NATIONAL_TEAMS_CPP, line);
        0
    };
    if z == 0 {
        return fail(0x1f7a);
    }
    covered!("sx15_8 load national teams");
    let records = exe::crt_malloc(read_u32(G_NATION_COUNT).wrapping_mul(0x6e));
    write_usize(this + f::NATION_RECORDS, records);
    if records == 0 {
        return fail(0x1f83);
    }
    write_usize(this + f::NEXT_COMPETITIONS, exe::crt_malloc(read_u32(G_NATION_COUNT) << 2));
    write_usize(this + f::OLYMPIC_NATION_MARKS, exe::crt_calloc(1, read_u32(G_NATION_COUNT)));
    if exe::zread(read_usize(this + f::NATION_RECORDS), 0x6e, read_u32(G_NATION_COUNT), z) != read_u32(G_NATION_COUNT) {
        return fail(0x1f91);
    }
    // the two reserve pools: sizes at +0x14 and +0x18, tables at RESERVE_POOLS (+0xc) and +0x10
    if !load_reserve_pool(this, 0x14, f::RESERVE_POOLS, z, [0x1f98, 0x1fa0, 0x1fa8])
        || !load_reserve_pool(this, 0x18, f::RESERVE_POOLS + 4, z, [0x1fc0, 0x1fc8, 0x1fd0])
    {
        return 0;
    }
    let mut count = 0u16;
    if exe::zread(&mut count as *mut u16 as usize, 2, 1, z) != 1 {
        return fail(0x1fe8);
    }
    if count != 0 {
        let mut call_up = [0u8; 0x12];
        exe::date_date_54a110(call_up.as_mut_ptr() as usize + 8); // the call-up's date, read over right after
        let list = exe::crt_operator_new(0xe);
        write_usize(this + f::CALL_UP_LIST, if list != 0 { exe::new_llist(list) } else { 0 });
        for _ in 0..count as i16 {
            if exe::zread(call_up.as_mut_ptr() as usize, 0x12, 1, z) != 1 {
                return fail(0x1ffa);
            }
            exe::llist_add_tail(read_usize(this + f::CALL_UP_LIST), call_up.as_ptr() as usize, 0x12);
        }
    }
    if exe::zread(read_usize(this + f::CLUB_CALL_UP_RECORDS), 6, clubs as u32, z) == clubs as u32 && clubs > 0 {
        for k in 0..clubs {
            let rec = read_usize(this + f::CLUB_CALL_UP_RECORDS) + k as usize * 6;
            let entries = read_i8(rec + 5);
            if entries == 0 {
                continue;
            }
            write_i8(rec + 4, entries);
            let vector = exe::crt_malloc((read_i8(rec + 4) as i32 * 9) as u32);
            write_usize(rec, vector);
            if vector == 0 {
                return fail(0x2013);
            }
            if exe::zread(vector, 9, entries as i32 as u32, z) != entries as i32 as u32 {
                return fail(0x201a);
            }
        }
    }
    let next = read_usize(this + f::NEXT_COMPETITIONS);
    if exe::zread(next, 4, read_u32(G_NATION_COUNT), z) != read_u32(G_NATION_COUNT) {
        for k in 0..read_u32(G_NATION_COUNT) as usize {
            write_i32(next + k * 4, -1);
        }
    }
    exe::zread(read_usize(this + f::OLYMPIC_NATION_MARKS), 1, read_u32(G_NATION_COUNT), z);
    exe::zclose(&mut z);
    1 // sx15_8: loaded (the constructor does not rebuild the tables)
}

/// Comparer for `nationalteams_load_dat`: it reads the save's item (the position moves; it cannot run twice), so only
/// the original runs; the copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_load_dat(this: usize, save: usize) -> u32 {
    // ponytail: reads the container once; the save proof loads the holiday save with and without the batch
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}
