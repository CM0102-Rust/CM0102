# As repescagens da Primeira Divisao finlandesa (Ykkonen) e os quatro que sobem da Kakkonen (fin_first.cpp; leitor sbH,
# rodada 5, 24/11/2026; a sobra do sbD, ver levas/fila/sbD_10/FORA.md). Simulacao com a Finlandia ('ENG SWE FIN NOR
# POR', ate 2002-08-01, como os irmaos cx09_14 e cx09_20): roda uma vez, no fim dos grupos da Ykkonen (outubro de 2001),
# chamada direto pelo FinFirst::advance_stage (0x594067).
# - 0x594080 FinFirst::create_play_off_stages thiscall() ret 0 (quadro alinhado and esp, -8 e SEH; o mapa diz 196
#   bytes, mas vai ate o ret de 0x59460c): ate 50 clubes (indice 0 ate [0xae235c] - 2 x [0xae2350]) com nacao (+0x53)
#   e divisao (+0x57) de id igual ao de uma das quatro chaves da Kakkonen (0x9cf7fc, 0x9cf804, 0x9cf800, 0x9cf808); com
#   menos de 6, a caixa da linha 0x312. comp_util_sort_and_shuffle_top_clubs 0x4afbd0 (lista, n, 6) cdecl. Os quatro
#   ids de +0xee: ate 2 do norte (a latitude, double +0x23 da cidade +0x38 do estadio +0x69, maior que 61.08 de
#   0x96a4b8: fcomp e test ah, 0x41), depois do sul (menor ou igual, ou NaN) ate 2 no total, depois quaisquer ate 4;
#   cada um usado sai da lista. malloc(0x10) (caixa 0x33f): [linha 0 daqui, linha 1 da fase 0, linha 1 daqui, linha 0
#   da fase 0]; virtual 0x3c (1, &rodadas, &extra, 0) e Cup_Stage 0x522e00 (this, 4, lista, rodadas, competicao,
#   tabela, ano, 1, 1, extra, 0x14, 0, 0, 0, 0) na vaga 1 (caixa 0x352, sem liberar); free da tabela; a lista de novo:
#   [linha 7 daqui, o primeiro do norte que sobrou, linha 7 da fase 0, o primeiro do sul que sobrou] e as vagas vazias
#   com os que sobraram, em ordem; fase 2 na vaga 2 (caixa 0x393); free da tabela, Cup::set_up_round 0x51c800 (0) nas
#   vagas 1 e 2, free da lista e +0x30 = 2. Os locais pela pilha: rodadas em [quadro+0x18], extra em [quadro+0x14] (o
#   push 0 vai antes dos lea). Prologo push ebp; mov ebp, esp; and esp, -8 (6 bytes).
# Nota ao FORA.md do sbD: o par da segunda fase leva o primeiro que sobrou do norte em [1] e o do sul em [3] (o mesmo
# teste de latitude da primeira passada dos promovidos), nao o contrario.
# Usa o league_knockout_stage do sbH_1 (integrar depois dele) e o FIN_FIRST_SOURCE do cx09_14 (e por ele cx09_12 e
# cx09_8).
# Comparador: so a original (o embaralhamento usa o gerador do jogo e a rotina monta fases), conferida depois: as fases
# 1 e 2, a fase atual 2, a fase 1 com os quatro dos grupos (em qualquer ordem), a fase 2 com os dois oitavos e os
# quatro ids de +0xee quatro clubes diferentes da Kakkonen.
# Defeito do original: sem fase, a tabela de rodadas e a lista nao sao liberadas. Iguais no cm0102-1080.exe (inteira).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('fin_first_create_play_off_stages', 0x594080, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('as repescagens da Primeira Divisão finlandesa e os que sobem da Kakkonen',
        'the Finnish First Division play-offs and the clubs promoted from the Kakkonen',
        [('fin_first_create_play_off_stages', 'FinFirst::create_play_off_stages', '0x594080', 'fin_first.cpp', 'thiscall, ret 0',
          'sobem da Kakkonen quatro clubes: dois do norte e dois do sul (pela latitude da cidade, 61,08), entre os de mais reputação embaralhados, completados por quaisquer; a repescagem de acesso cruza os dois primeiros do grupo norte com os dois primeiros do sul (primeiro contra segundo); a de rebaixamento põe o oitavo de cada grupo contra um clube da Kakkonen que sobrou do norte e um do sul',
          'four Kakkonen clubs go up: two from the north and two from the south (by the latitude of the city, 61.08), among the highest by reputation shuffled, completed by any; the promotion play-off crosses the top two of the north group with the top two of the south group (first against second); the relegation play-off puts the eighth of each group against a remaining Kakkonen club from the north and one from the south')])
CAMPOS = []
INTERESSA = []
