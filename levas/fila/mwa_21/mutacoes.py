# Mutacoes (mwa_21), na Supercopa da Irlanda, carreira ENG IRL NIR ate 2002-08-01 (se a edicao de 2002 for jogada no
# fim de junho: depende de quando a classificacao da UEFA vira):
# - o construtor monta a copa do ano seguinte;
# - a categoria das partidas (4, de copa nacional, em vez de 2, de liga: troca de 14/11/2026, provada na
#   bancada 5; os pontos por vitoria nao chegam ao diario da carreira, ver levas/296/trocas_prova.txt);
# - a primeira rodada uma semana depois (27/6 -> 4/7).
MUTACOES = [
    ('mwa21_supercopa_ano_seguinte',
     'construct_irish_super_cup(League(this), year, record) // mwa21: the Irish Super Cup',
     'construct_irish_super_cup(League(this), year + 1, record) // mutation: the next year',
     'mwa_21 irish super cup new'),
    ('mwa21_categoria_de_copa',
     'GroupStageRules { win: 3, draw: 1, category: 2, tiebreak: [4, 1, 2, 3], slots: 0, prize: None, bits: None }; // mwa21: the Super Cup tie-breaks',
     'GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [4, 1, 2, 3], slots: 0, prize: None, bits: None }; // mutation: a domestic-cup category',
     'mwa_21 irish super cup rules'),
    ('mwa21_primeira_rodada_semana_depois',
     '(27, 5, 0, 2, 2, &[(-1, -1, -1)]), // mwa21: the Super Cup opens on 27 June',
     '(4, 6, 0, 2, 2, &[(-1, -1, -1)]), // mutation: a week later',
     'mwa_21 irish super cup schedule'),
]
