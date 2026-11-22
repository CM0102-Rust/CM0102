# Mutation (tel35_6). The comparer runs only the original (the links open screens): this one is seen in Rust mode only
# (JOGAR.bat without comparing): a reserves link (a club's "Reserves" in a list) opens the first team's squad.
MUTACOES = [
    ('tel35_6_reserves_link', 'link_open_squad(tel26_item_value(item), 1);', 'link_open_squad(tel26_item_value(item), 0);', 'tel35_6 reserves link'),
]
