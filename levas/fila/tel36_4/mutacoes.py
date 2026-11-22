# Mutations (tel36_4). The automatic career opens no screen and the comparer runs only the original: the proof is Lucas
# playing in Rust mode (a player's profile > Action > "Make Offer"). The mutation shows on the bid screen: the
# "accepted" flag is never set.
MUTACOES = [
    ('tel36_4_accepted', 'set_screen_param(7, accepted as usize);', 'set_screen_param(7, 0);'),
]
