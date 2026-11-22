# Mutation (tel06_3), for a session of Lucas with JOGAR_COMPARANDO.bat: the tournament button always says "Invite
# Club": caught side by side by the action button comparer on the screen of a club already invited to the human's
# tournament.
MUTACOES = [
    ('tel06_3_sempre_convidar',
     '                    invited = true;',
     '                    invited = false;',
     'tel06_3 tournament button'),
]
