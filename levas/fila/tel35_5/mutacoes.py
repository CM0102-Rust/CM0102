# Mutations (tel35_5). The automatic career opens no screen: the chain runs none. The comparers catch them in a
# session of Lucas: any screen (the menu bar builds the separators and the nations and continents lists of the
# Competitions and Nations & Clubs menus).
# - the separator's flags 0x1000011;
# - the chosen continent not ticked.
MUTACOES = [
    ('tel35_5_separator_flags', '0x1000010, tel35_colour(MENU_ENTRY_COLOUR)', '0x1000011, tel35_colour(MENU_ENTRY_COLOUR)', 'tel35_5 separator'),
    ('tel35_5_continent_tick', 'let kind = if continent == chosen { 0x2002 } else { 2 };', 'let kind = if continent == chosen { 2 } else { 2 };', 'tel35_5 continents'),
]
