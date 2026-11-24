# O texto da noticia do destino dos clubes da Serie C1 e da Serie C2 italianas (ita_ser_c2a.cpp; leitor cx11, rodada 5,
# 24/11/2026). A rotina esta no arquivo da C2/A, mas e a virtual 0x48 das cinco: C1/A (0x96c534), C1/B, C2/A, C2/B e
# C2/C (5 referencias no exe). Roda com a Italia e a base minima pela C1 (a repescagem e o play-out de maio e junho de
# 2002), quando o texto de uma noticia do destino e escrito: 0 chamadas na cobertura do mundo (o texto so e escrito ao
# abrir a noticia); a prova pede o modo CM_TEXTO_NOTICIAS (simular.py --textos) e as 8 ligas ate 2002-06-01 (a
# repescagem da C1 joga as semifinais em 21 e 28 de maio).
# - 0x654390 ItaSerC2a::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, corpo, Text) ret
#   0x28 (virtual 0x48): com corpo, Comp::build_fate_news_text 0x48c6d0 (ecx = this; os 10 argumentos com a7 = 0);
#   sem corpo, desfecho 1 na fase 0 o texto 0x989c18 ("{} win promotion") e desfecho 0xff na fase 1 o texto 0x987564
#   ("{} relegated"), com o genero (+0x52) e o nome curto (+0x38) do clube (langlib_sprintf 0x66f4e0 cdecl no buffer
#   0xde1f64; Text::set_string 0x4ae660; Text::add_link 0x4ae8a0 (nome, 0x7d5, clube)): group_qualify_text do cx04_5;
#   qualquer outro, Comp::build_fate_news_text com a7 = 0 e corpo 0. Prologo mov eax, [esp+0x24]; push esi (5 bytes).
# Iguais no cm0102-1080.exe (inteira). Comparador: os dois textos proprios lado a lado (compare_text_writer, com o modo
# CM_TEXTO_NOTICIAS); o texto da base so a original.
# Depende do cx04_5 (group_qualify_text): integrar depois dele.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('ita_ser_c_build_fate_news_text', 0x654390, [0x8b, 0x44, 0x24, 0x24, 0x56])]
DOCS = ('o texto da notícia do destino dos clubes da Série C1 e da Série C2 italianas', 'the fate news text of the Italian Serie C1 and C2 clubs',
        [('ita_ser_c_build_fate_news_text', 'ItaSerC2a::build_fate_news_text', '0x654390', 'ita_ser_c2a.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de um clube nos grupos da Série C1 e da Série C2 (os cinco usam a mesma rotina): quem vence a repescagem "sobe" e quem perde o play-out "cai", com o nome do clube; os outros casos, e todos com corpo de notícia, o texto geral da competição',
          'writes the news text of a club\'s fate in the Serie C1 and C2 groups (all five use the same routine): the play-off winner "wins promotion" and the play-out loser is "relegated", with the club\'s name; the other cases, and every one with a news body, the competition\'s general text')])
CAMPOS = []
INTERESSA = []
