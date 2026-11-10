# Mutacoes das estatisticas da partida (ler, registrar e gravar o registro). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('l_tamanho', "    if exe::crt_fread(out, MATCH_STATS_RECORD_SIZE, 1, f) != 1 {", "    if exe::crt_fread(out, MATCH_STATS_RECORD_SIZE - 4, 1, f) != 1 {"),
    ('r_excluido', "|| r.no_statistics() == 0 {", "|| r.no_statistics() != 0 {"),
    ('g_data', "    write_u16(e + match_stats_entry::DAY, game.day());", "    write_u16(e + match_stats_entry::DAY, game.year());"),
]
