# The Polish season turnover (pol_first.cpp, pol_second.cpp). Reader r3_mwcd (19/11/2026), round 2 (1 call each, the
# turnover of summer 2002).
# - 0x7c9740 PolFirst::start_new_season: the template of the top-division turnovers (top_division_turnover, batch 305:
#   no stage destroyed, no current stage reset), with the chain (push 1) and 0x7ca030 before, and after the order
#   PolFirst::create_second_group_stage (0x7c9900, thiscall, ret 0, not ported) and the Second Division's turnover
#   (virtual +8, no null check), then +0x5c, 0x68aa80 and 0x79cee0, instruction by instruction.
# - 0x7ca030 pol_first_promote_lower_division_clubs: like the Greek swap (mwd_22) with a nation check (Poland) and
#   club +0x5b = +0x57 for every candidate; 4 up (picks 4: the best twelve shuffled); down: the rows club_count -
#   relegated_count (+0x3e word, +0xc1 signed byte) .. club_count of the Second Division. The map says cdecl(void) but
#   ecx (the First Division) is passed as this to 0x6830b0 and 0x6831a0: thiscall, ret 0.
# - 0x7cc000 PolSecond::start_new_season: instruction by instruction the template of FraSecond::start_new_season
#   (mwf_7): rebuild_season(.., 1) + set_expectations_and_renew_officials.
# Conventions: thiscall, ret 0. Comparers: the turnovers only the original (the year +1); the promotion only the
# original, the relegated clubs checked in the lowest division (with at least 4 candidates).
# Mutation: the draw among the best 24 instead of 12 (who goes up changes, if there are more than 4 candidates in the
# lowest Polish division: the clubs' divisions in the database and the 2002/03 fixtures change).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('pol_first_start_new_season', 0x7c9740, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pol_first_promote_lower_division_clubs', 0x7ca030, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('pol_second_start_new_season', 0x7cc000, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada da temporada polonesa', 'the Polish season turnover',
        [('pol_first_start_new_season', 'PolFirst::start_new_season', '0x7c9740', 'pol_first.cpp', 'thiscall, ret 0',
          'regra: a virada polonesa, puxada pela Primeira Divisão: o acesso e o rebaixamento pela cadeia; os quatro melhores da divisão mais baixa sobem à Segunda e os últimos dela descem; a Primeira refeita para o ano seguinte (regras, tabela, jogos, ordem) e a sua segunda fase de grupos; a virada da Segunda; as posições previstas, as expectativas da diretoria e os árbitros',
          'rule: the Polish turnover, led by the First Division: promotion and relegation along the chain; the four best clubs of the lowest division up into the Second Division and its bottom clubs down; the First Division rebuilt for the next year (rules, table, fixtures, order) and its second group stage; the Second Division turned over; the predicted positions, the board expectations and the officials'),
         ('pol_first_promote_lower_division_clubs', 'pol_first_promote_lower_division_clubs', '0x7ca030', 'pol_first.cpp', 'thiscall (ecx = a Primeira Divisão), ret 0',
          'regra: na virada polonesa, os quatro melhores clubes poloneses da divisão mais baixa (ordenados por força, os doze melhores embaralhados) sobem à Segunda Divisão, e os últimos da Segunda (tantos quantos os rebaixados diretos dela) descem; cada candidato guarda a divisão como a anterior; com menos de 4 candidatos ninguém sobe nem desce',
          "rule: at the Polish turnover, the four best Polish clubs of the lowest division (sorted by strength, the best twelve shuffled) go up to the Second Division, and the Second Division's bottom clubs (as many as its direct relegation places) go down; each candidate keeps its division as the last one; with fewer than 4 candidates nobody moves"),
         ('pol_second_start_new_season', 'PolSecond::start_new_season', '0x7cc000', 'pol_second.cpp', 'thiscall, ret 0',
          'regra: a virada da Segunda Divisão polonesa (chamada pela da Primeira): a temporada reaberta, as fases da temporada passada destruídas, o ano seguinte, as regras, a tabela, os jogos e a ordem refeitos; as posições previstas, as expectativas da diretoria e os árbitros',
          "rule: the Polish Second Division turnover (called by the First Division's): the season reopened, last season's stages destroyed, the next year, the rules, the table, the fixtures and the order rebuilt; the predicted positions, the board expectations and the officials")])
CAMPOS = [('liga +0x4c, +0xba, +8, +0xc, +0x30, +0x40, +0x3e, +0xc1; clube +0x53 nação, +0x57 divisão, +0x5b divisão anterior', 'league +0x4c, +0xba, +8, +0xc, +0x30, +0x40, +0x3e, +0xc1; club +0x53 nation, +0x57 division, +0x5b last division')]
