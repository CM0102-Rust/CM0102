# cqh_5 empty (02/11/2026)

The item had `League::set_club_season_expectations` (0x68aa80), `League::relegate_club_to_division` (0x6831a0) and `League::process_promotion_relegation_chain` (0x689c20). None ran in the automatic career up to 2002-06-01 (the expectations belong to the start of the season; the relegation and the promotion chain, to its end), and all three mutations escaped.

- 0x68aa80 and 0x6831a0 went as passengers to `cqh_4` (the Champions League round schedule, whose mutations the career catches).
- 0x689c20 left (back to the candidate list).
- This item has no routines left; it can leave the order.
