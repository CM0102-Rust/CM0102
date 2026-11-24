# A virada, os resultados e o fim da temporada da Primeira Divisao argentina (arg_prm.cpp; leitor cx08, rodada 5,
# 24/11/2026). Simulacao com a Argentina (as ligas do cx01_8, ate 2002-08-01): o add_result roda a cada jogo, o
# record_season_honours no fim do Clausura (meados de 2002) e o start_new_season na virada de 2002.
# - 0x405720 ArgPrm::start_new_season thiscall() ret 0: a virtual 0x34 (push 1: os promedios rolam), a virtual 0xb0
#   (push 1), 0x4068d0 (o troca-troca da Segunda com a Terceira), League::limit_club_division_tier 0x68a980 (thiscall,
#   push 1; push -3; push 3; push 1) nesta liga, na fase 0 e na Segunda ([[0xadadfc] + [0x9cf658] x 4]); +0x4c = 0;
#   free_table, as datas, archive_season_and_reset 0x4a1c50 (push 1), as fases ate a atual; +0x40 + 1 e +0x30 = -1; as
#   caixas 0x263, 0x26a, 0x270 e 0x277 (set_up_rules virtual 0x8c, 0x405540, create_fixtures 0x6835c0, sort_table
#   0x6827d0 push 0); 0x405950 (as duas fases); a virtual 8 da Segunda; a virtual 0x5c; set_club_season_expectations
#   0x68aa80 e renew_officials 0x79cee0 (o eax deste volta). Prologo sub esp, 0x200.
# - 0x405c50 ArgPrm::add_result thiscall(partida, arg) ret 8 -> 1: League::add_result 0x685d30 duas vezes, a segunda com
#   +8 = 0 e a fase da partida (+0x42) = 1. Prologo push ebx; push ebp; mov ebp, [esp+0x10] (6 bytes).
# - 0x405ca0 ArgPrm::record_season_honours thiscall() ret 0: a virtual 0x34 (push 0); as duas ultimas linhas dos
#   promedios (+0x18, linhas de 0x13, contagem +0x34): virtual 0x40 (push 0; push 0; push -1; push 3; push clube) e 0x68
#   (push 0x13 ou 0x14 tres vezes; push -2; push clube), ou as caixas 0x31c e 0x32b; status 3 nas linhas desses clubes na
#   tabela e nas fases 0 e 1; League::record_season_honours 0x684640 e a virtual 0x30 da fase 0. Prologo sub esp, 0x200.
# Os nomes UPDATE_RELEGATION_AVERAGES_SLOT, ARG_AVERAGES, ARG_AVERAGE_COUNT, ARG_AVERAGE_ROW_SIZE, arg_average_row_club e
# this_average_count ficam aqui (o cx08_5 e o cx08_6 os usam).
# Comparadores: so a original nas tres (mexem em clubes, resultados e noticias): a virada pelo ano + 1; o resultado pela
# fase da partida e as estatisticas de volta; o fim da temporada pelo status 3 dos dois rebaixados.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_prm_start_new_season', 0x405720, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('arg_prm_add_result', 0x405c50, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10]),
           ('arg_prm_record_season_honours', 0x405ca0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada, os resultados e o rebaixamento por promédio do campeonato argentino', 'the Argentine Premier Division turnover, results and relegation by averages',
        [('arg_prm_start_new_season', 'ArgPrm::start_new_season', '0x405720', 'arg_prm.cpp', 'thiscall, ret 0',
          'vira a temporada da Primeira Divisão argentina: os promédios rolam uma temporada; acesso e rebaixamento com a Segunda e o troca-troca da Segunda com a Terceira; o nível de divisão dos clubes limitado; a tabela, as datas, a temporada das estatísticas e as fases zeradas; o ano + 1; as regras, a tabela, os jogos, a ordem e as duas fases de novo; a virada da Segunda; as previsões, as expectativas da diretoria e os árbitros',
          'turns over the Argentine Premier Division season: the averages roll a season; promotion and relegation with the Second Division and the swap between the Second and Third; the clubs\' division tier capped; the table, the dates, the statistics\' season and the stages cleared; the year + 1; the rules, the table, the fixtures, the order and the two stages again; the Second Division\'s turnover; the predictions, the board expectations and the officials'),
         ('arg_prm_add_result', 'ArgPrm::add_result', '0x405c50', 'arg_prm.cpp', 'thiscall, ret 8',
          'um resultado do campeonato argentino conta na tabela da liga e de novo na fase geral (fase 1), sem as estatísticas',
          'an Argentine Premier Division result counts in the league table and again in the overall stage (stage 1), without the statistics'),
         ('arg_prm_record_season_honours', 'ArgPrm::record_season_honours', '0x405ca0', 'arg_prm.cpp', 'thiscall, ret 0',
          'no fim da temporada argentina os dois clubes de pior promédio de pontos em três temporadas caem: cada um é anunciado como rebaixado, tem o progresso gravado e fica marcado como rebaixado na tabela e nas duas fases; depois as honras da temporada da liga e da primeira fase',
          'at the end of the Argentine season the two clubs with the lowest points average over three seasons go down: each is announced as relegated, has its progress set and is marked relegated in the table and in both stages; then the season honours of the league and of the first stage')])
CAMPOS = [('liga +0x18 tabela dos promédios (linhas de 0x13), +0x34 número de linhas', 'league +0x18 relegation averages table (0x13-byte rows), +0x34 row count')]
INTERESSA = []
