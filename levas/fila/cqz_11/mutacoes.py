# Mutacoes (cqz_11), na Copa de 2002 montada na criacao: a Copa de 2006 em vez da de 2002 (sem a Copa de 2002 na
# carreira); Inglaterra e Escocia trocadas na lista padrao (as vagas ficam assim na memoria ate as classificadas as
# ocuparem); o par da CONCACAF sorteado pelo outro lado (Estados Unidos e Honduras com 1 em vez de 0).
MUTACOES = [
    ('cqz11_copa_de_2006',
     'world_cup_edition((year as u16).wrapping_add(1) as i16); // cqz11: the next World Cup',
     'world_cup_edition((year as u16).wrapping_add(5) as i16); // mutation: the one after',
     'cqz_11 world cup'),
    ('cqz11_lista_trocada',
     'KEY_NATION_UNITED_STATES, KEY_NATION_ENGLAND, KEY_NATION_SCOTLAND, KEY_NATION_DENMARK,',
     'KEY_NATION_UNITED_STATES, KEY_NATION_SCOTLAND, KEY_NATION_ENGLAND, KEY_NATION_DENMARK,',
     'cqz_11 world cup field'),
    ('cqz11_par_da_concacaf',
     'if exe::utils_random(3) == 0 { // cqz11: one in three, the United States and Honduras',
     'if exe::utils_random(3) == 1 { // mutation: the other side of the draw',
     'cqz_11 world cup 2002 qualifiers'),
]
