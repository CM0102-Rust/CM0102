# The Danish and Dutch top-division season turnovers (den_prm.cpp, hol_prm.cpp). Reader r3_mwcd (19/11/2026), round 2
# (1 call each, the turnover of summer 2002).
# - 0x553aa0 DenPrm::start_new_season: the template of the top-division turnovers (top_division_turnover, batch 305),
#   with the chain (push 1), 0x553cb0 and three League::limit_club_division_tier before (this and the First Division
#   with (1, 3, -3, 1); the Second Division with (2, -3, 3, 0): pushes 0, 3, -3, 2), and the First and Second Divisions'
#   turnovers after (virtual +8, no null check), instruction by instruction.
# - 0x553cb0 DenPrm::refresh_second_division_clubs: the club loop stops at the first national team (0x53b900, cdecl:
#   jne past the loop); nation Denmark and division KEY_CLUB_COMP_A_LOWER_DIVISION; with n != 0 (16-bit) the draw
#   (picks 4: the best twelve shuffled) and 4 moves even when n < 4 (defect of the original: the unwritten slots of the
#   malloc'd list; kept; the Danish lowest division has far more than 4 clubs); then the Second Division's status-3
#   rows down. The map says thiscall: ecx = the Superliga, ret 0.
# - 0x5f8610 HolPrm::start_new_season: top_division_turnover with the chain before and, after, the Eerste Divisie's tier
#   limit (1, 3, -3, 1) and turnover (KEY_CLUB_COMP_DUTCH_TOTO_DIVISION, virtual +8, no null check).
# Conventions: thiscall, ret 0. Comparers: the turnovers only the original (the year +1); the refresh only the original,
# the relegated clubs checked in the lowest division (with at least 4 candidates).
# Mutation: the draw among the best 24 instead of 12 (who goes up to the Danish Second Division changes: the clubs'
# divisions in the database and the 2002/03 fixtures change).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('den_prm_start_new_season', 0x553aa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('den_prm_refresh_second_division_clubs', 0x553cb0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('hol_prm_start_new_season', 0x5f8610, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada das primeiras divisões da Dinamarca e da Holanda', 'the Danish and Dutch top-division season turnovers',
        [('den_prm_start_new_season', 'DenPrm::start_new_season', '0x553aa0', 'den_prm.cpp', 'thiscall, ret 0',
          'regra: a virada dinamarquesa, puxada pela Superliga: o acesso e o rebaixamento pela cadeia; os quatro melhores dinamarqueses da divisão mais baixa sobem à Segunda e os rebaixados dela descem; o nível de divisão dos clubes limitado; a Superliga refeita para o ano seguinte (regras, tabela, jogos, ordem); as viradas da Primeira e da Segunda; as posições previstas, as expectativas da diretoria e os árbitros',
          'rule: the Danish turnover, led by the Superliga: promotion and relegation along the chain; the four best Danish clubs of the lowest division up into the Second Division and its relegated clubs down; the clubs\' division tier capped; the Superliga rebuilt for the next year (rules, table, fixtures, order); the First and Second Divisions turned over; the predicted positions, the board expectations and the officials'),
         ('den_prm_refresh_second_division_clubs', 'DenPrm::refresh_second_division_clubs', '0x553cb0', 'den_prm.cpp', 'thiscall (ecx = a Superliga), ret 0',
          'regra: na virada dinamarquesa, os quatro melhores clubes dinamarqueses da divisão mais baixa (ordenados por força, os doze melhores embaralhados; a busca para na primeira seleção) sobem à Segunda Divisão, e os rebaixados (situação 3) da Segunda descem; sem candidato ninguém sobe nem desce (com menos de quatro, o original move também posições não escritas da lista: defeito)',
          'rule: at the Danish turnover, the four best Danish clubs of the lowest division (sorted by strength, the best twelve shuffled; the search stops at the first national team) go up to the Second Division, and its relegated clubs (status 3) go down; with no candidate nobody moves (with fewer than four, the original also moves unwritten slots of the list: a defect)'),
         ('hol_prm_start_new_season', 'HolPrm::start_new_season', '0x5f8610', 'hol_prm.cpp', 'thiscall, ret 0',
          'regra: a virada holandesa, puxada pela Eredivisie: o acesso e o rebaixamento pela cadeia; a Eredivisie refeita para o ano seguinte (regras, tabela, jogos, ordem); o nível de divisão dos clubes da Eerste Divisie limitado e a virada dela; as posições previstas, as expectativas da diretoria e os árbitros',
          "rule: the Dutch turnover, led by the Eredivisie: promotion and relegation along the chain; the Eredivisie rebuilt for the next year (rules, table, fixtures, order); the Eerste Divisie clubs' division tier capped and the Eerste Divisie turned over; the predicted positions, the board expectations and the officials")])
CAMPOS = [('liga +0x4c, +0xba, +8, +0x40; clube +0x53 nação, +0x57 divisão; linha da tabela +0x37 situação', 'league +0x4c, +0xba, +8, +0x40; club +0x53 nation, +0x57 division; table row +0x37 status')]
