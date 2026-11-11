use super::*;

// The comparers of the routines in ../comp/leagues/arg_second.rs.

/// Comparer for `arg_second_record_season_honours`.
pub(crate) unsafe extern "thiscall" fn compare_arg_second_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ARG_SECOND_RECORD_SEASON_HONOURS)(this)
}
