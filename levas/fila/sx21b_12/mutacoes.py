# Mutacoes (sx21b_12). A semente roda na partida do jogo (a carreira de teste a roda a cada execucao); cai pelo
# comparador (o estado do sorteio lado a lado). Sem mutacao na leitura das linhas de configuracao e no corte de
# caracteres de controle: elas leem o auto_setup.cfg da carreira de teste e uma mutacao ali pode travar a montagem
# automatica (o comparador da leitura so roda a original; o do corte e lado a lado).
MUTACOES = [
    ('sx21b_12_tabela_menor',
     '((exe::crt_rand() % 0xc738) as u32).wrapping_mul(4)); // sx21b_12: 0xc738 entries',
     '((exe::crt_rand() % 0xc737) as u32).wrapping_mul(4)); // mutation: one entry fewer',
     'sx21b_12 random seeded'),
]
