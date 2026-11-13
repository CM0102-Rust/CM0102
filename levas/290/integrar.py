"""Integra a 290a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('try_add_player_to_club_shortlist', 8662544, [106, 255, 104, 248, 36, 150, 0]), ('contract_manager_daily_update', 5012160, [85, 139, 236, 131, 228, 248, 106, 255]), ('generate_initial_contract', 8776032, [106, 255, 104, 48, 42, 150, 0]), ('transfer_manager_move_player_to_club', 9213728, [106, 255, 104, 91, 62, 150, 0]), ('write_contract_talks_news_text', 9273264, [129, 236, 68, 1, 0, 0, 83, 85, 86]), ('write_contract_expiry_news_text', 5073408, [129, 236, 64, 1, 0, 0, 83, 85, 86]), ('write_contract_end_news_text', 5089792, [129, 236, 64, 1, 0, 0, 83, 85, 86]), ('write_offer_accepted_news_text', 9271408, [139, 68, 36, 8, 129, 236, 68, 1, 0, 0]), ('score_staff_for_job', 6934960, [100, 161, 0, 0, 0, 0]), ('appoint_manager_to_club', 6941632, [129, 236, 20, 2, 0, 0]), ('season_history_entry_construct', 4489824, [106, 255, 104, 65, 84, 149, 0]), ('club_records_write_record_news_fields', 4534560, [129, 236, 4, 2, 0, 0]), ('nationalteams_init_tables', 7776320, [129, 236, 8, 2, 0, 0]), ('discipline_create_rule_sets', 5624864, [100, 161, 0, 0, 0, 0]), ('build_board_expectation_text', 6898752, [139, 68, 36, 8, 86]), ('news_board_expectations_text', 6965984, [184, 136, 19, 0, 0]), ('news_board_confidence_update_text', 6966592, [184, 232, 39, 0, 0]), ('news_manager_appointment_text', 6959200, [139, 68, 36, 8, 129, 236, 20, 2, 0, 0]), ('news_manager_under_pressure_text', 6951312, [139, 68, 36, 8, 129, 236, 52, 5, 0, 0])]
BASE = 2215
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
