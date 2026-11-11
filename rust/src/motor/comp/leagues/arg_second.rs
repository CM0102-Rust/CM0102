use super::*;

/// end of the Argentine second division and Series A and B: the honours come from the first stage, not the overall
/// table
/// Original: `ArgSecond::record_season_honours` (0x408eb0, arg_second.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn arg_second_record_season_honours(this: usize) -> u32 {
    covered!("cqj_8 first stage history");
    record_first_stage_honours(League(this))
}
