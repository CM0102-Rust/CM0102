# O texto da notícia de recorde do clube (club_records_write_record_news_text 0x44c650, club_records.cpp,
# 0x44c650..0x45095f, 17,4 KB, ~4.770 instruções; leitor gra, rodada 4, 21/11/2026). A rotina inteira desviada num item
# só; chamada pela NewsItem::write_text (0x77bc80, categoria 7 tipo 0x3e8) para as notícias que a send_record_news (já
# em Rust) entrega. Convenções conferidas na desmontagem:
# - a rotina: thiscall(objeto [0xad9dac], notícia, corpo, Text), ret 0xc; o ecx não é usado; quadro SEH só com
#   destrutores vazios (0x5a53c0 é um `ret`);
# - a cópia local do recorde (0x38b bytes, o leiaute do club_record) construída pelos mesmos construtores, na ordem;
#   a leitura é o inverso da club_records_write_record_news_fields (0x453120, já em Rust: record_news_fields), conferida
#   caso a caso (cada get_slot 0x785830 e onde o valor é gravado); o tipo 20 grava slot × 0,01 (qword 0x967898) num
#   float; os tipos 25 e 26 leem dia e ano para Date::Date (0x549ef0) e Date::operator= (0x417c10); 0x7e5570 (os
#   recordes de série), 0x7e4fa0 e 0x7e52c0 cdecl(notícia, &índice, destino);
# - os argumentos dos 52 formatos (0x66f4e0, cdecl) conferidos por script (cada push seguido até a chamada, [esp+k]
#   normalizados pelos pushes, os alvos das duas tabelas de salto seguidos) e à mão onde o script não resolvia o
#   registrador (gêneros gravados na pilha); o placar "%d-%d" pelo sprintf do C (0x944c9f): na maior derrota (tipo 9)
#   os gols do adversário primeiro, nos jogos de mais gols (10, 11) o maior primeiro;
# - 0x4bfa30 cdecl(comp, dword +0xc, dword +0xe, 2, buf, 200) e 0x4bff70 cdecl(comp, dword +0xc, dword +0xe, 2);
#   0x729500 thiscall em [0xb6b7c0] (jogo, clube, adversário, dia, ano; dia e ano em ax com lixo em cima, passados
#   estendidos com zero); 0x53e140 cdecl(3 ids, buf, 200, 1, id); 0x53e990 cdecl(pessoa, 3 ids, &data);
#   0x54bef0 thiscall(data, buf, 200, 0xff); 0x540340 cdecl(data do jogo, nascimento, buf, 5); Cash::format 0x450aa0
#   = 0x43f460 thiscall(&valor, flags) ret 4 + 0x441c10 cdecl(buf, 200, i64, flags), flags 0x22 (renda) e 6;
#   0x90c310 cdecl(posição + 1, -1); 0x53b900 cdecl(clube).
# - a checagem "registro nulo" da original (lea de local nunca nulo, linha 0x19a8) não entra.
# Comparador: compare_text_writer (texto, vínculos e gerador) + o retorno; com caixa de erro (sem Text, sem notícia,
# clube fora da tabela, tipo desconhecido) só a original.
SAVE = False
ROTINAS = [('write_record_news_text', 0x44c650, [0x6a, 0xff, 0x68, 0x80, 0x56, 0x95, 0x00])]
DOCS = ('o texto da notícia de recorde do clube',
        'the news text of a broken club record',
        [('write_record_news_text', 'club_records_write_record_news_text', '0x44c650', 'club_records.cpp', 'thiscall, ret 0xc',
          'a manchete ("Novo recorde do <clube>", sem "clube" para seleção; no tipo 32 "<clube> <Jogador do Ano da torcida> decidido") ou o corpo da notícia de recorde quebrado, lido dos campos da notícia (id do clube, tipo, os campos do tipo): melhor e pior posição final na liga (ordinal e competição); maior e menor público e maior renda em casa (adversário e rodada); maior média de público; maior vitória, maior vitória na liga e maior derrota (placar, a forma pelo mando: em casa, fora, contra) e jogo e jogo de liga com mais gols (o maior número primeiro, a forma pelo resultado); séries de vitórias e derrotas (de uma data a outra) e invencibilidade e sem vencer (desde uma data), só acima de um jogo; do jogador: gols e gols na liga na temporada, gols num jogo, assistências, média de nota (acima de −1), prêmios de melhor em campo, cartões, jogos e gols de liga na carreira no clube, o mais novo e o mais velho a jogar (a idade na data do jogo), mais convocações pela seleção; maior compra e maior venda (valor, outro clube, "troca de jogadores" com a marca); gastos e receitas de transferências na temporada; o jogador do ano da torcida (nome longo do clube). Cada um só quando o recorde vale (senão o texto vazio); os vínculos do clube, do adversário, da rodada (competição), do placar (quando o jogo ainda confere) e do jogador (quando os nomes ainda conferem). Sem Text, sem notícia, clube fora da tabela ou tipo desconhecido: a caixa de erro e 0',
          "the headline (\"New <club> club record\", without \"club\" for a national side; for type 32 \"<club> <Fans Player of the Year> decided\") or the body of the news of a broken record, read from the news item's fields (club id, type, the type's fields): best and worst final league position (ordinal and competition); highest and lowest home attendance and highest gate receipts (opponent and round); highest average attendance; record win, league win and defeat (the score, the wording by the venue: at home, away, against) and highest scoring game and league game (the higher count first, the wording by the result); runs of wins and defeats (between two dates) and unbeaten and without winning (from a date), only above one game; for a player: goals and league goals in the season, goals in a match, assists, average rating (above -1), man of the match awards, cards, league appearances and goals in his club career, the youngest and oldest to play (the age on the match date), most caps for the nation; record purchase and sale (fee, other club, \"part-exchange\" with the flag); the season's transfer spending and income; the fans' player of the year (the club's long name). Each only when its record holds (otherwise an empty text); the links of the club, the opponent, the round (competition), the score (when the fixture still matches) and the player (when his names still match). Without the Text or the news item, with a club outside the table or an unknown type: the error box and 0")])
