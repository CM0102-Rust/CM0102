# Mutacoes (cqb_12), nas duas consultas que o calendario usa a toda hora: a 0x48dee0 (find_fixtures 0x5acbe0, 288865
# chamadas ate 2002-06-01, e find_fixture_clashes 0x5ad780) e a 0x48df40 (is_stadium_in_use 0x5ae870, 2203, e a
# 0x5ad780). O indice ativo e um dos tipos com que os jogos de um dia se comparam: com 3 (nenhuma nacao-chave) no lugar
# do indice do clube ou do estadio, ou com 0 (a primeira nacao-chave) no lugar do 3 de um clube de fora, os conflitos
# achados mudam e com eles as datas livres e os jogos remarcados (0x5ae380, 0x5ab510), lidos pela tabela. Podem escapar
# se nenhum conflito nem busca de data mudar de resposta ate 2002-06-01 (o tipo so conta quando outro jogo do mesmo
# dia o tem; os contadores mostram que os trechos rodam, nao que a resposta mudou).
MUTACOES = [
    ('cqb12_clube_sem_indice', '            return *((key_nation_entry(k) + 0x14) as *const u8); // cqb12: o índice ativo da nação do clube',
     '            return 3 + 0 * *((key_nation_entry(k) + 0x14) as *const u8); // mutacao: como fora das nacoes-chave',
     'cqb_12 nacao chave do clube'),
    ('cqb12_estadio_sem_indice', '            *((key_nation_entry(k) + 0x14) as *const u8) // cqb12: o índice ativo da nação do estádio',
     '            3 + 0 * *((key_nation_entry(k) + 0x14) as *const u8) // mutacao: como fora das nacoes-chave',
     'cqb_12 nacao chave do estadio'),
    ('cqb12_clube_de_fora_0', '    3 // cqb12: clube fora das nações-chave',
     '    0 // mutacao: clube de fora na primeira nacao-chave',
     'cqb_12 clube sem nacao chave'),
]
