# Mutacoes (sx12b_3). Os tres rodam no inicio do programa (o hall da fama e as tabelas estaticas), tambem na carreira:
# acusadas pelos comparadores lado a lado na primeira chamada (as datas vazias nao voltam para a simulacao: a tabela
# das nacoes-chave e o hall da fama sao zerados e reescritos depois).
MUTACOES = [
    ('sx12b_data_da_entrada_deslocada',
     '    exe::date_date_54a110(this + hall_of_fame_consts::DATE); // sx12b_3: the entry date',
     '    exe::date_date_54a110(this + hall_of_fame_consts::DATE - 4); // mutation: the date four bytes early'),
    ('sx12b_sem_data_do_fim',
     '    exe::date_date_54a110(this + key_nation_record::END_DATE); // sx12b_3: the end date',
     '    let _ = key_nation_record::END_DATE; // mutation: no end date',
     'sx12b_3 key nation entry'),
]
