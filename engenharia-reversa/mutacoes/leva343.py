# Mutações da 343a leva (itens sx05b_12, sx05b_13, sx04b_8, sx04c_8). Uso: python mutacoes.py mutacoes\leva343.py --carreira 2002-06-01
MUTACOES = [
    ('sx05b_12_goleiro_defensor', '        for (k, mask) in [(3, 1), (4, 6), (5, 0x38), (6, 0x40)] {', '        for (k, mask) in [(3, 6), (4, 6), (5, 0x38), (6, 0x40)] { // mutation', 'sx05b_12 swedish awards day'),
    ('sx05b_12_reputacao_menor', '        year(1, KEY_STAFF_COMP_SWEDISH_PLAYER_OF_THE_YEAR, 5000, 2500, 1),', '        year(1, KEY_STAFF_COMP_SWEDISH_PLAYER_OF_THE_YEAR, 500, 2500, 1), // mutation', 'sx05b_12 swedish awards built'),
    ('sx05b_12_janeiro', '    if awards_day_is(today, 1, 0xb) {', '    if awards_day_is(today, 1, 0) { // mutation'),
    ('sx05b_13_so_premier', '    nation_awards_team_of_the_week_of(this, award, &[7, 9]);', '    nation_awards_team_of_the_week_of(this, award, &[7]); // mutation'),
    ('sx05b_13_sem_marca', '    write_u32(this + transfer_rules::SKIP_CONTRACT_TYPE_5_INELIGIBILITY, 1);\n    covered!("sx05b_13 swedish rules");', '    covered!("sx05b_13 swedish rules"); // mutation: no flag', 'sx05b_13 swedish rules'),
    ('sx04b_8_sem_peso', 'write_f32(c + STATS_CANDIDATE_SCORE, (weight as f64 * read_f32(c + STATS_CANDIDATE_SCORE) as f64) as f32); // sx04b_8: weighed by reputation', 'let _ = weight; // mutation: no reputation weight', 'sx04b_8 statistics candidate'),
    ('sx04b_8_cinco_jogos_a_mais', 'if (read_i16(c + STATS_CANDIDATE_GAMES) as i32) < least { // sx04b_8: enough games', 'if (read_i16(c + STATS_CANDIDATE_GAMES) as i32) < least + 5 { // mutation: five games more', 'sx04b_8 statistics candidate'),
    ('sx04b_8_ordem_invertida', 'if above { // sx04b_8: the higher score first', 'if below { // mutation: the lower score first', 'sx04b_8 scores ordered'),
    ('sx04c_8_jovem', '    award_month_player(this, nation_award(this, 7), -1);', '    award_month_player(this, nation_award(this, 7), 0x14); // mutation', 'sx04c_8 norwegian month awards'),
    ('sx04c_8_tecnico_ingles', '    award_month_manager(nation_award(this, 6), KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION);', '    award_month_manager(nation_award(this, 6), KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION); // mutation', 'sx04c_8 norwegian month awards'),
]
