# O convocado machucado (national_teams.cpp): NationalTeams::withdraw_injured_player (0x775cd0, 1.315 chamadas na
# cobertura de 17/10, 140 com a retirada inteira), a notícia da saída ao clube do jogador (0x76f7d0, 140 chamadas, só
# monta a notícia com técnico humano) e o pedido de substituto ao técnico humano da seleção (0x7701f0, passageira: na
# carreira automática os técnicos das seleções são da IA). A 0x775cd0 só chama rotinas já em Rust ou puras (0x456740,
# 0x896c60) e as duas notícias; quadro sem SEH. As notícias têm quadro SEH sem destrutor (a NewsItem local não tem). Os
# dwords que a original empurra com o tipo e o nível só no byte de baixo (o resto é sobra de pilha ou o endereço do
# registro) vão como a original grava ou estendidos; as chamadas leem só o byte. Iguais no cm0102-1080.exe.
# Comparadores: a retirada só pela original, com o caminho decidido antes (só lendo) e conferido depois (o +0x24 e o
# +0x28 do jogador); as notícias só pela original (alocam e vão para a caixa de entrada).
SAVE = False
ROTINAS = [('tirar_convocado_machucado', 0x775cd0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('noticia_da_saida_do_convocado', 0x76f7d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_do_pedido_de_substituto', 0x7701f0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o convocado machucado: tirar da seleção, a notícia ao clube dele e o pedido de substituto',
        "the injured call-up: withdrawing him from the national squad, the news to his club and the replacement request",
        [('tirar_convocado_machucado', 'NationalTeams::withdraw_injured_player', '0x775cd0', 'national_teams.cpp', 'thiscall, ret 8',
          'com 14 dias de lesão ou mais, tira o convocado da tática e do elenco da seleção; antes da data do tipo, avisa o clube, abre a vaga e convoca o substituto ou pede ao técnico humano',
          'with 14 or more injury days, removes the call-up from the national tactics and squad; before the squad date, tells his club, opens the slot and calls up a replacement or asks the human manager'),
         ('noticia_da_saida_do_convocado', 'national_teams_post_player_withdrawn_news', '0x76f7d0', 'national_teams.cpp', 'thiscall, ret 0xc',
          'monta a notícia 0x20 (o jogador saiu da seleção machucado) e entrega ao clube dele e ao do contrato quando são de técnico humano',
          'builds news item 0x20 (the player left the national squad injured) and delivers it to his club and his contract club when human-managed'),
         ('noticia_do_pedido_de_substituto', 'national_teams_post_replacement_needed_news', '0x7701f0', 'national_teams.cpp', 'thiscall, ret 0x10',
          'monta a notícia 0x22 (é preciso um substituto) com o time, o elenco, o nível e o tipo, e entrega ao técnico humano da seleção',
          "builds news item 0x22 (a replacement is needed) with the team, squad count, level and type, and delivers it to the national side's human manager")])
CAMPOS = [('staff +0x24: o clube da seleção do convocado; ponteiro; lê (tirar_convocado_machucado; zerado pela 0x7762f0)',
           "staff +0x24: the call-up's national side club; pointer; reads (tirar_convocado_machucado; cleared by 0x7762f0)"),
          ('staff +0x28: a marca de convocado (0xb); valor; lê (tirar_convocado_machucado)',
           'staff +0x28: the call-up mark (0xb); value; reads (tirar_convocado_machucado)'),
          ('staff +0x5e: as marcas de elenco (bit 3 = tipo 1); valor; lê (tirar_convocado_machucado)',
           'staff +0x5e: the squad flags (bit 3 = type 1); value; reads (tirar_convocado_machucado)'),
          ('staff +0x39: o clube do jogador; ponteiro; lê (noticia_da_saida_do_convocado)',
           "staff +0x39: the player's club; pointer; reads (noticia_da_saida_do_convocado)"),
          ('registro da nação nas seleções ([NationalTeams+4] + 0x6e × id) + 20 × tipo: +0x3a a data do tipo; valor; lê; +0x45 as vagas a preencher (byte); valor; lê e grava +1 (tirar_convocado_machucado)',
           'nation record in the national teams ([NationalTeams+4] + 0x6e × id) + 20 × type: +0x3a the squad date; value; reads; +0x45 the open slots (byte); value; reads and writes +1 (tirar_convocado_machucado)'),
          ('[NationalTeams+0] a lista dos clubes com data (id, ?, data em +8); ponteiro; lê e anda o cursor +0xa (tirar_convocado_machucado)',
           '[NationalTeams+0] the list of clubs with a date (id, ?, date at +8); pointer; reads and moves the cursor +0xa (tirar_convocado_machucado)'),
          ('lesão (0x62ed70) +0: o tipo da lesão (0x51 = doença: só sai); valor; lê (tirar_convocado_machucado, as notícias)',
           'injury (0x62ed70) +0: the injury type (0x51 = illness: just leaves); value; reads (tirar_convocado_machucado, the news)'),
          ('clube +0xcf: o técnico do clube; ponteiro; lê (tirar_convocado_machucado)',
           "club +0xcf: the club's manager; pointer; reads (tirar_convocado_machucado)"),
          ('[0xae1080] o contrato achado por último; ponteiro; grava (noticia_da_saida_do_convocado)',
           '[0xae1080] the last contract found; pointer; writes (noticia_da_saida_do_convocado)')]
INTERESSA = ['CMRTE']
