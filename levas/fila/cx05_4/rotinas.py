# Os resultados da Copa da Asia: o texto do destino de uma selecao, os registros dos tecnicos e o progresso nas
# estatisticas (asia_nations.cpp; leitor cx05, rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2004-11-01 (os
# jogos de 13 a 29 de outubro de 2004; 0 chamadas na cobertura do mundo ate 2002-08-01).
# - 0x40f130 AsiaNations::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, noticia, Text)
#   ret 0x28 (virtual 0x48): grupos (fase < 2) 6 com noticia 0x4b4590 cdecl(clube, a4, competicao, 6, noticia, Text),
#   sem noticia o texto 0x98737c (group_qualify_text do cx04_5); 0xfe 0x4b4590 com -2; fase 2 com noticia 0x4b0b80
#   cdecl(clube, partida, a8, desfecho, a6, Text), sem noticia os modelos da Copa America (copa_america_fate_template,
#   european_cup_fate_text: sprintf 0x66f4e0 cdecl; Text::set_string 0x4ae660 e Text::add_link 0x4ae8a0 thiscall).
#   Prologo mov al, [esp+0xc]; push esi (5 bytes).
# - 0x40f5a0 AsiaNations::update_staff_records_for_status thiscall(clube, desfecho, fase, a4, partida, a6) ret 0x18
#   (virtual 0x44): o corpo da Copa America (national_tournament_staff_records com o mata-mata na fase 2; Staff_Records
#   0x86bdd0, 0x86c1d0, 0x86c000, 0x86a800, 0x86b0b0, 0x86b710 em [0xdd7f00], thiscall). Devolve 0. Prologo mov al,
#   [esp+0xc]; push esi (5 bytes).
# - 0x410560 AsiaNations::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68):
#   Comp_Stats::find_club_entry 0x4a4850 (thiscall em [this+8]); as tabelas de salto 0x410624 e 0x410634 (1 a 4) e a
#   conta do melhor (1 a 3 -> 1, 4 -> 9); grava +0x74, +0x73, +0x75. Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx04_5 (group_qualify_text, group_championship_progress, compare_group_championship_progress): integrar
# depois dele. Os rotulos covered! do national_tournament_staff_records ja em Rust sao os da Copa America (o mesmo corpo).
# Comparadores: o texto lado a lado nos caminhos so de texto (compare_text_writer); os registros so a original (devolve
# 0); o progresso lado a lado nos 3 bytes da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('asia_nations_build_fate_news_text', 0x40f130, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('asia_nations_update_staff_records_for_status', 0x40f5a0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('asia_nations_set_club_progress', 0x410560, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os resultados da Copa da Ásia', 'the Asian Cup results',
        [('asia_nations_build_fate_news_text', 'AsiaNations::build_fate_news_text', '0x40f130', 'asia_nations.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de uma seleção na Copa da Ásia: nos grupos, quem passa "classifica-se para as quartas-de-final" (ou a notícia do grupo) e quem cai a notícia do grupo; no mata-mata o campeão, o vice, o terceiro, o quarto, o eliminado ou "passa à fase seguinte" (ou o resultado do confronto)',
          'writes the news text of a nation\'s fate in the Asian Cup: in the groups, one going through "qualifies for the quarter finals" (or the group news) and one going out gets the group news; in the knockout the winner, runner-up, third, fourth, knocked out or "through to next round" (or the tie result)'),
         ('asia_nations_update_staff_records_for_status', 'AsiaNations::update_staff_records_for_status', '0x40f5a0', 'asia_nations.cpp', 'thiscall, ret 0x18',
          'regra: nos grupos da Copa da Ásia quem passa ganha o registro de classificação (peso 25) e quem cai o de não classificação; no mata-mata quem passa ganha a classificação pela rodada (peso 40 a 100), quem cai o de eliminado, e campeão, vice e terceiro os registros da final; o quarto não ganha nada',
          'rule: in the Asian Cup groups a nation going through gets the qualification record (weight 25) and one going out the failed-to-qualify record; in the knockout one going through gets the qualification by round (weight 40 to 100), one going out the knocked-out record, and the winner, runner-up and third the final records; the fourth gets nothing'),
         ('asia_nations_set_club_progress', 'AsiaNations::set_club_progress', '0x410560', 'asia_nations.cpp', 'thiscall, ret 0x14',
          'guarda o progresso de uma seleção nas estatísticas da Copa da Ásia: nos grupos as colocações 1 a 4 viram os códigos da copa (atual 1, 3, 5, 11; melhor 1, 1, 1, 9; pior 5, 5, 9, 11); depois, como vêm',
          'stores a nation\'s progress in the Asian Cup statistics: in the groups places 1 to 4 become the cup\'s codes (current 1, 3, 5, 11; best 1, 1, 1, 9; worst 5, 5, 9, 11); afterwards, as they come')])
CAMPOS = []
INTERESSA = []
