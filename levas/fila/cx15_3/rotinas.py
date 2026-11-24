# As ordens do ranking da UEFA: as nacoes pelo coeficiente, os clubes para a distribuicao das vagas e os classificados
# da noticia das copas europeias (uefa_seeding.cpp; leitor cx15, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS): as tres
# rodam quando o ano do jogo muda (update_season sem diario, em 1/1/2002) e o ranking e refeito.
# - 0x905cb0 uefa_compare_nations_by_coefficient cdecl(a, b) -> i32 (qsort do roll_coefficients 0x903c76): o float +8,
#   fld a; fcomp b; ah & 0x41 = 0 (a > b) -1; senao ah & 1 (a < b ou nao ordenados) 1; senao 0. Prologo mov ecx, [esp+4];
#   mov edx, [esp+8] (8 bytes).
# - 0x905ce0 uefa_compare_seed_clubs cdecl(a, b) -> i32 (qsort do collect_european_clubs 0x9041dc, bsearch do
#   uefa_find_nth_unplaced_club_of_nation 0x9050e5 e do apply_euro_cfg_order 0x905ad0, e duas chamadas em 0x4c6xxx):
#   nacao (+4, a - b); com 0xff em +8 de um dos dois, 0; +9 (palavra com sinal, b - a); +0xb (byte com sinal, a - b); +8
#   (byte com sinal, b - a). Prologo mov ecx, [esp+4]; mov edx, [esp+8] (8 bytes).
# - 0x906410 uefa_compare_qualified_clubs cdecl(a, b) -> i32 (qsort do uefa_post_qualifiers_news 0x905f8a): os clubes
#   [a] e [b]; nacoes (+0x53) diferentes: id de a - id de b; competicao europeia (+0x1db) diferente: reputacao (+0x69 do
#   registro de 0x6b bytes em [0xae23d0]) de b - de a; +0x1df (byte com sinal) diferente: a - b; senao 0x540080 (a ordem
#   dos clubes pelo nome, cdecl(a, b): push b; push a). Prologo push ebx; mov ebx, [esp+0xc]; push esi (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: as duas no mesmo par, resultados iguais.
SAVE = False
ROTINAS = [('uefa_compare_nations_by_coefficient', 0x905cb0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('uefa_compare_seed_clubs', 0x905ce0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('uefa_compare_qualified_clubs', 0x906410, [0x53, 0x8b, 0x5c, 0x24, 0x0c, 0x56])]
DOCS = ('as ordens do ranking da UEFA: as nações pelo coeficiente, os clubes da distribuição das vagas e os classificados da notícia',
        'the UEFA seeding sort orders: nations by coefficient, clubs for the places and the qualified clubs of the news',
        [('uefa_compare_nations_by_coefficient', 'uefa_compare_nations_by_coefficient', '0x905cb0', 'uefa_seeding.cpp', 'cdecl',
          'ordena as nações do ranking pelo coeficiente das cinco temporadas, do maior para o menor',
          'sorts the seeding nations by their five-season coefficient, highest first'),
         ('uefa_compare_seed_clubs', 'uefa_compare_seed_clubs', '0x905ce0', 'uefa_seeding.cpp', 'cdecl',
          'ordena os clubes europeus por nação e, dentro dela, pela reputação da divisão, pela última posição na liga e pela reputação do clube; com a marca 0xff só a nação conta (chave de busca)',
          'sorts the European clubs by nation and, within it, by division reputation, last league position and club reputation; with the 0xff mark only the nation counts (search key)'),
         ('uefa_compare_qualified_clubs', 'uefa_compare_qualified_clubs', '0x906410', 'uefa_seeding.cpp', 'cdecl',
          'ordena os classificados da notícia das copas europeias por nação, pela reputação da competição (a maior primeiro), pela fase de entrada e pelo nome',
          'sorts the qualified clubs of the European qualifiers news by nation, competition reputation (highest first), entry round and name')])
CAMPOS = []
INTERESSA = []
