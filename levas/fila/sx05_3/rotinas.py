# As ordens das notas do ano, modos 3 a 5 (award/year_ratings.cpp; leitor sx05, rodada 5, 23/11/2026): as funcoes de
# comparacao do qsort que os coletores das notas do ano passam pelo modo (3 0x9323b0, 4 0x932500, 5 0x932590). Usam
# o YearEntry, o year_entries_order e o compare_year_order do sx05_2 (integrar junto).
# Convencoes conferidas na desmontagem: cdecl(entrada a, entrada b) -> eax; constantes 0.0002 (f32 0x9713cc), 0.02
# (f32 0x96fe6c), 4.0 (f64 0x968938) e 0.5 (f32 0x968f18); o clube do cargo da pessoa em +0x39 (reputacao +0x80,
# word com sinal); a ficha +0x61 (habilidade atual +5, reputacao atual +0xb); determinacao +0x58, lealdade +0x59 e
# esportividade +0x5c (bytes com sinal, < 10).
# Os premios de jogador do ano pelas notas rodam no fim da temporada: LIGAS = 'ENG' e ATE = '2002-08-01', como o
# mwg_5.
# Comparadores: a ordem lado a lado, devolvendo a da original.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('year_ratings_compare_mode_3', 0x9323b0, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('year_ratings_compare_mode_4', 0x932500, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('year_ratings_compare_mode_5', 0x932590, [0x8b, 0x4c, 0x24, 0x04, 0x53])]
_ORDEM_PT = 'entrada com menos de 3 partidas vai depois de uma com 3 ou mais; senão a pontuação maior primeiro'
_ORDEM_EN = 'an entry with fewer than 3 matches goes after one with 3 or more; otherwise the higher score first'
DOCS = ('as ordens das notas do ano, modos 3 a 5',
        'the year ratings orders, modes 3 to 5',
        [('year_ratings_compare_mode_3', 'year_ratings_compare_mode_3', '0x9323b0', 'award/year_ratings.cpp', 'cdecl',
          'ordem do modo 3: ' + _ORDEM_PT + '; pontuação (5 × gols + 3 × assistências + a soma +0xc) / partidas, × a reputação do clube da pessoa × 0,0002 (com clube) × a habilidade atual × 0,02 (com ficha)',
          'order of mode 3: ' + _ORDEM_EN + '; score (5 × goals + 3 × assists + the +0xc sum) / matches, × the reputation of the person\'s club × 0.0002 (with a club) × the current ability × 0.02 (with a sheet)'),
         ('year_ratings_compare_mode_4', 'year_ratings_compare_mode_4', '0x932500', 'award/year_ratings.cpp', 'cdecl',
          'ordem do modo 4: ' + _ORDEM_PT + '; pontuação os cartões (amarelos + 2 × vermelhos) por partida',
          'order of mode 4: ' + _ORDEM_EN + '; score the cards (yellows + 2 × reds) per match'),
         ('year_ratings_compare_mode_5', 'year_ratings_compare_mode_5', '0x932590', 'award/year_ratings.cpp', 'cdecl',
          'ordem do modo 5: ' + _ORDEM_PT + '; pontuação a do modo 3 sem o clube, × (reputação atual × 0,0002 + 4) com ficha, pela metade com determinação, lealdade ou esportividade abaixo de 10',
          'order of mode 5: ' + _ORDEM_EN + '; score that of mode 3 without the club, × (current reputation × 0.0002 + 4) with a sheet, halved with determination, loyalty or sportsmanship below 10')])
CAMPOS = []
INTERESSA = []
