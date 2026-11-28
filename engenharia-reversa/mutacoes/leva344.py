# Mutações da 344a leva (itens sx05b_14, sx05b_15, sx04c_9, sx04c_10, sx04c_12). Uso: python mutacoes.py mutacoes\leva344.py --carreira 2002-06-01
MUTACOES = [
    ('sx05b_14_reputacao_menor', '        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_TURKISH_PLAYER_OF_THE_YEAR), 3000, 1500,', '        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_TURKISH_PLAYER_OF_THE_YEAR), 300, 1500, // mutation\n           ', 'sx05b_14 turkish awards built'),
    ('sx05b_14_sem_segunda', '        award_team_of_season(this, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A, nation_award(this, 5));', '        // mutation: no second division team of the year', 'sx05b_14 turkish awards day'),
    ('sx05b_14_abril', '    if awards_day_is(today, 0x1e, 4) {\n        covered!("sx05b_14 turkish awards day");', '    if awards_day_is(today, 0x1e, 3) { // mutation\n        covered!("sx05b_14 turkish awards day");'),
    ('sx05b_15_so_primeira', '    nation_awards_team_of_the_week_of(this, award, &[2, 4]);', '    nation_awards_team_of_the_week_of(this, award, &[2]); // mutation'),
    ('sx04c_9_jovem_sem_idade', 'nation_award(this, first + 2), [0x15, 0, 0x11, -1, -1, -1, -1]);', 'nation_award(this, first + 2), [-1, 0, 0x11, -1, -1, -1, -1]); // mutation', 'sx04c_9 polish awards day'),
    ('sx04c_9_reputacao_menor', '        year(1, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 3000, 0, d1, 1),', '        year(1, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 300, 0, d1, 1), // mutation', 'sx04c_9 polish awards built'),
    ('sx04c_9_junho', '    if awards_day_is(today, 0x1e, 4) {\n        covered!("sx04c_9 polish awards day");', '    if awards_day_is(today, 0x1e, 5) { // mutation\n        covered!("sx04c_9 polish awards day");'),
    ('sx04c_12_goleiro', '[-1, -1, -1, read_i32(KEY_NATION_RUSSIA), -1, 0, 0x11]);', '[-1, -1, -1, read_i32(KEY_NATION_RUSSIA), -1, 1, 0x11]); // mutation', 'sx04c_12 russian awards day'),
    ('sx04c_12_reputacao_menor', 'k(KEY_STAFF_COMP_RUSSIAN_PLAYER_OF_THE_YEAR), 4000, 2000,', 'k(KEY_STAFF_COMP_RUSSIAN_PLAYER_OF_THE_YEAR), 400, 2000, /* mutation */', 'sx04c_12 russian awards built'),
    ('sx04c_12_dezembro', '    if awards_day_is(today, 0x1e, 0xa) {', '    if awards_day_is(today, 0x1e, 0xb) { // mutation'),
]
