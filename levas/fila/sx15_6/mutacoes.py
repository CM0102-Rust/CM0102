# Mutacoes (sx15_6), pela carreira ENG ate 2002-06-01.
# - a remarcacao com mais de um convocado em vez de mais de dois: clubes da Premier com dois convocados nas datas FIFA
#   (set/out/nov de 2001) tem o jogo remarcado, e a tabela muda;
# - a selecao que sai da competicao mantem a competicao atual: a fila de competicoes da nacao (clear_current_competition)
#   e as convocacoes seguintes mudam [pode escapar: 1 chamada ate 2002-06-01; nesse caso fica para o comparador e a
#   auditoria].
MUTACOES = [
    ('sx15_6_dois_convocados',
     '        if away as i8 > 2 { // sx15_6: more than two players away',
     '        if away as i8 > 1 { // mutation: more than one player away',
     'sx15_6 club fixture moved'),
    ('sx15_6_competicao_mantida',
     '        write_i32(nation_record_by_id(this, nation.addr()) + national_squad_record::CURRENT_COMPETITION, -1); // sx15_6: no current competition',
     '        let _ = nation; // mutation: the current competition stays',
     'sx15_6 national team leaves the competition'),
]
