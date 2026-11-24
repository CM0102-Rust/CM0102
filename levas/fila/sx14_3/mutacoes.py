# Mutacoes (sx14_3), todas pela carreira ENG ate 2002-06-01 (o comparador so roda a original):
# - a confianca semanal no dia seguinte (dia do ano com resto 1): a diretoria de todos os clubes muda de semana, as
#   demissoes e as vagas mudam;
# - o boato da vaga sem a marca de feito: e postado de novo todo dia ate a chegada (sorteios a mais nas noticias);
# - o clube que perde o tecnico nao ganha expectativas novas: a confianca e as metas da diretoria dele ficam as antigas.
MUTACOES = [
    ('sx14_3_semanal_no_dia_seguinte',
     '    if read_i16(G_CURRENT_DATE) % 7 == 0 {',
     '    if read_i16(G_CURRENT_DATE) % 7 == 1 { // mutation',
     'sx14_3 appointment'),
    ('sx14_3_boato_repetido',
     '    write_u8(e + 1, 1);\n    let club = item.club();',
     '    let club = item.club(); // mutation: the rumour is not marked done',
     'sx14_3 rumour'),
    ('sx14_3_sem_expectativas_do_clube_deixado',
     '            set_season_expectations(this, from.addr(), 1);\n',
     '            // mutation: no new season expectations\n',
     'sx14_3 appointment'),
]
