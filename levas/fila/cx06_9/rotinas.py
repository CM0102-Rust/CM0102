# O texto da noticia do destino de uma selecao na Copa das Confederacoes (fifa_confed.cpp; leitor cx06, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas e ATE 2003-08-15 (a copa de 2003); 0 chamadas na cobertura do mundo ate
# 2002-08-01: sem a data longa o item nao prova nada (o mesmo ATE do cx04_6).
# - 0x58ba10 FifaConfed::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, noticia, Text)
#   ret 0x28 (virtual 0x48): o molde do da Oceania (group_championship_fate_text do cx04_5) com grupos na fase < 1, sem o
#   texto geral no meio, o texto 0x9c74b4 ("{selecao} qualify for semi-finals") de quem passa do grupo e, na fase 1, um
#   modelo para cada desfecho: 1 -> 0x98732c (campeao), 2 -> 0x9872c8 (vice), 3 -> 0x987264 (terceiro), 4 -> 0x987200
#   (quarto), 0xff -> 0x98713c (eliminado), outro -> 0x987198 (passa de fase); langlib_sprintf 0x66f4e0 (cdecl) no
#   buffer 0xde1f64, Text::set_string 0x4ae660 e os vinculos 0x7d5 (selecao) e 0x7d0 (copa). Prologo mov al, [esp+0xc];
#   push esi (5 bytes).
# Iguais no cm0102-1080.exe.
# Comparador: o texto lado a lado nos caminhos so de texto (compare_text_writer; com o modo CM_TEXTO_NOTICIAS); as
# noticias de grupo e de confronto so a original.
# Depende do cx04_5 (group_championship_fate_text, group_championship_text_only): integrar depois dele.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('fifa_confed_build_fate_news_text', 0x58ba10, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('o texto do destino de uma seleção na Copa das Confederações', "a nation's fate news text in the Confederations Cup",
        [('fifa_confed_build_fate_news_text', 'FifaConfed::build_fate_news_text', '0x58ba10', 'fifa_confed.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de uma seleção na Copa das Confederações: nos grupos, quem passa "classifica-se para as semifinais" (ou a notícia do grupo) e quem cai a notícia do grupo; no mata-mata a notícia do confronto ou o campeão, o vice, o terceiro, o quarto, o eliminado e quem passa de fase',
          "writes the news text of a nation's fate in the Confederations Cup: in the groups, a nation through \"qualify for semi-finals\" (or the group news) and one out the group news; in the knockout the tie news or the winners, runners-up, third, fourth, knocked out and through to the next round")])
CAMPOS = []
INTERESSA = []
