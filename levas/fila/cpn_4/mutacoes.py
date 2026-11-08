# Mutacoes (com o ramo de cada uma). O jogador que se aposenta vira treinador (5) em vez de aposentado (0x10), sem
# clube (~248 aposentadorias ate 2002-06-01: os desempregados da comissao entram nas buscas de tecnico e de comissao;
# o comparando acusa pelo cargo +0x3d). O melhor clube para o jovem fica o primeiro valido em vez do de maior nota (o
# jovem vai para outro clube: os elencos mudam). A nota do clube da lista da nacao
# sem o sorteio (um sorteio a menos por clube: o sorteio do jogo desanda).
MUTACOES = [
    ('cpn_aposentado_vira_treinador', 'cpn4_por(p + 0x3d, 0x10); // aposentado',
     'cpn4_por(p + 0x3d, 5); // mutacao: vira treinador', 'cpn_4 retired player'),
    ('cpn_jovem_primeiro_clube', 'if s > melhor_nota {',
     'if melhor == 0 { // mutacao: o primeiro clube valido', 'cpn_4 better club for the youngster'),
    ('cpn_nacao_sem_sorteio', 'let nota = sorteio(c as i16 as i32) as i16;',
     'let nota = c as i16; // mutacao: sem o sorteio', 'cpn_4 draw of the nation club'),
]
