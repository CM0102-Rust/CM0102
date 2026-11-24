# A noticia do destino de um clube na Copa da Grecia (gre_cup.cpp; leitor cx02, rodada 5, 24/11/2026). Competicoes da
# Grecia; ATE 2002-08-01. Escritor de texto (o modo de textos da cadeia).
# - 0x5e9140 GreCup::build_fate_news_text: igual a ItaCup::build_fate_news_text com o mata-mata na fase 10 e o modelo
#   "through to second round" (0x9a7e24); group_cup_fate_news_text do cx02_10 (integrar depois dele). Prologo mov al,
#   [esp+0xc]; cmp al, 0xa (6 bytes).
# Igual no cm0102-1080.exe.
# Comparador: a linha "through to" lado a lado (compare_text_writer); os outros caminhos so a original.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_cup_build_fate_news_text', 0x5e9140, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x0a])]
DOCS = ('a notícia do destino de um clube na Copa da Grécia', 'the Greek Cup fate news',
        [('gre_cup_build_fate_news_text', 'GreCup::build_fate_news_text', '0x5e9140', 'gre_cup.cpp', 'thiscall, ret 0x28',
          'regra: na Copa da Grécia, quem passa de um grupo (fases 0 a 9) sem notícia ganha a linha "{clube} na segunda fase" com o link do clube; com notícia, a notícia genérica do grupo; a eliminação, a notícia genérica; no mata-mata (fase 10), o resultado do confronto ou a vitória/derrota',
          'rule: in the Greek Cup a club going through a group (stages 0 to 9) with no news item gets the line "{club} through to second round" with a link to the club; with a news item, the generic group news; an exit, the generic news; in the knockout (stage 10), the tie result or the win/defeat news')])
CAMPOS = []
INTERESSA = []
