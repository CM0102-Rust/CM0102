# Mutacoes (mwb_10): a rodada do fim do ano da Third sempre em 28/12 (em 2001 o caso esperado e o 7, 26/12: outro
# dia, outro resultado); a virada da Third pula um ano (so o comparador e a temporada seguinte).
MUTACOES = [
    ('mwb10_third_fim_do_ano_sempre_28', 'cases: SCOTTISH_YEAR_END_CASES, // mwb10: the Third Division year-end cases', 'cases: [&SCOTTISH_DECEMBER_28; 8], // mutation: always 28 December', 'mwb_10 scottish third calendar'),
    ('mwb10_third_virada_pula_um_ano', '[0x1c5, 0x1cc, 0x1d2, 0x1d9], false, 1, "mwb_10 scottish third turnover") // mwb10: the next Third Division season', '[0x1c5, 0x1cc, 0x1d2, 0x1d9], false, 2, "mwb_10 scottish third turnover") // mutation: two years on', 'mwb_10 scottish third turnover'),
]
