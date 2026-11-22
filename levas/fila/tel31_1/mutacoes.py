# Mutations (tel31_1). The chain runs none for screen items; these prove the comparer in a session of Lucas's
# (JOGAR_COMPARANDO.bat). Both routines also run in the simulation (the statistics of every match), so both are
# caught early in a comparing career:
# - a tie in titles takes the most titles record (update_most_titles_record, side by side);
# - a midfielder rated 20 as attacker counts as an attacker only above 20 (position group, side by side).
MUTACOES = [
    ('tel31_1_titles_tie_takes',
     'if record == 0xff || n > record as i8 as i16 {',
     'if record == 0xff || n >= record as i8 as i16 {'),
    ('tel31_1_position_attacker',
     'if p.attacker() < 20 || p.central() < p.left_side()',
     'if p.attacker() <= 20 || p.central() < p.left_side()'),
]
