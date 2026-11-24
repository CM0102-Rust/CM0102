# As rodadas das copas irlandesas e o teste de Leinster (nir_cup.cpp, ire_chal_cup.cpp, ire_pres_cup.cpp; leitor cx02,
# rodada 5, 24/11/2026). Competicoes da Irlanda e da Irlanda do Norte; ATE 2002-08-01 (as set_up_rounds rodam na virada
# das copas).
# - 0x78afe0 NirCup::set_up_rounds: premio 0 e a tabela.
# - 0x634c10 IreChalCup::set_up_rounds: premio 100.000 e a tabela; a virtual 0x8c tambem da CroCup (0x9692e0), da
#   IreLeinsterCup (0x96bd80) e da IreMunsterCup (0x96bed4).
#   As duas usam set_up_prize_rounds do cx02_4 (integrar depois dele). Prologo push esi; mov esi, ecx; push 0.
# - 0x6399c0 ire_club_is_in_leinster cdecl(clube) -> 0/1: chamada pelos inscritos da Copa do Presidente e da Copa da Liga
#   irlandesa (mwa_2, mwa_4); strncmp 0x946fa0 (3 pushes; add esp, 0xc -> cdecl) com "Dubl" e os strcmp em linha com
#   Bray, Longford, Dundalk, Drogheda, Athlone e Kilkenny, sobre o nome da cidade (+4) do estadio (+0x69 -> +0x38).
#   Prologo push ebx; push esi; mov esi, [esp+0xc] (6 bytes).
# Iguais no cm0102-1080.exe.
# Comparadores: as rodadas lado a lado (cx02_4); a resposta do teste.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nir_cup_set_up_rounds', 0x78afe0, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('ire_chal_cup_set_up_rounds', 0x634c10, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('ire_pres_cup_club_is_in_leinster', 0x6399c0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('as rodadas das copas irlandesas e o teste de Leinster', "the Irish cups' rounds and the Leinster test",
        [('nir_cup_set_up_rounds', 'NirCup::set_up_rounds', '0x78afe0', 'nir_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Irlanda do Norte não tem prêmio; monta a tabela de rodadas',
          'rule: the Northern Irish Cup has no prize fund; builds the round table'),
         ('ire_chal_cup_set_up_rounds', 'IreChalCup::set_up_rounds', '0x634c10', 'ire_chal_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Irlanda, as copas de Leinster e de Munster e a Copa da Croácia têm um prêmio total de 100.000; monta a tabela de rodadas',
          'rule: the Irish Challenge Cup, the Leinster and Munster Cups and the Croatian Cup have a 100,000 prize fund; builds the round table'),
         ('ire_pres_cup_club_is_in_leinster', 'ire_club_is_in_leinster', '0x6399c0', 'ire_pres_cup.cpp', 'cdecl',
          'regra: um clube é de Leinster quando a cidade do seu estádio é Dublin (o nome começa com "Dubl"), Bray, Longford, Dundalk, Drogheda, Athlone ou Kilkenny; sem clube, estádio ou cidade, não é',
          'rule: a club is in Leinster when its stadium\'s city is Dublin (the name starts with "Dubl"), Bray, Longford, Dundalk, Drogheda, Athlone or Kilkenny; with no club, stadium or city, it is not')])
CAMPOS = [('cidade +4: nome (texto)', 'city +4: name (text)')]
INTERESSA = []
