# As tabelas de rodadas da Eurocopa e das eliminatórias da Copa na Ásia e na Oceania (euro_champ.cpp, wc_asia_league.cpp,
# wc_oceania_league.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x57ab90 (EuroChamp::create_schedule), 0x911f00 (WcAsiaLeague::create_schedule),
#   0x91e2e0 (WcOceaniaLeague::create_schedule).
# Como o cqz_6: a máquina da Copa América, as listas tiradas da original por emulação (unicorn) para as fases
# −128..127, conferidas contra a Copa Africana já portada. Só leem o ano (+0x40). As eliminatórias da Oceania devolvem
# o id 0x3e9 em todas as fases negativas menos a −1 (0x3e8), como a original.
# Convenções conferidas: thiscall, ret 0x10 (virtual +0x3c); os auxiliares cdecl.
# Comparadores: lado a lado (compare_phase_table).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('euro_champ_create_schedule', 0x57ab90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_asia_league_create_schedule', 0x911f00, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_oceania_league_create_schedule', 0x91e2e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas de rodadas da Eurocopa e das eliminatórias da Copa na Ásia e na Oceania',
        'the round tables of the European Championship and of the Asian and Oceania World Cup qualifying',
        [('euro_champ_create_schedule', 'EuroChamp::create_schedule', '0x57ab90', 'euro_champ.cpp', 'thiscall, ret 0x10', 'regra: a Eurocopa tem quatro grupos (fases −1 a 2) de três rodadas em 9, 15 e 19 de junho; o mata-mata (fase 3) em jogo único: quartas em 22/6, semifinais em 26/6 e final em 30/6, sem terceiro lugar', 'rule: the European Championship has four groups (stages −1 to 2) of three rounds on 9, 15 and 19 June; the knockout (stage 3) in single matches: quarter-finals on 22/6, semi-finals on 26/6 and final on 30/6, no third place'),
         ('wc_asia_league_create_schedule', 'WcAsiaLeague::create_schedule', '0x911f00', 'wc_asia_league.cpp', 'thiscall, ret 0x10', 'regra: as eliminatórias asiáticas têm os grupos da primeira fase (até a fase 8) de seis rodadas entre 30 de abril e 25 de maio do ano seguinte, os da segunda (fases 9 e 10) de dez rodadas entre 17 de agosto e 19 de outubro, o confronto dos segundos colocados (fase 11, 26 de outubro) e a repescagem intercontinental (fase 12, 11 de novembro)', 'rule: the Asian qualifying has the first-round groups (up to stage 8) of six rounds between 30 April and 25 May of the following year, the second-round ones (stages 9 and 10) of ten rounds between 17 August and 19 October, the tie of the runners-up (stage 11, 26 October) and the inter-continental play-off (stage 12, 11 November)'),
         ('wc_oceania_league_create_schedule', 'WcOceaniaLeague::create_schedule', '0x91e2e0', 'wc_oceania_league.cpp', 'thiscall, ret 0x10', 'regra: as eliminatórias da Oceania têm dois grupos de cinco rodadas em 7, 9, 11, 14 e 16 de abril do ano seguinte, o confronto dos vencedores dos grupos (fase 1, 20 de junho) e a repescagem intercontinental (fase 2, 2 de dezembro)', 'rule: the Oceania qualifying has two groups of five rounds on 7, 9, 11, 14 and 16 April of the following year, the tie of the group winners (stage 1, 20 June) and the inter-continental play-off (stage 2, 2 December)')])
CAMPOS = [('tabela de rodadas (0x41 bytes por rodada de grupo, 0x68 por rodada de copa) e as saídas rodadas/id da fase/é_liga; valor; grava', 'round table (0x41 bytes per group round, 0x68 per cup round) and the outputs rounds/stage id/is_league; value; writes')]
