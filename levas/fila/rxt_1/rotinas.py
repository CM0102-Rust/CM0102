# O texto da noticia da busca do olheiro, do player_search.cpp (leitor rxt, rodada 6, 25/11/2026): 0x7b81f0, 5 KB,
# inteira num desvio. Chamada so pelo despacho NewsItem::write_text (0x782a36; NEWS_WRITERS (1, 0x138a), Plain): a
# noticia 0x138a sai do dia de trabalho do olheiro (scout_day_final_report, gra_26) e do progresso da busca, que so
# correm com uma tarefa dada pelo tecnico humano (tela do olheiro). A carreira automatica nao passa.
# TELAS: a prova e o Lucas jogando comparando: dar a um olheiro uma busca por nacao, regiao, competicao, base e
# observar o proximo adversario; abrir as noticias de progresso e a do fim (manchete e corpo).
# Fronteiras: 0x7b81f0..0x7b95b5 (o unico ret em 0x7b95b4; depois enchimento ate 0x7b95c0). Prologo push -1; push
# 0x95f61b (7 bytes, sem relativas); igual no cm0102-1080.exe.
# Convencao: cdecl(noticia, corpo, Text) -> 1 (0 nas caixas de erro). Chamadas conferidas: 0x785830 get_slot
# thiscall(noticia, k (byte)) ret 4; 0x7e52c0 cdecl(noticia, &k, registro); 0x53e140 cdecl(n1, n2, n3, buf, 0xc8, 1,
# id); 0x53e990 cdecl(pessoa, n1, n2, n3, data ou 0); 0x6181d0 cdecl(nacao, buf, 0xc8, 1); 0x616f20 cdecl(regiao (byte),
# buf, 0xc8, 1); 0x7f6cd0 thiscall([0xb74448], vaga (byte), 0) ret 8; 0x7be670 cdecl(buf, clube do tecnico, clube
# observado, faixa (byte), &respostas, n, &jogadores, &ids, &quantos) (rxt_3); 0x4afa40 thiscall(Text); 0x4ae660
# thiscall(Text, s) ret 4; 0x4ae8a0 thiscall(Text, s, codigo, alvo) ret 0xc; 0x66f4e0 cdecl.
# O byte +8 das respostas (o quadro F-0x520) nunca e escrito, e o relatorio 0x7be670 nao o le.
# Defeito do original (a copia mantem): o id de um dos melhores jogadores igual ao total de pessoas passa na conferencia
# (jg, um alem da tabela).
TELAS = True
ROTINAS = [('write_search_update_news_text', 0x7b81f0, [0x6a, 0xff, 0x68, 0x1b, 0xf6, 0x95, 0x00])]
DOCS = ('o texto da noticia da busca do olheiro', "the text of a scout search news item",
        [('write_search_update_news_text', 'player_search_write_search_update_news_text', '0x7b81f0', 'player_search.cpp', 'cdecl',
          'escreve a manchete ou o corpo da noticia da busca do olheiro (0x138a). A tarefa decide o texto, nesta ordem: nacao (o adjetivo de nacionalidade), regiao, competicao (o nome), base ("youth search") e observacao de clube (o nome curto do clube); sem nenhuma, a caixa de erro e 0. Manchete: "busca <lugar> concluida" ou "atualizacao da busca <lugar>" ("Youth search completed"/"Youth scout update", "<clube> scouting completed"). Corpo: "o olheiro <nome> (concluiu a busca e) nao achou nenhum / achou um / achou <n> jogadores adequados na busca <lugar>"; na observacao de clube, o relatorio do adversario (0x7be670) e um vinculo para cada um dos melhores jogadores que ele cita, quando o nome ainda confere. Vinculos: no corpo, o nome do olheiro quando confere; o lugar a busca (0x7e4) quando o olheiro da busca e o da noticia. Defeito do original: o id de jogador igual ao total de pessoas passa na conferencia',
          "writes the headline or body of a scout search news item (0x138a). The assignment decides the wording, in this order: a nation (its nationality adjective), a region, a competition (its name), a youth search (\"youth search\") and club scouting (the club's short name); none of them: the error box and 0. Headline: \"<place> search completed\" or \"<place> search update\" (\"Youth search completed\"/\"Youth scout update\", \"<club> scouting completed\"). Body: \"Scout <name> has (completed his search and) found no / one / <n> suitable players in his <place> search\"; for club scouting, the opposition report (0x7be670) and a link for each best player it names, when his names still match. Links: in the body the scout's name when it matches; the place to the search (0x7e4) when the search's scout is the news item's. Defect of the original: a player id equal to the person count passes the check")])
