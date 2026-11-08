# Mutacoes (cqk_6), nas tabelas da CFA (4 grupos) e da divisao de baixo francesa (na criacao da carreira de 8 ligas).
# A marca da linha (+0x38) e lida na montagem das partidas da liga (0x683994 e vizinhas): a carreira muda (provado
# pelo agente: a marca invertida mudou a impressao ate 2002-06-01).
# - a marca dos clubes de divisao maior invertida nas duas;
# - na CFA, so os da primeira divisao marcados (os da segunda ficam sem a marca);
# - na divisao de baixo, ninguem marcado (os do National ficam sem a marca).
# Ficaram de fora (provado pelo agente): as linhas trocadas aos pares derrubam o jogo (o confronto direto da 0x687430
# le a linha seguinte ainda vazia) e a band do play-out da Serie C1 nas estatisticas (ita_ser_c1a_set_club_progress/b) escapa: nada
# le a melhor posicao prevista ate 2002-06-01.
MUTACOES = [
    ('cqk6_marca_invertida',
     'let marca = marcadas.iter().any(|&g| cqk6_d(g) == id) as u32; // cqk6: a marca dos clubes de divisão maior',
     'let marca = (!marcadas.iter().any(|&g| cqk6_d(g) == id)) as u32; // mutacao: a marca invertida',
     'cqk_6 tabela francesa de baixo'),
    ('cqk6_cfa_so_a_primeira_marcada',
     'create_french_table(this, clubes, 0x12, 0x9ccaf0, 0x185, &[0x9cf5cc, 0x9cf5d0])',
     'create_french_table(this, clubes, 0x12, 0x9ccaf0, 0x185, &[0x9cf5cc])',
     'cqk_6 tabela francesa de baixo'),
    ('cqk6_baixo_sem_marca',
     'create_french_table(this, clubes, 0x14, 0x9ccbd0, 0x175, &[0x9cf5d4])',
     'create_french_table(this, clubes, 0x14, 0x9ccbd0, 0x175, &[])',
     'cqk_6 tabela francesa de baixo'),
]
