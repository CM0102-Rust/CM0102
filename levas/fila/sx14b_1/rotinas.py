# Os pedidos a federacao (MANAGER_MANAGER::handle_fa_request 0x6a9e60, post_fa_request_reply_news 0x6aaa90,
# post_match_rescheduled_news 0x6aa7d0, manager_manager.cpp; leitor sx14b, rodada 5, 23/11/2026). As tres rotinas
# inteiras (0x6a9e60..0x6aa7cd, ret 8; 0x6aaa90..0x6aab65, ret 0x10; 0x6aa7d0..0x6aa901, ret 0xc; o resto ate a proxima
# fronteira e enchimento).
# Chamadas: a 0x6a9e60 por update_board_confidence_after_match (0x69594e, Rust: clubes da maquina, pedido 1, 7 vezes na
# carreira ENG ate 2002-06-01; 120 com todas as nacoes) e pela tela dos pedidos a federacao (0x58b296, FIFA_CONFED,
# Rust); a 0x6aaa90 so pela 0x6a9e60 (4 lugares; 6 vezes no mundo, 0 na ENG); a 0x6aa7d0 so pela 0x6a9e60 (0x6aa474,
# adiamento concedido: so clube humano).
# Convencoes conferidas na desmontagem:
# - 0x6a9e60 thiscall(this, clube, pedido), ret 8; quadro SEH com o vetor de dois blocos de 0x625 bytes (0x9463af /
#   0x9462bb, 0x453c30 / 0x453c10, sem uso), a LLIST (0x68d020 / 0x68d040, 0x68d1b0 / 0x68d1d0) e a data (0x54a110);
#   0x53b900, 0x600a50 (clube, 0, 1, 0, 0), 0x669360, 0x90cfc0, 0x539c60, 0x53a1b0 cdecl; 0x758de0 thiscall([0xb6b7c8],
#   clube, tecnico, tipo, -1, -1, c) ret 0x18; 0x5a0590 thiscall([0xae2a38], clube); 0x599850 thiscall(financa, 25000);
#   0x69aba0 thiscall([0xb67a44], clube, 0); 0x54c770 / 0x54cab0 thiscall(hoje, &tmp, 8); 0x5acbe0 thiscall([0xae2a58],
#   d1, a1, d2, a2, id, lista, -1, -2, 1) (as metades altas das palavras sao lixo de registrador no original);
#   0x549ef0 thiscall(tmp, dia, ano); 0x417c10 thiscall(data, &tmp, origem); 0x7757d0 thiscall([0xb74318], jogador,
#   &data, clube, 0xde1f64, 0); 0x4cf5a0 thiscall([0xae1050], jogador); 0x62c400 thiscall([0xb63d28], jogador);
#   0x5abc30, 0x5ab510 thiscall([0xae2a58], f); 0x5aa680 thiscall([0xae2a58], f, 1); 0x79d3e0 (float em st0, comparado
#   com 5.5 de 0x96d298), 0x79d3b0 (char), 0x79d190 thiscall([0xb74340], arbitro); 0x6aaa90 e 0x6aa7d0 pelo endereco.
# - 0x6aaa90 thiscall(this, clube, pedido, resultado, nota), ret 0x10 (os tres ultimos so o byte baixo).
# - 0x6aa7d0 thiscall(this, partida, id do clube, contagem), ret 0xc (a contagem so o byte baixo).
# - 0x77bbe0 thiscall(noticia, tipo, 0); 0x785790 thiscall(noticia, k, v) ret 8; 0x7861e0 / 0x7863f0 thiscall([0xb7431c],
#   noticia, clube/nacao).
# Prologos (fronteira de instrucao, sem relativo; iguais no 1080): 0x6a9e60 `push -1; push 0x95cfb8` (7); 0x6aaa90 e
# 0x6aa7d0 `mov eax, fs:[0]` (6).
# x87: so a comparacao da media do arbitro com 5.5 (a copia le o float devolvido e compara em double).
# Comparadores: so a original nas tres (a 0x6a9e60 mexe no calendario, renomeia arbitro, multa, sorteia e posta; as
# duas noticias vao para a caixa do tecnico, que aloca).
SAVE = False
ROTINAS = [('handle_fa_request', 0x6a9e60, [0x6a, 0xff, 0x68, 0xb8, 0xcf, 0x95, 0x00]),
           ('post_fa_request_reply_news', 0x6aaa90, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_match_rescheduled_news', 0x6aa7d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os pedidos à federação',
        'the requests to the FA',
        [('handle_fa_request', 'MANAGER_MANAGER::handle_fa_request', '0x6a9e60', 'manager_manager.cpp', 'thiscall, ret 8',
          'um pedido do técnico à federação (1 reclamação do árbitro, 2 adiamento): clube sem liga, sem técnico ou sem ficha de não jogador, ou seleção, nada. A contagem de pedidos do clube (registro da diretoria +0) sobe; o limite é 3 acima de reputação 3250, senão 2. Reclamação no limite com o estado do clube +4 abaixo de −1: a história da reclamação na mídia (tipo 4). Acima do limite, clube humano ou no limite + 1: a punição (a história do desabafo, tipo 5 com sorteio de 3 + 1, multa de 25000, a confiança da diretoria + 250 ou, com o estado +4 negativo, limite × 250 − a contagem, entre 1 e 10000, e o exame da demissão). Fora isso (os pedidos 5 não), no limite a resposta 3; acima, a resposta 5 e a confiança − 100. Abaixo do limite só o clube humano segue: na reclamação, a última partida jogada das 8 semanas passadas; árbitro com média abaixo de 5,5 e última nota abaixo de 7 é trocado em 1 de 200 (nome novo da nação dele, disciplina sorteada, nascimento sorteado, estatísticas zeradas, notícia 0x2b aos técnicos da nação do clube); senão a resposta 1 com a última nota. No adiamento, a primeira partida das próximas 8 semanas que não é hoje nem foi jogada: de outra competição que a liga do clube, ou com a temporada da nação-chave terminando a menos de 60 dias, a resposta 6; concedido com mais de 2 jogadores convocados naquela data, ou com 2 deles ou mais de 4 jogadores contentes (felicidade acima de 20) e machucados quando a reputação passa de 4750 + sorteio de 1250: a partida sai, ganha data nova e volta, com a resposta 0 e a notícia da remarcação; senão a resposta 1. Defeitos: as faixas de reputação são testadas da menor, então os limites 4 e 7 nunca valem; a notícia da remarcação lê a partida já tirada do calendário',
          "a manager's request to the FA (1 a referee complaint, 2 a postponement): a club without a league, a manager or a non-player record, or a national team, nothing. The club's request count (board record +0) goes up; the limit is 3 above reputation 3250, else 2. A complaint at the limit with the club state +4 below −1: the media story of the complaint (kind 4). Over the limit, for a human club or at the limit + 1: the sanction (the outburst story, kind 5 with random(3) + 1, a 25000 fine, the board confidence + 250 or, with the state +4 negative, limit × 250 − the count, within 1..10000, and the sacking check). Otherwise (not for request 5) at the limit the reply 3; over it, the reply 5 and the confidence − 100. Below the limit only a human club goes on: for the complaint, the last played fixture of the past 8 weeks; a referee with an average below 5.5 and a last rating below 7 is replaced in 1 of 200 (a new name from his nation, a random discipline and birth date, his stats reset, the news 0x2b to the managers of the club's nation); otherwise the reply 1 with his last rating. For the postponement, the first fixture of the next 8 weeks that is not today and not played: of another competition than the club's league, or with the key nation's season ending within 60 days, the reply 6; granted with more than 2 players on international duty that day, or with 2 of them or more than 4 happy (happiness above 20) injured players when the reputation beats 4750 + random(1250): the fixture leaves, gets a new date and goes back, with the reply 0 and the rescheduling news; otherwise the reply 1. Defects: the reputation bands are tested from the lowest, so the limits 4 and 7 never apply; the rescheduling news reads the fixture already taken out of the calendar"),
         ('post_fa_request_reply_news', 'MANAGER_MANAGER::post_fa_request_reply_news', '0x6aaa90', 'manager_manager.cpp', 'thiscall, ret 0x10',
          'posta a notícia 0x1789 da resposta da federação: sem clube ou clube sem nação, nada; campos 0 o clube, 1 o pedido, 2 o resultado, 3 a nação, 4 a nota do árbitro; ao técnico do clube (só entrega a humano)',
          "posts the 0x1789 news of the FA's reply: no club or a club without a nation, nothing; slots 0 the club, 1 the request, 2 the outcome, 3 the nation, 4 the referee's standing; to the club's manager (delivered only to a human)"),
         ('post_match_rescheduled_news', 'MANAGER_MANAGER::post_match_rescheduled_news', '0x6aa7d0', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'posta a notícia 0x29 da partida remarcada: sem partida ou antes da data de início das notícias, nada; campos 0 e 1 os clubes, 2 a competição, 3 o clube que pediu, 4 a nação da competição, 5 a contagem de convocados; aos técnicos dos dois clubes',
          "posts the 0x29 news of the rescheduled fixture: no fixture or before the news start date, nothing; slots 0 and 1 the clubs, 2 the competition, 3 the requesting club, 4 the competition's nation, 5 the count of players on duty; to the managers of both clubs")])
