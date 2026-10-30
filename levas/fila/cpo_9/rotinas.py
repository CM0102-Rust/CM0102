# A lista de observação e o empréstimo (shortlist_manager.cpp, transfer_manager.cpp): acrescentar o par (pessoa, clube)
# nos dois vetores da lista (Shortlist_Manager::add_shortlist_entry 0x8423f0, pelos clubes da IA que observam um
# jogador: 0x840ad3, 0x8437e5, 0x843df4), prever a resposta do clube vendedor a um lance ou pedido de empréstimo
# (shortlist_manager_predict_selling_club_response 0x84af00, 7 chamadores: mídia, listas, transferências) e o período
# do empréstimo limitado pelo fim do contrato (TRANSFER_MANAGER::get_loan_period 0x8d0590, 6 chamadores). Sem SEH. A
# 0x8423f0 aloca e realoca os vetores (sem memória: a caixa "Error" e o jogo para, erro_em). A 0x84af00 compara em
# double, como a original. A 0x8d0590 chama o período pelo método +0x28 das regras da nação. Iguais no cm0102-1080.exe.
# Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Comparadores: acrescentar só a
# original conferindo o plano; prever e o período lado a lado (com o estado do sorteio, as chamadas de preço podem
# sortear).
SAVE = False
ROTINAS = [('acrescentar_na_lista_de_observacao', 0x8423f0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('prever_resposta_do_vendedor', 0x84af00, [0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00]),
           ('periodo_do_emprestimo', 0x8d0590, [0x83, 0xec, 0x10, 0x32, 0xd2])]
DOCS = ('a lista de observação e o empréstimo: acrescentar o par na lista, prever a resposta do clube vendedor e o período do empréstimo',
        'the shortlist and loans: adding the pair to the shortlist, predicting the selling club\'s response and the loan period',
        [('acrescentar_na_lista_de_observacao', 'Shortlist_Manager::add_shortlist_entry', '0x8423f0', 'shortlist_manager.cpp', 'thiscall, ret 8',
          'cria os dois vetores de pares (100 de capacidade), e o par ainda não estando: cresce de 100 em 100, acrescenta e ordena (pela pessoa, pelo clube) e, fora de clube humano, soma 1 no nibble baixo do +0xb do registro de 79 bytes da pessoa',
          'creates both pair arrays (capacity 100) and, when the pair is new: grows by 100, adds and sorts (by person, by club) and, outside a human club, adds 1 to the low nibble of +0xb of the person\'s 79-byte record'),
         ('prever_resposta_do_vendedor', 'shortlist_manager_predict_selling_club_response', '0x84af00', 'shortlist_manager.cpp', 'stdcall, ret 0x20',
          '0 aceita, 1 negocia, 2 recusa: pelo contrato com o vendedor, o empréstimo (listado, reputação, estatuto), o preço (humano, cláusula, férias, pedido ou 0x4d1600) contra a oferta',
          '0 accepts, 1 haggles, 2 rejects: from the contract with the seller, the loan case (listed, reputation, status), the price (human, clause, holiday, asking or 0x4d1600) against the offer'),
         ('periodo_do_emprestimo', 'TRANSFER_MANAGER::get_loan_period', '0x8d0590', 'transfer_manager.cpp', 'thiscall, ret 0x1c',
          'o período pelas regras da nação do clube e, com contrato de tipo 2, recua o fim mês a mês até não passar do fim do contrato (o início acompanha); sem contrato, vencido, longe demais ou antes deste ano: o ano padrão 1972',
          'the period from the club nation\'s rules and, with a type 2 contract, moves the end back month by month until it no longer passes the contract end (the start follows); without a contract, expired, too far or before this year: the default year 1972')])
CAMPOS = [('Shortlist_Manager +0/+4: os vetores de pares (id da pessoa, id do clube) pela pessoa e pelo clube; +8/+0xc as capacidades; +0x10/+0x14 as contas; ponteiro e valor; lê e grava (acrescentar_na_lista_de_observacao)',
           'Shortlist_Manager +0/+4: the (person id, club id) pair arrays by person and by club; +8/+0xc the capacities; +0x10/+0x14 the counts; pointer and value; reads and writes (acrescentar_na_lista_de_observacao)'),
          ('registro de 79 bytes da pessoa ([0xae2d0c]) +0xb: nibble baixo, em quantas listas da IA ela está; valor; lê e grava (acrescentar_na_lista_de_observacao)',
           'person 79-byte record ([0xae2d0c]) +0xb: low nibble, on how many AI shortlists she is; value; reads and writes (acrescentar_na_lista_de_observacao)'),
          ('contrato +0x2d/+0x2f: o fim (dia do ano, ano); +0x35: o tipo; +0x3d, +0x49, +0x4f: estatuto e marcas; valor; lê (prever_resposta_do_vendedor, periodo_do_emprestimo)',
           'contract +0x2d/+0x2f: the end (day of year, year); +0x35: the type; +0x3d, +0x49, +0x4f: status and flags; value; reads (prever_resposta_do_vendedor, periodo_do_emprestimo)'),
          ('clube +0x53: a nação (regras de transferência); +0x80: a reputação; +0xcf: o técnico; +0x1db; ponteiro e valor; lê',
           'club +0x53: the nation (transfer rules); +0x80: the reputation; +0xcf: the manager; +0x1db; pointer and value; reads'),
          ('ficha de jogador (staff +0x61) +0xb, +0xf; staff +0x52: o valor; valor; lê (prever_resposta_do_vendedor)',
           'player data (staff +0x61) +0xb, +0xf; staff +0x52: the value; value; reads (prever_resposta_do_vendedor)'),
          ('TRANSFER_MANAGER +0x8ac: as 28 regras de transferência (+0xa a nação; método +0x28 o período do empréstimo); ponteiro; lê (periodo_do_emprestimo)',
           'TRANSFER_MANAGER +0x8ac: the 28 transfer rules (+0xa the nation; method +0x28 the loan period); pointer; reads (periodo_do_emprestimo)'),
          ('[0xae2c90] a data de hoje (dia do ano, ano); valor; lê (periodo_do_emprestimo)', '[0xae2c90] today\'s date (day of year, year); value; reads (periodo_do_emprestimo)')]
INTERESSA = ['CMRTE', 'CMScout']
