"""Integra a 324a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('write_transfer_finalised_news_text', 9283824, [106, 255, 104, 193, 68, 150, 0]), ('write_contract_response_news_text', 9278592, [106, 255, 104, 107, 68, 150, 0]), ('write_transfer_completed_news_text', 9290608, [106, 255, 104, 156, 69, 150, 0]), ('write_offer_made_news_text', 9254176, [106, 255, 104, 18, 67, 150, 0]), ('newsitem_write_text', 7847040, [106, 255, 100, 161, 0, 0, 0, 0]), ('update_records_for_match', 4475184, [85, 139, 236, 131, 228, 248]), ('write_ban_news_text', 5600992, [106, 255, 100, 161, 0, 0, 0, 0]), ('comp_util_write_cup_tie_result_news', 4918144, [106, 255, 104, 11, 105, 149, 0]), ('write_record_news_text', 4507216, [106, 255, 104, 128, 86, 149, 0]), ('weekly_board_confidence_update', 6913088, [85, 139, 236, 131, 228, 248]), ('cup_process_fixture', 5349712, [106, 255, 104, 137, 118, 149, 0]), ('construct_player_stats', 8125776, [106, 255, 104, 124, 246, 149, 0]), ('run_scout_day', 8076832, [85, 139, 236, 131, 228, 248])]
BASE = 3774
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
