# Mutacoes (sbF_11), carreira ENG ate 2002-08-01. Nenhuma abre caixa.
# - o torneio recuado ate o dia da semana 4 e nao 5: a data do torneio muda (a carreira acusa nos amistosos de julho de
#   2002; o comparador roda so a original). Se a carreira ENG nao chamar a rotina antes da data, a mutacao escapa: ver
#   rotinas.py.
MUTACOES = [
    ('sbF_11_dia_da_semana_4',
     'while exe::date_get_weekday(pd) as i16 != 5 { // sbF11: back to the weekday 5',
     'while exe::date_get_weekday(pd) as i16 != 4 { // mutation',
     'sbF_11 ai tournament'),
]
