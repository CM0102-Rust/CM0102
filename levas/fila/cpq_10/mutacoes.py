# Mutacoes. O registro da categoria pela vaga vizinha (categorias 1<->2, 3<->4, 5<->6: a PlayerStats::get_stat
# 0x7c1e30 le as estatisticas da outra competicao nas medias do motor de partida e a record_match grava nelas: as
# partidas mudam; quente, sem ramo). O registro novo com o indice do anterior (o jogador novo divide e zera o registro
# do ultimo: as estatisticas e as medias mudam). A aposentadoria marcada sem contar (quem nao e jogador e vai se
# aposentar passa a servir para tecnico na 0x69f810: as contratacoes de tecnico mudam; PODE ESCAPAR se nenhum
# candidato com aposentadoria marcada chegar a esse teste ate 2002-06-01, ver o ramo).
MUTACOES = [
    ('cpq10_categoria_vizinha', 'let i = cpq10_d(vaga) as i32; // o índice da categoria',
     'let i = cpq10_d(vaga ^ 4) as i32; // mutacao: a categoria vizinha'),
    ('cpq10_indice_do_anterior', 'cpq10_por(vaga, cpq10_d(this + 0xc)); // o índice novo',
     'cpq10_por(vaga, cpq10_d(this + 0xc).wrapping_sub(1)); // mutacao: o do anterior',
     'cpq_10 new record'),
    ('cpq10_aposentadoria_ignorada', 'return 1; // está na lista',
     'return 0; // mutacao: nao conta a lista',
     'cpq_10 retirement scheduled'),
]
