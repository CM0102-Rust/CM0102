# Mutations (tel06_2): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat (each one must show as a divergence):
# - the related clubs cut to 30 entries instead of 28: caught side by side by the squad menu comparer on the club
#   screen of a club whose division has more than about 24 clubs (Squad menu button);
# - the part exchange sort by column 7 does not reverse: caught side by side on the part exchange screen (a bid with
#   a player in exchange) clicking the same sort column twice.
MUTACOES = [
    ('tel06_2_corte_do_menu',
     '        let mut limit = 0x1c - count;',
     '        let mut limit = 0x1e - count;',
     'tel06_2 squad menu related clubs'),
    ('tel06_2_ordem_invertida',
     '            set_screen_param(5, (part_exchange_sort(p5, 0x400, v as u8 == p7) | 0x400) as usize);',
     '            set_screen_param(5, (part_exchange_sort(p5, 0x400, false) | 0x400) as usize);',
     'tel06_2 part exchange sort 7'),
]
