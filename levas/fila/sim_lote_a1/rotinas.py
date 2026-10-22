ROTINAS = [('pode_encerrar_emprestimo', 0x8d0530, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
           ('pode_contratar_jogador', 0x8ced40, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('trocas_oferecendo_o_jogador', 0x8c5e40, [0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10])]
DOCS = ('o fim do empréstimo, a contratação pelas regras da nação e as trocas que oferecem o jogador',
        'the loan ending, the signing by the nation rules and the exchanges offering the player',
        [('pode_encerrar_emprestimo', 'TRANSFER_MANAGER::can_end_loan', '0x8d0530', 'transfer_manager.cpp', 'thiscall',
          'se o empréstimo pode acabar: entre clubes da mesma nação pergunta às regras de transferência dela, senão 1',
          'whether the loan may end: between clubs of the same nation asks its transfer rules, else 1'),
         ('pode_contratar_jogador', 'TRANSFER_MANAGER::can_sign_player', '0x8ced40', 'transfer_manager.cpp', 'thiscall',
          'se o clube pode contratar o jogador pelas regras da nação, com o motivo da recusa e a segunda chance do clube humano no limite de estrangeiros',
          "whether the club may sign the player under the nation rules, with the refusal reason and the human club's second try on the foreign player limit"),
         ('trocas_oferecendo_o_jogador', 'TRANSFER_MANAGER::get_exchange_offers_for_player', '0x8c5e40', 'transfer_manager.cpp', 'thiscall',
          'quantas ofertas abertas do clube oferecem o jogador na troca, com a lista dos índices se pedida',
          "how many of the club's open offers include the player in the exchange, listing their indices when asked")])
