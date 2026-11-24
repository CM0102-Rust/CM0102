# O texto da noticia do relatorio dos olheiros, do scout_manager.cpp (leitor sbB, rodada 5, 24/11/2026): 0x7f9d10,
# 9,3 KB, inteira num desvio. Chamada so pelo despacho NewsItem::write_text (0x78280a; NEWS_WRITERS (1, 0x138e),
# Global(0xb74448)); a noticia 0x138e sai de SCOUT_MANAGER::send_scout_report_news (0x7f9a60) para o tecnico do clube
# (news_send_to_club_manager), a partir de send_scout_reports (0x7f9890, cerca de 5 mil chamadas ate 2002-06-01 na
# cobertura da simulacao; quantas noticias saem na carreira inglesa nao se sabe sem rodar). Na carreira automatica roda
# pelo modo CM_TEXTO_NOTICIAS, que escreve o texto de toda noticia enviada (o send_to_club_manager de um clube da
# maquina tambem escreve).
# O Ghidra nao a decompoe: lida pela desmontagem com os deslocamentos da pilha normalizados (2.709 instrucoes, 73
# langlib_sprintf). Fronteiras: 0x7f9d10..0x7fc0e0 (o unico ret 0xc em 0x7fc0dd), depois as tabelas de salto 0x7fc0e0
# (10), 0x7fc108 (5) e 0x7fc11c (10) ate 0x7fc144; 0x7fc150 ja e outra rotina. Prologo push -1; mov eax, fs:[0] (8 bytes,
# sem relativas); o _chkstk (0x1154) vem depois.
# Convencao: thiscall(SCOUT_MANAGER, noticia, corpo, Text), ret 0xc; o this nao e lido. Chamadas conferidas: 0x785830
# get_slot thiscall(noticia, k (byte)) ret 4; 0x7e52c0 e 0x7e4fa0 cdecl(noticia, &k, registro); 0x53e140 cdecl(n1, n2,
# n3, buf, 0xc8, estilo, id); 0x5461b0 cdecl(mascara, buf, 0xc8, 1) -> al; 0x5465a0 cdecl(id, mascara, forcas, buf,
# 0xc8, dword do genero, 1) -> al; 0x90c520 cdecl(n, buf, 0x32, 0); 0x4bfa30 cdecl(0, dword +0xc, dword +0xe, 1, buf,
# 0xc8); 0x4bff70 cdecl(0, +0xc, +0xe, 1); 0x53e990 cdecl(pessoa, n1, n2, n3, data ou 0); 0x729500
# thiscall([0xb6b7c0], partida, c1, c2, d0, d1) ret 0x14; 0x6181d0 cdecl(nacao, buf, 0xc8, 0x15); 0x66f4e0 e 0x944c9f
# cdecl; 0x4ae660/0x4ae770 thiscall(Text, s) ret 4; 0x4ae8a0 thiscall(Text, s, codigo, alvo) ret 0xc. O construtor dos
# 10 registros (0x450d40) so constroi a Date de +0x10; o destrutor (0x5a53c0) e um ret: a copia nao os chama.
# Conferida pela emulacao (unicorn): a original do exe e a copia da ddraw.dll compilada, no mesmo mundo com as chamadas
# trocadas por stubs, 20 mil noticias sorteadas (0..3 olheiros, modo, com e sem partida, fase, reserva, placares,
# veredictos, marcador, elegibilidade, nacao britanica ou nao, nomes que conferem ou nao): os mesmos textos, na mesma
# ordem, com os mesmos argumentos, e os mesmos vinculos.
# Defeitos do original: com varios olheiros, so o primeiro teste da categoria de cada um (veredicto 5) le o veredicto
# dele; os outros leem os do primeiro olheiro (0x7fb438: [F+0xc4] e [F+0x78] fixos); o jogador sem id (-1) no
# paragrafo do passaporte le a pessoa antes da tabela (staff_at(-1)).
SAVE = False
ROTINAS = [('write_scout_report_news_text', 0x7f9d10, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o texto da noticia do relatorio dos olheiros', "the text of a scout report news item",
        [('write_scout_report_news_text', 'SCOUT_MANAGER::write_scout_report_news_text', '0x7f9d10', 'scout_manager.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia do relatorio dos olheiros (0x138e). Manchete pelo numero de olheiros e pelo modo (2: impressionou): "<jogador> impressiona olheiro", "Relatorio sobre <jogador>", "Olheiros impressionados com", "Olheiros observam"; o vinculo do jogador quando os nomes conferem. Corpo: a posicao (ou a descricao do jogador, quando a noticia tem o marcador 0x420e9 e um veredicto do primeiro olheiro e 3 a 5); a frase de quem o observou (o olheiro pelo nome ou o numero por extenso, a idade, a posicao) e da partida, quando houve: o placar, a vitoria, o empate ou a derrota do primeiro clube, com a fase (reserva, ou o nome da fase, "Friendly" em minuscula) e o vinculo da fase a competicao; a opiniao (um olheiro pelos dois veredictos: excelente, util, nao serve para o clube...; varios por categoria, cada uma com um olheiro pelo nome ou a lista dos nomes); a elegibilidade (o visto de trabalho, ou o passaporte da segunda nacao, "britanico" para Inglaterra, Gales, Escocia e Irlanda do Norte); os vinculos do jogador, dos clubes, do placar e de cada olheiro. Defeitos do original: com varios olheiros, a categoria de cada um, depois do veredicto 5, e decidida pelos veredictos do primeiro; o passaporte de jogador sem id le a pessoa antes da tabela',
          "writes the headline or body of a scout report news item (0x138e). Headline by the number of scouts and the mode (2: impressed): \"<player> impresses scout\", \"Scout report on <player>\", \"Scouts impressed by\", \"Scouts watch\"; the player's link when his names match. Body: the position (or the player's description, when the item has the 0x420e9 marker and a verdict of the first scout is 3 to 5); the sentence of who watched him (the scout by name or the number in words, the age, the position) and of the match, when there was one: the score, the first club's win, draw or defeat, with the stage (reserve, or the stage's name, \"Friendly\" lower-cased) and the stage's link to the competition; the opinion (one scout by his two verdicts: excellent, useful, not good enough for the club...; several by category, each with one scout by name or the list of names); the eligibility (the work permit, or the passport of the second nation, \"British\" for England, Wales, Scotland and Northern Ireland); the links of the player, the clubs, the score and each scout. Defects of the original: with several scouts, each one's category after verdict 5 is decided by the first scout's verdicts; the passport of a player without an id reads the person before the table")])
