use super::*;

/// end of the Argentine second division and Series A and B: the honours come from the first stage, not the overall
/// table
/// Original: `ArgSecond::record_season_honours` (0x408eb0, arg_second.cpp, thiscall).
///
/// [pt] { 0x408eb0 } (thiscall, arg_second.cpp): `ArgSecond::record_season_honours` (slot 13 da segunda divisão argentina e
/// das Séries A e B): só o histórico do primeiro estágio, em salto final; devolve o eax dele.
pub(crate) unsafe extern "thiscall" fn arg_second_record_season_honours(this: usize) -> u32 {
    crate::comparison::branch("cqj_8 first stage history");
    record_first_stage_honours(League(this))
}
