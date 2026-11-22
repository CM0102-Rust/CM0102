# Mutations (tel36_7). The automatic career opens no screen; these prove the comparers in a session of Lucas's (the
# screen: a person's profile > Action > "Add Note" / "Edit Note").
# - the text entry takes 253 characters;
# - a reason click keeps the old reason.
MUTACOES = [
    ('tel36_7_entry_limit', 'note_text as u32, 0xfe, 8, 0, second]);', 'note_text as u32, 0xfd, 8, 0, second]);'),
    ('tel36_7_reason_kept', 'set_screen_param(3, tel26_item_value(item) as u8 as i8 as isize as usize);', 'set_screen_param(3, get_screen_param(3));'),
]
