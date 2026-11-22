# Mutations (tel36_6). The automatic career opens no screen; these prove the comparers in a session of Lucas's (the
# news screen of a multiplayer game > a manager's message > "Reply", or the menu "Send Message"; get_date runs in the
# news screen lists).
# - get_date copies the leap-year dword from the day dword;
# - the message text entry takes 99 characters.
MUTACOES = [
    ('tel36_6_date_leap', 'write_u32(out + 4, read_u32(item + NEWS_ITEM_DATE + 4));', 'write_u32(out + 4, read_u32(item + NEWS_ITEM_DATE));'),
    ('tel36_6_entry_limit', 'job_colour(JOB_COLOUR_3184), message as u32, 0x64, 6, 0, u32::MAX]);', 'job_colour(JOB_COLOUR_3184), message as u32, 0x63, 6, 0, u32::MAX]);'),
]
