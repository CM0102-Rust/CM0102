// ---------- batch %%LEVA%%: the human managers of a restored game: who takes the screens and who goes to the computer (Setup.cpp) ----------

/// The size of a human manager record (`human_manager_record`, 42 735 bytes) and the match day object
/// (`set_computer_controlled` tells it when it is set).
const HUMAN_MANAGER_RECORD_SIZE: usize = 0xa6ef;
const G_MATCH_DAY_OBJECT: usize = 0xb67a50;

/// Rule: when a saved game is restored (`-load` and the network game), each human manager with no password (in a game
/// with no network name) takes the screen manager: the first one the current record, the others a record each; a
/// manager with a password who is not on holiday goes to computer control until he resumes it. Returns whether
/// Manager Status must open (a manager locked, or one with a password on holiday).
///
/// For k from 0 to 15, the person `G_STAFF_COUNT` − 1 − k of the person table (`G_STAFF_TABLE`, `staff::SIZE` bytes
/// each; the count read anew each time) with `staff::CLASSIFICATION` 1 (a human manager): with an empty password
/// (`HUMAN_MANAGER::get_password` 0x5fde40) and `G_NETWORK_GAME_NAMED_FLAG` 0, a current record (current channel) with no
/// key gets him as its key and the join and leave hooks (`NETWORK_JOIN_CALLBACK`, `NETWORK_LEAVE_CALLBACK`) are set;
/// with a key he gets a record of his own (`ScreenManager::add_manager_record` with the two hooks and him). Otherwise,
/// not on holiday (`HUMAN_MANAGER::is_on_holiday` 0x5fd210): `HUMAN_MANAGER::set_computer_controlled(1, him)`
/// (0x5fe140); on holiday: the result 1. Returns 1 when the HUMAN_MANAGER's locked count (`HUMAN_MANAGER_LOCKED_COUNT`)
/// is nonzero, otherwise the result.
/// With `CM_FERIAS` (the save proof) `simulation.rs` hooks this entry first: the engine's detour is refused (the bytes
/// are its jump) and the original runs inside that wrapper; without it the copy runs.
/// Original: `setup_assign_restored_network_managers` (0x8387e0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_assign_restored_network_managers() -> u32 {
    covered!("tel40_5 restored managers");
    let sm = G_SCREEN_MANAGER;
    let mut result = 0u32;
    for k in 0..0x10isize {
        let index = read_i32(G_STAFF_COUNT) as isize - k - 1;
        let person = Staff((read_usize(G_STAFF_TABLE) as isize + index * staff::SIZE as isize) as usize);
        if person.classification() != 1 {
            continue;
        }
        let password = exe::human_manager_get_password(read_usize(G_HUMAN_MANAGER), person.addr());
        if read_u8(password) == 0 && read_u32(G_NETWORK_GAME_NAMED_FLAG) == 0 {
            let key = screen_record(sm) + screen_history::KEY;
            if read_u32(key) == 0 {
                covered!("tel40_5 the first manager takes the current record");
                write_usize(key, person.addr());
                write_usize(sm + SCRMAN_JOIN_HOOK, NETWORK_JOIN_CALLBACK);
                write_usize(sm + SCRMAN_LEAVE_HOOK, NETWORK_LEAVE_CALLBACK);
            } else {
                let add: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x7ff8f0usize);
                add(sm, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK, person.addr()); // tel40_5: a record of his own
            }
        } else if exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), person.addr()) == 0 {
            covered!("tel40_5 a manager with a password goes to the computer");
            let computer: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SET_COMPUTER_CONTROLLED);
            computer(read_usize(G_HUMAN_MANAGER), 1, person.addr());
        } else {
            result = 1;
        }
    }
    if read_u32(read_usize(G_HUMAN_MANAGER) + HUMAN_MANAGER_LOCKED_COUNT) != 0 {
        return 1;
    }
    result
}

/// Comparer for `setup_assign_restored_network_managers`: side by side over the screen manager's records
/// (`scrman_records_regions`), the HUMAN_MANAGER's header (records, count, locked and watched counts) and each human
/// record's computer control flag; during a match (the match day object set, which `set_computer_controlled` tells)
/// only the original.
pub(crate) unsafe extern "C" fn compare_setup_assign_restored_network_managers() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(%%I0%%);
    let hm = read_usize(G_HUMAN_MANAGER);
    if hm == 0 || read_usize(G_MATCH_DAY_OBJECT) != 0 {
        // simplification: the match day object is not told twice
        return o();
    }
    let mut reg = scrman_records_regions(G_SCREEN_MANAGER).to_vec();
    reg.push((hm, 0x10));
    let base = read_usize(hm);
    for i in 0..read_i32(hm + 4).clamp(0, 0x10) as usize {
        reg.push((base + i * HUMAN_MANAGER_RECORD_SIZE + human_manager_record::COMPUTER_CONTROLLED, 4));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || setup_assign_restored_network_managers());
    SCOREBOARD.record(same && r == n, || format!("setup_assign_restored_network_managers: returned {} / {}, differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}
