# Mutacoes (cx14_3), com a Turquia ate 2002-08-01:
# - as posicoes previstas 5k + 2 (acusada pelo comparador, lado a lado: a criacao do jogo roda a rotina);
# - o mata-mata com o codigo 0xc para quem nao passou (acusada pelo comparador do progresso, no mata-mata da primavera
#   de 2002).
# A lista dos clubes (0 chamadas na cobertura do mundo) fica sem mutacao: so o comparador a prova, se rodar.
MUTACOES = [
    ('cx14_3_previstas_mais_um',
     '|_, k| k.wrapping_mul(5).wrapping_add(1)); // cx14_3: 5k + 1',
     '|_, k| k.wrapping_mul(5).wrapping_add(2)); // mutation: 5k + 2',
     'cx14_3 grouped predicted positions'),
    ('cx14_3_mata_mata_0xc',
     'let play_off = |v: u8| if v == 1 { 1 } else { 0xb }; // cx14_3: the play-off codes',
     'let play_off = |v: u8| if v == 1 { 1 } else { 0xc }; // mutation: 0xc',
     'cx14_3 turkish category b progress'),
]
