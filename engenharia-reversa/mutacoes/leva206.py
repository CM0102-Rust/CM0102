# Mutacoes das estatisticas da partida (ler, registrar e gravar o registro). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('l_tamanho', "    if read(out, MATCH_STATS_RECORD_SIZE, 1, f) != 1 {", "    if read(out, MATCH_STATS_RECORD_SIZE - 4, 1, f) != 1 {"),
    ('r_excluido', "|| *((r + match_record::NO_STATISTICS) as *const u8) == 0 {", "|| *((r + match_record::NO_STATISTICS) as *const u8) != 0 {"),
    ('g_data', "    *((e + match_stats_entry::DAY) as *mut u16) = *((game + fixture::DAY) as *const u16);", "    *((e + match_stats_entry::DAY) as *mut u16) = *((game + fixture::YEAR) as *const u16);"),
]
