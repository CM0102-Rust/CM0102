# As telas do clube (2): abrir a tela do clube de um tecnico humano e monta-la (com a aba Transferencias, que no
# original esta dentro da montagem). Leitor tel01 (rodada das telas), 21/11/2026.
# A lista do grupo trazia pedacos (0x454455, 0x454741, 0x454950, 0x454e49, 0x454e4f, 0x45504f, 0x455255, 0x45534e,
# 0x45534f, 0x455441, 0x455547, 0x455551, 0x455942): sao o meio da open_manager_club_screen (0x4542f0..0x4548f6) e da
# club_screen_build, que comeca em 0x454900 (nao em 0x454950) e vai ate o ret em 0x456721 (a tabela de saltos das
# abas em 0x456724). As duas rotinas estao inteiras aqui.
# Convencoes conferidas na desmontagem: ambas cdecl, ret 0. Usa as rotinas do tel01_1 pelo endereco (0x4567f0,
# 0x456850, 0x456890, 0x4567a0, 0x456740/0x456770) e os auxiliares dele (screens_current_display, DisplayAddAreaFn,
# DisplayAddItemFn): o tel01_2 entra junto com o tel01_1.
# Iguais no cm0102-1080.exe (as rotinas inteiras).
# Defeitos do original: a data de 8 (24) semanas atras sai de duas contas (o ano de uma, o dia da outra); as metades
# altas das palavras empurradas ao find_fixtures e ao add_area (as linhas da lista) sao lixo de registrador ou da
# pilha (aqui zero); nas transferencias futuras a taxa de saida conta uma vez por jogador mas a de entrada desconta
# sempre; o total das transferencias passadas nao trata o negativo (o das futuras escreve "-").
# Comparadores: abrir a tela nao roda duas vezes: so a original, e os parametros da nova tela do tecnico (o canal dele)
# conferidos contra o plano calculado antes. A montagem pela lista de exibicao (compare_by_display_list_with) e
# [0x98ded8]; so a original quando aloca: sem clube, sem as copias da ultima e da proxima partida (params 2 e 3), a aba
# Fixtures (4, aloca), as transferencias passadas (o vetor dos params 0x15/0x16 cresce) e aba desconhecida.
TELAS = True
ROTINAS = [('open_manager_club_screen', 0x4542f0, [0x6a, 0xff, 0x68, 0xdb, 0x57, 0x95, 0x00]),
           ('club_screen_build', 0x454900, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as telas do clube (2): abrir a tela do clube e montá-la, com a aba Transferências',
        'the club screens (2): opening the club screen and building it, with the transfers tab',
        [('open_manager_club_screen', 'open_manager_club_screen', '0x4542f0', 'club_screens.cpp', 'cdecl',
          'abre a tela do clube que o técnico humano dirige (o da partida ao vivo, o clube ou a seleção) na aba do elenco ou da tática, com a visão que ele deixou no elenco, e marca os três jogadores de maior reputação',
          'opens the club screen of the club the human manager runs (the live match\'s, his club or his national team) on the squad or tactics tab, with the view he left the squad in, and marks the three players with the best reputation'),
         ('club_screen_build', 'club_screen_build', '0x454900', 'club_screens.cpp', 'cdecl',
          'monta a tela do clube: o título (nome ou batedores de pênalti), as dez abas (as de transferências e treino apagadas para quem não dirige o clube), a aba escolhida (a de transferências aqui mesmo: temporada, visões, lista e saldo), o menu do elenco, o botão de ação ou o menu de impressão; guarda a última e a próxima partida do clube',
          'builds the club screen: the title (name or penalty takers), the ten tabs (transfers and training greyed out for a club the human does not run), the chosen tab (the transfers one right here: season, views, list and balance), the squad menu, the action button or the print menu; keeps the club\'s last and next fixture')])
