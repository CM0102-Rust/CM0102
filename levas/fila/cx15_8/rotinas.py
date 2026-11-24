# A noticia dos classificados para as copas europeias e o texto dela (uefa_seeding.cpp; leitor cx15, rodada 5,
# 24/11/2026). Carreira ENG (sem LIGAS): a noticia sai quando o ranking e refeito (update_season sem diario, 0x90378b, na
# virada do ano, 1/1/2002); o texto quando a noticia e aberta e, com CM_TEXTO_NOTICIAS=1 (CM_TEXTOS=1 na comparacao da
# cadeia), quando e enviada (0x77bf1f no despachante dos textos).
# - 0x905f00 uefa_post_qualifiers_news thiscall() ret 0 (o ecx nao e lido): os clubes da tabela ([0xae23b4], 0x245
#   bytes) ate [0xae235c] - 2 x [0xae2350] com nacao (+0x53), competicao europeia (+0x1db != -1), nacao com +0x11c != 0 e
#   continente (+0x71) com o id da chave 0x9cfa10 numa lista na pilha (600 entradas, sem limite; contador de 16 bits);
#   qsort 0x945333 (cdecl) com 0x906410; noticia 0x2f local (NewsItem::NewsItem 0x77bbe0), slot 0x1e = ano (palavra sem
#   sinal); a cada nacao nova, a anterior fecha as tres listas com -1 (nos slots seguintes: Liga dos Campeoes de 0, Copa
#   da UEFA de 10, Intertoto de 20) e vai a News::send_to_nation_club_managers 0x786520 (ecx = [0xb7431c]; push nacao;
#   push noticia -> thiscall, ret 8), os contadores voltam; slot 0x1f = id da nacao; o id do clube no proximo slot da
#   lista da competicao (chaves 0x9cf6e8, 0x9cf6f0, 0x9cf6f8; outra competicao nao entra). A ultima nacao no fim. A mesma
#   noticia e reaproveitada. O quadro SEH so cobre o NewsItem local. Prologo push -1; push imm32 (7 bytes).
# - 0x906130 uefa_write_qualifiers_news_text thiscall(noticia, corpo, texto) ret 0xc (o ecx nao e lido): manchete
#   0xaa1b50 com o slot 0x1e & 0xffff e (ano + 1) % 100 (idiv); corpo 0xaa1b04 (set_string) e, por competicao com o
#   primeiro slot != -1 (NewsItem::get_slot 0x785830 le so o byte do indice), 0xaa1ae4 com o nome curto (+0x38) e o
#   genero (+0x52) da competicao (registro de 0x6b bytes em [0xae23d0]) e a ligacao 0x7d0; ate dez clubes, 0xaa1ab4 ou,
#   com o slot seguinte -1, 0xaa1acc, com a ligacao 0x7d5. langlib_sprintf 0x66f4e0 (cdecl) no buffer 0xde1f64,
#   Text::append_string 0x4ae770. Prologo mov eax, [esp+8]; sub esp, 0x1c (7 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: a noticia so a original (manda noticias); o texto lado a lado
# (compare_text_writer).
# Defeito do original (0x905f00): a lista dos clubes e um vetor de 600 entradas na pilha sem limite.
SAVE = False
ROTINAS = [('uefa_post_qualifiers_news', 0x905f00, [0x6a, 0xff, 0x68, 0x8b, 0x52, 0x96, 0x00]),
           ('uefa_write_qualifiers_news_text', 0x906130, [0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x1c])]
DOCS = ('a notícia dos classificados para as copas europeias e o texto dela',
        'the European qualifiers news and its text',
        [('uefa_post_qualifiers_news', 'uefa_post_qualifiers_news', '0x905f00', 'uefa_seeding.cpp', 'thiscall, ret 0',
          'junta os clubes europeus com vaga numa copa europeia, ordena-os por nação e competição e manda a cada nação, aos técnicos dos clubes dela, a notícia com os seus classificados para a Liga dos Campeões, a Copa da UEFA e a Intertoto',
          'gathers the European clubs with a place in a European cup, sorts them by nation and competition and sends each nation\'s club managers the news with its qualifiers for the Champions League, the UEFA Cup and the Intertoto'),
         ('uefa_write_qualifiers_news_text', 'uefa_write_qualifiers_news_text', '0x906130', 'uefa_seeding.cpp', 'thiscall, ret 0xc',
          'escreve o texto da notícia dos classificados: a manchete com a temporada; o corpo com cada competição e a lista dos seus clubes',
          'writes the text of the qualifiers news: the headline with the season; the body with each competition and the list of its clubs')])
CAMPOS = []
INTERESSA = []
