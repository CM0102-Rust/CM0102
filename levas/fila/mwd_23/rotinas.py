# The Russian season turnover (rus_first.cpp, rus_prm.cpp). Reader mwd (12/11/2026), round 2 (1 call each, the turnover
# between the 2001 and 2002 seasons).
# - 0x7ea8c0 RusFirst::start_new_season: +0x4c = 0 and the body of rebuild_season (bra_nat_first.rs: the stage objects
#   destroyed, the predicted positions inside), checked instruction by instruction; then 0x68aa80 and 0x79cee0.
# - 0x7eb800 RusPrm::start_new_season: the chain (push 1), 0x7ebe20 (the exchange), the Croatian rebuild with +0x4c = 1,
#   the tier cap and the turnover of the First Division, the end.
# - 0x7ebe20 rus_prm_exchange_first_and_second_division_clubs: the zones compared by record pointer (club +0x57 against
#   G_CLUB_COMP_TABLE + 0x6b x id); the last three relegated whatever their status; with fewer than 3 candidates
#   nothing moves. The map says cdecl(void) but ecx (the Premier League) is passed as this to 0x6830b0 and 0x6831a0:
#   thiscall, ret 0.
# Comparers: the turnovers only the original (the year +1); the exchange only the original (it draws), the three
# relegated clubs checked in their zones. Equal in cm0102-1080.exe.
# Depends on: rebuild_season / set_expectations_and_renew_officials (bra_nat_first.rs), rebuild_croatian_league /
# finish_croatian_turnover (mwd_5).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('rus_first_start_new_season', 0x7ea8c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rus_prm_start_new_season', 0x7eb800, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rus_prm_exchange_first_and_second_division_clubs', 0x7ebe20, [0x81, 0xec, 0x2c, 0x02, 0x00, 0x00])]
DOCS = ('a virada da temporada russa', 'the Russian season turnover',
        [('rus_first_start_new_season', 'RusFirst::start_new_season', '0x7ea8c0', 'rus_first.cpp', 'thiscall, ret 0',
          'regra: a virada da Primeira Divisão russa (chamada pela da liga principal): a temporada reaberta, a tabela, as datas e as fases liberadas, as estatísticas arquivadas, o ano seguinte, as regras, a tabela, os jogos, a ordem e as posições previstas; as expectativas da diretoria e os árbitros',
          "rule: the Russian First Division turnover (called by the Premier League's): the season reopened, the table, the dates and the stages freed, the statistics archived, the next year, the rules, the table, the fixtures, the order and the predicted positions; the board expectations and the officials"),
         ('rus_prm_start_new_season', 'RusPrm::start_new_season', '0x7eb800', 'rus_prm.cpp', 'thiscall, ret 0',
          'regra: a virada russa, puxada pela liga principal: o acesso e o rebaixamento pela cadeia; a troca entre a Primeira Divisão e as seis zonas da segunda; a liga principal refeita para o ano seguinte; o nível de divisão dos clubes da Primeira limitado e a virada da Primeira; as posições previstas, as expectativas da diretoria e os árbitros',
          "rule: the Russian turnover, led by the Premier League: promotion and relegation along the chain; the exchange between the First Division and the six second division zones; the Premier League rebuilt for the next year; the First Division clubs' tier capped and the First Division turned over; the predicted positions, the board expectations and the officials"),
         ('rus_prm_exchange_first_and_second_division_clubs', 'rus_prm_exchange_first_and_second_division_clubs', '0x7ebe20', 'rus_prm.cpp', 'thiscall (ecx = a liga principal), ret 0',
          'regra: na virada russa, o melhor clube de cada par de zonas da segunda divisão (oeste/centro, sul/Volga, leste/Urais; ordenados por força, os nove melhores embaralhados) sobe à Primeira Divisão, e os três últimos da Primeira descem: o último para a zona central, o penúltimo para a sul, o antepenúltimo para a dos Urais, qualquer que seja a situação; com menos de 3 candidatos ninguém se move',
          'rule: at the Russian turnover, the best club of each pair of second division zones (west/central, south/Volga, east/Ural; sorted by strength, the best nine shuffled) goes up to the First Division, and the First Division\'s last three go down: the last to the central zone, the second last to the south zone, the third last to the Ural zone, whatever their status; with fewer than 3 candidates nobody moves')])
CAMPOS = [('liga +0x4c, +0xba, +8, +0xc, +0x2c, +0x30, +0x40; clube +0x57 divisão (ponteiro do registro)', 'league +0x4c, +0xba, +8, +0xc, +0x2c, +0x30, +0x40; club +0x57 division (record pointer)')]
