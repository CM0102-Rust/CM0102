# O texto da notícia do destino de uma seleção na Copa América (goldcup.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x5e0f50 (GoldCup::build_fate_news_text).
# Nos grupos (fase < 2) só a classificação às quartas (destino 6) tem texto; no mata-mata (fase 2) campeão, vice,
# terceiro, quarto, eliminado (0xff) e qualquer outro ("through to next round"); da fase 3 em diante, nada. Com a frase
# completa (o 9º argumento), as frases compartilhadas 0x4b4590 (grupos) e 0x4b0b80 (mata-mata), cdecl, com os argumentos
# conferidos na desmontagem; sem ela, langlib_sprintf no buffer 0xde1f64, Text::set_string e os vínculos (0x7d5 a
# seleção, 0x7d0 a competição).
# Convenção conferida: thiscall, ret 0x28 (10 argumentos; o 7º não é lido); o eax de volta não é lido pelos chamadores.
# Comparador: lado a lado com compare_text_writer (compare/media.rs, 759c98f9d): o texto, os vínculos e o gerador.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('goldcup_build_fate_news_text', 0x5e0f50, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('o texto da notícia do destino de uma seleção na Copa América',
        "the news text of a nation's Copa América fate",
        [('goldcup_build_fate_news_text', 'GoldCup::build_fate_news_text', '0x5e0f50', 'goldcup.cpp', 'thiscall, ret 0x28', 'regra: o texto da notícia do destino de uma seleção na Copa América: nos grupos, só a classificação às quartas; no mata-mata, título, vice, terceiro, quarto, eliminação ou passagem de fase, com os vínculos da seleção e da competição', "rule: the news text of a nation's Copa América fate: in the groups, only the quarter-final qualification; in the knockout, the title, runners-up, third, fourth, elimination or going through, with the links of the team and the competition")])
CAMPOS = [('clube +0x38 nome curto e +0x52 gênero; competição +0x38 e +0x52; o buffer 0xde1f64; o Text; valor; lê e grava o texto', 'club +0x38 short name and +0x52 gender; competition +0x38 and +0x52; the buffer 0xde1f64; the Text; value; reads and writes the text')]
