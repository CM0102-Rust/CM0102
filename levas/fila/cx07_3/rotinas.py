# Os resultados da Copa das Nacoes da Oceania: o texto do destino de uma selecao, os registros dos tecnicos e o
# progresso nas estatisticas (oceania_nations.cpp; leitor cx07, rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE
# 2002-12-31: a edicao do jogo e a de 2002 (o construtor, ja em Rust, leva o ano a 2002 ou ao primeiro par seguinte), com
# os grupos em 10, 13 e 15 de novembro e o mata-mata em 17 e 19 de novembro de 2002 (0 chamadas na cobertura do mundo
# ate 2002-08-01).
# - 0x799300 OceaniaNations::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, noticia, Text)
#   ret 0x28 (virtual 0x48): o corpo da Copa da Asia (cx05_4) com o mata-mata na fase 1 e o modelo 0xa6a130 ("qualify
#   for semi finals"); 0x4b4590 e 0x4b0b80 cdecl (6 pushes; add esp, 0x18), sprintf 0x66f4e0 cdecl, Text::set_string
#   0x4ae660 e Text::add_link 0x4ae8a0 thiscall. Prologo mov al, [esp+0xc]; push esi (5 bytes).
# - 0x799760 OceaniaNations::update_staff_records_for_status thiscall(clube, desfecho, fase, a4, partida, a6) ret 0x18
#   (virtual 0x44): nos grupos o codigo de rodada 0x82 (e nao 0x78), no mata-mata (fase 1) o peso com 1 rodada
#   (comp_util_scale_by_round 0x4bf850 cdecl: push 1; push rodada; push 0x64; push 0x28) e a 4a colocada sem registro
#   (je explicito); Staff_Records 0x86bdd0, 0x86c1d0, 0x86c000, 0x86a800, 0x86b0b0, 0x86b710 em [0xdd7f00] (thiscall).
#   Devolve 0. Prologo mov al, [esp+0xc]; push esi (5 bytes).
# - 0x79a490 OceaniaNations::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68):
#   Comp_Stats::find_club_entry 0x4a4850; nos grupos (fase < 1) atual 1->1, 2->3, 3->5; melhor 1, 2->1, 3->5; pior 1,
#   2->3, 3->5 (sem tabelas de salto; o 4 fica); grava +0x74, +0x73, +0x75. Prologo mov eax, [esp+4]; mov ecx, [ecx+8].
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx04_5 (group_qualify_text, group_championship_progress, compare_group_championship_progress): integrar
# depois dele. O nations_tournament_fate_text, o nations_fate_text_only e o nations_tournament_staff_records daqui sao
# usados pelo cx07_5 (o corpo da Copa da Asia do cx05_4 e o mesmo com o mata-mata em 2: na integracao, a Asia pode
# passar a chama-lo).
# Comparadores: o texto lado a lado nos caminhos so de texto (compare_text_writer); os registros so a original (devolve
# 0); o progresso lado a lado nos 3 bytes da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-12-31'
ROTINAS = [('oceania_nations_build_fate_news_text', 0x799300, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('oceania_nations_update_staff_records_for_status', 0x799760, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('oceania_nations_set_club_progress', 0x79a490, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os resultados da Copa das Nações da Oceania: o texto do destino, os registros dos técnicos e o progresso',
        'the Oceania Nations Cup results: the fate text, the staff records and the progress',
        [('oceania_nations_build_fate_news_text', 'OceaniaNations::build_fate_news_text', '0x799300', 'oceania_nations.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de uma seleção na Copa da Oceania: nos dois grupos, a classificação ("se classifica para as semifinais") ou a notícia do grupo, e a eliminação; no mata-mata, a notícia do confronto ou o título, o vice, o terceiro, o quarto, a eliminação ou a passagem',
          "writes the news text of a nation's fate in the Oceania Nations Cup: in the two groups, going through (\"qualify for semi finals\") or the group news, and going out; in the knockout, the tie news or the win, runners-up, third, fourth, knocked out or through"),
         ('oceania_nations_update_staff_records_for_status', 'OceaniaNations::update_staff_records_for_status', '0x799760', 'oceania_nations.cpp', 'thiscall, ret 0x18',
          'grava os registros dos técnicos: nos grupos a classificação (peso 25) ou a eliminação; no mata-mata a passagem pela rodada (peso de 40 a 100), a eliminação e os registros de campeão, vice e terceiro; o quarto lugar não ganha registro',
          'writes the staff records: in the groups going through (weight 25) or out; in the knockout going through by round (weight 40 to 100), knocked out and the champion, runner-up and third-place records; the fourth place gets none'),
         ('oceania_nations_set_club_progress', 'OceaniaNations::set_club_progress', '0x79a490', 'oceania_nations.cpp', 'thiscall, ret 0x14',
          'grava o progresso de uma seleção nas estatísticas da Copa da Oceania: nos grupos a atual 1 fica 1, 2 vira 3 e 3 vira 5; a melhor 1 e 2 viram 1 e 3 vira 5; a pior 1 e 2 viram 3 e 3 vira 5; o 4 e o mata-mata como vêm',
          "stores a nation's progress in the Oceania Nations Cup statistics: in the groups current 1 stays 1, 2 becomes 3 and 3 becomes 5; best 1 and 2 become 1 and 3 becomes 5; worst 1 and 2 become 3 and 3 becomes 5; a 4 and the knockout as they come")])
CAMPOS = []
INTERESSA = []
