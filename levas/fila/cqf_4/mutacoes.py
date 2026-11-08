# Mutacoes (cqf_4), no sorteio dos grupos da Copa do Mundo de 2002 (uma vez, em 27/12/2001, pelo dia da Copa):
# - o limit de times por grupo sobe um: menos sorteios repetidos na parte aleatoria (que roda antes do sorteio real
#   de 2002), e a contagem dos sorteios da carreira muda dali em diante;
# - as nacoes do sorteio real na ordem inversa: os grupos (e o jogo de abertura de 31/05, a Franca detentora no grupo A)
#   mudam;
# - o sorteio um dia depois (28/12): os sorteios da Copa se intercalam de outro jeito com os do resto do jogo.
# Os contadores ficam colados no efeito: a vaga no grupo depois do laco da repeticao; o sorteio de 2002 depois da
# match_condition do ano; o dia do sorteio dentro da match_condition inteira.
MUTACOES = [
    ('cqf4_limite_do_grupo_mais_um',
     'let lim = ((k / 8) as i8).wrapping_add(1); // cqf4: o limit de times por grupo',
     'let lim = ((k / 8) as i8).wrapping_add(2); // mutacao: um time a mais por grupo',
     'cqf_4 vaga no grupo'),
    ('cqf4_sorteio_2002_invertido',
     'for (i, &chave) in WORLD_CUP_2002_DRAW.iter().enumerate() { // cqf4: as nações do sorteio real',
     'for (i, &chave) in WORLD_CUP_2002_DRAW.iter().rev().enumerate() { // mutacao: a ordem inversa',
     'cqf_4 sorteio de 2002'),
    ('cqf4_sorteio_um_dia_depois',
     'let dia_do_sorteio = cqf4_w(this + 0xb6) as u16; // cqf4: o dia do sorteio',
     'let dia_do_sorteio = (cqf4_w(this + 0xb6) as u16).wrapping_add(1); // mutacao: um dia depois',
     'cqf_4 dia do sorteio'),
]
