# Mutacoes (cpq_12). Em casa vira fora quando o jogo e no estadio do clube (o caso comum: a ficha de jogo da
# record_utils 0x7e4895 grava o lado em +0x10; o comparando acusa no primeiro jogo). O maior coeficiente depois (o
# qsort da 0x5d6187 fica ao contrario; a original libera o vetor ordenado, entao a carreira em modo Rust NAO acusa: so
# o comparando, no dia ~20 do mes, pelo --confirmar). Sem pedido de observacao vira pedido (a busca 0x7b5be0 trata
# todo jogador observado como pedido pelo clube; o comparando acusa no primeiro jogo observado).
MUTACOES = [
    ('cpq_lado_em_casa', '    1 // o estádio do clube',
     '    2 // mutacao: em casa vira fora',
     'cpq_12 at home'),
    ('cpq_coeficiente_maior', '        return -1; // o maior antes',
     '        return 1; // mutacao: o maior depois'),
    ('cpq_pedido_sempre', '        return 0; // sem pedido',
     '        return 1; // mutacao: todo jogador pedido',
     'cpq_12 without request'),
]
