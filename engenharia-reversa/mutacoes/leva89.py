# Mutacoes da leva89 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva89.py --jogo 2001-08-20
MUTACOES = [
    ('g_24', "*((this + league::ROUND_COUNT) as *const i16) < 24", "*((this + league::ROUND_COUNT) as *const i16) < 20"),
    ('g_situacao', "5 | 2 | 3 | -2 | -4 | -5", "5 | 2 | -2 | -4 | -5"),
    ('g_alcance', "if reach >= *((t + league_table_row::POINTS) as *const i16)", "if reach > *((t + league_table_row::POINTS) as *const i16)"),
    ('n_titular', "if k <= 10 || *((j + match_player::MINUTE_ON) as *const u8) != 0xff {", "if k <= 9 || *((j + match_player::MINUTE_ON) as *const u8) != 0xff {"),
    ('n_7', "if average < read_f64_at(rdata::F64_7) &&", "if average < read_f64_at(rdata::F64_7) + 0.5 &&"),
    ('n_08', "strength(0x20) * read_f64_at(rdata::F64_0_8)", "strength(0x20)"),
    ('n_2120', "casa == fora && classificado != 0", "casa == fora && classificado == 0"),
    ('n_2001', "fala(call, 0x2001, 0);", "fala(call, 0x2001, 1);"),
]
