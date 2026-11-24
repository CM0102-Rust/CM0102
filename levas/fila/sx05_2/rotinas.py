# As ordens das notas do ano, modos 0 a 2 (award/year_ratings.cpp; leitor sx05, rodada 5, 23/11/2026): as funcoes de
# comparacao do qsort (0x945333) que YEAR_RATINGS::collect_best_players (0x931ae0) e collect_candidates (0x931d50)
# passam pelo modo (0 0x931f90, 1 0x9320e0, 2 0x932230). Os modos 3 a 5 estao no sx05_3 e os coletores no sx05_4, que
# usam o YearEntry e o year_entries_order deste item (integrar junto).
# Convencoes conferidas na desmontagem: cdecl(entrada a, entrada b) -> eax (o qsort passa os enderecos das entradas de
# 21 bytes, nao ponteiros para elas); a escala x87: fild/fidiv/fmul com as constantes 0.02 (f64 0x968a98), 1.2
# (f32 0x96d7bc), 1.05 (f32 0x96d3b8) e 0.00625 (f32 0x9713c8); a pessoa em [0xae23bc] + id × 0x6e, a ficha em +0x61,
# atacante +0x15 e meia-atacante +0x14 da ficha (com sinal, > 14), habilidade atual +5 (word com sinal).
# Os premios de jogador do ano pelas notas (NationAwards::award_player_of_year_from_ratings 0x7683c0) rodam no fim da
# temporada: LIGAS = 'ENG' e ATE = '2002-08-01', como o mwg_5 (as notas do ano).
# Comparadores: a ordem lado a lado (os mesmos dois enderecos), devolvendo a da original.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('year_ratings_compare_mode_0', 0x931f90, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('year_ratings_compare_mode_1', 0x9320e0, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('year_ratings_compare_mode_2', 0x932230, [0x8b, 0x44, 0x24, 0x04, 0x53])]
_ORDEM_PT = 'entrada com menos de 3 partidas vai depois de uma com 3 ou mais (duas assim empatam); senão a pontuação maior primeiro'
_ORDEM_EN = 'an entry with fewer than 3 matches goes after one with 3 or more (two such are equal); otherwise the higher score first'
_NOTA_PT = '(gols + 2 × a soma de notas do modo + as outras duas) / partidas + 2 × partidas × 0,02; × 1,2 para quem tem atacante acima de 14, senão × 1,05 com meia-atacante acima de 14'
_NOTA_EN = '(goals + 2 × the mode\'s rating sum + the other two) / matches + 2 × matches × 0.02; × 1.2 for an attacker rated above 14, else × 1.05 for an attacking midfielder above 14'
DOCS = ('as ordens das notas do ano, modos 0 a 2',
        'the year ratings orders, modes 0 to 2',
        [('year_ratings_compare_mode_0', 'year_ratings_compare_mode_0', '0x931f90', 'award/year_ratings.cpp', 'cdecl',
          'ordem do modo 0 (a soma +8 dobrada): ' + _ORDEM_PT + '; pontuação ' + _NOTA_PT,
          'order of mode 0 (the +8 sum doubled): ' + _ORDEM_EN + '; score ' + _NOTA_EN),
         ('year_ratings_compare_mode_1', 'year_ratings_compare_mode_1', '0x9320e0', 'award/year_ratings.cpp', 'cdecl',
          'ordem do modo 1 (a soma +0xa dobrada), como o modo 0',
          'order of mode 1 (the +0xa sum doubled), as mode 0'),
         ('year_ratings_compare_mode_2', 'year_ratings_compare_mode_2', '0x932230', 'award/year_ratings.cpp', 'cdecl',
          'ordem do modo 2 (a soma +0xc dobrada), como o modo 0, e a pontuação vezes a habilidade atual × 0,00625 de quem tem ficha de jogador',
          'order of mode 2 (the +0xc sum doubled), as mode 0, and the score times the current ability × 0.00625 for a player with a sheet')])
CAMPOS = []
INTERESSA = []
