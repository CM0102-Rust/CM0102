# Mutacoes (mwa_21), na Supercopa da Irlanda, carreira ENG IRL NIR ate 2002-08-01 (se a edicao de 2002 for jogada no
# fim de junho: depende de quando a classificacao da UEFA vira):
# - o construtor monta a copa do ano seguinte;
# - 2 pontos por vitoria (outro campeao);
# - a primeira rodada uma semana depois (27/6 -> 4/7).
MUTACOES = [
    ('mwa21_supercopa_ano_seguinte',
     'construct_irish_super_cup(League(this), year, record) // mwa21: the Irish Super Cup',
     'construct_irish_super_cup(League(this), year + 1, record) // mutation: the next year',
     'mwa_21 irish super cup new'),
    ('mwa21_dois_pontos_por_vitoria',
     'GroupStageRules { win: 3, draw: 1, category: 2, tiebreak: [4, 1, 2, 3], slots: 0, prize: None, bits: None }; // mwa21: the Super Cup tie-breaks',
     'GroupStageRules { win: 2, draw: 1, category: 2, tiebreak: [4, 1, 2, 3], slots: 0, prize: None, bits: None }; // mutation: two points',
     'mwa_21 irish super cup rules'),
    ('mwa21_primeira_rodada_semana_depois',
     '(27, 5, 0, 2, 2, &[(-1, -1, -1)]), // mwa21: the Super Cup opens on 27 June',
     '(4, 6, 0, 2, 2, &[(-1, -1, -1)]), // mutation: a week later',
     'mwa_21 irish super cup schedule'),
]
