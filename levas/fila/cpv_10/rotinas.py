# O fim da lesao do jogador (Injury::end_player_injury 0x62e700, pelo fisico do dia 0x62c680, pelas batidas da partida
# 0x62c080 e pelo recomeco do fisico 0x62bfa0: toda lesao que termina na carreira), a noticia do pedido de desculpas
# (media_queue_player_apology_news 0x72bbf0, pelos efeitos da historia 0x736c00 e pela resposta a historia do jogador
# 0x741020) e a caixa de entrada esvaziada (News::clear_inbox 0x787630, pela 0x6b28b0 e pela carga do jogo 0x82a9c0).
# Chamadas originais pelo endereco: 0x58e340 (historico das lesoes), 0x630ee0 (a noticia da volta), a caixa de erro
# (aviso_em); o resto ja em Rust. Iguais no cm0102-1080.exe. Trocas: das candidatas de media.cpp e news.cpp, as que
# rodam na carreira sao grandes (0x74b060 2,9 KB, 0x754a30, 0x755340 1,3 KB so com boato de transferencia, raro;
# 0x784100 3,5 KB) ou so texto de noticia; por isso o item de mutacao e do injury.cpp (familia do agente C) e as de
# media/news vao de passageiras. Simulacao: as mutacoes pela carreira ate 2002-06-01, todas no fim da lesao.
# Comparadores: o fim da lesao lado a lado no tipo e no +0xe quando nada sai da rotina (ferias sem noticia); com o
# historico (arquivo) ou a noticia so a original, conferindo o plano (tipo 0xff, +0xe -1); a noticia so a original; a
# caixa lado a lado (caixa e as 100 noticias), com resposta padrao a dar so a original conferindo o plano.
SAVE = False
ROTINAS = [('fim_da_lesao', 0x62e700, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('noticia_do_pedido_de_desculpas', 0x72bbf0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('esvaziar_caixa_de_entrada', 0x787630, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o fim da lesão do jogador, a notícia do pedido de desculpas e a caixa de entrada esvaziada',
        "the end of the player's injury, the apology news and the emptied inbox",
        [('fim_da_lesao', 'Injury::end_player_injury', '0x62e700', 'injury.cpp', 'thiscall, ret 8',
          'encerra a lesão atual da pessoa: o histórico (fora das férias), a notícia da volta no clube humano (com o pedido), o tipo da entrada 0xff e o +0xe do registro −1',
          "ends the person's current injury: the history (except holidays), the return news at the human club (when asked), entry type 0xff and the record's +0xe −1"),
         ('noticia_do_pedido_de_desculpas', 'media_queue_player_apology_news', '0x72bbf0', 'media.cpp', 'stdcall, ret 0xc',
          'monta a notícia 0xbdb (jogador, nomes, clube, história e a variante sorteada em algumas histórias) e manda ao técnico do clube',
          "builds news 0xbdb (player, names, club, story and the random variant for some stories) and sends it to the club's manager"),
         ('esvaziar_caixa_de_entrada', 'News::clear_inbox', '0x787630', 'news.cpp', 'thiscall, ret 4',
          'dá as respostas padrão do clube e da seleção do técnico (0x7874b0) e esvazia a caixa dele (as 100 notícias com +0xd6 = 0 e +0xda = 1)',
          "gives the default replies for the manager's club and national team (0x7874b0) and empties his inbox (the 100 news items with +0xd6 = 0 and +0xda = 1)")])
CAMPOS = [('Injury +0: registro de 31 bytes por pessoa (+0xe índice da lesão atual, −1 nenhuma; +0x14/+0x18 histórico); +4: tabela de lesões de 0x30 bytes (+0 tipo, 0xff livre, 0x51 férias); +0xc: arquivo do histórico; índice e valor; lê e grava (fim_da_lesao)',
           'Injury +0: 31-byte record per person (+0xe current injury index, −1 none; +0x14/+0x18 history); +4: 0x30-byte injury table (+0 type, 0xff free, 0x51 holiday); +0xc: history file; index and value; reads and writes (fim_da_lesao)'),
          ('pessoa +0x39: clube (humano: a notícia da volta); +0x24: seleção; +0x61: ficha; ponteiro; lê (fim_da_lesao, esvaziar_caixa_de_entrada, noticia_do_pedido_de_desculpas)',
           'person +0x39: club (human: the return news); +0x24: national team; +0x61: player data; pointer; reads (fim_da_lesao, esvaziar_caixa_de_entrada, noticia_do_pedido_de_desculpas)'),
          ('News +8: caixas dos humanos (12 bytes: +0 primeira notícia, +4 cópia, +8 0); +0: notícias de 0xdf bytes (+0xd6 0, +0xda 1); índice e valor; grava (esvaziar_caixa_de_entrada)',
           'News +8: human inboxes (12 bytes: +0 first news item, +4 copy, +8 0); +0: 0xdf-byte news items (+0xd6 0, +0xda 1); index and value; writes (esvaziar_caixa_de_entrada)')]
INTERESSA = ['CMRTE']
