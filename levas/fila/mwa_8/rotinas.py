# The constructors of the Irish President's Cup, the Northern Irish Charity Shield and the Northern Irish Cup
# (ire_pres_cup.cpp, nir_charity.cpp, nir_cup.cpp, in cups\). Whole-world front, round 2 (reader mwa, 12/11/2026),
# career with ENG IRL NIR to 2002-08-01: the constructors run once, when the career is created (2001).
# Uses mwa_7's construct_irish_cup and compare_irish_cup_constructor: it goes in the same batch or after it.
# - 0x639280 (IrePresCup::IrePresCup): FraSuper::FraSuper's template (0x5bf320, same.py: only the vtable, the
#   discipline rule, the entrant routine and the file).
# - 0x789df0 (NirCharity::NirCharity): SpaSuper::SpaSuper's template (0x8586b0: the same steps, the year and the record
#   written in another order, and the bench 3/3).
# - 0x78a5b0 (NirCup::NirCup): JapEmpCup's / FraLgeCup's template (the prize fund 0 written from ebx).
# Conventions checked at the call sites: thiscall, ret 8 (see mwa_7). Inside: the same calls as mwa_7's.
# Comparers: only the original, with the plan.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_ire_pres_cup', 0x639280, [0x6a, 0xff, 0x68, 0x59, 0xb5, 0x95, 0x00]),
           ('construct_nir_charity', 0x789df0, [0x6a, 0xff, 0x68, 0x09, 0xe3, 0x95, 0x00]),
           ('construct_nir_cup', 0x78a5b0, [0x6a, 0xff, 0x68, 0x59, 0xe3, 0x95, 0x00])]
DOCS = ('os construtores da Copa do Presidente (Irlanda), da Supercopa e da Copa da Irlanda do Norte',
        "the constructors of the Irish President's Cup, the Northern Irish Charity Shield and the Northern Irish Cup",
        [('construct_ire_pres_cup', 'IrePresCup::IrePresCup', '0x639280', 'ire_pres_cup.cpp', 'thiscall, ret 8',
          'regra: o construtor da Copa do Presidente: copa de clubes de uma nação, do ano do jogo, sem fundo de prêmios próprio e com o banco da base, regra de disciplina 16 (Irlanda); sem nada salvo, em ordem: os dois clubes (a caixa se devolver negativo), a rodada, a árvore dos confrontos (a caixa se devolver negativo) e as estatísticas (sem classificação final)',
          "rule: the constructor of the President's Cup: a cup of one nation's clubs of the game's year, with no prize fund of its own and the base's bench, discipline rule 16 (Ireland); with nothing saved, in order: the two clubs (the error box if it returns a negative value), the round, the tie tree (the error box if it returns a negative value) and the statistics (no final ranking)"),
         ('construct_nir_charity', 'NirCharity::NirCharity', '0x789df0', 'nir_charity.cpp', 'thiscall, ret 8',
          'regra: o construtor da Supercopa da Irlanda do Norte: copa de clubes de uma nação, do ano do jogo, sem fundo de prêmios próprio, 3 reservas no banco e 3 substituições, regra de disciplina 36 (Irlanda do Norte); sem nada salvo, em ordem: os dois clubes (a caixa se devolver negativo), a rodada, a árvore dos confrontos (a caixa se devolver negativo) e as estatísticas (sem classificação final)',
          "rule: the constructor of the Northern Irish Charity Shield: a cup of one nation's clubs of the game's year, with no prize fund of its own, 3 substitutes on the bench and 3 substitutions, discipline rule 36 (Northern Ireland); with nothing saved, in order: the two clubs (the error box if it returns a negative value), the round, the tie tree (the error box if it returns a negative value) and the statistics (no final ranking)"),
         ('construct_nir_cup', 'NirCup::NirCup', '0x78a5b0', 'nir_cup.cpp', 'thiscall, ret 8',
          'regra: o construtor da Copa da Irlanda do Norte: copa de clubes de uma nação, do ano do jogo, com fundo de prêmios 0 (as rodadas pagam prêmios fixos), 3 reservas no banco e 3 substituições, regra de disciplina 36 (Irlanda do Norte); sem nada salvo, em ordem: os 32 inscritos (a caixa se devolver 0), as rodadas, a árvore dos confrontos (a caixa se devolver 0), as estatísticas e a classificação final',
          "rule: the constructor of the Northern Irish Cup: a cup of one nation's clubs of the game's year, with a prize fund of 0 (the rounds pay fixed prizes), 3 substitutes on the bench and 3 substitutions, discipline rule 36 (Northern Ireland); with nothing saved, in order: the 32 entrants (the error box if it returns 0), the rounds, the tie tree (the error box if it returns 0), the statistics and the final ranking")])
CAMPOS = []
INTERESSA = []
