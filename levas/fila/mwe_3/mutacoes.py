# Mutations (mwe_3), in the rules set when the leagues are created (and again at each turnover): the Norwegian
# Premier Division relegating 3 instead of 2 and the First Division promoting 1 instead of 2 (who goes up and down
# at the end of the 2001 season, in November); the Allsvenskan with 2 points for a win (the table order changes in
# the 2001 autumn).
MUTACOES = [
    ('mwe3_tippeligaen_rebaixa_3',
     'v.push((league::RELEGATED_COUNT, 1, 2)); // mwe3: the Norwegian Premier relegates 2',
     'v.push((league::RELEGATED_COUNT, 1, 3)); // mutation: 3 relegated',
     'mwe_3 norwegian premier rules'),
    ('mwe3_primeira_norueguesa_sobe_1',
     'v.push((league::PROMOTED_COUNT, 1, 2)); // mwe3: the Norwegian First Division promotes 2',
     'v.push((league::PROMOTED_COUNT, 1, 1)); // mutation: 1 promoted',
     'mwe_3 norwegian first division rules'),
    ('mwe3_allsvenskan_2_pontos',
     'v.push((league::WIN_POINTS, 1, 3)); // mwe3: the Allsvenskan gives 3 points for a win',
     'v.push((league::WIN_POINTS, 1, 2)); // mutation: 2 points for a win',
     'mwe_3 swedish premier rules'),
]
