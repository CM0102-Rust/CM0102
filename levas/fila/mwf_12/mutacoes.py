# Mutacoes (mwf_12): o vice da grande final gravado como campeao da NSL (muda o historico, que o campeao anterior e a
# Liga dos Campeoes da Oceania leem; o comparador lado a lado do historico a pega); o ano da virada da NSL sobe 2 (as
# datas da nova temporada mudam de ano; acusada pelo estado no fim da carreira, 2002-08-01).
MUTACOES = [
    ('mwf12_vice_como_campeao',
     'exe::comp_util_add_history_record(this, winner, runner_up, read_usize(this.table()), 0) // mwf12: the NSL champion, runner-up and leader',
     'exe::comp_util_add_history_record(this, runner_up, winner, read_usize(this.table()), 0) // mutation: the runner-up as champion',
     'mwf_12 nsl honours'),
    ('mwf12_nsl_ano_mais_2',
     'this.set_year(a.wrapping_add(1)); // mwf12: the year of the NSL',
     'this.set_year(a.wrapping_add(2)); // mutation: the year plus 2',
     'mwf_12 nsl rollover'),
]
