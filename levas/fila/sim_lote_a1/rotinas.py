ROTINAS = [('pode_contratar_jogador', 0x8ced40, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('trocas_oferecendo_o_jogador', 0x8c5e40, [0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10])]
DOCS = ('a contratação pelas regras da nação e as trocas que oferecem o jogador',
        'the signing by the nation rules and the exchanges offering the player',
        [('pode_contratar_jogador', 'TRANSFER_MANAGER::can_sign_player', '0x8ced40', 'transfer_manager.cpp', 'thiscall',
          'se o clube pode contratar o jogador pelas regras da nação, com o motivo da recusa e a segunda chance do clube humano no limite de estrangeiros',
          "whether the club may sign the player under the nation rules, with the refusal reason and the human club's second try on the foreign player limit"),
         ('trocas_oferecendo_o_jogador', 'TRANSFER_MANAGER::get_exchange_offers_for_player', '0x8c5e40', 'transfer_manager.cpp', 'thiscall',
          'quantas ofertas abertas do clube oferecem o jogador na troca, com a lista dos índices se pedida',
          "how many of the club's open offers include the player in the exchange, listing their indices when asked")])
