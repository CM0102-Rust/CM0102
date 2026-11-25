# Mutacoes (rx06_3), item de TELAS: a carreira automatica nao passa por esta rotina; servem para provar o comparador
# numa sessao do Lucas jogando comparando (um olheiro observando jogos e um pedido de observacao):
# - o relatorio geral (modo 2) com nota 3 em vez de 4 (o comparador acusa o registro do relatorio);
# - todos aceitam vir (o comparador acusa os resultados, o registro e as noticias de quem tem interesse abaixo de 3,15);
# - o boato todo dia (o comparador acusa o gerador de sorteios e os interesses da imprensa).
MUTACOES = [
    ('rx06_3_nota_geral',
     'return false; // rx06_3: a general report needs a 4',
     'if grades.0 < 3 && grades.1 < 3 { return false; } // mutation: a 3 is enough'),
    ('rx06_3_interesse',
     'return; // rx06_3: he would not come',
     '// mutation: every player would come'),
    ('rx06_3_boato',
     'return; // rx06_3: no rumour today',
     '// mutation: a rumour every day'),
]
