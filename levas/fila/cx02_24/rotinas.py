# A tabela do primeiro grupo e as honrarias da temporada da Copa da Holanda (hol_cup.cpp; leitor cx02, rodada 5,
# 24/11/2026). Competicoes da Holanda; ATE 2002-08-01 (a tabela na montagem da copa, no primeiro dia completo da
# temporada; as honrarias no fim da copa, 1 chamada no mundo).
# - 0x5f3d70 HolCup::create_group_table thiscall() ret 0 -> 1/0: o molde da WelPrmCup::create_group_table
#   (welsh_cup_group_table, 4 clubes) com o passo 1 no vetor de inscritos (+0xee), a linha 0x196. Prologo sub esp, 0x204.
# - 0x5f3f00 HolCup::record_season_honours thiscall() (a virtual 0x30, 16 bytes): mov eax, [ecx+0xc]; mov ecx,
#   [eax+0x4c]; jmp [vtable+0x30]: a fase 19 (o mata-mata) responde. Prologo os dois mov (6 bytes). Nao e um thunk de um
#   jmp so (le a fase): entra.
# Iguais no cm0102-1080.exe.
# Comparadores: a tabela pelo plano (so a original: compare_welsh_cup_group_table); as honrarias so a original.
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('hol_cup_create_group_table', 0x5f3d70, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('hol_cup_record_season_honours', 0x5f3f00, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x4c])]
DOCS = ('a tabela do primeiro grupo e as honrarias da temporada da Copa da Holanda', 'the Dutch Cup first group table and season honours',
        [('hol_cup_create_group_table', 'HolCup::create_group_table', '0x5f3d70', 'hol_cup.cpp', 'thiscall, ret 0',
          'regra: o primeiro grupo da Copa da Holanda (a própria copa) tem quatro clubes, os quatro primeiros inscritos, que saem da lista de espera; depois vem a notícia do sorteio do grupo',
          'rule: the Dutch Cup first group (the cup itself) has four clubs, the first four entrants, who leave the waiting list; the group draw news follows'),
         ('hol_cup_record_season_honours', 'HolCup::record_season_honours', '0x5f3f00', 'hol_cup.cpp', 'thiscall, ret 0',
          'regra: as honrarias da temporada da Copa da Holanda são as do seu mata-mata (a fase 19)',
          "rule: the Dutch Cup's season honours are those of its knockout (stage 19)")])
CAMPOS = []
INTERESSA = []
