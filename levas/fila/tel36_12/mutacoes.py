# Mutations (tel36_12). fix_division_club_count only runs in a new game with Greece loaded (the world career, ATE
# 2002-08-01, or Lucas creating a game with Greece in JOGAR_COMPARANDO.bat); the comparer catches these:
# - one club more is kept in the division;
# - the fill-up ignores the reputation test.
MUTACOES = [
    ('tel36_12_keep_one_more', 'if found < expected {\n                found += 1;', 'if found <= expected {\n                found += 1;'),
    ('tel36_12_fill_reputation', '|| read_i16(division + competition::REPUTATION) < read_i16(comp_at(comp_id as u32) + competition::REPUTATION)',
     '|| read_i16(division + competition::REPUTATION) <= read_i16(comp_at(comp_id as u32) + competition::REPUTATION)'),
]
