# O texto do destino, o progresso e o banco de reservas da liga australiana (aus_nsl.cpp; leitor cx08, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas (a NSL roda com a Australia, de outubro de 2001 a maio de 2002; as finais em
# abril e maio, dentro de 2002-06-01).
# - 0x411a30 AusNsl::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, corpo, Text) ret 0x28
#   (virtual 0x48): fase 0xff ou 0 so o desfecho 6: com corpo comp_util_write_group_qualify_news 0x4b4590 (cdecl; clube,
#   a4, +4, 6, corpo, Text), sem corpo o texto 0x987c88 ("through to playoff phase") ou 0x987c58 ("through to final")
#   com o nome curto e o genero do clube e o vinculo 0x7d5; outras fases com corpo 0x4b0b80 (clube, partida, a8,
#   desfecho, a6, Text), sem corpo 0x4b4420 (clube, partida, desfecho, Text). Prologo mov al, [esp+0xc]; push esi.
# - 0x412380 AusNsl::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: find_club_entry 0x4a4850;
#   fase 0xff: 1-2 -> 1 (pior 7), 3 -> 9; fases < 1: as tabelas de salto 0x4124ac, 0x4124bc, 0x4124cc (1-2 -> 1, pior 3;
#   3 -> 5; 4 -> 7); grava +0x74, +0x73, +0x75. Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# - 0x4124e0 AusNsl::set_fixture_substitutes thiscall(partida) ret 4 (virtual 0x7c): partida de fase (+0x42 >= 0, com
#   sinal) o byte baixo de +0x4d = 0x35 (5 reservas, 3 substituicoes); senao +0x49 & 0xf e +0x4a & 0xf nos dois nibbles.
#   Prologo mov edx, [esp+4]; push esi (5 bytes).
# Fica de fora 0x4110b0 AusNsl::get_table_layout_type (mov al, 3; ret: 3 bytes, prologo de menos de 5).
# Comparadores: o texto lado a lado nos caminhos so de texto (compare_text_writer; com o modo CM_TEXTO_NOTICIAS); o
# progresso lado a lado nos 3 bytes (compare_group_championship_progress do cx04_5); o banco lado a lado na palavra.
# Depende do cx04_5 (group_qualify_text e compare_group_championship_progress): integrar depois.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('aus_nsl_build_fate_news_text', 0x411a30, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('aus_nsl_set_club_progress', 0x412380, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('aus_nsl_set_fixture_substitutes', 0x4124e0, [0x8b, 0x54, 0x24, 0x04, 0x56])]
DOCS = ('o texto do destino, o progresso e o banco na liga australiana', "the Australian league's fate news, club progress and bench",
        [('aus_nsl_build_fate_news_text', 'AusNsl::build_fate_news_text', '0x411a30', 'aus_nsl.cpp', 'thiscall, ret 0x28',
          'escreve a notícia do destino de um clube na liga australiana: classificado na temporada regular, "{clube} passa à fase final"; classificado nas finais, "{clube} vai à grande final" (ou a notícia de grupo); na grande final, os textos de confronto das copas',
          "writes the news of a club's fate in the Australian league: through from the regular season, \"{club} through to playoff phase\"; through from the finals series, \"{club} through to final\" (or the group news); in the grand final, the cup tie lines"),
         ('aus_nsl_set_club_progress', 'AusNsl::set_club_progress', '0x412380', 'aus_nsl.cpp', 'thiscall, ret 0x14',
          'guarda o progresso de um clube na liga australiana: na temporada regular os dois primeiros viram 1 (o pior, 7) e o terceiro 9; nas finais 1 e 2 viram 1 (o pior, 3), o 3 vira 5 e o 4 vira 7; na grande final, como vêm',
          "stores a club's progress in the Australian league: in the regular season the top two become 1 (the worst, 7) and third 9; in the finals series 1 and 2 become 1 (the worst, 3), 3 becomes 5 and 4 becomes 7; in the grand final, as they come"),
         ('aus_nsl_set_fixture_substitutes', 'AusNsl::set_fixture_substitutes', '0x4124e0', 'aus_nsl.cpp', 'thiscall, ret 4',
          'o banco da liga australiana: um jogo das finais leva 5 reservas e 3 substituições; um jogo da liga, os números da liga',
          "the Australian league's bench: a finals fixture names 5 substitutes and allows 3; a league fixture takes the league's numbers")])
CAMPOS = []
INTERESSA = []
