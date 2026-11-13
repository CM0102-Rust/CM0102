# Mutacoes (mwg_6), nos premios das nacoes:
# - find_nation_awards devolve os premios da primeira OUTRA nacao (o comparador pega na primeira consulta; na carreira,
#   premios e noticias de outra nacao).
# Os dois repasses ao arquivo de historico (add_year/team_award_record) nao tem mutacao: o historico so e lido pelas
# telas de historia, nada da carreira volta dele, e o comparador e so a original.
MUTACOES = [
    ('mwg6_premios_de_outra_nacao',
     '.find(|&e| e != 0 && read_u8(e + am::AWARDS_KIND) == 1 && read_usize(e + am::AWARDS_NATION) == nation) // mwg6',
     '.find(|&e| e != 0 && read_u8(e + am::AWARDS_KIND) == 1 && read_usize(e + am::AWARDS_NATION) != nation) // mutation: another nation',
     'mwg_6 nation awards'),
]
