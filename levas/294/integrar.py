"""Integra a 294a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('player_regen_write_retirement_announcement_text', 8037568, [139, 68, 36, 8, 129, 236, 164, 1, 0, 0]), ('game_season_update_pass', 6118112, [100, 161, 0, 0, 0, 0]), ('write_player_media_story_news_text', 7524976, [106, 255, 100, 161, 0, 0, 0, 0]), ('write_player_praise_news_text', 7622528, [100, 161, 0, 0, 0, 0]), ('write_transfer_rumour_news_text', 7690336, [106, 255, 104, 219, 216, 149, 0]), ('post_team_media_story', 7647328, [106, 255, 104, 131, 216, 149, 0]), ('apply_player_story_response', 7606304, [131, 236, 16, 83, 85]), ('get_action_button', 7875440, [100, 161, 0, 0, 0, 0])]
BASE = 2316
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
