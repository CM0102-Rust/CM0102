# As escapadas da 341a leva (a rodada das eliminatorias da Copa de 2006 e o que roda de 2003 a 2005, ENG ate
# 2005-12-31). A confirmacao comparando da cadeia vai ate 2001-08-20, antes dessas campanhas; a comparacao que as
# alcanca e o jogo real ate 2005-12-31 (levas/341/jogo_real_longo.txt), sem mutacao. Ficam para uma comparacao longa com
# mutacao ate 2005, ou para a auditoria.

_PT = (' A mutacao roda so nas campanhas de 2003 a 2005: a carreira ate 2005-12-31 terminou igual e a confirmacao '
       'comparando (ate 2001-08-20) nao chega la.')
_EN = (' The mutation only runs in the 2003 to 2005 campaigns: the career to 2005-12-31 ended equal and the comparing '
       'confirmation (to 2001-08-20) does not reach them.')

ESCAPADAS = {
    'cx16_11_repescagem_so_na_propria_fase': (
        'EM CONFIRMACAO. A eliminatoria europeia so monta a repescagem quando chega a fase dela, e nao tambem quando '
        'chega a fase dos segundos colocados: so pesa quando as duas fases sao diferentes.' + _PT,
        'BEING CONFIRMED. The European qualifying only builds the play-off when its own stage comes, and no longer also '
        'when the runners-up stage comes: it only matters when the two stages differ.' + _EN),
    'cx16_7_europa_segundo_19': (
        'EM CONFIRMACAO. O codigo de progresso dos grupos europeus usa a faixa 0x13 em vez de 0x10 para a posicao atual e '
        'a pior: so muda a posicao gravada nas estatisticas da selecao.' + _PT,
        'BEING CONFIRMED. The European groups\' progress code uses band 0x13 instead of 0x10 for the current and worst '
        'positions: it only changes the position stored in the national team\'s statistics.' + _EN),
    'rx09_19_vencedor_de_grupo_8': (
        'EM CONFIRMACAO. O codigo de progresso da primeira fase asiatica grava 8 em vez de 9 para o vencedor de grupo: so '
        'muda a posicao gravada nas estatisticas da selecao.' + _PT,
        'BEING CONFIRMED. The Asian first round progress code writes 8 instead of 9 for a group winner: it only changes '
        'the position stored in the national team\'s statistics.' + _EN),
    'rx11_2_africana_eliminada_segue': (
        'EM CONFIRMACAO. A selecao eliminada num grupo africano continua marcada na campanha: so pesa quando algo le '
        'essa marca depois da eliminacao.' + _PT,
        'BEING CONFIRMED. The nation knocked out of an African group stays marked in the campaign: it only matters when '
        'something reads that mark after the elimination.' + _EN),
}  # nome: (explicacao pt, explicacao en)
