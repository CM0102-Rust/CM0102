# Mutacoes (mwf_17), na virada italiana de meados de 2002: o ano da Serie A sobe 2 (as datas de 2002/03 mudam de ano;
# acusada pelo estado no fim da carreira, 2002-08-01). Sem a mutacao do reagrupamento: sem ele a C1 A ficaria com mais
# clubes do que a tabela dela (a caixa de erro).
MUTACOES = [
    ('mwf17_serie_a_ano_mais_2',
     'this.set_year(a.wrapping_add(1)); // mwf17: the year of Serie A',
     'this.set_year(a.wrapping_add(2)); // mutation: the year plus 2',
     'mwf_17 serie a rollover'),
]
