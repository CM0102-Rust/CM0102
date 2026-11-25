# Os textos das notícias do destino dos clubes nas três divisões nacionais do Brasil (bra_nat_first.cpp,
# bra_nat_second.cpp, bra_nat_third.cpp; leitor rx13, rodada 6, 25/11/2026). Simulação com as 8 ligas: os campeonatos
# brasileiros de 2001 vão até dezembro; o texto só sai com o modo de textos das notícias (CM_TEXTO_NOTICIAS, --textos) e
# um técnico humano como destinatário (na cobertura do mundo até 2002-08-01: 0 chamadas). Usa o club_at e o
# compare_text_writer do motor; o fate_base_text, o fate_headline e o compare_fate_text daqui servem ao rx13_9, ao
# rx13_10 e ao rx13_11.
# Todas thiscall(clube, destino, fase, a4, a5, a6, a7, a8, noticia, texto) ret 0x28; o texto base é
# Comp::build_fate_news_text 0x48c6d0 thiscall com os mesmos argumentos, a fase do ramo e a7 = 0 (cada push conferido).
# langlib_sprintf 0x66f4e0 cdecl em 0xde1f64; Text::set_string 0x4ae660 e Text::add_link 0x4ae8a0 (clube 0x7d5 pelo
# nome curto +0x38 do clube; competição 0x7d0 pelo nome curto +0x38 de [this+4]).
# - 0x42ac70 BraNatFirst: fase 0xff, destino 6: "have qualified for the quarter final of the {liga}." (com notícia; os
#   gêneros do clube e da competição) ou "qualify for quarter final" (sem), os dois vínculos sempre; fase 0xff, outro
#   destino: base(-1, a8, notícia). Fase 0: com notícia base(0, a8, notícia); sem: destino 1 "win {liga}", 2 "lose in
#   {liga} final" (os dois vínculos), outro base(0, 0, 0). Outras fases: nada. Prólogo mov al, [esp+0xc]; push ebx (5).
# - 0x42d0d0 BraNatSecond: fase 0xff base(-1); fase 0 como a da primeira. Prólogo igual (5).
# - 0x42ec60 BraNatThird: destino 6 e fase (byte com sinal) < 7: "have qualified for the second round of the {liga}." ou
#   "qualify for second round" (os dois vínculos); fase 7 base(7); fase < 10 base(a fase como veio, o dword); fase 10:
#   sem notícia e destino 1, o título 0x98abe4 com os argumentos do original (DEFEITO: o gênero da competição vai uma vez
#   só, então o nome do clube vira o quarto gênero e o nome da competição sai no lugar do clube) e os dois vínculos;
#   senão base(10) e de novo Text::set_string com 0xde1f64 e os dois vínculos (o texto base reescrito). Acima de 10:
#   nada. Prólogo mov eax, [esp+8]; push ebx (5).
# Iguais no cm0102-1080.exe. Comparadores: os caminhos só de texto (as manchetes) lado a lado (compare_text_writer); os
# caminhos pelo texto base só a original.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('bra_nat_first_build_fate_news_text', 0x42ac70, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('bra_nat_second_build_fate_news_text', 0x42d0d0, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('bra_nat_third_build_fate_news_text', 0x42ec60, [0x8b, 0x44, 0x24, 0x08, 0x53])]
DOCS = ('os textos das notícias do destino dos clubes nas três divisões nacionais do Brasil',
        "the fate news texts of the three Brazilian national divisions",
        [('bra_nat_first_build_fate_news_text', 'BraNatFirst::build_fate_news_text', '0x42ac70', 'bra_nat_first.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino na primeira divisão brasileira: na temporada, quem passa às quartas de final ("se classificaram para as quartas de final da liga", com a notícia completa, ou a manchete curta); na final, o campeão ("vence a liga") e o vice ("perde a final"); os outros destinos, o texto base da competição',
          'the fate news text in the Brazilian first division: over the season, a club through to the quarter finals ("have qualified for the quarter final of the league", with the full news, or the short headline); in the final, the winner ("win the league") and the loser ("lose in the final"); every other fate, the base competition text'),
         ('bra_nat_second_build_fate_news_text', 'BraNatSecond::build_fate_news_text', '0x42d0d0', 'bra_nat_second.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino na segunda divisão brasileira: na final, o campeão e o vice; os outros destinos, o texto base da competição',
          'the fate news text in the Brazilian second division: in the final, the winner and the loser; every other fate, the base competition text'),
         ('bra_nat_third_build_fate_news_text', 'BraNatThird::build_fate_news_text', '0x42ec60', 'bra_nat_third.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino na terceira divisão brasileira: na primeira fase, quem passa à segunda; na última fase (10), o campeão sem a notícia completa (com os argumentos trocados do original: sai o nome da competição no lugar do clube); os outros destinos até a fase 10, o texto base (na fase 10 o texto é posto de novo com os vínculos)',
          "the fate news text in the Brazilian third division: in the first round, a club through to the second; in the last stage (10), the champion without the full news (with the original's shifted arguments: the competition's name comes out in the club's place); every other fate up to stage 10, the base text (at stage 10 the text is set again with the links)")])
CAMPOS = []
INTERESSA = []
