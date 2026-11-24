# Mutacoes (sx02_4). O comparador confere a conta e os registros (a copia num vetor proprio): as transferencias passadas
# existem desde a janela de julho de 2001 e as telas/noticias que as pedem rodam 227 vezes ate 2002-06-01.
# - os registros do primeiro mes deixados de fora (> em vez de >=);
# - os registros da memoria deixados de fora (so os arquivos anuais).
MUTACOES = [
    ('sx02_4_sem_o_primeiro_mes',
     'exe::date_get_month(r + past_transfer::DATE) as i8 >= start.0 as i8;',
     'exe::date_get_month(r + past_transfer::DATE) as i8 > start.0 as i8; // mutation: without the first month',
     'sx02_4 yearly file'),
    ('sx02_4_sem_a_memoria',
     'append_past_transfer(list, count, &mut n, r, 0x291f); // sx02_4: a record in memory',
     '{} // mutation: the records in memory left out',
     'sx02_4 yearly file'),
]
