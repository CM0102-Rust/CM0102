# Mutacoes (mke_2), no dia dos contratos (0x4c7ac0, cada fatia do dia; a passada das pessoas na fatia 2):
# - os jogadores da IA sao avaliados nos dias pares em vez dos impares: o valor de todos muda de dia, entao os
#   precos pedidos e as avaliacoes das listas de compras (contador "mke_2 renewal at the end" e o valor de todos);
# - quem cumpre a lei Bosman nao fica marcado como saindo (+0x49 continua 0): no dia seguinte passa de novo pela
#   regra e a noticia da saida se repete (contador "mke_2 leaves on a Bosman"; trocada em 13/11/2026: alargar a
#   lealdade para ate 20 escapava, nenhum jogador de lealdade 15-20 cumpria as outras condicoes);
# - a renovacao tardia (faixas 9, 11, 13 e o fim) dos jogadores de status 1-2 na lista do clube oferece 1,0 em vez de
#   1,4: menos renovacoes aceitas, mais jogadores livres.
MUTACOES = [
    ('mke2_valor_dias_pares',
     'if (read_i16(date) as i32) % 2 == 0 { // mke2: the AI\'s players are valued on odd days',
     'if (read_i16(date) as i32) % 2 != 0 { // mutation: valued on even days',
     'mke_2 renewal at the end'),
    ('mke2_bosman_sem_marca',
     '            c.set_leaving_on_bosman(1);\n            c.set_transfer_status(c.transfer_status() & 0xf7);\n            contract_day_shortlist(p);',
     '            c.set_leaving_on_bosman(0); // mutation: not marked as leaving\n            c.set_transfer_status(c.transfer_status() & 0xf7);\n            contract_day_shortlist(p);',
     'mke_2 leaves on a Bosman'),
    ('mke2_renovacao_sem_premio',
     'let f = if contract_day_on_list(club, p) { 1.4 } else { 1.0 }; // mke2: the late renewal of a key player',
     'let f = if contract_day_on_list(club, p) { 1.0 } else { 1.0 }; // mutation: no premium',
     'mke_2 renewal at the end'),
]
