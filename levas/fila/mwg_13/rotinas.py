# As viradas das eliminatórias da Copa do Mundo da Europa, da África e da CONCACAF (wc_europe_league.cpp,
# wc_african_cup.cpp, wc_concacaf_cup.cpp em worldcup\; leitor mwg, rodada 3 do mundo inteiro, 19/11/2026). Cada uma
# roda uma vez até 2002-08-01 (cobertura_mundo.txt); as seções existem em qualquer carreira: LIGAS = 'ENG'.
# DEPENDE DE mwg_12 (os ajudantes qualifying_turnover_free_table, advance_qualifying_campaign, destroy_comp_stages e
# compare_qualifying_turnover): entra na mesma leva ou depois; se mwg_12 sair, este sai junto.
# Convenções conferidas na desmontagem (0x919430..0x919611, 0x910410..0x910694, 0x915ff0..0x91624e): as três
# thiscall, ret 0; o ano +0x40 + 4 enquanto abaixo de [0xae2c92], como as da Ásia (mwg_12);
# - WcEuropeLeague (League, quadro SEH só para o new da lista): +0x4c = 0, free_table(0), +0xba; a lista +0x10
#   (LLIST::~LLIST 0x68d040 thiscall + operator delete 0x944c94 cdecl) sem zerar; o arquivo; o ano; +0x45 = 0, +0xfb =
#   0; 0x919e30 build_entry_pool (retorno não lido); as fases destruídas e o vetor +0xc liberado; +0x2c pela isenção
#   +0xf6 (0 -> 0xb, 1 e 2 -> 0xa, 3 -> 9, outra fica); +0xc = malloc(4 x +0x2c); +0x30 = -1; +0x8c (0: caixa 0x36a);
#   a lista nova (operator new 0x944cf1(0xe) e LLIST::LLIST 0x68d020, thiscall -> eax), devolvida;
# - WcAfricanCup (Cup): o molde das tabelas de copa, +0xb2 liberado sem testar e sem zerar, as fases destruídas, +0xc
#   liberado, +0x30 = -1, o arquivo, o ano, +0xab = 0, +0x45 = 1, +0xb1 = 0; 0x9106a0 (0: caixa 0x36a); +0x2c pelo byte
#   +0xba (0 ou 3 -> 5, 1 -> 4, outro -> 7); +0xc = malloc; +0x5c; 0x910120 (negativo: 0x37f), +0x8c (0x386), +0x94
#   (0x38d), cujo eax devolve;
# - WcConcacafCup (Cup): o molde, +0xb2 liberado se houver (sem zerar), as fases destruídas (o vetor fica), +0x2c = 7,
#   +0x30 = -1, o arquivo, +0x45 = 1, o ano, +0xab = 0, +0xb1 = 0; 0x916250 (0: 0x41f), +0x5c, 0x915d40 (0: 0x429),
#   +0x8c (0: 0x430), +0x94 (0: 0x437) e 0x916cd0 create_central_american_groups, cujo eax devolve.
# Depois de uma caixa o original devolve o eax dela, a cópia 0.
# Comparadores: só o original roda; o plano de mwg_12 (o ano + múltiplo de 4, não abaixo do do jogo; +0x4c = 0).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('wc_europe_league_start_new_season', 0x919430, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('wc_african_cup_start_new_season', 0x910410, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_concacaf_cup_start_new_season', 0x915ff0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas das eliminatórias da Copa do Mundo da Europa, da África e da CONCACAF',
        'the World Cup qualifying turnovers of Europe, Africa and CONCACAF',
        [('wc_europe_league_start_new_season', 'WcEuropeLeague::start_new_season', '0x919430', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'a virada das eliminatórias europeias: a tabela, as datas, a lista e as fases da campanha passada liberadas, a temporada arquivada, o ano de 4 em 4 até o do jogo; as seleções de novo e o número de fases pelo de isentas (nenhuma: 11; uma ou duas: 10; três: 9), as regras (caixa 0x36a) e uma lista nova',
          "the European qualifying turnover: last campaign's table, dates, list and stages freed, the season archived, the year 4 at a time up to the game's; the national teams again and the stage count by the exempt count (none: 11; one or two: 10; three: 9), the rules (box 0x36a) and a new list"),
         ('wc_african_cup_start_new_season', 'WcAfricanCup::start_new_season', '0x910410', 'wc_african_cup.cpp', 'thiscall, ret 0',
          'a virada das eliminatórias africanas: as tabelas, a lista e as fases liberadas, a temporada arquivada, o ano de 4 em 4 até o do jogo, a temporada montada; os inscritos (caixa 0x36a), as fases pelo formato (5, 4 ou 7), as posições previstas, a primeira rodada (0x37f), a tabela de rodadas (0x386) e a árvore (0x38d)',
          "the African qualifying turnover: the tables, list and stages freed, the season archived, the year 4 at a time up to the game's, the season set up; the entrants (box 0x36a), the stages by the format (5, 4 or 7), the predicted positions, the first round (0x37f), the round table (0x386) and the tree (0x38d)"),
         ('wc_concacaf_cup_start_new_season', 'WcConcacafCup::start_new_season', '0x915ff0', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'a virada das eliminatórias da CONCACAF: as tabelas, a lista e as fases liberadas, 7 fases, a temporada arquivada e montada, o ano de 4 em 4 até o do jogo; os inscritos (0x41f), as posições previstas, os cabeças de chave (0x429), a tabela de rodadas (0x430), a árvore (0x437) e os dois grupos da América Central',
          "the CONCACAF qualifying turnover: the tables, list and stages freed, 7 stages, the season archived and set up, the year 4 at a time up to the game's; the entrants (0x41f), the predicted positions, the seeds (0x429), the round table (0x430), the tree (0x437) and the two Central American groups")])
