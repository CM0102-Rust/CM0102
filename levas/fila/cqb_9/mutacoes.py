# Mutacoes (cqb_9), todas na 0x557120 (160 chamadas ate 2002-06-01 na cobertura de 17/10, pelo jogo da selecao). Com
# suspensao internacional na nacao, o jogo cumprido muda o contador (+1 da suspensao) e, cumprida, solta a suspensao e
# grava o historico: o jogador volta a ser escalavel e o banco muda.
# - O jogo nao conta (conta 0): a suspensao por jogos fica parada.
# - O jogo conta na regra 1 em vez da 0: outro orgao (sem a suspensao, nada; com outra, a errada).
# - O jogo conta duas vezes: a suspensao acaba antes.
# Pode escapar se a carreira ate 2002-06-01 nao tiver suspensao internacional em jogo de selecao (o contador do ramo
# mede: sem ramo, a mutacao e pulada como lacuna).
MUTACOES = [
    ('cqb9_jogo_nao_conta', '            contar(this, cqb9_d(p) as u32, 0, 1); // cqb9: o jogo cumprido na regra 0',
     '            contar(this, cqb9_d(p) as u32, 0, 0); // mutacao: o jogo cumprido na regra 0', 'cqb_9 jogo cumprido'),
    ('cqb9_regra_1', '            contar(this, cqb9_d(p) as u32, 0, 1); // cqb9: o jogo cumprido na regra 0',
     '            contar(this, cqb9_d(p) as u32, 1, 1); // mutacao: o jogo cumprido na regra 0', 'cqb_9 jogo cumprido'),
    ('cqb9_conta_duas_vezes', '            contar(this, cqb9_d(p) as u32, 0, 1); // cqb9: o jogo cumprido na regra 0',
     '            contar(this, cqb9_d(p) as u32, 0, 1); contar(this, cqb9_d(p) as u32, 0, 1); // mutacao: o jogo cumprido na regra 0', 'cqb_9 jogo cumprido'),
]
