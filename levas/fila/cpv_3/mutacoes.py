# Mutacoes (com o ramo de cada uma). Todas nas ofertas em espera no match_wrap_up da janela (0x8c4e10, 37 chamadas ate
# 2002-06-01 na cobertura de 17/10: uma por janela que fecha); as noticias so vao as caixas dos humanos. Sem a
# retirada, a oferta do computador pela nacao que fechou a janela continua viva (sai depois ou fecha a transferencia).
# Com o status 1 (e nao 3), a oferta reagendada volta pelo outro caminho da fila. De volta em 3 dias (e nao no dia
# seguinte), a oferta e tratada em outro dia.
MUTACOES = [
    ('cpv3_sem_retirar', 'retirar(this, cpv3_d(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o comprador retira',
     'let _ = retirar; // mutacao: nao retira',
     'cpv_3 retirada na janela'),
    ('cpv3_status_1', 'cpv3_pb(o + 0x2c, 3); // cpv3: status 3',
     'cpv3_pb(o + 0x2c, 1); // mutacao: status 1',
     'cpv_3 reagendada na janela'),
    ('cpv3_volta_em_3', 'agendar(this, o, 1, 0); // cpv3: de volta amanhã',
     'agendar(this, o, 3, 0); // mutacao: 3 dias',
     'cpv_3 reagendada na janela'),
]
