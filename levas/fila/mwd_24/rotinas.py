# The Turkish season turnover (tur_first.cpp, tur_second.cpp). Reader mwd (12/11/2026), round 2 (1 call each, the
# turnover of summer 2002).
# - 0x8fe040 TurFirst::start_new_season: the chain (push 1), 0x8fe3a0 (mwd_25) and 0x8fe580 (the third tier reshuffle,
#   not in the list: called by address), the Turkish rebuild (no stage fields, no +0x4c = 1), the tier cap of both
#   second divisions (category A 0x9cf8dc, then B 0x9cf8e0), their turnovers, the end.
# - 0x8ff330 TurSecond::start_new_season: the Turkish rebuild and the end.
# - 0x8fe250 TurFirst::process_promotion_relegation_chain: the three record_final_positions, then six
#   promote_and_relegate (0x689c80, 6 pushes -> thiscall, ret 0x18); the argument is not read (ret 4).
# Comparers: the turnovers only the original (the year +1); the chain only the original (as the Brazilian chain's).
# Equal in cm0102-1080.exe.
# Depends on: finish_croatian_turnover (mwd_5), bra_nat_first_call_virtual_no_args / comp_by_global (bra_nat_first.rs).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('tur_first_start_new_season', 0x8fe040, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tur_second_start_new_season', 0x8ff330, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tur_first_process_promotion_relegation_chain', 0x8fe250, [0x56, 0x8b, 0xf1, 0x8b, 0x06])]
DOCS = ('a virada da temporada turca', 'the Turkish season turnover',
        [('tur_first_start_new_season', 'TurFirst::start_new_season', '0x8fe040', 'tur_first.cpp', 'thiscall, ret 0',
          'regra: a virada turca, puxada pela Primeira Divisão: o acesso e o rebaixamento; os dez melhores da divisão mais baixa sobem à segunda divisão categoria B e os rebaixados dela descem; o remanejamento da terceira divisão; a Primeira refeita para o ano seguinte; o nível de divisão dos clubes das duas segundas divisões limitado e a virada das duas; as posições previstas, as expectativas da diretoria e os árbitros',
          'rule: the Turkish turnover, led by the First Division: promotion and relegation; the ten best clubs of the lowest division up into the second division category B and its relegated clubs down; the third tier reshuffle; the First Division rebuilt for the next year; the division tier of both second divisions\' clubs capped and both turned over; the predicted positions, the board expectations and the officials'),
         ('tur_second_start_new_season', 'TurSecond::start_new_season', '0x8ff330', 'tur_second.cpp', 'thiscall, ret 0',
          'regra: a virada da Segunda Divisão turca (chamada pela da Primeira): a tabela e as datas liberadas, as estatísticas arquivadas, o ano seguinte, as regras, a tabela, os jogos e a ordem refeitos; as posições previstas, as expectativas da diretoria e os árbitros',
          "rule: the Turkish Second Division turnover (called by the First Division's): the table and the dates freed, the statistics archived, the next year, the rules, the table, the fixtures and the order rebuilt; the predicted positions, the board expectations and the officials"),
         ('tur_first_process_promotion_relegation_chain', 'TurFirst::process_promotion_relegation_chain', '0x8fe250', 'tur_first.cpp', 'thiscall, ret 4 (o argumento não é lido)',
          'regra: o acesso e o rebaixamento turcos: as posições finais da Primeira e das duas segundas divisões registradas; depois os clubes marcados trocados entre a Primeira e a categoria A (aos pares), e entre a categoria A e a categoria B e cada um dos quatro grupos da categoria B (em bloco)',
          'rule: Turkish promotion and relegation: the final positions of the First Division and both second divisions recorded; then the marked clubs swapped between the First Division and category A (in pairs), and between category A and category B and each of category B\'s four groups (as a block)')])
CAMPOS = [('liga +0x4c, +0xba, +8, +0x40, +0xc (as fases da categoria B)', 'league +0x4c, +0xba, +8, +0x40, +0xc (category B\'s stages)')]
