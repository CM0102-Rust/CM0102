# O objeto dos premios do Brasil e o dia deles (award/brazil_awards.cpp; leitor rx09, rodada 6, 25/11/2026). Simulacao com
# as 8 ligas (o Brasil carregado): o construtor na criacao do jogo; os premios estaduais em 10 de julho de 2001 e os
# nacionais em 16 de dezembro de 2001.
# - 0x43d2b0 BrazilAwards::BrazilAwards thiscall(arquivo) ret 4 (SEH so do estado): NationAwards::NationAwards 0x768290,
#   vtable 0x968824, +0xe a nacao ([0xae23a8] + [0x9cf260] x 0x122), +0x13 = 5, +0x12 = 1; com arquivo,
#   NationAwards::load_from_disk 0x76a280 e, carregado, fim; senao (ou sem arquivo) init_awards 0x43d340(arquivo).
#   O molde nation_awards_construct do sx05b_5 (sem as notas do mes).
# - 0x43e990 BrazilAwards::daily_update thiscall(hoje) ret 4: Date::Date 0x549f70(10, 6, ano de hoje, -1) e a comparacao
#   das duas palavras (awards_day_is); os 36 premios estaduais por NationAwards::award_player_of_year 0x768cf0 (os 10
#   argumentos de cada chamada conferidos: filtros -1/0/0x11, -1/0/2, -1/1/0x11); 16 de dezembro: os nacionais
#   (0x768cf0 e award_team_of_season 0x7698f0, na ordem jogador, equipe, artilheiro, jovem ate 21); sempre
#   NationAwards::daily_update 0x768310.
# Prologos: push -1; push imm32 (7 bytes); sub esp, 0x10; push esi; push edi (5 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original nos dois (o construtor aloca; os premios nao se dao duas vezes); o objeto conferido
# depois (vtable, nacao, tipo, 58 premios sem arquivo).
# Depende do sx05b_5 (NationAwardsClass, nation_awards_construct, compare_nation_awards_built) e do sx03_2
# (award_player_of_year, award_team_of_season, nation_award, awards_day_is ja no motor).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('construct_brazil_awards', 0x43d2b0, [0x6a, 0xff, 0x68, 0x78, 0x4b, 0x95, 0x00]),
           ('brazil_awards_daily_update', 0x43e990, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('o objeto dos prêmios do Brasil e o dia deles', 'the Brazilian awards object and its day',
        [('construct_brazil_awards', 'BrazilAwards::BrazilAwards', '0x43d2b0', 'award/brazil_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios do Brasil (nação 5): com o save, os carregados; senão os 58 prêmios novos',
          'builds the Brazilian awards (nation 5): with a save, the loaded ones; otherwise the 58 new awards'),
         ('brazil_awards_daily_update', 'BrazilAwards::daily_update', '0x43e990', 'award/brazil_awards.cpp', 'thiscall, ret 4',
          'regra: em 10 de julho, os prêmios de cada um dos doze estaduais: melhor jogador, artilheiro e melhor goleiro; em 16 de dezembro, os da Primeira e da Segunda Divisão: jogador do ano, equipe do ano, artilheiro e jovem do ano (até 21)',
          'rule: on 10 July, the awards of each of the twelve state championships: best player, top goalscorer and best goalkeeper; on 16 December, those of the First and Second Divisions: player of the year, team of the year, top goalscorer and young player of the year (up to 21)')])
CAMPOS = []
INTERESSA = []
