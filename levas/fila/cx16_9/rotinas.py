# A Copa do Mundo que as eliminatórias alimentam, o fim de ciclo da CONCACAF e o tamanho dos grupos europeus
# (wc_oceania_league.cpp, wc_concacaf_cup.cpp, wc_europe_league.cpp; leitor cx16, rodada 5, 24/11/2026). Simulação: as
# virtuais rodam na campanha de 2004 (o sorteio europeu de 2004 monta os grupos pelo tamanho; a CONCACAF e a Oceania
# com os resultados): ATE = '2005-12-31' (o pedido da rodada 5). A cobertura até 2002-08-01 não as viu.
# - 0x91e1c0 WcOceaniaLeague::get_finals_comp thiscall ret 0 (também nas vtables da CONCACAF e da Europa, +0x78): o
#   registro de [0x9cf79c] (a Copa do Mundo) em [0xae23d0] + 0x6b x id. Prólogo mov ecx, [0x9cf79c] (6 bytes).
# - 0x915fe0 WcConcacafCup::skip_season_reset thiscall ret 0 (vtable +0x2c): +0xb1 = 0 e 1. Prólogo mov byte [ecx+0xb1],
#   0 (7 bytes); a rotina inteira tem 16 bytes (com o enchimento), não é corpo vazio.
# - 0x918dd0 WcEuropeLeague::get_group_size thiscall(grupo) ret 4: pelo byte +0xf6 (tabela de saltos 0x918eac): 0 -> 5;
#   1 -> 6 nos grupos 2, 4, 6 e 7; 2 -> 6 nos grupos 2, 4 e 7; 3 -> 5 no grupo 2 e 6 nos outros; outro, caixa 0x25f e 5
#   (ax). As contas fecham com o grupo -1 (o próprio objeto): 10 x 5 = 50, 4 x 6 + 5 x 5 = 49, 3 x 6 + 6 x 5 = 48 e
#   7 x 6 + 5 = 47 seleções. Prólogo movsx eax, byte [ecx+0xf6] (7 bytes).
# Iguais no cm0102-1080.exe. Comparadores: lado a lado no retorno (o reset também no byte +0xb1; o tamanho fora da
# caixa).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_oceania_league_get_finals_comp', 0x91e1c0, [0x8b, 0x0d, 0x9c, 0xf7, 0x9c, 0x00]),
           ('wc_concacaf_cup_skip_season_reset', 0x915fe0, [0xc6, 0x81, 0xb1, 0x00, 0x00, 0x00, 0x00]),
           ('wc_europe_league_get_group_size', 0x918dd0, [0x0f, 0xbe, 0x81, 0xf6, 0x00, 0x00, 0x00])]
DOCS = ('a Copa do Mundo das eliminatórias, o fim de ciclo da CONCACAF e o tamanho dos grupos europeus',
        'the World Cup of the qualifying, the CONCACAF reset and the European group sizes',
        [('wc_oceania_league_get_finals_comp', 'WcOceaniaLeague::get_finals_comp', '0x91e1c0', 'wc_oceania_league.cpp', 'thiscall, ret 0',
          'regra: a competição que as eliminatórias alimentam é a Copa do Mundo da FIFA (o mesmo método nas seções da CONCACAF, da Europa e da Oceania)',
          'rule: the competition the qualifying sections feed is the FIFA World Cup (the same method in the CONCACAF, European and Oceania sections)'),
         ('wc_concacaf_cup_skip_season_reset', 'WcConcacafCup::skip_season_reset', '0x915fe0', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'regra: as eliminatórias da CONCACAF zeram a marca de copa tocada pelas fases e devolvem 1',
          'rule: the CONCACAF qualifying clears its run-by-stages flag and returns 1'),
         ('wc_europe_league_get_group_size', 'WcEuropeLeague::get_group_size', '0x918dd0', 'wc_europe_league.cpp', 'thiscall, ret 4',
          'regra: os grupos das eliminatórias europeias (o -1 é o grupo do próprio objeto) têm cinco seleções; seis nos grupos 2, 4, 6 e 7 com uma isenta, nos 2, 4 e 7 com duas e em todos menos o 2 com três',
          'rule: the European qualifying groups (-1 is the section own group) have five nations; six in groups 2, 4, 6 and 7 with one exempt, in 2, 4 and 7 with two, and in all but group 2 with three')])
CAMPOS = []
INTERESSA = []
