"""Integra a 292a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('bra_nat_third_create_schedule', 4380192, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('usa_mls_create_season_fixtures', 9471712, [106, 255, 104, 184, 83, 150, 0]), ('comp_util_get_stage_name', 4933712, [129, 236, 0, 2, 0, 0]), ('comp_util_format_comp_stage_round_label', 4979248, [129, 236, 48, 3, 0, 0]), ('cup_stage_map_fixture_tree', 5390816, [129, 236, 28, 2, 0, 0]), ('comp_util_write_cup_win_or_defeat_news', 4932640, [129, 236, 244, 1, 0, 0]), ('comp_util_write_group_qualify_news', 4933008, [138, 68, 36, 16, 131, 236, 100]), ('asia_nations_set_up_rules', 4256960, [106, 255, 104, 232, 56, 149, 0]), ('oceania_nations_set_up_rules', 7968400, [106, 255, 104, 184, 235, 149, 0]), ('olympics_set_up_rules', 7995952, [106, 255, 104, 248, 239, 149, 0]), ('euro_champ_set_up_rules', 5748272, [106, 255, 104, 208, 135, 149, 0]), ('fifa_confed_set_up_rules', 5815728, [131, 236, 8, 176, 2]), ('world_cup_set_up_rules', 9622016, [106, 255, 104, 208, 95, 150, 0]), ('construct_asia_nations', 4253792, [106, 255, 104, 169, 56, 149, 0]), ('construct_olympics', 7992624, [106, 255, 104, 185, 239, 149, 0]), ('construct_fifa_confed', 5813008, [106, 255, 104, 105, 141, 149, 0]), ('construct_wc_asia_league', 9509872, [106, 255, 104, 9, 87, 150, 0]), ('construct_wc_oceania_league', 9559952, [106, 255, 104, 185, 90, 150, 0]), ('construct_wc_south_american_league', 9568944, [106, 255, 104, 169, 91, 150, 0]), ('asia_nations_update_for_date', 4260208, [106, 255, 104, 102, 57, 149, 0]), ('olympics_update_for_date', 7999936, [106, 255, 104, 102, 240, 149, 0]), ('fifa_confed_update_for_date', 5817520, [106, 255, 104, 235, 141, 149, 0]), ('asia_nations_create_schedule', 4254464, [129, 236, 0, 2, 0, 0]), ('oceania_nations_create_schedule', 7965536, [129, 236, 0, 2, 0, 0]), ('olympics_create_schedule', 7993296, [129, 236, 0, 2, 0, 0]), ('euro_champ_create_schedule', 5745552, [129, 236, 0, 2, 0, 0]), ('wc_asia_league_create_schedule', 9510656, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('wc_oceania_league_create_schedule', 9560800, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('wc_south_american_league_create_schedule', 9569680, [138, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('wc_south_american_league_count_exempt_nations', 9571600, [129, 236, 8, 2, 0, 0]), ('wc_south_american_league_build_team_slots', 9572016, [129, 236, 12, 2, 0, 0])]
BASE = 2271
m = re.search(r'\n\}\nconst N: usize = (\d+);', s)
assert m and int(m.group(1)) == BASE, ('o motor.rs mudou desde a materialização', m and m.group(1), BASE)
# one line per routine in `routines! { .. }` (rust/src/motor.rs; read by engenharia-reversa/tabela_motor.py)
linhas = ''.join('\n    Prologue { address: %#x, bytes: &[%s] } => %s, compare_%s;' % (e, ', '.join('0x%02x' % b for b in bs), a, a)
                 for a, e, bs in ROTINAS)
s = s[:m.start()] + linhas + '\n}\nconst N: usize = %d;' % (BASE + len(ROTINAS)) + s[m.end():]
anc = 'compare_value!(compare_staff_criterion, staff_criterion, IDX_STAFF_CRITERION, () -> u32);\n'
assert s.count(anc) == 1
s = s.replace(anc, anc + '\n' + open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read().strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
