# Mutacoes (cqi_7): as regras das tres divisoes nacionais, gravadas na montagem da carreira (e na virada da temporada
# de ano civil): a tabela de cada divisao muda desde a primeira rodada (os pontos da linha +0x19, as posicoes e os
# destinos), e a memoria do jogo (impressao diaria) com ela.
# - A vitoria da Serie A vale 2 pontos em vez de 3.
# - O empate da Serie B vale 0 em vez de 1.
# - O primeiro desempate da Serie C passa do saldo (1) para as vitorias (5): grupos de 7 em dois turnos empatam em
#   pontos com frequencia, e as vitorias e o saldo ordenam diferente.
MUTACOES = [
    ('cqi7_vitoria_serie_a', 'cqi7_b(o, 0xc2, 3); // cqi7: os pontos por vitória da Série A',
     'cqi7_b(o, 0xc2, 2); // mutacao: os pontos por vitória da Série A', 'cqi_7 regras da serie a'),
    ('cqi7_empate_serie_b', 'cqi7_b(o, 0xc3, 1); // cqi7: os pontos por empate da Série B',
     'cqi7_b(o, 0xc3, 0); // mutacao: os pontos por empate da Série B', 'cqi_7 regras da serie b'),
    ('cqi7_desempate_serie_c', 'cqi7_b(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora',
     'cqi7_b(o, 0xc5, 5); // mutacao: desempates', 'cqi_7 regras da serie c'),
]
