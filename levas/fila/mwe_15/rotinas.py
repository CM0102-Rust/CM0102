# The Portuguese turnover and its promotion and relegation chain (reader mwe, whole-world round, 12/11/2026). Both run
# once in the test career, at the Portuguese turnover in the summer of 2002 (before 2002-08-01):
# - 0x7cf9e0 PorPrm::start_new_season (528 bytes): the skeleton of the top-division turnovers (mwe_14) with the
#   Portuguese steps before (0x7d0320 with the Second Division B; the chain; 0x7cff70 and 0x7d0210 with the Second
#   Division B, otherwise 0x7cfd30 PorPrm::exchange_second_division_with_zones, called by address, not ported here)
#   and the Liga de Honra and Second Division B turnovers after;
# - 0x7cfbf0 PorPrm::process_promotion_relegation_chain (320 bytes): the final positions and League::promote_and_relegate
#   between the Premier Division and the Liga de Honra, and between the Liga de Honra and the three Second Division B
#   zones when they are played (the Brazilian chain's mould, cqq_8).
# Equal in cm0102-1080.exe (whole).
# Conventions checked: start_new_season through the vtable (+8) -> thiscall, ret 0; the chain through the vtable +0xb0
# (push 1 -> thiscall, ret 4; the `ret 4` checked; the argument is not read); RECORD_FINAL_POSITIONS (+0xa4, no push ->
# thiscall, ret 0); League::promote_and_relegate 0x689c80 (6 pushes -> thiscall, ret 0x18); the steps (mov ecx, esi;
# call -> thiscall, ret 0); the rest as mwe_14.
# Comparers: only the original: the chain (it moves clubs; as the Brazilian one's); the turnover, the year + 1.
# Determinism: without the Second Division B the chain returns the G_COMP_OBJECT_TABLE value left in eax (the copy
# returns the same); nothing else reads stack, heap or an address as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('por_prm_process_promotion_relegation_chain', 0x7cfbf0, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('por_prm_start_new_season', 0x7cf9e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada portuguesa e a sua cadeia de acesso e rebaixamento',
        'the Portuguese turnover and its promotion and relegation chain',
        [('por_prm_process_promotion_relegation_chain', 'PorPrm::process_promotion_relegation_chain', '0x7cfbf0', 'por_prm.cpp', 'thiscall, ret 4',
          'o acesso e o rebaixamento em Portugal: registra as posições finais da primeira divisão e da Liga de Honra e troca os clubes marcados entre elas; com a segunda divisão B jogada, registra as posições das suas três zonas e a Liga de Honra troca clubes com cada uma',
          'promotion and relegation in Portugal: records the final positions of the Premier Division and of the Liga de Honra and swaps the marked clubs between them; with the Second Division B played, records the positions of its three zones and the Liga de Honra swaps clubs with each one'),
         ('por_prm_start_new_season', 'PorPrm::start_new_season', '0x7cf9e0', 'por_prm.cpp', 'thiscall, ret 0',
          'a virada portuguesa, puxada pela primeira divisão: as verificações dos times B (com a segunda divisão B), o acesso e o rebaixamento, a troca da Liga de Honra com as zonas regionais (sem a segunda divisão B), a primeira divisão refeita para o ano seguinte, depois as viradas da Liga de Honra e da segunda divisão B, as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'the Portuguese turnover, led by the Premier Division: the B team checks (with the Second Division B), promotion and relegation, the Liga de Honra exchange with the regional zones (without the Second Division B), the Premier Division rebuilt for the next year, then the Liga de Honra and Second Division B turnovers, the predicted positions, the board expectations and the referee renewal')])
CAMPOS = []
INTERESSA = []
