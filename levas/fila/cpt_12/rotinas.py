# As excursões da pré-temporada (friendly.cpp): Friendly::schedule_tour_matches (0x5c4040, 48 chamadas na cobertura de
# 17/10, pela 0x5c3040: sorteia o começo na janela e marca os jogos da excursão) e os dois comparadores dos candidatos
# do amistoso, pela faixa (0x5c5270: o qsort da 0x5c27a0 e o bsearch do adversário pela faixa) e pela nação e faixa
# (0x5c5290: os qsort e bsearch da 0x5c27a0, 0x5c2d40, 0x5c3040, 0x5c4770 e do adversário da excursão). O SEH da
# 0x5c4040 só cobre datas (destrutor 0x5a53c0, um ret) e as listas novas durante o construtor. A largura (2n + 1) e o
# começo andado vão à 0x54c5b0 como dword com sobra em cima; ela lê a palavra (a cópia passa limpo). Esquisitices
# mantidas: o começo fica andado entre as tentativas; qualquer data já guardada na lista do candidato estraga a
# tentativa, e na quinta segue assim mesmo. Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a
# confirmação no jogo real comparando. Comparadores: a excursão só a original (sorteia, monta amistosos); os dois
# comparadores pela devolução.
SAVE = False
ROTINAS = [('marcar_jogos_da_excursao', 0x5c4040, [0x6a, 0xff, 0x68, 0x9e, 0x9f, 0x95, 0x00]),
           ('ordem_pela_faixa', 0x5c5270, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('ordem_pela_nacao_e_faixa', 0x5c5290, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('as excursões da pré-temporada: marcar os jogos da excursão e as duas ordens dos candidatos do amistoso',
        'the pre-season tours: booking the tour matches and the two orders of the friendly candidates',
        [('marcar_jogos_da_excursao', 'Friendly::schedule_tour_matches', '0x5c4040', 'friendly.cpp', 'thiscall, ret 0xc',
          'corta a janela pelo primeiro adversário, sorteia o começo (até 5 tentativas) e marca um amistoso por adversário, de 2 em 2 dias, guardando as datas na lista dos dois',
          "trims the window by the first opponent, draws the start (up to 5 tries) and books one friendly per opponent, 2 days apart, storing the dates in both lists"),
         ('ordem_pela_faixa', 'compare_friendly_candidates_by_band', '0x5c5270', 'friendly.cpp', 'cdecl',
          'comparador dos candidatos pela faixa (+5), a maior primeiro',
          'candidate comparator by band (+5), highest first'),
         ('ordem_pela_nacao_e_faixa', 'compare_friendly_candidate_ptrs_by_nation', '0x5c5290', 'friendly.cpp', 'cdecl',
          'comparador dos ponteiros para candidatos pela nação (+6) e, na mesma nação, pela faixa (salvo 0xff)',
          'comparator of candidate pointers by nation (+6) and, within a nation, by band (except 0xff)')])
CAMPOS = [('candidato do amistoso (0x16 bytes) +0: o id do clube; +5 a faixa (byte com sinal; 0xff = qualquer); +6 o id da nação; +0xe/+0x10 a janela (palavras com sinal); +0x12 a lista das datas (8 bytes cada); valor e ponteiro; lê e grava a lista (marcar_jogos_da_excursao, ordem_pela_faixa, ordem_pela_nacao_e_faixa)',
           'friendly candidate (0x16 bytes) +0: club id; +5 the band (signed byte; 0xff = any); +6 nation id; +0xe/+0x10 the window (signed words); +0x12 the date list (8 bytes each); value and pointer; reads and writes the list (marcar_jogos_da_excursao, ordem_pela_faixa, ordem_pela_nacao_e_faixa)'),
          ('Friendly +0xa7: o início da temporada (data); valor; lê (marcar_jogos_da_excursao)',
           'Friendly +0xa7: the season start (date); value; reads (marcar_jogos_da_excursao)')]
INTERESSA = ['GPF2']
