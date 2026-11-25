# Mutacoes (rx06_2), item de TELAS: a carreira automatica nao passa por esta rotina; servem para provar o comparador
# numa sessao do Lucas jogando comparando (dar uma tarefa a um olheiro):
# - o desconto do veterano x0,75 em vez de x0,85 (o comparador acusa o valor atual e as notas de quem tem mais de 34);
# - o corte da lista em 40 + sorteio em vez de 50 (o comparador acusa a contagem quando a lista passa de 40);
# - todos aceitam vir (o comparador acusa a contagem: entram os que tem interesse abaixo de 3,15).
MUTACOES = [
    ('rx06_2_veterano',
     "worth = scout_scaled(worth, rdata::F64_0_85); // rx06_2: the veteran's discount",
     'worth = scout_scaled(worth, rdata::F64_0_75); // mutation: a bigger discount'),
    ('rx06_2_corte',
     'let keep = exe::utils_random(range) + 50; // rx06_2: how many the scout keeps',
     'let keep = exe::utils_random(range) + 40; // mutation: a shorter list'),
    ('rx06_2_interesse',
     'return; // rx06_2: he would not come',
     '// mutation: every player would come'),
]
