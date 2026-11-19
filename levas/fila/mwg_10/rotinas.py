# As viradas de temporada da Copa Intertoto e do Mundial de Clubes e as rodadas da Copa da UEFA (intertoto_cup.cpp,
# uefa_cup.cpp, world_club_cup.cpp em eurocomp\; leitor mwg, rodada 3 do mundo inteiro, 19/11/2026). Cada uma roda uma
# vez até 2002-08-01 (cobertura_mundo.txt); as três copas existem na carreira só com a Inglaterra (o Setup cria as
# copas internacionais de clubes sem depender das nações jogáveis, ex. 0x831216): LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x633920..0x633ae6, 0x9030b0..0x9030d9, 0x92bda0..0x92bf48):
# - IntertotoCup::start_new_season: thiscall, ret 0; o molde da virada (mwg_9) sem o livro de inscritos e sem as fases,
#   e +0xb2 (vencedores já mandados à UEFA) = 0 entre +0xab e +0xb1; a semeadura da UEFA (0x9035a0, só com o objeto em
#   0xde1f58); os inscritos 0x6336c0; caixas nas linhas 0x1e9, 0x1f0 e 0x1f7; no fim as posições previstas (virtual
#   +0x5c) e depois Cup::set_up_round(0) (0x51c800, push 0 -> thiscall, ret 4), cujo eax devolve;
# - UefaCup::set_up_rounds: thiscall, ret 0, virtual +0x8c; o fundo +0x28 = 3.000.000 e +0xa3 = virtual +0x3c(-1,
#   &+0x3c, &+0x3a, 0) (thiscall, ret 0x10); devolve 1 (igual a ConChamp::set_up_rounds 0x4c15f0 com outro fundo);
# - WorldClubCup::start_new_season: o molde start_new_cup_season inteiro (os clubes 0x92b8b0; caixas 0x176, 0x17d,
#   0x184).
# Comparadores: só o original roda (as viradas liberam e refazem a copa: o ano + 1; a tabela de rodadas aloca: o fundo
# e o retorno 1).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('intertoto_cup_start_new_season', 0x633920, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('uefa_cup_set_up_rounds', 0x9030b0, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('world_club_cup_start_new_season', 0x92bda0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas de temporada da Copa Intertoto e do Mundial de Clubes e as rodadas da Copa da UEFA',
        'the season turnovers of the Intertoto Cup and the World Club Cup and the UEFA Cup rounds',
        [('intertoto_cup_start_new_season', 'IntertotoCup::start_new_season', '0x633920', 'intertoto_cup.cpp', 'thiscall, ret 0',
          'a virada da Intertoto: as tabelas da temporada passada liberadas, a temporada arquivada, o ano + 1, a rodada atual e o "por fases" zerados e os vencedores ainda não mandados à Copa da UEFA; a semeadura da UEFA na temporada nova; os inscritos, a tabela de rodadas e a árvore (caixas nas linhas 0x1e9, 0x1f0 e 0x1f7 se negativas), as posições previstas e a primeira rodada montada',
          "the Intertoto's turnover: last season's tables freed, the season archived, the year + 1, the current round and the run-by-stages flag cleared and the winners not yet sent to the UEFA Cup; the UEFA seeding in the new season; the entrants, the round table and the tree (boxes at lines 0x1e9, 0x1f0 and 0x1f7 when negative), the predicted positions and the first round set up"),
         ('uefa_cup_set_up_rounds', 'UefaCup::set_up_rounds', '0x9030b0', 'uefa_cup.cpp', 'thiscall, ret 0',
          'as rodadas da Copa da UEFA: o fundo de prêmios de 3.000.000 e a tabela de rodadas da própria copa (sem liberar a anterior); devolve 1',
          "the UEFA Cup's rounds: a prize fund of 3,000,000 and the cup's own round table (the old one not freed); returns 1"),
         ('world_club_cup_start_new_season', 'WorldClubCup::start_new_season', '0x92bda0', 'world_club_cup.cpp', 'thiscall, ret 0',
          'a virada do Mundial de Clubes: o molde das viradas de copa (tabelas liberadas, temporada arquivada, ano + 1, os clubes, a tabela de rodadas e a árvore com as caixas nas linhas 0x176, 0x17d e 0x184, e as posições previstas)',
          "the World Club Cup's turnover: the cup turnover template (tables freed, season archived, year + 1, the clubs, the round table and the tree with the boxes at lines 0x176, 0x17d and 0x184, and the predicted positions)")])
