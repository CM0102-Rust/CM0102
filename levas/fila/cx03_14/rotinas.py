# As noticias do destino dos clubes nas tres continentais americanas (con_merc_cup.cpp, conmebol_liber.cpp,
# conmebol_merc.cpp; leitor cx03, rodada 5, 24/11/2026). Carreira de 8 ligas (o Brasil e os EUA jogaveis).
# - 0x4c1cc0 ConMercCup::build_fate_news_text, 0x4c3680 ConmebolLiber::build_fate_news_text e 0x4c4d40
#   ConmebolMerc::build_fate_news_text thiscall(clube, destino, fase, a4..a8, noticia, texto) ret 0x28: o mesmo corpo das
#   duas galesas do cx03_9 (conferido instrucao a instrucao), com o mata-mata nas fases 3, 7 e 4 e os textos 0x9a7dac
#   "through to semi finals", 0x9a7e24 "through to second round" e 0x9a7e9c "through to quarter finals". Prologo mov
#   al, [esp+0xc]; cmp al, k (6 bytes).
# Usa write_group_cup_fate_news e compare_group_cup_fate_news do cx03_9 (integrar depois dele).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: o texto "through to ..." lado a lado (compare_text_writer, modo CM_TEXTO_NOTICIAS); os outros caminhos
# passam o objeto da noticia adiante: so a original.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('con_merc_cup_build_fate_news_text', 0x4c1cc0, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x03]),
           ('conmebol_liber_build_fate_news_text', 0x4c3680, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x07]),
           ('conmebol_merc_build_fate_news_text', 0x4c4d40, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x04])]
DOCS = ('as notícias do destino dos clubes nas três continentais americanas', 'the fate news of the three American continental cups',
        [('con_merc_cup_build_fate_news_text', 'ConMercCup::build_fate_news_text', '0x4c1cc0', 'con_merc_cup.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Copa Merconorte: nos grupos (fases abaixo de 3) a classificação e a eliminação pela frase genérica dos grupos, ou "{clube} through to semi finals" para quem passa sem objeto de notícia; no mata-mata (fase 3) o resultado do confronto',
          "rule: the Merconorte news of a club's fate: in the groups (stages below 3) qualification and elimination through the generic group sentence, or \"{club} through to semi finals\" for a club going through with no news object; in the knockout (stage 3) the tie result"),
         ('conmebol_liber_build_fate_news_text', 'ConmebolLiber::build_fate_news_text', '0x4c3680', 'conmebol_liber.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Libertadores, como a da Merconorte, com "{clube} through to second round" e o mata-mata na fase 7',
          "rule: the Copa Libertadores news of a club's fate, as the Merconorte's, with \"{club} through to second round\" and the knockout at stage 7"),
         ('conmebol_merc_build_fate_news_text', 'ConmebolMerc::build_fate_news_text', '0x4c4d40', 'conmebol_merc.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Mercosul, como a da Merconorte, com "{clube} through to quarter finals" e o mata-mata na fase 4',
          "rule: the Copa Mercosur news of a club's fate, as the Merconorte's, with \"{club} through to quarter finals\" and the knockout at stage 4")])
CAMPOS = []
INTERESSA = []
