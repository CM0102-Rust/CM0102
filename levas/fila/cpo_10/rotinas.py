# As transferências (transfer_manager.cpp): o prazo em dias e fatias até o clube responder a uma oferta
# (TRANSFER_MANAGER::get_offer_response_delay 0x8cc5b0, pelas ofertas da IA: 0x4cd39a, 0x8be8a1), pagar as prestações
# das transferências no fim do mês (TRANSFER_MANAGER::pay_transfer_instalments 0x8caee0, 0x8c5aff) e tirar uma oferta
# (TRANSFER_MANAGER::remove_offer 0x8d0f60: 0x8be553, 0x8c6757). Sem SEH. A 0x8cc5b0 sorteia (0x90cfc0), compara o
# interesse (0x846920) em double e mostra a caixa "Error" sem ficha de jogador (segue). A 0x8caee0 paga pelas finanças
# (0x599890/0x599800), tira as prestações acabadas e aloca as marcas (0x945501/0x9452ca). A 0x8d0f60 destrói e libera
# as ofertas (0x8e5710, 0x944c94). Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no
# jogo real comparando. Comparadores: o prazo lado a lado com o estado do sorteio; pagar e tirar só a original,
# conferindo o plano (as prestações que acabam; a vaga, as contas, a menor vaga livre, as ofertas pelo jogador).
SAVE = False
ROTINAS = [('prazo_da_resposta_a_oferta', 0x8cc5b0, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('pagar_prestacoes_das_transferencias', 0x8caee0, [0x83, 0xec, 0x0c, 0xa1, 0x5c, 0x23, 0xae, 0x00]),
           ('tirar_oferta_de_transferencia', 0x8d0f60, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('as transferências: o prazo da resposta à oferta, pagar as prestações do mês e tirar a oferta',
        'transfers: the offer response delay, paying the month\'s instalments and removing the offer',
        [('prazo_da_resposta_a_oferta', 'TRANSFER_MANAGER::get_offer_response_delay', '0x8cc5b0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'pelo prazo até fechar a janela (as regras das nações do comprador e do clube do jogador): 0 dia, 1 ou 2 dias com fatias sorteadas, ou um valor pelo interesse do jogador (0x846920) ou sorteado pelo estatuto; dias = valor / 3, fatias = valor % 3',
          'from the time until the window closes (the rules of the buyer\'s and the player club\'s nations): 0 days, 1 or 2 days with drawn slices, or a value from the player\'s interest (0x846920) or drawn from the status; days = value / 3, slices = value % 3'),
         ('pagar_prestacoes_das_transferencias', 'TRANSFER_MANAGER::pay_transfer_instalments', '0x8caee0', 'transfer_manager.cpp', 'thiscall, ret 0',
          'paga cada prestação (finanças do pagador e do recebedor), soma as frações na figura das prestações (+0x1c) nas 8 últimas, tira as acabadas e zera a figura dos clubes sem prestação no sentido dela',
          'pays each instalment (payer\'s and receiver\'s finances), adds the fractions to the instalment figure (+0x1c) in the last 8, removes the finished ones and zeroes the figure of clubs without an instalment in its direction'),
         ('tirar_oferta_de_transferencia', 'TRANSFER_MANAGER::remove_offer', '0x8d0f60', 'transfer_manager.cpp', 'thiscall, ret 4',
          'destrói a oferta ligada, tira o par da lista de empréstimos e as propostas de contrato da troca e do jogador, baixa as contas de ofertas do jogador (+0x86e) e do gerente, a menor vaga livre acompanha e a oferta é destruída',
          'destroys the linked offer, removes the pair from the loan list and the contract offers of the exchange and of the player, lowers the player\'s (+0x86e) and the manager\'s offer counts, the lowest free slot follows and the offer is destroyed')])
CAMPOS = [('oferta de transferência +4: a lista dos jogadores da troca; +8: o clube comprador; +0xc: o jogador; +0x11: o cargo; +0x1f: empréstimo; +0x20: a vaga; +0x28: a oferta ligada; ponteiro, id e valor; lê (prazo_da_resposta_a_oferta, tirar_oferta_de_transferencia)',
           'transfer offer +4: the exchange player list; +8: the buying club; +0xc: the player; +0x11: the job; +0x1f: loan; +0x20: the slot; +0x28: the linked offer; pointer, id and value; reads (prazo_da_resposta_a_oferta, tirar_oferta_de_transferencia)'),
          ('TRANSFER_MANAGER +0/+8: as ofertas e a conta; +0xc/+0x14: as ligadas e a conta; +0x85e: a menor vaga livre; +0x86e: o registro de 8 bytes por pessoa (+1 ofertas, +2 da troca); +0x946: a lista de empréstimos; ponteiro e valor; lê e grava (tirar_oferta_de_transferencia)',
           'TRANSFER_MANAGER +0/+8: the offers and the count; +0xc/+0x14: the linked ones and the count; +0x85e: the lowest free slot; +0x86e: the 8-byte per-person record (+1 offers, +2 exchange); +0x946: the loan list; pointer and value; reads and writes (tirar_oferta_de_transferencia)'),
          ('TRANSFER_MANAGER +0x872: a lista das prestações (+0 valor, +4 clube que paga, +8 clube que recebe, +0xc quantas faltam); ponteiro e valor; lê e grava (pagar_prestacoes_das_transferencias)',
           'TRANSFER_MANAGER +0x872: the instalment list (+0 amount, +4 paying club, +8 receiving club, +0xc how many remain); pointer and value; reads and writes (pagar_prestacoes_das_transferencias)'),
          ('finanças do clube (0x5a0590) +0x1c: a figura das prestações; valor; lê e grava (pagar_prestacoes_das_transferencias)',
           'club finances (0x5a0590) +0x1c: the instalment figure; value; reads and writes (pagar_prestacoes_das_transferencias)'),
          ('TRANSFER_MANAGER +0x8ac: as 28 regras (+0xa a nação); +0x856: o gerente da lista de observação; ponteiro; lê (prazo_da_resposta_a_oferta)',
           'TRANSFER_MANAGER +0x8ac: the 28 rules (+0xa the nation); +0x856: the shortlist manager; pointer; reads (prazo_da_resposta_a_oferta)'),
          ('staff +0x18: a idade; ficha +7: a habilidade; registro de 79 bytes +0xb (nibble baixo): em quantas listas; contrato +0x3d: o estatuto; clube +0x80: a reputação; valor; lê (prazo_da_resposta_a_oferta)',
           'staff +0x18: the age; player data +7: the ability; 79-byte record +0xb (low nibble): on how many shortlists; contract +0x3d: the status; club +0x80: the reputation; value; reads (prazo_da_resposta_a_oferta)'),
          ('[0xae1080] o último contrato buscado; ponteiro; grava', '[0xae1080] the last contract looked up; pointer; writes')]
INTERESSA = ['CMRTE', 'SGE']
