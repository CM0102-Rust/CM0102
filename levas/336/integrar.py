"""Integra a 336a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('reset_human_record', 6273040, [106, 255, 104, 22, 176, 149, 0]), ('key_nation_create_all_comps', 6721920, [129, 236, 32, 2, 0, 0]), ('hall_of_fame_load', 6224544, [129, 236, 12, 2, 0, 0]), ('hall_of_fame_unscramble_and_check', 6227584, [129, 236, 4, 2, 0, 0]), ('hall_of_fame_compare_entries', 6227488, [86, 139, 116, 36, 8]), ('post_ban_news', 5599872, [106, 255, 104, 1, 125, 149, 0]), ('compare_nation_index_entries', 5629424, [139, 84, 36, 4, 86]), ('halve_club_fine_count', 5921072, [139, 68, 36, 4, 133, 192]), ('clear_club_fine_count', 5921120, [139, 68, 36, 4, 133, 192]), ('get_scope_text', 5623360, [139, 68, 36, 12, 129, 236, 0, 2, 0, 0]), ('is_in_dispute', 5676592, [86, 139, 116, 36, 8]), ('get_dispute_club_id', 5677104, [83, 139, 92, 36, 8]), ('process_hearings', 5676752, [81, 83, 139, 217, 85]), ('dispute_post_news', 5678000, [100, 161, 0, 0, 0, 0]), ('write_board_fine_warning_news_text', 5930704, [139, 68, 36, 8, 133, 192]), ('team_award_construct', 9106720, [106, 255, 104, 204, 55, 150, 0]), ('year_award_construct', 9633408, [106, 255, 104, 131, 96, 150, 0]), ('year_ratings_construct', 9638752, [102, 139, 68, 36, 4]), ('awol_start_absence', 4298816, [106, 255, 104, 176, 62, 149, 0]), ('awol_post_news', 4299472, [100, 161, 0, 0, 0, 0]), ('cash_set_currency', 4454384, [85, 139, 236, 131, 228, 248]), ('cash_step_up', 4453600, [131, 236, 8, 83, 139, 217]), ('cash_step_down', 4453760, [131, 236, 8, 83, 139, 217]), ('cash_get_currency', 4462048, [160, 64, 177, 152, 0]), ('swap_bytes_64_array', 4453200, [139, 68, 36, 8, 139, 200])]
BASE = 3921
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
