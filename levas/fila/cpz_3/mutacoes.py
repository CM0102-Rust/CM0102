# Mutacoes (cpz_3). A 0x684cd0 roda a cada atualizacao da tabela de cada liga (1057 chamadas na cobertura); as
# situacoes decididas saem de marco a maio de 2002: o titulo da Premier e das tres divisoes, as vagas no mata-mata e os
# rebaixamentos. As tres mudam o que vai ao anuncio (0x686670 -> registros +0x44) ou aos contratos: o titulo anunciado
# como acesso (sem o registro de campeao: reputacao, notas do tecnico, moral), a vaga no mata-mata anunciada como 2
# (sem os pontos do tecnico pela vaga) e as clausulas de rebaixamento que nao disparam (os contratos dos jogadores dos
# rebaixados). Os contadores ficam colados em cada efeito, depois de todas as condicoes.
MUTACOES = [
    ('cpz3_titulo_como_acesso',
     'cpz3_situacao(this, ii, 5, alto_linha); // cpz3: o título na pior posição = o primeiro + 1',
     'cpz3_situacao(this, ii, 0, alto_linha); // mutacao: o titulo anunciado como acesso',
     'cpz_3 titulo'),
    ('cpz3_mata_mata_como_2',
     'cpz3_situacao(this, ii, 1, alto_linha); // cpz3: a vaga no mata-mata (situação 1)',
     'cpz3_situacao(this, ii, 2, alto_linha); // mutacao: a vaga no mata-mata como situacao 2',
     'cpz_3 vaga no mata-mata'),
    ('cpz3_sem_clausulas_de_rebaixamento',
     'alto = clausulas(cpz3_d(0xae1050), cpz3_d(li), cpz3_d(this + 4)) & !0xff; // cpz3: as cláusulas de rebaixamento',
     'alto = { let _ = clausulas; alto_linha }; // mutacao: sem as clausulas de rebaixamento',
     'cpz_3 clausulas de rebaixamento'),
]
