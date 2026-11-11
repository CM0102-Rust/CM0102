use super::*;

// The comparers of the routines in ../file_screens.rs.

/// Comparer for `build_save_game_summary`.
pub(crate) unsafe extern "C" fn compare_build_save_game_summary(summary: *mut u8) {
    let regions = [(summary as usize, 0xf6c), (summary as usize + save_summary::PLAY_TIME_TOTAL, 8), (G_GAME_PLAY_TIME_TOTAL, 8)];
    let (_, _, _, mo, mr) = crate::comparison::side_by_side(&regions,
        || original_fn::<unsafe extern "C" fn(*mut u8)>(IDX_BUILD_SAVE_GAME_SUMMARY)(summary),
        || build_save_game_summary(summary));
    SCOREBOARD.record(mo.0[0] == mr.0[0], || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("build_save_game_summary: first difference em +{:#x}: {:#04x} x {:#04x}", k, a[k], b[k])
    });
}
