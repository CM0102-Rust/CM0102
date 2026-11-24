# Mutacoes (sbE_9), com a Polonia ate 2002-08-01 (os construtores na criacao do jogo):
# - a regra de disciplina inglesa na Primeira polonesa (outras suspensoes, outros escalados: a carreira);
# - a regra de disciplina inglesa na Segunda polonesa (idem);
# - sem as posicoes previstas da Primeira (as expectativas da diretoria e a ordem prevista: a carreira; o comparador do
#   construtor so roda a original).
MUTACOES = [
    ('sbE_9_disciplina_da_primeira',
     'discipline: discipline_rule::POLAND,\n    stages: 2,',
     'discipline: discipline_rule::ENGLAND, // mutation\n    stages: 2,',
     'sbE_9 new polish first division'),
    ('sbE_9_disciplina_da_segunda',
     'discipline: discipline_rule::POLAND,\n    stages: 1,',
     'discipline: discipline_rule::ENGLAND, // mutation\n    stages: 1,',
     'sbE_9 polish second division'),
    ('sbE_9_sem_previstas',
     '        call_on_league(0x7c9900, l);\n        pol_first_set_predicted_positions(this);',
     '        call_on_league(0x7c9900, l); // mutation: no predicted positions',
     'sbE_9 new polish first division'),
]
