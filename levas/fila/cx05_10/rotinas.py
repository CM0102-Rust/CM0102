# Os resultados da Eurocopa: o texto do destino de uma selecao, os registros dos tecnicos e o progresso nas
# estatisticas (euro_champ.cpp; leitor cx05, rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2004-07-15 (os
# jogos de 9 a 30 de junho de 2004; 0 chamadas na cobertura do mundo ate 2002-08-01).
# - 0x57b330 EuroChamp::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, noticia, Text)
#   ret 0x28 (virtual 0x48): fase < 3 sempre 0x4b4590 cdecl(clube, a4, competicao, desfecho, noticia, Text) (qualquer
#   desfecho); fase 3 com noticia 0x4b0b80 cdecl(clube, partida, a8, desfecho, a6, Text); sem noticia 1 -> 0x98732c,
#   2 -> 0x9872c8, 0xff -> 0x98713c, outro -> 0x987198, com o registro da competicao [0x9cf7a4] (nao o +4 do objeto).
#   Prologo mov al, [esp+0xc]; cmp al, 3 (6 bytes).
# - 0x57b1d0 EuroChamp::update_staff_records_for_status thiscall(clube, desfecho, fase, a4, partida, a6) ret 0x18
#   (virtual 0x44): Staff_Records 0x86bdd0, 0x86c1d0, 0x86c000, 0x86a800, 0x86b0b0 (thiscall em [0xdd7f00]),
#   comp_util_scale_by_round 0x4bf850 (cdecl), NationalTeams::clear_current_competition 0x775000 (push nacao; mov ecx,
#   [0xb74318] -> thiscall, ret 4). a4 e um ponteiro de byte (o campeao grava 3). Prologo mov al, [esp+0xc]; push esi.
# - 0x57cc30 EuroChamp::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68): os
#   codigos da Copa Africana (as tabelas de salto 0x57ccf8, 0x57cd08, 0x57cd18). Prologo mov eax, [esp+4]; mov ecx,
#   [ecx+8] (7 bytes).
# Diferencas da Copa Africana que a copia segue: no mata-mata o registro de classificacao vai com a fase 0 (a Copa
# Africana usa a +0x32 da partida) e o eliminado com a fase 0; quem cai, o campeao e o vice deixam a competicao
# (0x775000); o terceiro nao tem registro.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx04_5 (group_championship_progress, compare_group_championship_progress): integrar depois dele.
# Comparadores: o texto do mata-mata lado a lado (compare_text_writer); os registros so a original (devolve 0); o
# progresso lado a lado nos 3 bytes da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-07-15'
ROTINAS = [('euro_champ_build_fate_news_text', 0x57b330, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x03]),
           ('euro_champ_update_staff_records_for_status', 0x57b1d0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('euro_champ_set_club_progress', 0x57cc30, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os resultados da Eurocopa', 'the European Championship results',
        [('euro_champ_build_fate_news_text', 'EuroChamp::build_fate_news_text', '0x57b330', 'euro_champ.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de uma seleção na Eurocopa: nos grupos a notícia do grupo; no mata-mata o campeão, o vice, o eliminado ou "passa à fase seguinte" (ou o resultado do confronto), com o nome da Eurocopa',
          'writes the news text of a nation\'s fate in the European Championship: in the groups the group news; in the knockout the winner, the runner-up, knocked out or "through to next round" (or the tie result), with the European Championship\'s name'),
         ('euro_champ_update_staff_records_for_status', 'EuroChamp::update_staff_records_for_status', '0x57b1d0', 'euro_champ.cpp', 'thiscall, ret 0x18',
          'regra: nos grupos da Eurocopa quem passa ganha o registro de classificação (peso 25) e quem cai o de não classificação e deixa a competição; no mata-mata quem passa ganha a classificação pela rodada (peso 40 a 100), quem cai o de eliminado, o campeão e o vice os registros da final, e os três deixam a competição',
          'rule: in the European Championship groups a nation going through gets the qualification record (weight 25) and one going out the failed-to-qualify record and leaves the competition; in the knockout one going through gets the qualification by round (weight 40 to 100), one going out the knocked-out record, the winner and runner-up the final records, and those three leave the competition'),
         ('euro_champ_set_club_progress', 'EuroChamp::set_club_progress', '0x57cc30', 'euro_champ.cpp', 'thiscall, ret 0x14',
          'guarda o progresso de uma seleção nas estatísticas da Eurocopa: nos grupos as colocações 1 a 4 viram os códigos da copa (atual 5, 9, 9, 13; melhor 1, 1, 9, 13; pior 5, 9, 9, 13); depois, como vêm',
          'stores a nation\'s progress in the European Championship statistics: in the groups places 1 to 4 become the cup\'s codes (current 5, 9, 9, 13; best 1, 1, 9, 13; worst 5, 9, 9, 13); afterwards, as they come')])
CAMPOS = []
INTERESSA = []
