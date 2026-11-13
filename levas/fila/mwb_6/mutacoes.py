# Mutacoes (mwb_6): a 9a rodada da First uma semana depois (2/10 -> 9/10, a semana sem jogo: outro dia, outro
# resultado); a virada pula um ano (so o comparador e a temporada seguinte); o melhor e o pior trocados nos codigos de
# progresso (o comparador lado a lado).
MUTACOES = [
    ('mwb6_first_rodada_uma_semana_depois',
     '(2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 8: 2/out, mwb6: a First Division round',
     '(9, 9, 0, 5, 1, &[(-1, -1, -1)]), // 8: mutation: 9/out',
     'mwb_6 scottish first calendar'),
    ('mwb6_first_virada_pula_um_ano',
     '[0x1fe, 0x205, 0x20b, 0x212], false, 1, "mwb_6 scottish first turnover") // mwb6: the next First Division season',
     '[0x1fe, 0x205, 0x20b, 0x212], false, 2, "mwb_6 scottish first turnover") // mutation: two years on',
     'mwb_6 scottish first turnover'),
    ('mwb6_progresso_melhor_e_pior_trocados',
     'return common(this, club, stage, current, best, worst); // mwb6: the common progress codes',
     'return common(this, club, stage, current, worst, best); // mutation: best and worst swapped',
     'mwb_6 first division progress'),
]
