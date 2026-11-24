# Mutacoes (cx08_4), com a Argentina ate 2002-08-01:
# - o resultado vai de novo para a fase 0 (o Clausura) em vez da fase geral: as tabelas das fases mudam (a carreira);
# - os rebaixados por promedio ficam com o status 4 (o comparador do fim da temporada acusa);
# - a virada passa o ano em 2 (o comparador da virada acusa).
MUTACOES = [
    ('cx08_4_resultado_na_fase_0',
     '    fixture.set_stage_index(1); // cx08_4: the result also goes to the overall stage',
     '    fixture.set_stage_index(0); // mutation',
     'cx08_4 argentine result in both tables'),
    ('cx08_4_rebaixado_status_4',
     '                if read_i32(row.club() as usize) == id {\n                    row.set_status(3);',
     '                if read_i32(row.club() as usize) == id {\n                    row.set_status(4); // mutation',
     'cx08_4 argentine relegation by averages'),
    ('cx08_4_ano_mais_dois',
     'this.set_year(this.year().wrapping_add(1)); // cx08_4: the Argentine season moves on a year',
     'this.set_year(this.year().wrapping_add(2)); // mutation',
     'cx08_4 argentine premier turnover'),
]
