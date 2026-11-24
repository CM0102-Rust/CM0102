# O texto da notícia de recorde quebrado de uma competição (comp_stats_write_record_news_text 0x4a7f50, comp_screens.cpp,
# 0x4a7f50..0x4ab00a e as duas tabelas de salto até 0x4ab0c0, 12,6 KB; leitor tel44, rodada 5, 23/11/2026). A rotina
# inteira desviada num item só, em pedaços com nome; chamada pela NewsItem::write_text (0x77bc80, categoria 7 tipo
# 0x3e9: news_club_record do gra_1) pelo objeto Comp_Stats da competição, para as notícias que a send_record_news
# (0x4a2450, já em Rust) entrega. Simulação (notícias), sem LIGAS.
# Convenções conferidas na desmontagem:
# - a rotina: thiscall(Comp_Stats, notícia, Text), ret 8; o ecx só é lido no tipo 1 (a competição +0x502 -> +4 -> id);
#   quadro SEH só com destrutores vazios (0x5a53c0 é um `ret`);
# - a cópia local do recorde (leiaute Comp_Season_Records, 0x269 bytes) construída pelos mesmos construtores, na ordem
#   (0x645800, 0x448670, 0x450d40, 0x450c60, 0x453c50, 0x450d30, 0x49f390, 0x49f3f0); a leitura é o inverso da
#   comp_stats_write_record_to_news (0x4ad9c0, já em Rust), conferida caso a caso pela primeira tabela de salto (cada
#   get_slot 0x785830 e onde o valor é gravado); o tipo 18 grava slot × 0,01 (qword 0x967898) num float; os tipos 21 e
#   22 leem dia e ano para Date::Date (0x549ef0) e Date::operator= (0x417c10); 0x7e5570 (séries), 0x7e4fa0 (jogo) e
#   0x7e52c0 (nome) cdecl(notícia, &índice, destino);
# - os 44 formatos (0x66f4e0, cdecl) conferidos push a push pela segunda tabela de salto, [esp+k] normalizados pelos
#   pushes pendentes; o placar "%d-%d" pelo sprintf do C (0x944c9f): no tipo 10 o maior primeiro;
# - 0x4bffc0 cdecl(rodada, fase, buf, 200, 0) e 0x4bff70 cdecl(0, dword +0xc, dword +0xe, forma): forma 1 nos públicos
#   (6, 7) e 0 nos resultados e nos jogadores num jogo (9, 10, 16, 21, 22); 0x729500 thiscall em [0xb6b7c0] (jogo,
#   clube, adversário, dia, ano; só as palavras são comparadas); 0x53e140 cdecl(3 ids, buf, 200, 1, id); 0x53e990
#   cdecl(pessoa, 3 ids, &data); 0x54bef0 thiscall(data, buf, 200, 0xff); 0x540340 cdecl(data do jogo, nascimento, buf,
#   5); 0x4b0700 cdecl(id da competição);
# - a checagem "registro local nulo" da original (lea de local nunca nulo, linha 0x1bdf) não entra.
# Prólogo: push -1; push 0x95685e (7 bytes, sem relativo). Corpo igual no cm0102.exe e no cm0102-1080.exe.
# Comparador: compare_text_writer (texto, vínculos e gerador) + o retorno; com caixa de erro (sem Text, sem notícia,
# tipo desconhecido) só a original.
# Defeito do original: no tipo 10 (jogo com mais gols) os gêneros seguem o clube do recorde e o adversário mesmo quando
# o adversário venceu e é nomeado primeiro.
ROTINAS = [('comp_stats_write_record_news_text', 0x4a7f50, [0x6a, 0xff, 0x68, 0x5e, 0x68, 0x95, 0x00])]
DOCS = ('o texto da notícia de recorde quebrado de uma competição',
        "the news text of a competition's broken record",
        [('comp_stats_write_record_news_text', 'comp_stats_write_record_news_text', '0x4a7f50', 'comp_screens.cpp', 'thiscall, ret 8',
          'o corpo da notícia de recorde quebrado de uma competição, lido dos campos da notícia (o id da competição, sem uso, o tipo e os campos do tipo): mais títulos (ou troféus, fora de uma divisão de liga), pontos, gols marcados e sofridos, cartões amarelos e vermelhos e média de público do time; maior público de um jogo (dois tipos, com os dois clubes e a rodada); maior vitória (o placar, a forma pelo mando: em casa, fora, contra) e jogo com mais gols (o maior número primeiro, o vencedor nomeado primeiro, vitória ou empate), com o vínculo do placar quando o jogo ainda confere; séries de vitórias e derrotas (de uma data a outra) e invencibilidade e sem vencer (desde uma data), só acima de um jogo; do jogador: gols, assistências, média de nota (acima de −1, com os jogos), prêmios de melhor em campo e cartões na temporada, mais gols num jogo e o mais novo e o mais velho a jogar (a idade na data do jogo), com os dois clubes e a rodada. Cada um só quando o recorde vale (senão o texto vazio); os vínculos dos clubes, da rodada (a competição do jogo, quando há rótulo) e do jogador (quando os nomes ainda conferem). Sem Text, sem notícia ou tipo desconhecido: a caixa de erro e 0',
          "the body of the news of a competition's broken record, read from the news item's fields (the competition id, unused, the kind and the kind's fields): most titles (or trophies, outside a league division), points, goals scored and conceded, yellow and red cards and average attendance of a team; highest attendance at a match (two kinds, with both clubs and the round); record win (the score, the wording by the venue: at home, away, against) and highest scoring game (the higher count first, the winner named first, win or draw), with the score's link when the fixture still matches; runs of wins and defeats (between two dates) and unbeaten and without winning (from a date), only above one game; for a player: goals, assists, average rating (above -1, with the games), man of the match awards and cards in the season, most goals in a match and the youngest and oldest to play (the age on the match date), with both clubs and the round. Each only when its record holds (otherwise an empty text); the links of the clubs, the round (the match's competition, when there is a label) and the player (when his names still match). Without the Text or the news item or with an unknown kind: the error box and 0")])
