# A cadeia de acesso e rebaixamento da Croacia e o reagrupamento da Segunda Divisao (cro_a1.cpp; leitor cx09, rodada 5,
# 24/11/2026). Simulacao com a Croacia ('ENG POL TUR GRE HRV RUS', ate 2002-08-01): as duas rodam na virada da
# Primeira Divisao croata (CroA1::start_new_season, ja em Rust: a cadeia pela virtual 0xb0 e o reagrupamento pela
# chamada direta), em julho de 2002.
# - 0x5150f0 CroA1::process_promotion_relegation_chain thiscall(x) ret 4 (virtual 0xb0): virtual 0xa4 de [0x9cf8a4]
#   (Primeira), [0x9cf8a8] (norte) e [0x9cf8ac] (sul); league_promote_and_relegate 0x689c80 (this; Primeira/norte e
#   Primeira/sul; 1, 1, -1, -1: o argumento x nao e lido). Prologo mov eax, [0xadadfc] (5 bytes).
# - 0x515190 cro_a1_realign_second_division_groups cdecl(): os registros do norte e do sul ([0xae23d0] + id x 0x6b); os
#   clubes (indice dword ate [0xae235c], lido uma vez) com +0x57 = um deles, ate 32 (contagem num byte); qsort 0x945333
#   (lista, n, 4, 0x53fd40, a latitude); para as 32 vagas (sempre 32: com menos clubes le a pilha nao iniciada, defeito):
#   16 primeiras o norte, as outras o sul; clube que muda de grupo e tem tecnico humano (news_club_has_human_manager
#   0x7875e0 em [0xb7431c]) recebe um NewsItem local (0x77bbe0 (0x13, 0); slot 0 o id do clube, slot 1 o id do grupo;
#   News::send_to_club_manager 0x7861e0); +0x57 = o grupo. Prologo mov eax, fs:[0] (6 bytes).
# Comparadores: a cadeia so a original; o reagrupamento so a original, conferida contra a ordem da copia feita antes.
# Iguais no cm0102-1080.exe. Sem dependencias (comp_by_global, comp_record_of_key publicados).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('cro_a1_process_promotion_relegation_chain', 0x5150f0, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('cro_a1_realign_second_division_groups', 0x515190, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a cadeia de acesso e rebaixamento da Croácia e o reagrupamento da Segunda Divisão',
        'the Croatian promotion and relegation chain and the second division regrouping',
        [('cro_a1_process_promotion_relegation_chain', 'CroA1::process_promotion_relegation_chain', '0x5150f0', 'cro_a1.cpp', 'thiscall, ret 4',
          'as posições finais da Primeira e dos dois grupos da Segunda; as trocas da Primeira com o grupo norte e com o grupo sul',
          'the final positions of the First Division and both second division groups; the exchanges of the First Division with the north and the south groups'),
         ('cro_a1_realign_second_division_groups', 'cro_a1_realign_second_division_groups', '0x515190', 'cro_a1.cpp', 'cdecl',
          'os 32 clubes da Segunda Divisão croata são ordenados pela latitude da cidade do estádio: os 16 mais ao norte formam o grupo norte e os outros o grupo sul; quem muda de grupo e tem técnico humano recebe a notícia',
          'the 32 Croatian second division clubs are ordered by the latitude of their stadium city: the 16 northernmost form the north group and the others the south group; a club that changes group and has a human manager gets the news')])
CAMPOS = []
INTERESSA = []
