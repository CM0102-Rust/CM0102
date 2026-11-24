# Mutacoes (sx02b_1).
# - o construtor sem as permissoes de trabalho iniciais: os clubes ingleses deixam de esbarrar na permissao de trabalho
#   dos estrangeiros nas compras (carreira ENG; o comparador do construtor so roda a original);
# - a tabela das janelas sem as datas de fechamento: o comparador acusa a tabela reconstruida, e a carreira as janelas.
MUTACOES = [
    ('sx02b_1_sem_permissoes',
     'init_work_permits(this); // sx02b_1: the work permits of a new manager',
     '{} // mutation: no initial work permits',
     'sx02b_1 new transfer manager'),
    ('sx02b_1_sem_fechamento',
     'dates.push(window + 6); // sx02b_1: the closing date',
     '{} // mutation: without the closing dates',
     'sx02b_1 new transfer manager'),
]
