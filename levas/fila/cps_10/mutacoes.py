# Mutacoes. Os gols pelo clube contam todos os clubes da carreira e, na outra, um a mais: a MATCH_PLAYER::setup grava o
# valor em +0x3d do jogador da partida, dentro do motor (a impressao das partidas pega; quentes, sem contador). Na ordem
# dos candidatos a tecnico do ano, menos pontos primeiro: os tres candidatos mudam (premio da nacao pela 0x768940; so
# pega se a carreira chegar ao premio e ele mexer no que a carreira le). O registro da selecao do ano so grava o
# historico em arquivo (fora da impressao): sem mutacao.
MUTACOES = [
    ('cps10_todos_os_clubes', 'if v >= 0 && v == cps10_i(clube) { // cps10: só os grupos do clube',
     'if v >= 0 { // mutacao: todos os clubes'),
    ('cps10_um_gol_a_mais', 's // cps10: os gols pelo clube',
     's.wrapping_add(1) // mutacao: um gol a mais'),
    ('cps10_menos_pontos_primeiro', 'cps10_i(rb + 0xba).wrapping_sub(cps10_i(ra + 0xba)) // cps10: mais pontos primeiro',
     'cps10_i(ra + 0xba).wrapping_sub(cps10_i(rb + 0xba)) // mutacao: menos pontos primeiro',
     'cps_10 pontos dos dois'),
]
