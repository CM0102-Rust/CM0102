# Os resultados do futebol olimpico: o texto do destino de uma selecao, os registros dos tecnicos e o progresso nas
# estatisticas (olympics.cpp; leitor cx07, rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2004-11-01: a edicao do
# jogo e a de 2004 (o construtor, ja em Rust, leva o ano ao ciclo de 4 a partir de 1996); os grupos em 13, 16 e 19 de
# setembro e o mata-mata de 23 a 30 de setembro de 2004 (0 chamadas na cobertura do mundo ate 2002-08-01). A mesma data
# final das levas da Copa da Asia (cx05): podem ir na mesma leva.
# - 0x79fea0 Olympics::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, noticia, Text)
#   ret 0x28 (virtual 0x48): o corpo da Copa da Asia com o mata-mata na fase 3 e o modelo 0x98737c ("qualify for
#   quarter finals"). Prologo mov al, [esp+0xc]; push esi (5 bytes).
# - 0x7a0300 Olympics::update_staff_records_for_status thiscall(clube, desfecho, fase, a4, partida, a6) ret 0x18
#   (virtual 0x44): nos grupos o codigo 0x78; no mata-mata (fase 3, [+0xc][3]) o peso com 3 rodadas (push 3) e so os
#   registros de campeao (0x86a800) e vice (0x86b0b0): o terceiro e o quarto sem registro. Devolve 0. Prologo mov al,
#   [esp+0xc]; push esi (5 bytes).
# - 0x7a15d0 Olympics::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68):
#   Comp_Stats::find_club_entry 0x4a4850; as tabelas de salto 0x7a1698 (melhor), 0x7a16a8 (pior) e 0x7a16b8 (atual), 1
#   a 4, nos grupos (fase < 3); grava +0x74, +0x73, +0x75. Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx07_3 (nations_tournament_fate_text, nations_fate_text_only, nations_tournament_staff_records) e do cx04_5
# (group_championship_progress, compare_group_championship_progress): integrar depois deles.
# Comparadores: o texto lado a lado nos caminhos so de texto (compare_text_writer); os registros so a original (devolve
# 0); o progresso lado a lado nos 3 bytes da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('olympics_build_fate_news_text', 0x79fea0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('olympics_update_staff_records_for_status', 0x7a0300, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('olympics_set_club_progress', 0x7a15d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os resultados do futebol olímpico: o texto do destino, os registros dos técnicos e o progresso',
        'the Olympic football results: the fate text, the staff records and the progress',
        [('olympics_build_fate_news_text', 'Olympics::build_fate_news_text', '0x79fea0', 'olympics.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de uma seleção no futebol olímpico: nos quatro grupos, a classificação ("se classifica para as quartas de final") ou a notícia do grupo, e a eliminação; no mata-mata, a notícia do confronto ou o título, o vice, o terceiro, o quarto, a eliminação ou a passagem',
          "writes the news text of a nation's fate in the Olympic football: in the four groups, going through (\"qualify for quarter finals\") or the group news, and going out; in the knockout, the tie news or the win, runners-up, third, fourth, knocked out or through"),
         ('olympics_update_staff_records_for_status', 'Olympics::update_staff_records_for_status', '0x7a0300', 'olympics.cpp', 'thiscall, ret 0x18',
          'grava os registros dos técnicos: nos grupos a classificação (peso 25) ou a eliminação; no mata-mata a passagem pela rodada (peso de 40 a 100 em três rodadas), a eliminação e os registros de campeão e vice; o terceiro e o quarto lugares não ganham registro',
          'writes the staff records: in the groups going through (weight 25) or out; in the knockout going through by round (weight 40 to 100 over three rounds), knocked out and the champion and runner-up records; third and fourth places get none'),
         ('olympics_set_club_progress', 'Olympics::set_club_progress', '0x7a15d0', 'olympics.cpp', 'thiscall, ret 0x14',
          'grava o progresso de uma seleção nas estatísticas do futebol olímpico: nos grupos a atual 1 vira 5, 2 e 3 viram 9 e 4 vira 13; a melhor 1 e 2 viram 1, 3 vira 9 e 4 vira 13; a pior como a atual; no mata-mata como vêm',
          "stores a nation's progress in the Olympic football statistics: in the groups current 1 becomes 5, 2 and 3 become 9 and 4 becomes 13; best 1 and 2 become 1, 3 becomes 9 and 4 becomes 13; worst as current; in the knockout as they come")])
CAMPOS = []
INTERESSA = []
