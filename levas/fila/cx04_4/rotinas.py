# O texto da noticia do destino de um clube na Intertoto (intertoto_cup.cpp; leitor cx04, rodada 5, 24/11/2026).
# Simulacao, carreira ENG (os clubes ingleses jogam a Intertoto em julho de 2001): a virtual 0x48 da copa (o texto das
# noticias de competicao, chamado quando o texto da noticia e escrito) so roda com o modo CM_TEXTO_NOTICIAS
# (levas/reorganizacao/proximas/textos_noticias.md) ou abrindo a noticia jogando comparando.
# - 0x633af0 IntertotoCup::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, noticia, Text)
#   ret 0x28: desfecho 1 com noticia: 0x4b0b80 cdecl(clube, partida, a8, 1, a6, Text); sem noticia: o texto 0x9e9e54 com o
#   nome curto da Copa da UEFA (o registro pela chave 0x9cf6f0) e os vinculos do clube (0x7d5) e da copa (0x7d0); outro
#   desfecho: Comp::build_fate_news_text 0x48c6d0 (thiscall, ret 0x28) com os mesmos argumentos e a7 = 0. Prologo mov
#   eax, [esp+8]; cmp al, 1 (6 bytes).
# Igual no cm0102-1080.exe. Comparador: o caminho so de texto lado a lado (compare_text_writer); os outros so a original.
SAVE = False
ROTINAS = [('intertoto_cup_build_fate_news_text', 0x633af0, [0x8b, 0x44, 0x24, 0x08, 0x3c, 0x01])]
DOCS = ('o texto do destino de um clube na Intertoto', "the text of a club's fate in the Intertoto Cup",
        [('intertoto_cup_build_fate_news_text', 'IntertotoCup::build_fate_news_text', '0x633af0', 'intertoto_cup.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de um clube na Intertoto: quem passa (desfecho 1) "classifica-se para a Copa da UEFA", com os vínculos do clube e da copa, ou, com notícia, o resultado do confronto; os outros desfechos ficam com o texto geral da competição',
          'writes the news text of a club\'s fate in the Intertoto Cup: one that goes through (outcome 1) "qualifies for the UEFA Cup", with the club\'s and the cup\'s links, or, with a news item, the tie result; the other outcomes get the competition\'s general text')])
CAMPOS = []
INTERESSA = []
