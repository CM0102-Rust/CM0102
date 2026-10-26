# Mutacoes do nome da selecao, do nome do atributo e do texto das posicoes. As tres so rodam nas telas (selecao,
# clube, pessoa, busca, tatica, treino) e so mudam texto: a carreira oculta nao passa por elas e ESCAPAM (lacuna); os
# comparadores rodam no jogo real comparando, nas telas (ver o LEIA da leva): a da selecao (sub-23 no lugar de
# sub-21 na forma 2, telas da selecao de base), a do atributo (abreviado e inteiro trocados) e a das posicoes (forma
# 2 sem os parenteses dos lados, jogadores com lado >= 15 na tela da pessoa).
MUTACOES = [
    ('dbu_selecao', '            (2, false) => 0x9b776c, // Under 21 (dbu)',
     '            (2, false) => 0x9b778c, // mutacao: Under 23 no lugar de Under 21'),
    ('dbu_atributo', 'let (s, vazio) = if abreviado != 0 { (c, cv) } else { (l, lv) }; // (dbu)',
     'let (s, vazio) = if abreviado == 0 { (c, cv) } else { (l, lv) }; // mutacao: abreviado e inteiro trocados'),
    ('dbu_posicoes', 'sprintf(texto, 0x9a3d8c, buf, lado); // "%s (%s)" (dbu)',
     'sprintf(texto, 0x98f0b8, buf, lado); // mutacao: sem os parenteses'),
]
