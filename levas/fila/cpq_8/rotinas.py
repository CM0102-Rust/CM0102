# As ofertas de transferência (transfer_manager.cpp): desfazer a oferta (TRANSFER_MANAGER::collapse_offer 0x8c44e0,
# pelas restrições 0x8c34e0, pelas transferências futuras 0x8cbbe0 e pelas telas), fazer a oferta de empréstimo
# (TRANSFER_MANAGER::make_loan_offer 0x8bed60, pela lista dos clubes da IA 0x842740 e pela tela 0x8fb5e0) e concluir as
# transferências futuras do dia (TRANSFER_MANAGER::process_due_future_transfers 0x8cbbe0, pela atualização diária
# 0x8c5880). Quadro SEH só na 0x8bed60 (TRANSFER_OFFER local: construtor 0x8e5680 e destrutor 0x8e5710 na ordem da
# original; STAFF_CONTRACT local 0x85e410 sem destrutor). Iguais no cm0102-1080.exe. Comparadores só da original
# (estados, listas, alocações, notícias e transferências); as cópias são provadas pelos ramos e pelas mutações na
# carreira.
SAVE = False
ROTINAS = [('desfazer_oferta', 0x8c44e0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('fazer_oferta_de_emprestimo', 0x8bed60, [0x6a, 0xff, 0x68, 0xc3, 0x3a, 0x96, 0x00]),
           ('concluir_transferencias_futuras', 0x8cbbe0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('as ofertas de transferência: desfazer a oferta, fazer a oferta de empréstimo e concluir as transferências futuras do dia',
        'the transfer offers: collapsing an offer, making a loan offer and completing the day\'s future transfers',
        [('desfazer_oferta', 'TRANSFER_MANAGER::collapse_offer', '0x8c44e0', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'estado 0x10, fecha as propostas, solta os lances, posta a notícia, agenda a remoção, tira as marcas, a oferta de contrato e os contratos da troca',
          'state 0x10, closes the proposals, releases the bids, posts the news, schedules the removal, clears the flags, the contract offer and the exchange contracts'),
         ('fazer_oferta_de_emprestimo', 'TRANSFER_MANAGER::make_loan_offer', '0x8bed60', 'transfer_manager.cpp', 'thiscall, ret 0x18',
          'monta a oferta de empréstimo e o contrato pelo atual (porcentagem do salário), acrescenta os dois e devolve a vaga da oferta',
          'builds the loan offer and the contract from the current one (wage percentage), adds both and returns the offer slot'),
         ('concluir_transferencias_futuras', 'TRANSFER_MANAGER::process_due_future_transfers', '0x8cbbe0', 'transfer_manager.cpp', 'thiscall',
          'as transferências futuras com data chegada: desfaz as que as restrições seguram, conclui as outras (0x8c9390) e as tira do vetor',
          'the future transfers whose date has come: collapses those the restrictions block, completes the others (0x8c9390) and removes them from the array')])
CAMPOS = [('oferta de transferência +0x2c: o estado (0x13, 0x10, 0x14...); +0x2e: o passo; valor; lê e grava (desfazer_oferta, concluir_transferencias_futuras)',
           'transfer offer +0x2c: the state (0x13, 0x10, 0x14...); +0x2e: the step; value; reads and writes (desfazer_oferta, concluir_transferencias_futuras)'),
          ('oferta +0 / +4: as listas das ofertas rivais e dos jogadores da troca (LLIST, cursor +0xa); ponteiro; lê (desfazer_oferta)',
           'offer +0 / +4: the lists of rival offers and of exchange players (LLIST, cursor +0xa); pointer; reads (desfazer_oferta)'),
          ('oferta +0x10: zerado ao desfazer; +0x20 / +0x24: a vaga e o valor da oferta de contrato; valor; grava e lê',
           'offer +0x10: cleared on collapse; +0x20 / +0x24: the contract offer slot and value; value; writes and reads'),
          ('TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bits 0x10 e 1; valor; grava (desfazer_oferta)',
           'TRANSFER_MANAGER +0x86e: person\'s 8-byte record, bits 0x10 and 1; value; writes (desfazer_oferta)'),
          ('TRANSFER_MANAGER +0x920 / +0x928: as transferências futuras (12 bytes: data, índice da oferta) e a conta; ponteiro e valor; lê e grava (concluir_transferencias_futuras)',
           'TRANSFER_MANAGER +0x920 / +0x928: the future transfers (12 bytes: date, offer index) and the count; pointer and value; reads and writes (concluir_transferencias_futuras)'),
          ('TRANSFER_MANAGER +0: as ofertas (ponteiros pelo índice); +0x856: a Shortlist_Manager; ponteiro; lê',
           'TRANSFER_MANAGER +0: the offers (pointers by index); +0x856: the Shortlist_Manager; pointer; reads'),
          ('[0xae1080] o último contrato achado; ponteiro; grava (desfazer_oferta, concluir_transferencias_futuras)',
           '[0xae1080] the last contract found; pointer; writes (desfazer_oferta, concluir_transferencias_futuras)')]
INTERESSA = ['CMRTE', 'SGE']
