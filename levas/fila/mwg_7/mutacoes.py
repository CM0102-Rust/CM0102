# Mutacoes (mwg_7), no hall da fama (nada da carreira le o hall da fama: sao pegas pelos comparadores lado a lado):
# - o ano civil vai para a tabela 1 (a da Argentina);
# - a liga da nacao vai para a tabela seguinte (a Inglaterra para a 8);
# - a entrada guarda um ponto a mais.
MUTACOES = [
    ('mwg7_ano_civil_na_tabela_1',
     'hall_of_fame_try_add(this, 0, staff, score) // mwg7: the calendar year table',
     'hall_of_fame_try_add(this, 1, staff, score) // mutation: table 1',
     'mwg_7 calendar year entry'),
    ('mwg7_tabela_seguinte',
     '.map_or(0xffff_ffff, |i| i as u32 + 1); // mwg7: the league\'s table',
     '.map_or(0xffff_ffff, |i| i as u32 + 2); // mutation: the next table',
     'mwg_7 league entry'),
    ('mwg7_ponto_a_mais',
     "write_u32(last + hf::SCORE, score); // mwg7: the entry's score",
     "write_u32(last + hf::SCORE, score + 1); // mutation: one point more",
     'mwg_7 entry beats the last one'),
]
