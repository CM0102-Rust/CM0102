# Mutacoes (cx13_4), com a Polonia ate 2002-08-01 (a fase de grupos acaba em novembro de 2001):
# - o progresso dos grupos com a faixa do grupo do titulo em 4 (o melhor codigo muda: acusada pelo comparador, lado a
#   lado nos 3 bytes da entrada; as expectativas da diretoria leem o codigo);
# - o texto do grupo do titulo com o clube seguinte (acusada pelo comparador, lado a lado no buffer);
# - o recorde de classificado devolve -1 em vez de 0 (acusada pelo comparador, contra o retorno da original).
MUTACOES = [
    ('cx13_4_faixa_do_grupo_em_4',
     'e.set_best_position(if b < 5 { 1 } else { 9 }); // cx13_4: the championship group band',
     'e.set_best_position(if b < 4 { 1 } else { 9 }); // mutation: the band at 4',
     'cx13_4 polish group progress'),
    ('cx13_4_texto_do_clube_seguinte',
     'championship_group_text(League(this), club_id, news); // cx13_4: the championship group text',
     'championship_group_text(League(this), club_id + 1, news); // mutation: the next club',
     'cx13_4 polish championship group text'),
    ('cx13_4_classificado_devolve_menos_um',
     'return (Some(GroupStageRecord::Qualified), 0);',
     'return (Some(GroupStageRecord::Qualified), 0xffff_ffff);',
     'cx13_4 polish championship group record'),
]
