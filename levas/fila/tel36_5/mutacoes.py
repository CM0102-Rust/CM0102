# Mutations (tel36_5). The automatic career opens no screen; these prove the comparer in a session of Lucas's (the
# screen: the news item about the new transfer regulations > "Information", or the rules link of the transfer screens).
# - the text area starts one pixel lower;
# - the Young Players tab takes the Contract Protection text.
MUTACOES = [
    ('tel36_5_text_top', 'tel26_text_area([0x6e, top as u32, 0x30c,', 'tel26_text_area([0x6e, top as u32 + 1, 0x30c,'),
    ('tel36_5_young_players', '3 => Some(0xa64e30),', '3 => Some(0xa64c24),'),
]
