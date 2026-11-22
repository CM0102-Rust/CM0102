# Mutations (tel36_5). The automatic career opens no screen; these prove the comparer in a session of Lucas's (the
# screen: the news item about the new transfer regulations > "Information", or the rules link of the transfer screens).
# - the tabs carry the values 2..5 (a tab click shows the next tab's text);
# - the Young Players tab takes the Contract Protection text.
MUTACOES = [
    ('tel36_5_tab_values', 'write_u32(item + TRANSFER_RULES_TAB_VALUE, k as u32 + 1);', 'write_u32(item + TRANSFER_RULES_TAB_VALUE, k as u32 + 2);'),
    ('tel36_5_young_players', '3 => Some(0xa64e30),', '3 => Some(0xa64c24),'),
]
