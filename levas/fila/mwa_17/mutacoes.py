# Mutacoes (mwa_17), na Copa da Liga da Irlanda do Norte, carreira ENG IRL NIR ate 2002-08-01:
# - o construtor monta a copa do ano seguinte;
# - o primeiro grupo com quatro clubes em vez de cinco (um inscrito fica de fora; os grupos seguintes mudam de clubes);
# - dois pontos por vitoria nos grupos (outros classificados).
MUTACOES = [
    ('mwa17_copa_da_liga_ano_seguinte',
     'construct_irish_league_cup(League(this), year, record, &NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR, "mwa_17 northern irish league cup new")',
     'construct_irish_league_cup(League(this), year + 1, record, &NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR, "mwa_17 northern irish league cup new")',
     'mwa_17 northern irish league cup new'),
    ('mwa17_primeiro_grupo_de_quatro',
     'create_first_group_table(League(this), 5, 0xa69698, 0x17a, "mwa_17 northern irish league cup first group")',
     'create_first_group_table(League(this), 4, 0xa69698, 0x17a, "mwa_17 northern irish league cup first group")',
     'mwa_17 northern irish league cup first group'),
    ('mwa17_dois_pontos_por_vitoria',
     'GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [1, 2, 4, 0], slots: 2, prize: Some(0x186a0), bits: None }; // mwa17: two through per group',
     'GroupStageRules { win: 2, draw: 1, category: 4, tiebreak: [1, 2, 4, 0], slots: 2, prize: Some(0x186a0), bits: None }; // mutation: two points',
     'mwa_17 northern irish league cup rules'),
]
