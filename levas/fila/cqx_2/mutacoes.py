# Mutacoes (cqx_2), no Mundial de Clubes (o ano da competicao e 2003 ou depois: nada dele muda a carreira ate junho de
# 2002; as tres sao para os comparadores lado a lado, pela confirmacao no modo comparando ate 2001-08-20): a primeira
# rodada dos grupos uma semana depois (a tabela das rodadas difere); o sorteio uma semana depois (o +0xb6 difere); a
# volta antecipada devolvendo 1 em vez de 0 (o byte devolvido difere todo dia).
MUTACOES = [
    ('cqx2_primeira_rodada_uma_semana_depois',
     'GoldcupScheduleOp::D(0, 0x1d, 6, 0, 6, 1, 1)',
     'GoldcupScheduleOp::D(0, 5, 7, 0, 6, 1, 1) /* mutation: one week later */',
     'cqx_2 world club championship groups'),
    ('cqx2_sorteio_uma_semana_depois',
     'let day = 5; // cqx2: the draw day',
     'let day = 12; // mutation: one week later',
     'cqx_2 world club championship rules'),
    ('cqx2_volta_antecipada_com_1',
     'return 0; // cqx2: nothing before the draw date',
     'return 1; // mutation: 1 before the draw date',
     'cqx_2 before the draw'),
]
