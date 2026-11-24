# Mutacoes (sx18_5), acusadas pelo comparador de cada rotina (as vagas so voltam ao texto). O registro de periodo nao
# rodou na carreira do mundo ate 2002-08-01: as duas dele podem escapar (so o comparador as ve, quando rodarem).
# - a referencia lida com o segundo nome no lugar do primeiro;
# - o periodo gravado sem o valor;
# - o fim do periodo lido das vagas do inicio.
MUTACOES = [
    ('sx18_5_primeiro_nome',
     'write_u32(r, v[0]); // sx18_5: the first name id',
     'write_u32(r, v[1]); // mutation',
     'sx18_5 name reference from news'),
    ('sx18_5_periodo_sem_valor',
     '    [read_u32(r), d0, y0, d1, y1]',
     '    [0, d0, y0, d1, y1] // mutation',
     'sx18_5 span record to news'),
    ('sx18_5_fim_trocado',
     'news_slot_date(r + run_record::END_DATE, d1, y1); // sx18_5: the end date',
     'news_slot_date(r + run_record::END_DATE, d0, y0); // mutation',
     'sx18_5 span record from news'),
]
