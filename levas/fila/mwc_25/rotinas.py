# The Belgian season turnover and the Eerste Divisie and Dutch Cup stages (bel_first.cpp, hol_first.cpp, hol_cup.cpp).
# Reader r3_mwcd (19/11/2026), round 2.
# - 0x41ea90 BelFirst::start_new_season (1 call, summer 2002): top_division_turnover (batch 305) with, before, the chain
#   (push 1), 0x41ec90 and 0x41ef70 (bel_first_move_third_division_clubs / bel_first_split_third_division_groups,
#   cdecl(void) by the map, called with ecx = this; not in the coverage list, not ported) and the tier limits (this
#   with (1, 3, -3, 1); the Second Division with (2, -3, 3, 0)); after, the Second and Third Divisions' turnovers (no
#   null check), instruction by instruction.
# - 0x5f6650 HolFirst::advance_stage: like advance_first_division_stage (batch 304: SweFirst/NorFirst) but the play-off
#   groups (0x5f6720) at stage 4 instead of 0; line 0x263; the Eredivisie = KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION.
# - 0x5f4190 HolCup::advance_stage (1 call, spring 2002): the knockout (0x5f4480) at stage 0x13; line 0x24f.
# Conventions: thiscall, ret 0. Comparers: the turnover only the original (the year +1); HolFirst only the original
# (compare_first_division_advance: it asks and updates the Eredivisie); HolCup side by side on +0x30 when nothing is
# built.
# Mutations: none of their own (the turnover builds the 2002/03 season, after the chain's date; an advance_stage that
# builds another stage or none would hang the competition rather than change who plays). The comparers carry the proof.
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_first_start_new_season', 0x41ea90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hol_first_advance_stage', 0x5f6650, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hol_cup_advance_stage', 0x5f4190, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a virada belga e as fases da Eerste Divisie e da Copa da Holanda', 'the Belgian turnover and the Eerste Divisie and Dutch Cup stages',
        [('bel_first_start_new_season', 'BelFirst::start_new_season', '0x41ea90', 'bel_first.cpp', 'thiscall, ret 0',
          'regra: a virada belga, puxada pela Primeira Divisão: o acesso e o rebaixamento pela cadeia; os movimentos da Terceira Divisão e a sua divisão nos grupos A e B; o nível de divisão dos clubes limitado; a Primeira refeita para o ano seguinte (regras, tabela, jogos, ordem); as viradas da Segunda e da Terceira; as posições previstas, as expectativas da diretoria e os árbitros',
          "rule: the Belgian turnover, led by the First Division: promotion and relegation along the chain; the Third Division's movements and its split into groups A and B; the clubs' division tier capped; the First Division rebuilt for the next year (rules, table, fixtures, order); the Second and Third Divisions turned over; the predicted positions, the board expectations and the officials"),
         ('hol_first_advance_stage', 'HolFirst::advance_stage', '0x5f6650', 'hol_first.cpp', 'thiscall, ret 0',
          'regra: a fase seguinte da Eerste Divisie: depois da última, a caixa de erro; senão só com a Eredivisie terminada: as posições e situações dela atualizadas e a fase atual sobe uma; ao chegar à fase 4 (depois dos quatro períodos) são montados os grupos do mata-mata; com a Eredivisie ainda em jogo, nada',
          'rule: the next stage of the Eerste Divisie: past the last stage, the error box; otherwise only when the Eredivisie is complete: its positions and statuses updated and the current stage up by one; on reaching stage 4 (after the four periods) the play-off groups are built; with the Eredivisie still playing, nothing'),
         ('hol_cup_advance_stage', 'HolCup::advance_stage', '0x5f4190', 'hol_cup.cpp', 'thiscall, ret 0',
          'regra: a fase seguinte da Copa da Holanda: depois da última, a caixa de erro; senão a fase atual sobe uma; ao chegar à fase 19 (depois das fases de grupos) é montado o mata-mata',
          'rule: the next stage of the Dutch Cup: past the last stage, the error box; otherwise the current stage goes up by one; on reaching stage 19 (after the group stages) the knockout stage is built')])
CAMPOS = [('liga +0x2c número de fases, +0x30 fase atual, +0x4c, +0xba, +8, +0x40', 'league +0x2c stage count, +0x30 current stage, +0x4c, +0xba, +8, +0x40')]
