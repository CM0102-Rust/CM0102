# Mutacoes (mwb_22): os inscritos sem a ordem pela reputacao (outros potes, outros grupos); faixas de 3 nas posicoes
# previstas (o comparador lado a lado pega). A virada fica sem mutacao (ver rotinas.py).
MUTACOES = [
    ('mwb22_sem_ordem_de_reputacao', 'exe::crt_qsort(read_usize(t + ita_cup_fields::ENTRANT_CLUBS), n, 4, 0x4b07d0usize); // mwb22: the entrants by reputation', '// mutation: the entrants in table order', 'mwb_22 welsh league cup entrants'),
    ('mwb22_faixas_de_3', 'italian_cup_predicted_positions(this, 0xad56f4, (0x3b5, 0x3bd), 5, 6, ', 'italian_cup_predicted_positions(this, 0xad56f4, (0x3b5, 0x3bd), 5, 3, ', 'mwb_22 welsh league cup predicted'),
]
