# Mutações da 346a leva (itens sx05b_18, sx05b_19, sx04c_14). Uso: python mutacoes.py mutacoes\leva346.py --carreira 2002-06-01
MUTACOES = [
    ('sx05b_18_jovem_sem_idade', '(2, 0x15, 0x11)] {', '(2, -1, 0x11)] { // mutation', 'sx05b_18 welsh awards day'),
    ('sx05b_18_setembro', '2000, 1000, premier, 7, year, 3, year + 1, f, 0]),', '2000, 1000, premier, 8, year, 3, year + 1, f, 0]), // mutation', 'sx05b_18 welsh awards built'),
    ('sx05b_18_abril', '    if awards_day_is(today, 0xb, 4) {', '    if awards_day_is(today, 0xb, 3) { // mutation'),
    ('sx05b_19_tecnico_ingles', '    award_month_manager(nation_award(this, 5), KEY_CLUB_COMP_WELSH_PREMIER_DIVISION);', '    award_month_manager(nation_award(this, 5), KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION); // mutation', 'sx05b_19 welsh manager of the month'),
    ('sx05b_19_sem_semana', '    nation_awards_team_of_the_week_of(this, award, &[0]);', '    nation_awards_team_of_the_week_of(this, award, &[]); // mutation'),
    ('sx04c_14_jovem_sem_idade', '(2, 0x15, 0x11), (1, -1, 2)] {', '(2, -1, 0x11), (1, -1, 2)] { // mutation', 'sx04c_14 scottish awards day'),
    ('sx04c_14_setembro', 'comp, 7, y, 4, y + 1, f, players]);', 'comp, 8, y, 4, y + 1, f, players]); // mutation', 'sx04c_14 scottish awards built'),
    ('sx04c_14_junho', 'make_date_dword(0x17, 4, read_u16(today + cm_date::YEAR) as u32, 6)', 'make_date_dword(0x17, 5, read_u16(today + cm_date::YEAR) as u32, 6) /* mutation */'),
]
