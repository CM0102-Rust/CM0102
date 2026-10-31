# Os amistosos marcados (friendly.cpp): Friendly::create_due_planned_friendlies (0x5c2ae0, 515 chamadas na cobertura de
# 17/10: o plano da pré-temporada da IA, dia a dia), Friendly::add_friendly_match (0x5c6f00, 464: monta ou reaproveita o
# amistoso e o põe na lista +0x34c8) e Friendly::plan_friendly_on_free_date (0x5c4380, 356: sorteia a data da excursão
# na janela dos dois clubes). O SEH das três só cobre datas e blocos novos durante os construtores (sem destrutor a
# reproduzir). A 0x5c4380 zera o argumento de fora antes de usá-lo como marca de achado (a cópia começa a marca em
# falso). A 0x5c5570 (o pedido do amistoso) fica com a original: monta a notícia na pilha só para o humano. Sobras de
# registrador nas palavras empurradas à 0x5ae870 (dia, ano) e à 0x54c5b0 (o dia sorteado): as duas leem a palavra; a
# cópia passa limpo (comentado). Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo
# real comparando. Comparadores: o plano e a data sorteada rodam só a original (alocam, sorteiam); o amistoso, a
# original faz e a cópia confere o resultado com o estádio de quem joga em casa visto antes.
SAVE = False
ROTINAS = [('criar_amistosos_planejados', 0x5c2ae0, [0x6a, 0xff, 0x68, 0x10, 0x9e, 0x95, 0x00]),
           ('acrescentar_amistoso', 0x5c6f00, [0x6a, 0xff, 0x68, 0x5c, 0xa1, 0x95, 0x00]),
           ('planejar_amistoso_em_data_livre', 0x5c4380, [0x6a, 0xff, 0x68, 0xe4, 0x9f, 0x95, 0x00])]
DOCS = ('os amistosos marcados: criar os amistosos planejados da semana, acrescentar um amistoso e sortear a data livre do amistoso',
        "the arranged friendlies: creating the week's planned friendlies, adding a friendly and drawing the friendly's free date",
        [('criar_amistosos_planejados', 'Friendly::create_due_planned_friendlies', '0x5c2ae0', 'friendly.cpp', 'thiscall, ret 0',
          'anda o plano dos amistosos da IA até hoje + 1 semana: na data, com os clubes livres, cria o amistoso e manda o pedido; plano acabado, libera o vetor',
          "walks the AI friendly plan up to today + 1 week: on the date, with the clubs free, creates the friendly and sends the request; when the plan is used up, frees the array"),
         ('acrescentar_amistoso', 'Friendly::add_friendly_match', '0x5c6f00', 'friendly.cpp', 'thiscall, ret 0x10',
          'reaproveita o convite aberto do clube ou monta um amistoso novo; com o estádio de quem joga em casa livre, grava data, clubes e mando e o põe na lista',
          "reuses the club's open invitation or builds a new friendly; with the home side's stadium free, stores date, clubs and venue and puts it in the list"),
         ('planejar_amistoso_em_data_livre', 'Friendly::plan_friendly_on_free_date', '0x5c4380', 'friendly.cpp', 'thiscall, ret 0xc',
          'sorteia uma data na janela dos dois clubes e, com os dois livres, marca o amistoso e liga as datas e os adversários dos dois',
          'draws a date in both clubs\' window and, with both free, books the friendly and links both clubs\' dates and opponents')])
CAMPOS = [('Friendly +0x34d8: o plano dos amistosos da IA (20 bytes: +0 a data, +8/+0xc/+0x10 os índices dos clubes A, B e C); +0x34dc o índice; +0x34e0 a conta; +0x34e4 a capacidade; ponteiro e valor; lê e grava (criar_amistosos_planejados)',
           'Friendly +0x34d8: the AI friendly plan (20 bytes: +0 the date, +8/+0xc/+0x10 club indices A, B and C); +0x34dc the index; +0x34e0 the count; +0x34e4 the capacity; pointer and value; reads and writes (criar_amistosos_planejados)'),
          ('Friendly +0x34c8: a lista dos amistosos; +0xa7 o início da temporada (data); ponteiro e valor; lê e cria (acrescentar_amistoso, planejar_amistoso_em_data_livre)',
           'Friendly +0x34c8: the friendly list; +0xa7 the season start (date); pointer and value; reads and creates (acrescentar_amistoso, planejar_amistoso_em_data_livre)'),
          ('amistoso (0x19 bytes) +0: a data; +8 zero; +0xc o clube; +0x10 o mandante; +0x14 o visitante; +0x18 a situação (1); ponteiro e valor; grava (acrescentar_amistoso)',
           'friendly (0x19 bytes) +0: the date; +8 zero; +0xc the club; +0x10 home club; +0x14 away club; +0x18 the state (1); pointer and value; writes (acrescentar_amistoso)'),
          ('candidato do amistoso +4: a conta (byte); +0xa a lista dos adversários; +0xe/+0x10 a janela (palavras com sinal); +0x12 a lista das datas; valor e ponteiro; lê e grava (planejar_amistoso_em_data_livre)',
           'friendly candidate +4: the count (byte); +0xa the opponent list; +0xe/+0x10 the window (signed words); +0x12 the date list; value and pointer; reads and writes (planejar_amistoso_em_data_livre)'),
          ('clube +0x69: o estádio; ponteiro; lê (acrescentar_amistoso)',
           'club +0x69: the stadium; pointer; reads (acrescentar_amistoso)')]
INTERESSA = ['GPF2']
