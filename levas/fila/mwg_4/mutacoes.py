# Mutacoes (mwg_4), nas linhas dos tecnicos humanos (leituras: o comparador de cada rotina as pega; na carreira de
# ferias o humano existe, entao as contagens sao lidas):
# - a linha livre passa a ser a de classificacao 1 (a do humano em uso);
# - a contagem passa a valer para qualquer classificacao diferente de 0;
# - o tecnico em criacao passa a ser o de classificacao 1.
MUTACOES = [
    ('mwg4_linha_livre_errada',
     '.find(|s| s.classification() == 0).map_or(0, |s| s.addr()) // mwg4: a free row',
     '.find(|s| s.classification() == 1).map_or(0, |s| s.addr()) // mutation: the row in use',
     'mwg_4 free row'),
    ('mwg4_conta_qualquer',
     '.filter(|s| s.classification() == 1 && (network_only',
     '.filter(|s| s.classification() != 0 && (network_only',
     'mwg_4 count'),
    ('mwg4_em_criacao_errado',
     '(!s.is_null() && s.classification() == 5) as u32 // mwg4: being created',
     '(!s.is_null() && s.classification() == 1) as u32 // mutation: the manager in use',
     'mwg_4 being created'),
]
