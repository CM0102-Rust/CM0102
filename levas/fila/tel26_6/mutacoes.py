# Mutations (tel26_6): the screens batch runs no mutations in the chain (the automatic career opens no screen); these
# are for a session of Lucas with JOGAR_COMPARANDO.bat: any screen with the team menu bar (e.g. the squad screen).
# - "Web Sites" in Game Options with id 0x42d: the item's id differs in the display list.
# - "FIFA Rankings" in the World Cup / International competitions submenu with link 0x3f4.
MUTACOES = [
    ('tel26_6_web_sites_id',
     '(0x9b53b4, 0x42c)',
     '(0x9b53b4, 0x42d)',
     'tel26_6 team menu'),
    ('tel26_6_fifa_rankings_link',
     'MENUBAR_TEXT, 0x3f3, 0);',
     'MENUBAR_TEXT, 0x3f4, 0); // tel26_6 mutation',
     'tel26_6 competitions menu'),
]
