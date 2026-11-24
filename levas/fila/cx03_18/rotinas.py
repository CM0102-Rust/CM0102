# A virada das duas copas de clubes da Asia (asia_club_champ.cpp, asia_cup_winner.cpp; leitor cx03, rodada 5, 24/11/2026).
# Carreira de 8 ligas (o Japao jogavel).
# - 0x40b700 AsiaClubChamp::start_new_season e 0x40d5b0 AsiaCupWinner::start_new_season thiscall() ret 0 (a virtual +8): o
#   mesmo corpo (conferido instrucao a instrucao; so as rotinas, o arquivo e as linhas mudam): +0x4c = 0; free e zera
#   +0xa7, +0xa3 e a lista +0xb2; os confrontos de cada rodada (free e zera) e o vetor (free e zera); as fases 0..+0x30
#   destruidas (virtual 0 com 1; os slots e o vetor ficam) e +0x30 = -1; Comp_Stats::archive_season_and_reset 0x4a1c50
#   (push 1 -> thiscall, ret 4); +0x40 + 1, +0xab = 0; os clubes (0x40b950 / 0x40d830; 0: a caixa 0x21c / 0x1d4),
#   +0xb1 = 0, os times (0x40b620 / 0x40d4d0; negativo: 0x226 / 0x1de), as virtuais 0x8c (0x22d / 0x1e5) e 0x94 (0x234
#   / 0x1ec), a primeira fase (0x40bec0 / 0x40ddf0), a lista +0xb2 liberada e zerada de novo e a virtual 0x5c, cujo eax
#   devolve. Todas thiscall, ret 0. Prologo sub esp, 0x200.
# DATA: nenhuma das duas roda no mundo ate 2002-08-01 (docs/cobertura_mundo.txt: 0 chamadas). A virada da edicao vem
# depois; a data nao se sabe sem rodar a cobertura de 8 ligas mais longa. ATE provisorio 2003-01-31 (o fim da carreira
# ampla): medir antes de integrar (sem chamada ate a data, a mutacao escapa e o item fica para a proxima auditoria).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Comparadores: so a original (liberam, alocam e sorteiam), com o plano (o ano sobe 1: compare_cup_new_season).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-01-31'
SAVE = False
ROTINAS = [('asia_club_champ_start_new_season', 0x40b700, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('asia_cup_winner_start_new_season', 0x40d5b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada das duas copas de clubes da Ásia', 'the season turnover of the two Asian club cups',
        [('asia_club_champ_start_new_season', 'AsiaClubChamp::start_new_season', '0x40b700', 'asia_club_champ.cpp', 'thiscall, ret 0',
          'regra: na virada a Liga dos Campeões da Ásia é limpa (times, rodadas, confrontos, os clubes classificados e as fases), as estatísticas arquivadas e o ano avança; depois os clubes classificados, os times, as regras, a árvore dos confrontos, a fase da segunda zona e as posições previstas de novo',
          'rule: at the turnover the Asian Club Championship is cleared (teams, rounds, ties, the qualified clubs and the stages), its statistics archived and the year moves on; then the qualified clubs, the team entries, the rules, the tie tree, the second zone stage and the predicted positions again'),
         ('asia_cup_winner_start_new_season', 'AsiaCupWinner::start_new_season', '0x40d5b0', 'asia_cup_winner.cpp', 'thiscall, ret 0',
          'regra: a virada da Recopa da Ásia, como a da Liga dos Campeões, com os seus clubes, times e fase de zona',
          "rule: the Asian Cup Winners' Cup turnover, as the Asian Club Championship's, with its own clubs, team entries and zone stage")])
CAMPOS = [('copas de clubes da Ásia +0xb2: a lista dos clubes classificados (liberada na virada)', 'Asian club cups +0xb2: the qualified club list (freed at the turnover)')]
INTERESSA = []
