# Mutacoes (sbG_3), com a Suecia e a base NORMAL ate 2002-08-01 (a distribuicao na virada sueca do fim de 2001; as
# secoes novas jogam a Segunda Divisao de 2002, a partir de abril): as tres sao acusadas pelos comparadores lado a lado
# e mudam os grupos da Segunda Divisao de 2002 na carreira:
# - o limiar descendo de 20 em 20 (outra ordem de colocacao entre as secoes cheias);
# - Gotaland do Sul pela latitude com a primeira faixa ate 56 (a nota muda);
# - sem as trocas em par (clubes que podiam voltar ficam na secao nova).
MUTACOES = [
    ('sbG_3_limiar_de_20',
     'threshold -= 0xa; // sbG_3: the threshold steps down by 10',
     'threshold -= 0x14; // mutation: steps of 20',
     'sbG_3 swedish sections dealt'),
    ('sbG_3_sul_ate_56',
     '[(54.0, 0x64), (55.0, 0x50), (56.0, 0x46), (57.0, 0x3c), (59.0, 0x28)]',
     '[(56.0, 0x64), (55.0, 0x50), (56.0, 0x46), (57.0, 0x3c), (59.0, 0x28)]',
     'sbG_3 swedish region score'),
    ('sbG_3_sem_trocas_em_par',
     'swap_swedish_section_pairs(&all[..n], &old);',
     'let _ = swap_swedish_section_pairs; // mutation: no pair swaps',
     'sbG_3 swedish sections dealt'),
]
