# Mutacoes (cx14_10), com a Turquia ate 2002-08-01:
# - a primeira rodada da Primeira Divisao turca uma semana antes (6 de agosto de 2001 em vez de 13: o calendario de
#   2001/02 e montado na criacao do jogo; acusada tambem pelo comparador, lado a lado);
# - as rodadas com TV dos grupos da categoria B de 4 em 4 (acusada pelo comparador, lado a lado);
# - a final do mata-mata da categoria B com o nome das semifinais (acusada pelo comparador, lado a lado, no mata-mata
#   de maio de 2002).
MUTACOES = [
    ('cx14_10_primeira_rodada_antes',
     '(13, 7, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 0: 13/ago',
     '(6, 7, 0, 6, 1, &[(5, 1, 1), (4, 2, 1), (-1, -1, -1)]), // mutation: a week earlier',
     'cx14_10 turkish first division calendar'),
    ('cx14_10_tv_de_4_em_4',
     "if !no_tv && (stage as i32 + k + 1) % 5 == 0 { // cx14_10: the group's TV rounds",
     'if !no_tv && (stage as i32 + k + 1) % 4 == 0 { // mutation: every fourth',
     'cx14_10 category b group calendar'),
    ('cx14_10_final_semifinal',
     '0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 0, 0), (0, 0, 0)), // cx14_10: the final on 30 May',
     '0x82, (0, 3, 0), 5, (2, 1, 0, 0), (0, 0, 0), (0, 0, 0)), // mutation: the semi-final name',
     'cx14_10 category b play-off calendar'),
]
