# Mutacoes da leva89 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva89.py --jogo 2001-08-20
MUTACOES = [
    ('g_24', "this.round_count() < 24", "this.round_count() < 20"),
    ('g_situacao', "5 | 2 | 3 | -2 | -4 | -5", "5 | 2 | -2 | -4 | -5"),
    ('g_alcance', "if reach >= t.points()", "if reach > t.points()"),
    ('n_titular', "if k <= 10 || (j.minute_on() as u8) != 0xff {", "if k <= 9 || (j.minute_on() as u8) != 0xff {"),
    ('n_7', "if average < read_f64(rdata::F64_7) &&", "if average < read_f64(rdata::F64_7) + 0.5 &&"),
    ('n_08', "strength(0x20) * read_f64(rdata::F64_0_8)", "strength(0x20)"),
    ('n_2120', "casa == fora && classificado != 0", "casa == fora && classificado == 0"),
    ('n_2001', "say(call, event::FINAL_WHISTLE, 0);", "say(call, event::FINAL_WHISTLE, 1);"),
]
