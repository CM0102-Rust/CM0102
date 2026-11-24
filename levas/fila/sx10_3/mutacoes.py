# Mutations (sx10_3): TELAS item, not run by the chain (the test career has attribute masking off). Each is caught by
# the routine's comparer in a game with attribute masking on (JOGAR_COMPARANDO.bat), to prove the comparers:
# - the national team link of get_knowledge_level is lost (the national manager sees his players' stored level);
# - the minimum level of the same division drops to 1 (daily_update and the minimum level's comparers);
# - the division players known at the start must be older than 20 (the initial level's comparer, at the map build).
MUTACOES = [
    ('sx10_3_national_team',
     'if !person.national_job().is_null() && person.national_job() == manager.national_job() { // sx10_3: same national team',
     'if false { // mutation: the national team link lost'),
    ('sx10_3_division_floor',
     'return 2; // sx10_3: the division floor',
     'return 1; // mutation: a lower division floor'),
    ('sx10_3_division_age',
     'if same_division && (person.age() as i8) > 0x13 { // sx10_3: older players of the division',
     'if same_division && (person.age() as i8) > 0x14 { // mutation: one year older'),
]
