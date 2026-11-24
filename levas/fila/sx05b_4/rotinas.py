# O texto da noticia do premio do ano (award/year_award.cpp; leitor sx05b, rodada 5, 23/11/2026): YEAR_AWARD::write_news,
# 2,7 KB (0x930880 a 0x931349 mais a tabela de saltos de 5 entradas em 0x93134c), sozinha no item, partida em pedacos
# com nome (YearNews::read, year_news_body, year_news_headline, year_news_links). Usa award_base e year_award_obj
# (sx05b_5): integrar depois dele.
# O texto e pedido pelo AWARD_MANAGER::write_award_news (0x4143f0, ja Rust, virtual +0xc do premio) quando a noticia
# de tipo 0x1b do sx05b_3 (YEAR_AWARD::post_winner_news) e lida: os premios do ano ingleses no fim da temporada.
# LIGAS = 'ENG' e ATE = '2002-08-01', como o sx05b_3.
# Convencoes conferidas na desmontagem (cada push e cada [esp+k] seguidos ate a chamada, com um script que segue a
# pilha, e conferidos a mao em cada um dos 19 modelos):
# - thiscall(noticia, Text, corpo) ret 0xc; NewsItem::get_slot 0x785830 das vagas 2 a 0x16 em ordem; a vaga 0xd igual
#   a 0x420e9 marca a noticia atual; write_position_name 0x5461b0 cdecl(mascara, texto, 0xc8, 1) -> al so na noticia
#   atual de vencedor sem ficha de nao jogador; 0x53e140 cdecl(n1, n2, n3, buf, 0xc8, estilo, id) (o vencedor nos
#   estilos 1 e 5, o segundo, e o terceiro so na noticia atual com terceiro; senao o buffer zerado); sprintf "%d/%02d"
#   (o ano menos o byte +0x1d com sinal, o ano modulo 100) ou "%d";
# - o corpo: o modelo simples 0xad5b68 com noticia antiga, o id do primeiro nome do vencedor multiplo de 10 (idiv com
#   sinal), vencedor com ficha de nao jogador ou o premio de chave KEY_STAFF_COMP_FRENCH_MOST_UNPOPULAR_PLAYER_OF_THE_YEAR;
#   senao doze modelos pelo terceiro (id > -1), o sorteio (0, 1, outro) e o clube do vencedor (id > -1);
#   write_player_description 0x5465a0 cdecl(id, mascara, marcas, texto, 0xc8, o dword do genero da posicao, 1) -> al
#   nos modelos sem clube (e no de dois vencedores, sorteio 2, com clube); o original empilha o dword [esp+0x10] cujos
#   bytes altos sao lixo de pilha: a copia passa o byte (a 0x5465a0 so usa o byte; o comparador acusaria);
# - a manchete: o premio da chave acima "award decided" (0xad59e4, o nome curto +0x38 e o genero +0x52 da competicao);
#   senao (id do segundo / max(sorteio, 1) + id do vencedor) % 6 (idiv com sinal) pela tabela 0x93134c, 5 ou negativo
#   "award decided";
# - os vinculos: os clubes (0x7d5; o terceiro so na noticia atual), os vencedores cujos nomes batem (0x53e990 cdecl;
#   0x7d7; o vencedor duas vezes, nos estilos 1 e 5) e a competicao duas vezes (nome +4 e nome curto +0x38, 0x7d4).
# Comparador: compare_text_writer (o texto, os vinculos e o gerador).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('year_award_write_news', 0x930880, [0x81, 0xec, 0x48, 0x04, 0x00, 0x00])]
DOCS = ('o texto da notícia do prêmio do ano',
        'the year award news text',
        [('year_award_write_news', 'YEAR_AWARD::write_news', '0x930880', 'award/year_award.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia do prêmio do ano: lê as vagas e monta os nomes (o vencedor também no estilo curto), o texto da posição do vencedor e o período ("%d" ou "%d/%02d"); a manchete é uma de cinco pela conta dos ids dos dois primeiros (ou "prêmio decidido"); o corpo é o modelo simples (notícia antiga, primeiro nome com id múltiplo de 10, vencedor com ficha de não jogador ou o prêmio de jogador mais impopular da França) ou um de doze pelo terceiro, o sorteio e o clube do vencedor; depois os vínculos dos clubes, dos vencedores e da competição',
          'the headline or the body of the year award news: reads the slots and builds the names (the winner also in the short style), the winner\'s position text and the period ("%d" or "%d/%02d"); the headline is one of five by the sum of the first two\'s ids (or "award decided"); the body is the plain template (an old item, a first name id that is a multiple of 10, a winner with a non-player sheet or the French most unpopular player award) or one of twelve by the third, the draw and the winner\'s club; then the links of the clubs, the winners and the competition')])
CAMPOS = []
INTERESSA = []
