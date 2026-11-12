"""Integra a 287a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('bra_nat_first_create_schedule', 4365136, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('bra_nat_second_create_schedule', 4374848, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('jap_j1_create_schedule', 6696496, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('aus_nsl_create_schedule', 4264352, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('construct_asia_super_cup', 4261456, [106, 255, 104, 137, 57, 149, 0]), ('construct_inter_amer_cup', 6496384, [106, 255, 104, 25, 178, 149, 0]), ('construct_con_merc_cup', 4986608, [106, 255, 104, 137, 105, 149, 0]), ('construct_usa_mls_all_stars', 9478480, [106, 255, 104, 105, 84, 150, 0]), ('construct_african_nations', 4198400, [106, 255, 104, 153, 52, 149, 0]), ('construct_goldcup', 6162016, [106, 255, 104, 41, 168, 149, 0]), ('construct_oceania_nations', 7964848, [106, 255, 104, 121, 235, 149, 0]), ('wc_oceania_league_build_entrant_pool', 9566848, [129, 236, 12, 2, 0, 0]), ('wc_oceania_league_create_final_stage', 9565760, [100, 161, 0, 0, 0, 0]), ('oceania_nations_create_group_stage', 7969376, [100, 161, 0, 0, 0, 0]), ('construct_con_champ', 4983872, [106, 255, 104, 57, 105, 149, 0]), ('construct_world_club_cup', 9614512, [106, 255, 104, 57, 95, 150, 0]), ('construct_asia_club_champ', 4238048, [106, 255, 104, 89, 55, 149, 0]), ('construct_asia_cup_winner', 4246496, [106, 255, 104, 25, 56, 149, 0]), ('france_rules_can_end_loan', 6037200, [139, 68, 36, 16, 131, 236, 100]), ('italy_rules_can_end_loan', 6691056, [131, 236, 100, 86, 87]), ('australia_rules_can_loan_player', 4271568, [131, 236, 100, 86, 106, 0]), ('wales_rules_get_loan_period', 9499040, [86, 87, 185, 144, 44, 174, 0]), ('russia_rules_get_loan_period', 8308960, [86, 87, 185, 144, 44, 174, 0]), ('turkey_rules_can_loan_player', 9445952, [131, 236, 100, 86, 106, 0])]
BASE = 2167
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
