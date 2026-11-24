# Mutacoes (sx16_1). A rotina roda 108 vezes na carreira ENG ate 2002-06-01 (as aposentadorias de nao jogadores);
# a impressao da carreira hasheia a tabela de pessoas e as fichas de nao jogador.
# - os treinadores regenerados nascem 16 anos antes (faixa 0x30 em vez de 0x20): data de nascimento e idade mudam;
# - a reputacao em casa com capacidade x 20 (e nao x 25): a ficha muda (e a reputacao atual, que soma a de casa);
# - a formacao preferida sem a primeira faixa (os sorteios abaixo de 0x22 passam a 1): a ficha muda.
MUTACOES = [
    ('sx16_1_treinador_mais_velho',
     '    (0xf, 0x20),  // 8 coach',
     '    (0xf, 0x30),  // 8 coach (mutation: 16 years older)',
     'sx16_1 regenerate as non-player'),
    ('sx16_1_reputacao_em_casa',
     'np.set_home_reputation(r.wrapping_add(np.current_ability().wrapping_mul(0x19)).wrapping_add(1)); // sx16_1: home reputation',
     'np.set_home_reputation(r.wrapping_add(np.current_ability().wrapping_mul(0x14)).wrapping_add(1)); // mutation: x 20',
     'sx16_1 regenerate as non-player'),
    ('sx16_1_formacao',
     'const NON_PLAYER_FORMATION_BANDS: [i32; 9] = [0x22, 0x28, 0x3c, 0x5a, 0x5e, 0x60, 0x6c, 0x6f, 0x79];',
     'const NON_PLAYER_FORMATION_BANDS: [i32; 9] = [0, 0x28, 0x3c, 0x5a, 0x5e, 0x60, 0x6c, 0x6f, 0x79]; // mutation',
     'sx16_1 regenerate as non-player'),
]
