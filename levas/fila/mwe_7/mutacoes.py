# Mutations (mwe_7), in the tables built at the creation of the new game (and at the turnover): the first and last
# clubs of the Superettan and of the Ykkonen North group swapped in the table (the fixtures are drawn by the row
# order: who plays whom on each date changes in the 2001 autumn); the Norwegian First Division taken as 14 clubs (the
# database has 16: the last two are left out of the league).
MUTACOES = [
    ('mwe7_superettan_primeiro_e_ultimo_trocados',
     'fill_first_division_rows(this, |c| club_in_division(c, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION)) as u32 // mwe7: the Superettan clubs',
     '{ let r = fill_first_division_rows(this, |c| club_in_division(c, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION)); let t = this.table(); let (a, b) = (read_usize(t), read_usize(t + 15 * league_table_row::SIZE)); write_usize(t, b); write_usize(t + 15 * league_table_row::SIZE, a); r as u32 } // mutation: rows 0 and 15 swapped',
     'mwe_7 superettan table'),
    ('mwe7_grupo_norte_primeiro_e_ultimo_trocados',
     'fill_first_division_rows(this, |c| club_in_finnish_north_group(c)) as u32 // mwe7: the North group clubs',
     '{ let r = fill_first_division_rows(this, |c| club_in_finnish_north_group(c)); let t = this.table(); let (a, b) = (read_usize(t), read_usize(t + 9 * league_table_row::SIZE)); write_usize(t, b); write_usize(t + 9 * league_table_row::SIZE, a); r as u32 } // mutation: rows 0 and 9 swapped',
     'mwe_7 ykkonen north table'),
    ('mwe7_primeira_norueguesa_com_14',
     'if norwegian_first_division_count() == 0xe { // mwe7: the 14-club Norwegian First Division',
     'if norwegian_first_division_count() != 0xe { // mutation: 14 clubs',
     'mwe_7 norwegian first division table'),
]
