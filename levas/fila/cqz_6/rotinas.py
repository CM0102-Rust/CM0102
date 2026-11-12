# As tabelas de rodadas da Copa da Ásia, da Copa das Nações da Oceania e do futebol olímpico (asia_nations.cpp,
# oceania_nations.cpp, olympics.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x40eb00 (AsiaNations::create_schedule), 0x798b60 (OceaniaNations::create_schedule), 0x79f7d0 (Olympics::create_schedule).
# A máquina da Copa América (GoldcupScheduleOp, build_phase_table, compare_phase_table, já em Rust). As listas de
# operações foram tiradas da original por emulação (unicorn sobre o cm0102.exe: cada chamada a 0x68a160, 0x68a1c0,
# 0x521e60, 0x521eb0, 0x521f10 e cada escrita na tabela, na ordem), para todas as fases −128..127 (o byte com sinal),
# com as saídas, o tamanho da tabela e a linha da caixa de memória; conferido antes contra a Copa Africana já portada
# (0x4012e0: as mesmas listas da AFRICAN_NATIONS_*). Só leem o ano (+0x40); a Copa da Oceania tem duas metades pelo
# ano (resto 2 por 4: 2002, 2006... em novembro; os outros anos em junho), com linhas de caixa diferentes.
# Convenções conferidas: thiscall, ret 0x10 (virtual +0x3c); os auxiliares cdecl.
# Comparadores: lado a lado (compare_phase_table: as saídas e os bytes das duas tabelas; a da cópia é liberada).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('asia_nations_create_schedule', 0x40eb00, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('oceania_nations_create_schedule', 0x798b60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('olympics_create_schedule', 0x79f7d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas de rodadas da Copa da Ásia, da Copa das Nações da Oceania e do futebol olímpico',
        'the round tables of the Asian Cup, the Oceania Nations Cup and the Olympic football',
        [('asia_nations_create_schedule', 'AsiaNations::create_schedule', '0x40eb00', 'asia_nations.cpp', 'thiscall, ret 0x10', 'regra: a Copa da Ásia tem três grupos (fases −1 a 1) de três rodadas em 13, 16 e 19 de outubro, com os jogos fixos de cada grupo; o mata-mata (fase 2) em jogo único: quartas em 23/10, semifinais em 26/10, terceiro lugar e final em 29/10', 'rule: the Asian Cup has three groups (stages −1 to 1) of three rounds on 13, 16 and 19 October, with each group\'s fixed matches; the knockout (stage 2) in single matches: quarter-finals on 23/10, semi-finals on 26/10, third place and final on 29/10'),
         ('oceania_nations_create_schedule', 'OceaniaNations::create_schedule', '0x798b60', 'oceania_nations.cpp', 'thiscall, ret 0x10', 'regra: a Copa das Nações da Oceania tem dois grupos de três rodadas e o mata-mata (semifinais, terceiro lugar e final); nos anos de resto 2 por 4 (2002...) os grupos jogam em 10, 13 e 15 de novembro e o mata-mata em 17 e 19/11; nos outros anos em 19, 21 e 23 de junho e 25 e 28/6', 'rule: the Oceania Nations Cup has two groups of three rounds and the knockout (semi-finals, third place and final); in the years with remainder 2 by 4 (2002...) the groups play on 10, 13 and 15 November and the knockout on 17 and 19/11; in the other years on 19, 21 and 23 June and 25 and 28/6'),
         ('olympics_create_schedule', 'Olympics::create_schedule', '0x79f7d0', 'olympics.cpp', 'thiscall, ret 0x10', 'regra: o futebol olímpico tem quatro grupos (fases −1 a 2) de três rodadas em 13, 16 e 19 de setembro; o mata-mata (fase 3) em jogo único: quartas em 23/9, semifinais em 26/9, terceiro lugar em 29/9 e final em 30/9', 'rule: the Olympic football has four groups (stages −1 to 2) of three rounds on 13, 16 and 19 September; the knockout (stage 3) in single matches: quarter-finals on 23/9, semi-finals on 26/9, third place on 29/9 and final on 30/9')])
CAMPOS = [('tabela de rodadas (0x41 bytes por rodada de grupo, 0x68 por rodada de copa) e as saídas rodadas/id da fase/é_liga; valor; grava', 'round table (0x41 bytes per group round, 0x68 per cup round) and the outputs rounds/stage id/is_league; value; writes')]
