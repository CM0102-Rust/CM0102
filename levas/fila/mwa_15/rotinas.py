# The season turnover of the Northern Irish and the Irish Premier Divisions (nir_prm.cpp, ire_prm.cpp, in leagues\).
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_13/mwa_14 only
# through the exe (the First Divisions' turnovers are called through their virtual).
# - 0x78ee30 (NirPrm::start_new_season, 1 call) and 0x63a9b0 (IrePrm::start_new_season, 1 call): the same body
#   (same.py: the chain argument 1/0, the table routine, the First Division's key global, the file and the lines),
#   start_irish_premier_season; close to BraNatSecond's rebuild_season but with the promotion chain first, no stages to
#   destroy and the First Division's turnover before the predicted positions.
# Conventions checked at the call sites: the virtual +8 (thiscall, ret 0). Inside: the virtual +0xb0 (push chain ->
# thiscall, ret 4), League::free_table 0x687970 (push 0 -> thiscall, ret 4), free 0x9452ca (cdecl),
# Comp_Stats::archive_season_and_reset 0x4a1c50 (push 1 -> thiscall, ret 4), the virtual +0x8c, the table routine,
# League::create_fixtures 0x6835c0, League::sort_table 0x6827d0 (push 0), the First Division's virtual +8 (through
# G_COMP_OBJECT_TABLE), the virtual +0x5c, League::set_club_season_expectations 0x68aa80 (thiscall, ret 0),
# OfficialsManager::renew_officials 0x79cee0 (push competition record; mov ecx, [0xb74340] -> thiscall, ret 4).
# Comparers: only the original (it frees, archives and draws the fixtures): the year up by one.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nir_prm_start_new_season', 0x78ee30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_prm_start_new_season', 0x63a9b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada das Premier da Irlanda do Norte e da Irlanda',
        'the season turnover of the Northern Irish and the Irish Premier Divisions',
        [('nir_prm_start_new_season', 'NirPrm::start_new_season', '0x78ee30', 'nir_prm.cpp', 'thiscall, ret 0',
          'regra: a virada da Irlanda do Norte é puxada pela Premier: acesso e rebaixamento, as estatísticas arquivadas, o ano seguinte, as regras, a tabela de dez clubes, a tabela de jogos e a ordem refeitas; depois a First Division vira, e vêm as posições previstas, as expectativas das diretorias e a renovação dos árbitros',
          'rule: the Northern Irish turnover is led by the Premier Division: promotion and relegation, the statistics archived, the next year, the rules, the ten-club table, the fixtures and the order rebuilt; then the First Division turns over, and the predicted positions, the board expectations and the referee renewal follow'),
         ('ire_prm_start_new_season', 'IrePrm::start_new_season', '0x63a9b0', 'ire_prm.cpp', 'thiscall, ret 0',
          'regra: a virada da Irlanda é puxada pela Premier: acesso e rebaixamento, as estatísticas arquivadas, o ano seguinte, as regras, a tabela, a tabela de jogos e a ordem refeitas; depois a First Division vira, e vêm as posições previstas, as expectativas das diretorias e a renovação dos árbitros',
          'rule: the Irish turnover is led by the Premier Division: promotion and relegation, the statistics archived, the next year, the rules, the table, the fixtures and the order rebuilt; then the First Division turns over, and the predicted positions, the board expectations and the referee renewal follow')])
CAMPOS = []
INTERESSA = []
