# Mutacoes (mwa_16), na Copa da Liga da Irlanda, carreira ENG IRL NIR ate 2002-08-01:
# - o construtor monta a copa do ano seguinte;
# - o primeiro grupo com dois clubes em vez de tres (um inscrito fica de fora; os grupos seguintes mudam de clubes);
# - dois pontos por vitoria nos grupos (outros classificados).
MUTACOES = [
    ('mwa16_copa_da_liga_ano_seguinte',
     'construct_irish_league_cup(League(this), year, record, &IRISH_LEAGUE_CUP_CONSTRUCTOR, "mwa_16 irish league cup new")',
     'construct_irish_league_cup(League(this), year + 1, record, &IRISH_LEAGUE_CUP_CONSTRUCTOR, "mwa_16 irish league cup new")',
     'mwa_16 irish league cup new'),
    ('mwa16_primeiro_grupo_de_dois',
     'create_first_group_table(League(this), 3, 0x9e9f4c, 0x172, "mwa_16 irish league cup first group")',
     'create_first_group_table(League(this), 2, 0x9e9f4c, 0x172, "mwa_16 irish league cup first group")',
     'mwa_16 irish league cup first group'),
    ('mwa16_dois_pontos_por_vitoria',
     'GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [1, 2, 4, 0], slots: 1, prize: Some(0x186a0), bits: None }; // mwa16: one through per group',
     'GroupStageRules { win: 2, draw: 1, category: 4, tiebreak: [1, 2, 4, 0], slots: 1, prize: Some(0x186a0), bits: None }; // mutation: two points',
     'mwa_16 irish league cup rules'),
]
