# Os códigos de progresso das seleções nas eliminatórias da CONCACAF, da Europa e da Oceania (wc_concacaf_cup.cpp,
# wc_europe_league.cpp, wc_oceania_league.cpp; leitor cx16, rodada 5, 24/11/2026). Simulação: a virtual +0x68 roda com
# os resultados das fases das eliminatórias, que na carreira de teste só têm jogos na campanha de 2004 (CONCACAF de
# janeiro de 2004 a outubro de 2005, Europa de setembro de 2004 a novembro de 2005; a Oceania joga a campanha de 2000 no
# começo de 2001, antes da carreira). Daí ATE = '2005-12-31' (o pedido da rodada 5: o que roda depois de 2002-06-01 leva
# ATE); a cobertura até 2002-08-01 não as viu.
# - 0x918040 WcConcacafCup::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: a entrada
#   (Comp_Stats::find_club_entry 0x4a4850, sem conferir nulo); o mata-mata (fase -1, ou 0 com +0xbc < 2): melhor 1 se 1
#   senão 13, atual e pior 13; os grupos (fase != 0 abaixo de 4 com +0xbc < 2, ou abaixo de 3 com +0xbc = 2): 1 e 2 viram
#   1 no melhor e 6 no atual e no pior, 3 vira 7 e 4 vira 10 (tabelas de saltos 0x9181d0/e0/f0); o grupo final (fase 4 com
#   +0xbc < 2, ou 3 com +0xbc = 2): até 3 - (+0xba) vira 1; o resto, como veio. Grava +0x74, +0x73, +0x75.
# - 0x91b510 WcEuropeLeague::set_club_progress: fase até +0xff (com sinal) 1 -> 1, 2 -> 16 (o melhor 10), 3 -> 19, 4 ->
#   28, 5 -> 37, 6 -> 46 (tabelas 0x91b690/a8/c0); fase = +0x101: 1 -> 11, 2 -> 16; o resto, como veio.
# - 0x9200e0 WcOceaniaLeague::set_club_progress: fase >= 1 como veio; abaixo, o melhor 1,2 -> 1, 3 -> 5, 4 -> 7, 5 -> 9,
#   6 -> 11; o pior 1,2 -> 3, 3 -> 5; o atual 1 -> 1, 2 -> 3, 3 -> 8; pior ou atual 4, 5 e 6 gravam 7, 9 e 11 no MELHOR
#   (defeito do original [inferência]: o caso escreve na variável errada; tabelas 0x920218/30/48).
# Prólogos: mov eax, [esp+4]; push ebx; push esi (6 bytes) nas duas primeiras; mov eax, [esp+4]; mov ecx, [ecx+8] (7
# bytes) na terceira. Iguais no cm0102-1080.exe. Comparadores: lado a lado nos 3 bytes da entrada (side_by_side_light).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_concacaf_cup_set_club_progress', 0x918040, [0x8b, 0x44, 0x24, 0x04, 0x53, 0x56]),
           ('wc_europe_league_set_club_progress', 0x91b510, [0x8b, 0x44, 0x24, 0x04, 0x53, 0x56]),
           ('wc_oceania_league_set_club_progress', 0x9200e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os códigos de progresso das seleções nas eliminatórias da CONCACAF, da Europa e da Oceania',
        'the progress codes of the nations in the CONCACAF, European and Oceania qualifying',
        [('wc_concacaf_cup_set_club_progress', 'WcConcacafCup::set_club_progress', '0x918040', 'wc_concacaf_cup.cpp', 'thiscall, ret 0x14',
          'regra: o progresso de uma seleção nas eliminatórias da CONCACAF: quem vence o mata-mata é 1 no melhor, os outros 13; num grupo os dois primeiros são 1 no melhor e 6 no atual, o terceiro 7 e o quarto 10; no grupo final as vagas (três menos as isentas) são 1',
          'rule: a nation progress in the CONCACAF qualifying: a knockout winner is best 1, the rest 13; in a group the first two are 1 at best and 6 now, third 7 and fourth 10; in the final group the qualifying places (three less the exempt) are 1'),
         ('wc_europe_league_set_club_progress', 'WcEuropeLeague::set_club_progress', '0x91b510', 'wc_europe_league.cpp', 'thiscall, ret 0x14',
          'regra: o progresso de uma seleção nas eliminatórias europeias: num grupo o primeiro é 1, o segundo 16 (10 no melhor), depois 19, 28, 37 e 46; na repescagem o vencedor 11 e o perdedor 16',
          'rule: a nation progress in the European qualifying: in a group the winner is 1, the runner-up 16 (10 at best), then 19, 28, 37 and 46; in the play-off the winner 11 and the loser 16'),
         ('wc_oceania_league_set_club_progress', 'WcOceaniaLeague::set_club_progress', '0x9200e0', 'wc_oceania_league.cpp', 'thiscall, ret 0x14',
          'regra: o progresso de uma seleção nos grupos das eliminatórias da Oceania: no melhor 1 para os dois primeiros, depois 5, 7, 9 e 11; no atual 1, 3 e 8 para os três primeiros; as fases seguintes ficam como vieram (o pior e o atual de 4 a 6 vão para o melhor: defeito do original)',
          'rule: a nation progress in the Oceania qualifying groups: best 1 for the top two, then 5, 7, 9 and 11; current 1, 3 and 8 for the first three; later stages keep their codes (a worst or current of 4 to 6 goes into the best: defect of the original)')])
CAMPOS = []
INTERESSA = []
