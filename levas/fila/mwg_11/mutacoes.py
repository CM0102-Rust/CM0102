# Mutacoes (mwg_11):
# - a sede da Oceania fica sempre na Australia (o comparador lado a lado da alternancia pega na virada de 2002).
# As viradas (Interamericana, Oceania) ficam sem mutacao propria: o comparador so roda o original e a edicao seguinte
# comeca depois da data da cadeia; o molde da virada de copa (mwf_1) e o da Copa America (cqs_12) ja estao provados.
MUTACOES = [
    ('mwg11_sede_sempre_australia',
     "let host = if read_u32(e + host_entry::HOST) == australia { read_u32(KEY_NATION_NEW_ZEALAND) } else { australia }; // mwg11: Australia, then New Zealand",
     "let host = australia; // mutation: always Australia",
     'mwg_11 oceania host alternated'),
]
