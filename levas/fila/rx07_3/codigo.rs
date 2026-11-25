// ---------- rx07_3: the contract manager constructor (contract_manager.cpp) ----------

/// The sixteen static contract dates the constructor sets, in its order: (global, days after the current date).
pub(crate) const CONTRACT_MANAGER_STATIC_DATES: [(usize, u32); 16] = [
    (G_CONTRACT_DATE_PLUS_7_DAYS, 7),
    (G_CONTRACT_DATE_PLUS_30_DAYS, 0x1e),
    (G_CONTRACT_DATE_PLUS_60_DAYS, 0x3c),
    (G_CONTRACT_DATE_PLUS_91_DAYS, 0x5b),
    (G_CONTRACT_DATE_PLUS_121_DAYS, 0x79),
    (G_CONTRACT_DATE_PLUS_152_DAYS, 0x98),
    (G_CONTRACT_DATE_PLUS_182_DAYS, 0xb6),
    (G_CONTRACT_DATE_PLUS_365_DAYS, 0x16d),
    (contract_day_globals::PLUS_550_DAYS, 0x226),
    (contract_day_globals::PLUS_1095_DAYS, 0x447),
    (contract_day_globals::RULE_PLUS_7_DAYS, 7),
    (contract_day_globals::RULE_PLUS_30_DAYS, 0x1e),
    (contract_day_globals::RULE_PLUS_42_DAYS, 0x2a),
    (contract_day_globals::RULE_PLUS_60_DAYS, 0x3c),
    (contract_day_globals::RULE_PLUS_70_DAYS, 0x46),
    (contract_day_globals::RULE_PLUS_91_DAYS, 0x5b), // rx07_3: the last rule date
];

/// `CONTRACT_MANAGER::load_from_disk` thiscall(this, save) ret 4 -> 1 loaded, 0 failed.
pub(crate) const FN_CONTRACT_MANAGER_LOAD_FROM_DISK: usize = 0x4db5c0;

/// The 15 dwords of a contract manager (0x3c bytes) the constructor clears, in its order.
pub(crate) const CONTRACT_MANAGER_CLEARED_FIELDS: [usize; 15] = [0, 8, 4, 0x20, 0x24, 0x28, 0x2c, 0x30, 0xc, 0x14, 0x10, 0x18, 0x1c, 0x34, 0x38];

/// Rule: the contract manager of a new or loaded game. It waits for the game CD (`find_game_cd`, the no-CD patch read
/// at 0x4c7097 and 0x4c70d9), clears its 15 fields (the contract, decision and talks arrays with their counts, the
/// two offer lists) and sets the sixteen static contract dates to the current date plus 7, 30, 60, 91, 121, 152, 182,
/// 365, 550, 1095, 7, 30, 42, 60, 70 and 91 days (`CONTRACT_MANAGER_STATIC_DATES`); given a saved game, it loads
/// contract.dat (`FN_CONTRACT_MANAGER_LOAD_FROM_DISK`, the result ignored). Returns this.
/// Original: `CONTRACT_MANAGER::CONTRACT_MANAGER` (0x4c7040, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_contract_manager(this: usize, save: usize) -> usize {
    find_game_cd(0x4c7097, 0x4c70d9);
    for field in CONTRACT_MANAGER_CLEARED_FIELDS {
        write_u32(this + field, 0);
    }
    covered!("rx07_3 contract dates");
    for (date, days) in CONTRACT_MANAGER_STATIC_DATES {
        let (mut later, mut scratch) = ([0u32; 2], [0u32; 2]);
        let d = exe::date_add_days(G_CURRENT_DATE, later.as_mut_ptr() as usize, days);
        exe::date_operator_3d(date, scratch.as_mut_ptr() as usize, d);
    }
    if save != 0 {
        covered!("rx07_3 contract manager loaded");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_LOAD_FROM_DISK);
        load(this, save);
    }
    this
}

/// Comparer for `new_contract_manager`: only the original (it waits for the CD and may read the save); then the
/// sixteen dates must be the current date plus their days and, without a save, the fields cleared.
pub(crate) unsafe extern "thiscall" fn compare_new_contract_manager(this: usize, save: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save);
    let wrong: Vec<usize> = CONTRACT_MANAGER_STATIC_DATES.iter().filter(|&&(date, days)| {
        let mut later = [0u32; 2];
        let d = exe::date_add_days(G_CURRENT_DATE, later.as_mut_ptr() as usize, days);
        read_u32(d) != read_u32(date)
    }).map(|&(date, _)| date).collect();
    let cleared = save != 0 || CONTRACT_MANAGER_CLEARED_FIELDS.iter().all(|&f| read_u32(this + f) == 0);
    SCOREBOARD.record(r == this && wrong.is_empty() && cleared, || format!("new_contract_manager(save {:#x}): dates off {:x?}, fields cleared {}", save, wrong, cleared));
    r
}
