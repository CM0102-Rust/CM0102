# O dia das eliminatórias sul-americanas e asiáticas e a lista de inscritas da Ásia (wc_south_american_league.cpp,
# wc_asia_league.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x921160 (WcSouthAmericanLeague::update_for_date), 0x913140 (WcAsiaLeague::update_for_date),
#   0x913f60 (WcAsiaLeague::build_entrant_list).
# Os dois dias: o teste da data do sorteio do molde da Copa da Oceania (oceania_nations_draw_date, já em Rust): antes
# dela devolvem 0 sem o dia da liga; no dia, inativa e com `full`: a América do Sul cria os jogos, ordena, ativa, faz
# as previsões, solta as seleções sul-americanas das competições do continente (0x775070) e põe cada seleção da tabela
# nesta competição (0x775220); a Ásia sorteia os grupos da primeira fase (0x912bd0), monta a tabela do primeiro
# (0x912ad0), os jogos, ordena, os grupos 0..8 (0x913540), as previsões, libera +0xf7 e ativa. A lista asiática: o
# molde da Oceania (0x91fa80, já em Rust) com 44 vagas, o continente asiático e o teto de 40 no fim.
# Convenções conferidas: thiscall, ret 8 e ret 0; 0x775070 e 0x775220 (2 pushes; mov ecx -> thiscall, ret 8), 0x913540
# (push k -> ret 4), 0x5fa730 (ret 0x14), os cdecl de sempre.
# Comparadores: só a original nas três (jogos, sorteios, a lista alocada), com o plano.
# Atenção da prova: as duas campanhas de 2004 começam depois de 2002-06-01; as mutações dos dois dias antecipam o
# começo (ver mutacoes.py).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('wc_south_american_league_update_for_date', 0x921160, [0x6a, 0xff, 0x68, 0x16, 0x5c, 0x96, 0x00]),
           ('wc_asia_league_update_for_date', 0x913140, [0x6a, 0xff, 0x68, 0x76, 0x57, 0x96, 0x00]),
           ('wc_asia_league_build_entrant_list', 0x913f60, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('o dia das eliminatórias sul-americanas e asiáticas e a lista de inscritas da Ásia',
        'the day of the South American and Asian World Cup qualifying and the Asian entrant list',
        [('wc_south_american_league_update_for_date', 'WcSouthAmericanLeague::update_for_date', '0x921160', 'wc_south_american_league.cpp', 'thiscall, ret 8', 'regra: as eliminatórias sul-americanas começam na data de início: com a atualização completa e a liga ainda parada, os jogos são criados, a tabela ordenada, as previsões feitas e cada seleção sul-americana sai das competições do continente para jogar esta; antes do início nada roda, nem o dia da liga', 'rule: the South American qualifying starts on its start date: with the full update and the league not yet started, the fixtures are made, the table sorted, the predictions set and each South American national team leaves its continental competitions to play this one; before the start nothing runs, not even the league day'),
         ('wc_asia_league_update_for_date', 'WcAsiaLeague::update_for_date', '0x913140', 'wc_asia_league.cpp', 'thiscall, ret 8', 'regra: na data do sorteio das eliminatórias asiáticas, com a atualização completa e a liga parada, os grupos da primeira fase são sorteados, o primeiro grupo ganha tabela e jogos e os outros nove as suas fases; antes nada roda', 'rule: on the draw date of the Asian qualifying, with the full update and the league not yet started, the first round groups are drawn, the first group gets its table and fixtures and the other nine their stages; before it nothing runs'),
         ('wc_asia_league_build_entrant_list', 'WcAsiaLeague::build_entrant_list', '0x913f60', 'wc_asia_league.cpp', 'thiscall, ret 0', 'regra: as eliminatórias asiáticas têm 44 vagas menos as isentas (as sedes asiáticas da Copa seguinte e o último campeão, se asiático), preenchidas com as outras seleções asiáticas na ordem do banco e ordenadas pela reputação; seguem no máximo 40', 'rule: the Asian qualifying has 44 places minus the exempt (the Asian hosts of the next World Cup and the last winner, when Asian), filled with the other Asian national teams in database order and sorted by reputation; at most 40 go on')])
CAMPOS = [('eliminatórias asiáticas +0xee isentas (byte), +0xef lista de inscritas, +0xf3 contagem (dword), +0xf7 grupos sorteados (liberados no sorteio); +0x45, +0xa7; valor/ponteiro; lê e grava', 'Asian qualifying +0xee exempt (byte), +0xef entrant list, +0xf3 count (dword), +0xf7 drawn groups (freed at the draw); +0x45, +0xa7; value/pointer; reads and writes')]
