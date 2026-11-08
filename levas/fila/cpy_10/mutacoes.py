# Mutacoes (cpy_10), no premio do ano (0x86c390, os tres primeiros de cada premio do ano: as ligas que terminam antes
# de 2002-06-01 e os premios de dezembro). As vitorias e derrotas maiores (0x7e4a30, 0x7e4b00) so alimentam os
# recordes do clube, que nao entram na carreira (as mutacoes dos recordes da 243a escaparam): nenhuma mutacao nelas.
# 1) sem o sorteio do agradecimento pelo +0x59 (como a cpv13_sem_sorteio_59, acusada na 258a): a contagem de sorteios
#    muda no primeiro vencedor que sortearia (contador "cpy_10 thanks draw");
# 2) o segundo e o terceiro tambem sorteiam (sem conferir o credito 1): sorteios a mais quando algum deles passa das
#    marcas (contador "cpy_10 without credit": os nao creditados que chegam ali; pode escapar se nenhum deles tiver
#    contrato sem a marca +0x45 e +0x59/+0x5b altos);
# 3) a renovacao so com mais de 1200 dias de contrato (em vez de 300): os premiados na lista do clube com contrato
#    entre 300 e 1200 dias ficam sem a oferta (contador "cpy_10 renewal"; escapa se todos os renovados tiverem mais
#    de 1200 dias).
MUTACOES = [
    ('cpy10_sem_sorteio_59', '    if v59 <= 0xf { // cpy10: o +0x59',
     '    if v59 <= 0x7f { // mutacao: sem sorteio pelo +0x59',
     'cpy_10 thanks draw'),
    ('cpy10_todos_sorteiam', '        return; // cpy10: só o creditado',
     '        // mutacao: o segundo e o terceiro seguem',
     'cpy_10 without credit'),
    ('cpy10_renovacao_1200', '> 0x12c { // cpy10: mais de 300 dias',
     '> 0x4b0 { // mutacao: mais de 1200 dias',
     'cpy_10 renewal'),
]
