# Mutações da 328a leva (itens tel44_1, tel44_2, tel44_3). Uso: python mutacoes.py mutacoes\leva328.py --carreira 2002-06-01
MUTACOES = [
    ('tel44_1_club_link', "exe::text_add_link(out, self.name.as_ptr(), LINK_CLUB, self.club); // tel44_1: the club's link", "exe::text_add_link(out, self.name.as_ptr(), 0x7d6, self.club); // mutation: the club's link with another type", 'tel44_1 comp record news text'),
    ('tel44_1_team_gender', "CompRecordTeam { club, name, gender: record_gender(club) } // tel44_1: the team's gender", 'CompRecordTeam { club, name, gender: record_gender(club) ^ 1 } // mutation: the other gender', 'tel44_1 comp record news text'),
    ('tel44_2_name_order', "exe::utils_string_compare(name_a.as_ptr(), name_b.as_ptr(), 1, 0) // tel44_2: a's name against b's", 'exe::utils_string_compare(name_b.as_ptr(), name_a.as_ptr(), 1, 0) // mutation: the reverse order'),
    ('tel44_2_score_order', '-1 // tel44_2: the higher score first', '1 // mutation: the lower score first'),
    ('tel44_2_votes_order', 'read_i16(b as usize + asl::ENTRY_VOTES) as i32 - read_i16(a as usize + asl::ENTRY_VOTES) as i32 // tel44_2: the most votes first', 'read_i16(a as usize + asl::ENTRY_VOTES) as i32 - read_i16(b as usize + asl::ENTRY_VOTES) as i32 // mutation: the fewest votes first'),
    ('tel44_3_votes_cleared', 'write_i16(this + AWARD_SHORTLIST_VOTES + 2 * k, -1); // tel44_3: no manager has voted', 'write_i16(this + AWARD_SHORTLIST_VOTES + 2 * k, 0); // mutation: the votes at 0'),
]
