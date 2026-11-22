# Telas do clube (5b): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas provam
# o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat (cada uma deve aparecer como divergencia):
# - o nome de cada jogador da lista ligando para 0x7d8 em vez da pessoa (qualquer visao com jogadores);
# - o cabecalho "Inf" com o dado 3 (visoes Contract, Selection, Stats, Attributes, Other).
MUTACOES = [
    ('tel05_2_link_do_nome',
     'part_exchange_colour(0xae3184), 0xde1f64, if s.choosing != 0 { 2 } else { 0x7d7 },',
     'part_exchange_colour(0xae3184), 0xde1f64, if s.choosing != 0 { 2 } else { 0x7d8 },'),
    ('tel05_2_dado_do_inf',
     '        s.heading(area, 1, 0xc, 6, 2);',
     '        s.heading(area, 1, 0xc, 6, 3);'),
]
