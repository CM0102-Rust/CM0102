# Mutacoes (cx12_5), com a Italia e a base NORMAL ate 2002-09-30:
# - as fases do fim da C2/C montadas pelo molde da C1 (create_end_of_league_stages), que antes poe o campeao na
#   Supercopa da Serie C1: o campeao da C2/C entra numa vaga da Supercopa (a mesma mutacao do cx11_8). Acusada se a
#   Supercopa da C1 for jogada ate 2002-09-30; senao escapa, e as fases ficam provadas pelo molde (cx11_7, com as
#   mutacoes dele no mesmo auxiliar);
# - a melhor e a pior colocacao trocadas na previsao: como na C1 (cqk_6) e nas C2/A e C2/B (cx11_9), a carreira quase
#   nao le a previsao; acusada pelo comparador, lado a lado nos 3 bytes da entrada, a cada atualizacao de status.
# O avanco fica sem mutacao propria: o molde (ita_ser_c1a_advance_stage_cql5_advance_stage) e provado na C1 (cql_5).
MUTACOES = [
    ('cx12_5_c2c_na_supercopa_da_c1',
     'create_serie_c2_playoff_stages(League(this), 0x9ea83c, [0x275, 0x289, 0x2ac]) // cx12_5: the C2/C end stages',
     'create_end_of_league_stages(League(this), 0x9ea83c, [0x275, 0x289, 0x2ac], true) // mutation: the C1 template with the Super Cup',
     'cx12_5 serie c2 c end stages'),
    ('cx12_5_c2c_melhor_e_pior_trocadas',
     'set_c1_club_progress(this, club, stage as u8, current as u8, best as u8, worst as u8) // cx12_5: the C2/C progress',
     'set_c1_club_progress(this, club, stage as u8, current as u8, worst as u8, best as u8) // mutation: best and worst swapped',
     'cx12_5 serie c2 c progress'),
]
