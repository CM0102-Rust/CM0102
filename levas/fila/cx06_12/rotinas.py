# Os registros dos tecnicos, o texto do destino e a mudanca de situacao das selecoes nas eliminatorias da Eurocopa
# (euro_champ_qual.cpp; leitor cx06, rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2004-11-01 (os classificados
# saem no fim dos grupos, out/2003, e na repescagem, nov/2003; a mudanca de situacao a cada rodada desde set/2002); na
# cobertura do mundo ate 2002-08-01 as tres tem 0 chamadas. O mesmo ATE do cx06_11 (e dos itens da Copa da Asia do cx05).
# - 0x57d5b0 EuroChampQual::update_staff_records_for_status thiscall(clube, desfecho, fase, &saida, partida, a6) ret 0x18
#   (virtual 0x44): antes da repescagem (fase < +0x101, com sinal): 7 -> Staff_Records::add_qualified_for_round_record
#   0x86bdd0 para a fase final (o registro [0xae23d0] + 0x6b x [0x9cf7a4]; 0, 0, 0x64), EuroChamp::add_qualified_team
#   0x57c120 (thiscall, ret 4) no objeto [0xadadfc][id], *saida = 3 se houver, NationalTeams::add_qualified_nation
#   0x7779b0 (thiscall, ret 8) e -1; 1 e 0xfe -> -1; outros 0. Na repescagem: 1 -> o mesmo com peso 0x28 e *saida = 3, 0;
#   2 -> add_knocked_out_record 0x86c000 (+0x32, +0x30 da partida, 0x28) e NationalTeams::clear_current_competition
#   0x775000, 0. Prologo mov al, [esp+0xc]; mov dl, [ecx+0x101] (10 bytes).
# - 0x57d700 EuroChampQual::build_fate_news_text thiscall(clube, desfecho, fase, grupo, partida, a6, a7, a8, noticia,
#   Text) ret 0x28 (virtual 0x48): grupo 0x3e8..0x3f0: 7 com noticia comp_util_write_group_qualify_news 0x4b4590 (cdecl,
#   7), sem ela "qualify for" (0x9c470c) com a fase final; 0xfe a noticia do grupo com -2; grupo 0x3fc com 7: com noticia
#   "best second placed" (0x9c4800 com +0xf6 = 1, senao 0x9c4764) com o nome completo (+4/+0x37) das eliminatorias, sem
#   ela "qualify for"; senao, partida da repescagem (+0x30 = 0xa4): com noticia comp_util_write_cup_tie_result_news
#   0x4b0b80, sem ela 1 "qualify for" e o resto "out of" (0x9c46b8). langlib_sprintf 0x66f4e0 no buffer 0xde1f64,
#   Text::set_string 0x4ae660, os vinculos 0x7d5 e 0x7d0. Prologo mov eax, [esp+0x10]; push esi (5 bytes).
# - 0x580b80 EuroChampQual::get_qualification_status_change thiscall(clube) ret 4 (virtual 0x74): as estatisticas 0x13
#   (antes) e 0x12 (agora) do clube (Comp_Stats::get_club_stat 0x4a2e10, _ftol 0x945640) contra 14 (o byte baixo, com
#   sinal): agora < 14 -> -1 se antes nao, senao 0; agora >= 14 -> 1 se antes < 14, senao 0 (os bytes altos de eax sao os
#   do _ftol do agora). Prologo push ebx; push esi; push edi; mov edi, [esp+0x10] (7 bytes).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: os registros so a original (historicos, fase final, convocacoes), com o plano (o retorno e a saida 3);
# o texto lado a lado nos caminhos so de texto (compare_text_writer; com o modo CM_TEXTO_NOTICIAS); a mudanca de
# situacao lado a lado (so le).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('euro_champ_qual_update_staff_records_for_status', 0x57d5b0, [0x8a, 0x44, 0x24, 0x0c, 0x8a, 0x91, 0x01, 0x01, 0x00, 0x00]),
           ('euro_champ_qual_build_fate_news_text', 0x57d700, [0x8b, 0x44, 0x24, 0x10, 0x56]),
           ('euro_champ_qual_get_qualification_status_change', 0x580b80, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10])]
DOCS = ('os registros dos técnicos, o texto do destino e a mudança de situação das seleções nas eliminatórias da Eurocopa',
        "the European Championship qualifiers' staff records, fate news text and status change",
        [('euro_champ_qual_update_staff_records_for_status', 'EuroChampQual::update_staff_records_for_status', '0x57d5b0', 'euro_champ_qual.cpp', 'thiscall, ret 0x18',
          'registra o destino de uma seleção nas eliminatórias: classificada nos grupos (peso 100) ou na repescagem (peso 40), entra na fase final da Eurocopa e na lista das classificadas; eliminada na repescagem, o registro de eliminação e o fim da competição da seleção',
          "records a nation's fate in the qualifiers: qualified from the groups (weight 100) or the play-off (weight 40), it goes into the European Championship finals and the qualified list; out in the play-off, the knocked-out record and the end of the nation's competition"),
         ('euro_champ_qual_build_fate_news_text', 'EuroChampQual::build_fate_news_text', '0x57d700', 'euro_champ_qual.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de uma seleção nas eliminatórias: classificada nos grupos ("classifica-se para a Eurocopa" ou a notícia do grupo), eliminada nos grupos (a notícia do grupo), entre os melhores segundos colocados, e na repescagem classificada ou eliminada (ou a notícia do confronto)',
          "writes the news text of a nation's fate in the qualifiers: qualified from the groups (\"qualify for the European Championship\" or the group news), out in the groups (the group news), among the best runners-up, and in the play-off qualified or out (or the tie news)"),
         ('euro_champ_qual_get_qualification_status_change', 'EuroChampQual::get_qualification_status_change', '0x580b80', 'euro_champ_qual.cpp', 'thiscall, ret 4',
          'diz como mudou a situação de uma seleção nas eliminatórias pelos códigos de posição antes e agora contra 14: −1 se entrou numa vaga, 1 se saiu, 0 se não mudou',
          "tells how a nation's qualifying situation changed from its previous and current position codes against 14: −1 if it moved into a place, 1 if it dropped out, 0 if unchanged")])
CAMPOS = []
INTERESSA = []
