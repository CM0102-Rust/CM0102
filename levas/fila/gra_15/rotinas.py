# Os recordes do clube depois de cada partida (Club_Records::update_records_for_match 0x444930, club_records.cpp;
# leitor gra, rodada 4, 21/11/2026). O mapa dá 28 bytes (vizinhanca), mas a função vai de 0x444930 a 0x447423 (ret 8;
# ~11 KB, quadro SEH sem destrutor; a 0x444e49 do gra.txt é um ponto dentro dela). Roda em toda partida da carreira.
# A rotina inteira desviada, partida em pedaços com nome: update_club_match_records (o público, os resultados e as
# séries: attendance_record, result_record, run_record), update_player_match_records (por linha da escalação:
# internacionalizações, artilheiro, artilheiro da liga, gols num jogo, assistências, melhor em campo, disciplina,
# league_tally, player_age_records) e favoured_by_points (os sorteios da preferência pelo jogador que foi bem).
# Convenções conferidas na desmontagem:
# - a rotina: thiscall(Club_Records, partida, registro da partida), ret 8; partida ou registro nulos: a caixa (linhas
#   0x10c e 0x11b) e 0 (a original); 0x7e6f20 thiscall([0xb74434], competição) diferente de 0: 0 sem caixa;
# - 0x449590 stdcall(tipo, clube, 1); 0x4b02e0, 0x53b900, 0x5409b0, 0x669950, 0x4b0700, 0x7e4750, 0x7e4920, 0x7e4a30,
#   0x7e4b00, 0x7e4be0, 0x547a10, 0x547870, 0x90cfc0 cdecl; 0x7c2150 stdcall(registro, campo, 6.5f) (o ecx
#   [0xb74350] não é lido); 0x7c1e30 thiscall([0xb74350], jogador, 2, 2, 6.5f); 0x7c3920 thiscall([0xb74350], jogador,
#   5 ou 6 pelo neg/sbb); 0x7c47b0 thiscall([0xb74350], jogador, 1 ou 2) devolve float; 0x54cc30 thiscall(data, saída,
#   2 dwords) (a idade); 0x417c10 thiscall(destino, temporária, origem); 0x8cca10 thiscall([0xde1f54], pessoa, 0);
#   0x945ff0 pow(st1, st0) (game_pow); 0x945640 _ftol;
# - a cópia da visão da partida (0x1f bytes) é feita em linha pelo original (0x417c10 + os campos): a cópia usa
#   match_record_assign (0x448110), com as mesmas escritas;
# - a reputação do jogo [esp+0x6c] vale pela palavra baixa (cmp di, 5), como os pontos [esp+0x40] nas contas finais
#   (movsx bx);
# - a média de público divide pelos jogos em casa da temporada (byte com sinal): com 0 (depois de 256 jogos) o
#   original falha; a cópia grava 0.
# A "No player stats" mostra a caixa e segue com o registro nulo (como o original).
# Comparador: lado a lado depois de criar os quatro recordes na ordem do original (o mandante: temporada e de sempre;
# o visitante: temporada e de sempre), com regiões nomeadas (os quatro recordes, as duas entradas de 0x6a bytes, o
# gerador e as listas de preferidos do clube, do presidente, do técnico e do elenco); a fila de notícias pela cópia a
# seco (MATCH_RECORDS_NEWS_DRY_RUN: os itens que a original acrescentou, mesmo se realocou). A 0x44c650 (o texto da
# notícia de recorde) não é chamada aqui (só pela send_record_news, já em Rust).
SAVE = False
ROTINAS = [('update_records_for_match', 0x444930, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('os recordes do clube depois de cada partida',
        "the club's records after each match",
        [('update_records_for_match', 'Club_Records::update_records_for_match', '0x444930', 'club_records.cpp', 'thiscall, ret 8',
          'depois de cada partida (fora das competições de reservas), para o mandante e o visitante: mais um jogo na temporada; em casa, o público soma na média da temporada; os recordes da temporada e de sempre (cada um só se o tipo vale para o clube): maior público, maior renda e menor público em casa, maior vitória, maior vitória na liga, maior derrota, jogo com mais gols e jogo da liga com mais gols, e as séries de vitórias, derrotas, invencibilidade e sem vencer (a vitória soma nas vitórias e na invencibilidade e encerra as derrotas e o sem vencer; o empate soma na invencibilidade e no sem vencer; a derrota soma nas derrotas e no sem vencer); depois, para cada jogador que entrou: o mais convocado (seleção, só de sempre); fora dos amistosos, o artilheiro, o artilheiro da liga, os gols num jogo, as assistências, o melhor em campo, a pior disciplina, os jogos e gols pela liga no clube e o jogador mais novo e o mais velho; o recorde de sempre batido entra na fila de notícias quando já não está lá (por limites: 50 jogos em casa para o público, 6 gols de diferença, 8 gols no jogo, séries de 6 e 12, 3 temporadas para os dos jogadores, menos de 20 e mais de 35 anos para as idades); os pontos do jogador ((nota − 6)^4 acima de 7, (gols + 1)^4, (assistências + 1)^3, 30 pelo melhor em campo, 100 ou 50 por recorde, × 1,25 pelo mais velho) de 40 para cima, num jogo de reputação 5 ou mais e sem transferência futura, ajustados pelo rival, pelo clássico e pela pequena importância do jogo, sorteiam se o presidente, o técnico, o clube e cada jogador do elenco passam a preferi-lo; fora dos amistosos a melhor média de nota da temporada é refeita depois',
          "after each match (outside reserve competitions), for the home and away sides: one more game this season; at home, the attendance adds to the season average; the season and all-time records (each only when its type applies to the club): highest attendance, highest gate receipts and lowest attendance at home, biggest win, biggest league win, biggest defeat, highest scoring game and highest scoring league game, and the runs of wins, defeats, games without losing and without winning (a win adds to the wins and the unbeaten run and ends the defeats and the winless run; a draw adds to the unbeaten and winless runs; a defeat adds to the defeats and the winless run); then, for each player who played: the most capped (national side, all-time only); outside friendlies, the top scorer, top league scorer, goals in a match, assists, man of the match, worst discipline, league appearances and goals at the club and the youngest and oldest players; a broken all-time record joins the news queue when it is not there yet (with thresholds: 50 home games for the attendance, a 6-goal margin, 8 goals in the match, runs of 6 and 12, 3 seasons for the players', under 20 and over 35 for the ages); the player's points ((rating − 6)^4 above 7, (goals + 1)^4, (assists + 1)^3, 30 for the man of the match, 100 or 50 per record, × 1.25 for the oldest) from 40 up, in a match of reputation 5 or more and without a future transfer, adjusted for a rival, a clash and a match of little importance, draw whether the chairman, the manager, the club and each squad player come to favour him; outside friendlies the season's best average rating is recomputed later")])
