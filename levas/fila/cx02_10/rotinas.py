# As noticias do destino de um clube nas Copas da Liga da Irlanda e da Irlanda do Norte (ire_lge_cup.cpp,
# nir_lge_cup.cpp; leitor cx02, rodada 5, 24/11/2026). Competicoes da Irlanda e da Irlanda do Norte; ATE 2002-08-01.
# Escritores de texto: rodam quando a noticia e montada e aberta (o modo de textos da cadeia).
# - 0x6374a0 IreLgeCup::build_fate_news_text e 0x78ceb0 NirLgeCup::build_fate_news_text thiscall(clube_id, desfecho,
#   fase, a4, a5, a6, a7, a8, noticia, texto) ret 0x28 (a virtual 0x50): instrucao por instrucao iguais a
#   ItaCup::build_fate_news_text 0x63f130 (ja Rust) menos a fase do mata-mata (7 e 3) e o modelo "through to quarter
#   finals" (0x9a7e9c); 0x4b4590 (6 pushes; add esp, 0x18 -> cdecl), 0x4b0b80 (6 pushes -> cdecl), 0x4b4420 (4 pushes ->
#   cdecl), langlib_sprintf (5 pushes -> cdecl), Text::set_string e Text::add_link (thiscall).
#   O auxiliar group_cup_fate_news_text fica neste item: o cx02_12 (Copa da Serie C), o Gre, o Hol e o Kor o usam.
#   Prologo mov al, [esp+0xc]; cmp al, fase (6 bytes).
# Iguais no cm0102-1080.exe.
# Comparadores: a linha "through to" lado a lado com compare_text_writer; os outros caminhos, so a original.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_lge_cup_build_fate_news_text', 0x6374a0, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x07]),
           ('nir_lge_cup_build_fate_news_text', 0x78ceb0, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x03])]
DOCS = ('as notícias do destino de um clube nas Copas da Liga da Irlanda e da Irlanda do Norte',
        "the Irish and Northern Irish League Cups' fate news",
        [('ire_lge_cup_build_fate_news_text', 'IreLgeCup::build_fate_news_text', '0x6374a0', 'ire_lge_cup.cpp', 'thiscall, ret 0x28',
          'regra: na Copa da Liga irlandesa, quem passa de um grupo (fases 0 a 6) sem notícia ganha a linha "{clube} nas quartas de final" com o link do clube; com notícia, a notícia genérica do grupo; a eliminação no grupo, a notícia genérica; no mata-mata (fase 7), o resultado do confronto ou a vitória/derrota',
          'rule: in the Irish League Cup a club going through a group (stages 0 to 6) with no news item gets the line "{club} through to quarter finals" with a link to the club; with a news item, the generic group news; a group exit, the generic news; in the knockout (stage 7), the tie result or the win/defeat news'),
         ('nir_lge_cup_build_fate_news_text', 'NirLgeCup::build_fate_news_text', '0x78ceb0', 'nir_lge_cup.cpp', 'thiscall, ret 0x28',
          'regra: na Copa da Liga da Irlanda do Norte, o mesmo com os grupos nas fases 0 a 2 e o mata-mata na fase 3 ("{clube} nas quartas de final")',
          'rule: in the Northern Irish League Cup, the same with the groups in stages 0 to 2 and the knockout in stage 3 ("{club} through to quarter finals")')])
CAMPOS = []
INTERESSA = []
