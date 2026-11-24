# Mutacoes (cx01_4), carreira ENG ate 2002-06-01. Nenhuma abre caixa.
# - a situacao 5 (decidida) deixa de tirar o clube da disputa: o comparador acusa (escolha do time, diretoria) e a
#   carreira muda quando o clube decide a posicao antes do fim;
# - a primeira data de um calendario de liga com o ano seguinte: o comparador acusa (pessoal e diretoria);
# - o primeiro jogo extra com o codigo + 2 e nao + 3: o comparador acusa (o plano contra o que a original gravou).
MUTACOES = [
    ('cx01_4_situacao_5_disputa',
     '                            0xfe | 7 | 6 | 5 => 0,',
     '                            0xfe | 7 | 6 => 0, // mutation'),
    ('cx01_4_primeira_data_ano_seguinte',
     '            (read_u16(s), read_u16(s + 2).wrapping_add(comp.year() as u16))',
     '            (read_u16(s), read_u16(s + 2).wrapping_add(comp.year() as u16 + 1)) // mutation'),
    ('cx01_4_extra_mais_2',
     '    for (k, step) in [3i16, 4].into_iter().enumerate() {',
     '    for (k, step) in [2i16, 4].into_iter().enumerate() { // mutation'),
]
