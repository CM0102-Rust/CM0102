# Mutacoes da leva171 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva171.py --carreira 2002-06-01
MUTACOES = [
    ('m_segunda', "    let (n, q) = if second != 0 { (read_f64(t + player_stats_fields::FLAGGED_RATING_SUM), read_f64(t + player_stats_fields::FLAGGED_RATED_COUNT)) } else { (read_f64(t + player_stats_fields::RATING_SUM), read_f64(t + player_stats_fields::RATED_COUNT)) };", "    let (n, q) = if second == 0 { (read_f64(t + player_stats_fields::FLAGGED_RATING_SUM), read_f64(t + player_stats_fields::FLAGGED_RATED_COUNT)) } else { (read_f64(t + player_stats_fields::RATING_SUM), read_f64(t + player_stats_fields::RATED_COUNT)) };"),
    ('m_padrao', "        return read_f64(rdata::F64_6_8);", "        return 6.0;"),
    ('v_campo', "    sheet.wage() as u32 ", "    read_u32(sheet.addr() + player_sheet::SWEEPER)"),
    ('n_rep', "    if *rep < if base != 0 { 0x32 } else { 0x64 } {", "    if *rep < if base != 0 { 0x32 } else { 0x50 } {"),
    ('n_meio', "    } else if n(player_sheet::DEFENSIVE_MIDFIELDER) || n(player_sheet::MIDFIELDER) || n(player_sheet::ATTACKING_MIDFIELDER) {", "    } else if n(0x12) || n(0x13) {"),
]
