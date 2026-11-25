# A terceira divisao belga: o texto da noticia do destino e a lista dos clubes (bel_third.cpp; leitor rx12, rodada 6,
# 25/11/2026). Usa club_of_index, qualifying_fate_line, link_fate_club e link_fate_competition do rx11_1 (ver
# INTEGRAR_ANTES.txt) e compare_text_writer (compare/media.rs).
# Rodam com a Belgica carregada (o levantamento do cx09 deu zero chamadas porque as 8 ligas nao tem a Belgica): o
# texto pela virtual +0x48 (as noticias do fim da temporada: a decisao do titulo e os play-offs de acesso e de
# rebaixamento, maio de 2002), a lista pela virtual +0xa0 (quem pede os clubes da liga: o tecnico do ano de uma liga,
# a IA). LIGAS da Belgica e ATE 2002-08-01.
# Convencoes conferidas na desmontagem:
# - 0x423310 thiscall(clube, destino, fase, codigo, a5..a8, frase, texto) ret 0x28 (vtable 0x967b48 + 0x48): fase
#   (byte com sinal) < 7: Comp::build_fate_news_text 0x48c6d0 com os 10 argumentos; 7, 8, 9 com frase: 0x4b0b80
#   (clube, a5, a8, destino, a6, texto) cdecl; sem frase: 7 0x989da4 (generos do clube x2 e da competicao [+4]+0x52
#   x2, nomes curtos +0x38), vinculos clube e competicao; 8: a7 == [KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION] escolhe
#   0x987594/0x989d78, senao 0x989c18/0x989d40 (destino 1 ou nao); 9: destino 1 0x987594, 2 ou 0xff 0x987564, outro
#   0x4b4420 (clube, a5, destino, texto) cdecl; vinculo do clube; outra fase nada;
# - 0x423f00 thiscall(&lista, &conta) ret 8 (vtable + 0xa0): conta (word) = 0x20; malloc(0x80) em *lista; nulo: caixa
#   linha 0x69b e conta 0; os clubes da tabela +0xb1 (word i < +0x3e) e depois os da tabela de [[+0xc]] a partir de
#   +0x3e, com a contagem do grupo A (relida a cada passo).
# Comparadores: o texto lado a lado em dois Texts; a lista so a original e conferida contra o plano.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_third_build_fate_news_text', 0x423310, [0x53, 0x8b, 0x5c, 0x24, 0x10]),
           ('bel_third_get_club_array', 0x423f00, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a terceira divisão belga: o texto do destino e a lista dos clubes',
        'the Belgian Third Division: the fate text and the club list',
        [('bel_third_build_fate_news_text', 'BelThird::build_fate_news_text', '0x423310', 'comp/leagues/bel_third.cpp', 'thiscall, ret 0x28',
          'regra: na terceira divisão belga, a decisão do título diz que o clube venceu a competição; o play-off de acesso, que subiu ou ficou sem o acesso (ou, contra um clube da segunda divisão, que escapou do rebaixamento ou caiu); o play-off de rebaixamento, que escapou, caiu ou a frase comum de vitória ou derrota; nas fases da liga, a frase da base',
          "rule: in the Belgian Third Division, the title decider says the club won the competition; the promotion play-off, that it won promotion or missed out (or, against a Second Division club, that it avoided relegation or went down); the relegation play-off, that it avoided relegation, went down or the shared win-or-defeat sentence; in the league stages, the base's sentence"),
         ('bel_third_get_club_array', 'BelThird::get_club_array', '0x423f00', 'comp/leagues/bel_third.cpp', 'thiscall, ret 8',
          'devolve uma lista nova com os 32 clubes da terceira divisão belga, os do grupo A e depois os do grupo B',
          'returns a new list of the 32 clubs of the Belgian Third Division, group A first and group B second')])
CAMPOS = []
INTERESSA = []
