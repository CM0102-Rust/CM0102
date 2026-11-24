// ---------- sx04c_15: the Scottish month awards (award/scotland_awards.cpp) ----------

/// The Scottish month: the manager of the month of each division (`award_month_manager`, awards 4, 10, 16 and 22),
/// then, division by division, the player of the month (no age limit) and the young player of the month (up to 21),
/// each the best of the month ratings in its division (`award_month_best_player`; awards 5 and 6, 11 and 12, 17 and
/// 18, 23 and 24).
/// Original: `ScotlandAwards::on_month_change` (0x7f6010, scotland_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn scotland_awards_on_month_change(this: usize) {
    covered!("sx04c_15 scottish month awards");
    for (d, key) in SCOTLAND_DIVISIONS.iter().enumerate() {
        award_month_manager(nation_award(this, scotland_block(d) + 1), *key);
    }
    for (d, key) in SCOTLAND_DIVISIONS.iter().enumerate() {
        award_month_best_player(this, nation_award(this, scotland_block(d) + 2), *key, -1);
        award_month_best_player(this, nation_award(this, scotland_block(d) + 3), *key, 0x15);
    }
}

/// Comparer for `scotland_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_scotland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}
