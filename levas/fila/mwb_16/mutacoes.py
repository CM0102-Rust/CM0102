# Mutacoes (mwb_16): a Copa da Liga escocesa com 2 substituicoes (outros jogos); as quartas uma semana antes (1/12 ->
# 24/11, depois do sorteio de 23/9: outro dia, outro resultado); sem as posicoes previstas na montagem da temporada (a
# expectativa da diretoria na copa; se escapar da carreira, so o comparador da 0x5223a0 nao ve: ver PRONTO).
MUTACOES = [
    ('mwb16_copa_da_liga_duas_substituicoes',
     'write_u8(this + cup::SUBSTITUTIONS_ALLOWED, 3); // mwb16: the League Cup substitutions',
     'write_u8(this + cup::SUBSTITUTIONS_ALLOWED, 2); // mutation: 2 substitutions',
     'mwb_16 new scottish league cup'),
    ('mwb16_quartas_uma_semana_antes',
     'cql10r((0x17, 8, 0, 3), (1, 0xb, 0, 2, 2, 0), 0x78, (0, 3, 0), 6, (8, 4, 0, 0), (0, 1, 4), (0, 0xffff_ffff, 0)), // mwb16: the League Cup quarter-finals',
     'cql10r((0x17, 8, 0, 3), (0x18, 0xa, 0, 2, 2, 0), 0x78, (0, 3, 0), 6, (8, 4, 0, 0), (0, 1, 4), (0, 0xffff_ffff, 0)), // mutation: on 24/11',
     'mwb_16 scottish league cup rounds'),
    ('mwb16_sem_posicoes_previstas',
     'call_cup_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS); // mwb16: the League Cup predicted positions',
     '// mutation: no predicted positions',
     'mwb_16 scottish league cup season'),
]
