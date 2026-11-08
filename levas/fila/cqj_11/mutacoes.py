# Mutacoes (cqj_11), nas regras dos grupos da Libertadores (0x4c3890, na montagem; os grupos de 2002 jogam de fevereiro
# a abril com os clubes brasileiros, liga jogavel) e do Campeonato de Clubes da Oceania (0x7979b0, com os clubes
# australianos): a vitoria valendo 2 muda a tabela e quem passa; o comeco uma semana depois muda as datas dos jogos
# (a tabela das rodadas parte de +0xb6) e os choques com os campeonatos nacionais. A carreira pega pelos jogos.
MUTACOES = [
    ('cqj11_vitoria_vale_dois_na_libertadores',
     'let vitoria = 3; // cqj11: a vitória nos grupos da Libertadores',
     'let vitoria = 2; // mutacao: a vitoria vale 2',
     'cqj_11 libertadores rules'),
    ('cqj11_libertadores_uma_semana_depois',
     'let dia = 0xf; // cqj11: o dia do começo da Libertadores',
     'let dia = 0x16; // mutacao: comeco uma semana depois',
     'cqj_11 libertadores rules'),
    ('cqj11_oceania_uma_semana_depois',
     'let dia = 0x14; // cqj11: o dia do começo na Oceania',
     'let dia = 0x1b; // mutacao: comeco uma semana depois',
     'cqj_11 oceania rules'),
]
