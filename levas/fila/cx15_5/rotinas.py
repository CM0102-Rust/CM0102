# A divisao da MLS de um clube e a ordem das entradas dos play-offs pelos pontos (usa_mls.cpp; leitor cx15, rodada 5,
# 24/11/2026). Competicoes com as 8 ligas: a MLS 2001 vai de abril a outubro de 2001, os play-offs em setembro e outubro.
# - 0x909ec0 usa_mls_get_club_division cdecl(clube) -> palavra (push clube; add esp, 4 nos 10 chamadores: o
#   create_table 0x908560, o create_division_stage 0x909050/0x909270, o sorteio do All-Star 0x90a696, telas 0x49dcb2,
#   0x49e77b, 0x719dcd): sem divisao (+0x57) ou divisao que nao e a MLS (chave 0x9cf590), 0; o id do clube contra as
#   chaves 0x9d0390, 0x9d0394, 0x9d0384, 0x9d0380 -> 0x413; 0x9d03a4, 0x9d0388, 0x9d03a0, 0x9d037c -> 0x47b; 0x9d03a8,
#   0x9d039c, 0x9d0398, 0x9d038c -> 0x414; senao a caixa "Error" (usa_mls.cpp, linha 0x71b; [0xb67a34] = 0) e 0. So ax
#   e o resultado. Prologo mov ecx, [esp+4]; sub esp, 0x200 (10 bytes).
# - 0x90a130 usa_mls_compare_points_desc cdecl(a, b) -> i32 (qsort do create_playoff_stage 0x909565 e 0x909685): a
#   palavra com sinal +4 de b - a de a. Prologo mov eax, [esp+4]; mov edx, [esp+8] (8 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: as duas nos mesmos argumentos, resultados iguais.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('usa_mls_get_club_division', 0x909ec0, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('usa_mls_compare_points_desc', 0x90a130, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('a divisão da MLS de um clube e a ordem das entradas dos play-offs',
        'the MLS division of a club and the order of the play-off entries',
        [('usa_mls_get_club_division', 'usa_mls_get_club_division', '0x909ec0', 'usa_mls.cpp', 'cdecl',
          'dá a divisão da MLS de um clube (Leste, Central ou Oeste) pelas chaves dos doze clubes; fora da MLS, 0; um clube da MLS fora dos doze dá a caixa de erro',
          'gives the MLS division of a club (East, Central or West) by the keys of the twelve clubs; outside the MLS, 0; an MLS club outside the twelve shows the error box'),
         ('usa_mls_compare_points_desc', 'usa_mls_compare_points_desc', '0x90a130', 'usa_mls.cpp', 'cdecl',
          'ordena as entradas dos play-offs da MLS pelos pontos, do maior para o menor',
          'sorts the MLS play-off entries by points, highest first')])
CAMPOS = []
INTERESSA = []
