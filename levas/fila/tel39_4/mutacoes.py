# Mutacoes (tel39_4). O que estes construtores gravam nao muda a simulacao: acusadas pelos comparadores (lado a lado).
# O destruidor do par de times nao tem mutacao: os destruidores dos elementos sao um ret, nada muda (dito no PRONTO).
# - a data do registro construida 4 bytes antes (+0x10 em vez de +0x14);
# - a palavra da visao padrao trocada (0x154a20 em vez de 0x154a21).
MUTACOES = [
    ('tel39_4_data',
     'exe::date_date_54a110(this + RATING_RECORD_DATE);',
     'exe::date_date_54a110(this + RATING_RECORD_DATE - 4);',
     'tel39_4 rating record constructor'),
    ('tel39_4_visao',
     'write_u32(this + 8, 0x154a21);',
     'write_u32(this + 8, 0x154a20);',
     'tel39_4 squad view settings'),
]
