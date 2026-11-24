"""Integra a 329a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('comp_stats_record_match_result', 4846960, [106, 255, 104, 231, 102, 149, 0]), ('award_manager_process_fixture', 4273344, [83, 85, 86, 87, 139, 124, 36, 20]), ('award_manager_process_date', 4273472, [83, 85, 139, 108, 36, 12]), ('construct_award', 4271680, [139, 193, 86, 139, 76, 36, 16]), ('england_awards_daily_update', 5740048, [131, 236, 16, 86, 87]), ('england_awards_on_month_change', 5740800, [161, 188, 245, 156, 0]), ('england_awards_update_team_of_the_week', 5741296, [139, 81, 4, 139, 68, 36, 4]), ('construct_europe_rules', 5780096, [139, 68, 36, 20, 139, 84, 36, 12]), ('default_squad_restrictions', 5780160, [87, 139, 124, 36, 12]), ('europe_rules_get_ineligibility_code', 5780192, [83, 86, 139, 116, 36, 12]), ('international_awards_daily_update', 6499872, [131, 236, 16, 83, 86]), ('nation_awards_record_league_match_ratings', 5936816, [86, 87, 139, 124, 36, 12]), ('construct_denmark_rules', 5592160, [139, 68, 36, 20, 139, 84, 36, 12]), ('collect_player_news_recipients', 9248160, [106, 255, 104, 251, 64, 150, 0]), ('collect_offer_news_recipients', 9244976, [106, 255, 104, 226, 64, 150, 0]), ('cancel_offers_for_released_player', 9304496, [106, 255, 104, 92, 70, 150, 0]), ('post_contract_response_news', 9277072, [106, 255, 104, 75, 68, 150, 0]), ('get_past_transfers', 9224240, [106, 255, 104, 123, 62, 150, 0]), ('contract_manager_post_offer_response_news', 5065120, [106, 255, 104, 102, 110, 149, 0]), ('contract_manager_write_offer_response_news_text', 5066672, [106, 255, 100, 161, 0, 0, 0, 0]), ('cash_get_rounding_step', 4466400, [138, 68, 36, 12, 131, 236, 24]), ('cash_get_display_amount', 4453472, [219, 1, 83, 85, 220, 13, 184, 156, 173, 0]), ('cash_format_money', 4463632, [129, 236, 36, 2, 0, 0]), ('write_awol_news_text', 4299872, [139, 68, 36, 8, 129, 236, 168, 1, 0, 0]), ('write_missed_training_news_text', 4303072, [139, 68, 36, 8, 129, 236, 60, 3, 0, 0]), ('write_fitness_worry_news_text', 4304944, [184, 116, 27, 0, 0])]
BASE = 3797
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
