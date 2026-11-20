# Mutações da 314a leva (itens mwg_9, mwg_10, mwg_12, mwg_13, mwg_14, mwg_15, mwg_16, mwg_17). Uso: python mutacoes.py mutacoes\leva314.py --carreira 2002-06-01
MUTACOES = [
    ('mwg9_semeadura_parada', "exe::uefaseeding_update_season(seeding, 0); // mwg9: the seeding's new season", 'let _ = seeding; // mutation: the seeding stays in the old season', 'mwg_9 european cup rollover'),
    ('mwg10_intertoto_sem_primeira_rodada', 'exe::cup_set_up_round(this, 0) // mwg10: the first round set up', '0 // mutation: the first round not set up', 'mwg_10 intertoto rollover'),
    ('mwg14_oitavas_trocadas', '(0, 0), (1, 1), (3, 0), (2, 1), (4, 0), (5, 1), (7, 0), (6, 1), // mwg14: the group winners and runners-up paired', '(0, 0), (1, 1), (4, 0), (2, 1), (3, 0), (5, 1), (7, 0), (6, 1), // mutation: D1 and E1 swapped', 'mwg_14 world cup round of 16'),
    ('mwg15_eliminado_sem_texto', '0xfe => -2, // mwg15: the group exit has its text', '0xfe => return, // mutation: no text for the group exit', 'mwg_15 world cup group fate'),
    ('mwg15_corpo_trocado', 'write(object, item, text, body) // mwg15: headline or body', 'write(object, item, text, (body == 0) as u32) // mutation: headline and body swapped', 'mwg_15 award news'),
    ('mwg16_candidato_com_voto', 'write_u16(e + asl::ENTRY_VOTES, 0); // mwg16: a new candidate without votes', 'write_u16(e + asl::ENTRY_VOTES, 1); // mutation: a new candidate with one vote', 'mwg_16 shortlist candidate'),
    ('mwg16_lista_maior', "let capacity = read_i8(scope + 0xd) as i16; // mwg16: the shortlist's size from its scope", 'let capacity = read_i8(scope + 0xd) as i16 + 1; // mutation: one more candidate', 'mwg_16 shortlist built'),
]
