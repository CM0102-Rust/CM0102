# Mutacoes (mke_2), no dia dos contratos (0x4c7ac0, cada fatia do dia; a passada das pessoas na fatia 2):
# - os jogadores da IA sao avaliados nos dias pares em vez dos impares: o valor de todos muda de dia, entao os
#   precos pedidos e as avaliacoes das listas de compras (contador "mke_2 renewal at the end" e o valor de todos);
# - a lei Bosman aceita lealdade ate 20 (antes abaixo de 15): mais jogadores saem de graca no fim do contrato
#   (contador "mke_2 leaves on a Bosman");
# - a renovacao tardia (faixas 9, 11, 13 e o fim) dos jogadores de status 1-2 na lista do clube oferece 1,0 em vez de
#   1,4: menos renovacoes aceitas, mais jogadores livres.
MUTACOES = [
    ('mke2_valor_dias_pares',
     'if (read_i16(date) as i32) % 2 == 0 { // mke2: the AI\'s players are valued on odd days',
     'if (read_i16(date) as i32) % 2 != 0 { // mutation: valued on even days',
     'mke_2 renewal at the end'),
    ('mke2_bosman_leal',
     '&& p.loyalty() < 15 // mke2: the Bosman needs loyalty below 15',
     '&& p.loyalty() < 21 // mutation: any loyalty',
     'mke_2 leaves on a Bosman'),
    ('mke2_renovacao_sem_premio',
     'let f = if contract_day_on_list(club, p) { 1.4 } else { 1.0 }; // mke2: the late renewal of a key player',
     'let f = if contract_day_on_list(club, p) { 1.0 } else { 1.0 }; // mutation: no premium',
     'mke_2 renewal at the end'),
]
