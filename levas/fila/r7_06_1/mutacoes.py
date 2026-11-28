# Mutacoes (r7_06_1). As tres rotinas nunca rodam no jogo como ele e (a campanha de 2002 e codigo morto): nenhuma
# carreira as acusa. Acusadas na emulacao (rascunho r7_06\check_r7_06.py: a original e a copia da DLL compilada com a
# mutacao, rastros comparados), ver PRONTO.txt:
# - a fatia do dia trocada (fim de semana 2, meio de semana 1);
# - o placar de Colombia x Brasil (29/3/2000) 1 x 0 em vez de 0 x 0;
# - a tabela ordenada depois de um jogo europeu sempre a da secao, nunca a do grupo.
MUTACOES = [
    ('r7_06_1_fatia_trocada',
     'r.set_time_slice(if weekend { 1 } else { 2 }); // r7_06_1: weekend slice',
     'r.set_time_slice(if weekend { 2 } else { 1 }); // mutation: slices swapped',
     'r7_06_1 south american fixed fixture'),
    ('r7_06_1_colombia_brasil',
     '(28, 2, 2000, KEY_NATION_COLOMBIA, 0, KEY_NATION_BRAZIL, 0), // r7_06_1: Colombia-Brazil',
     '(28, 2, 2000, KEY_NATION_COLOMBIA, 1, KEY_NATION_BRAZIL, 0), // mutation: Colombia-Brazil 1-0',
     'r7_06_1 south american 2002 campaign'),
    ('r7_06_1_tabela_da_secao',
     'exe::league_sort_table(if stage == -1 { this } else { stage_object(stage) }, 0usize); // r7_06_1: the group table sorted',
     'exe::league_sort_table(this, 0usize); // mutation: always the section table',
     'r7_06_1 european fixed match'),
]
