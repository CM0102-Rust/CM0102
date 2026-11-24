# escapadas da 328a para a comparacao longa (bancada 4)
MUTACOES = [
    ('tel44_2_name_order', "exe::utils_string_compare(name_a.as_ptr(), name_b.as_ptr(), 1, 0) // tel44_2: a's name against b's", 'exe::utils_string_compare(name_b.as_ptr(), name_a.as_ptr(), 1, 0) // mutation: the reverse order'),
    ('tel44_2_score_order', '-1 // tel44_2: the higher score first', '1 // mutation: the lower score first'),
    ('tel44_2_votes_order', 'read_i16(b as usize + asl::ENTRY_VOTES) as i32 - read_i16(a as usize + asl::ENTRY_VOTES) as i32 // tel44_2: the most votes first', 'read_i16(a as usize + asl::ENTRY_VOTES) as i32 - read_i16(b as usize + asl::ENTRY_VOTES) as i32 // mutation: the fewest votes first'),
    ('tel44_3_votes_cleared', 'write_i16(this + AWARD_SHORTLIST_VOTES + 2 * k, -1); // tel44_3: no manager has voted', 'write_i16(this + AWARD_SHORTLIST_VOTES + 2 * k, 0); // mutation: the votes at 0'),
]
