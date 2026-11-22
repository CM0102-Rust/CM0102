# Mutations (tel36_9). The automatic career opens no screen; these prove the comparers in a session of Lucas's (a
# person's profile: the history tab's page buttons and filters, the attribute "-" / "+"; the menu "Compare Players"
# with fewer than two players chosen).
# - the history page can go down to 0;
# - the zoom "+" goes up to 4;
# - the compare dialog's Okay gets id 0x39.
MUTACOES = [
    ('tel36_9_history_page', 'set_screen_param(0xf, (page - 1).max(1) as usize);', 'set_screen_param(0xf, (page - 1).max(0) as usize);'),
    ('tel36_9_zoom_limit', 'if zoom < 4 {', 'if zoom < 5 {'),
    ('tel36_9_okay_id', 'JOB_TEXT as u32, 0x38, 0]);', 'JOB_TEXT as u32, 0x39, 0]);'),
]
