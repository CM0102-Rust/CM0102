# As transferências futuras (transfer_manager.cpp): a do jogador (TRANSFER_MANAGER::get_future_transfer 0x8cca10, por
# 29 chamadores: listas de venda e de alvos, escolhas do elenco, regens, telas; quadro SEH da LLIST local, construída e
# destruída como na original), tirar a da oferta (TRANSFER_MANAGER::remove_future_transfer 0x8cbf70, pelo fim da oferta
# 0x8c44e0, pelas retiradas 0x8bf680/0x8bfcb0, por 0x8c47d0 e pelo jogador dispensado 0x8df9b0) e acrescentar a da
# oferta acertada (TRANSFER_MANAGER::add_future_transfer 0x8cb9a0, pela aprovação dos clubes 0x8c3310, pela conclusão
# 0x8c34e0 e pelo arquivo de configuração 0x8bf060). Sem SEH nas duas últimas; iguais no cm0102-1080.exe. Comparadores:
# lado a lado (marca e saída; vetor, conta, marcas e cursor) e, na que cresce o vetor e reordena, só a original
# conferida pelo plano.
SAVE = False
ROTINAS = [('transferencia_futura_do_jogador', 0x8cca10, [0x6a, 0xff, 0x68, 0x98, 0x3e, 0x96, 0x00]),
           ('tirar_transferencia_futura', 0x8cbf70, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('acrescentar_transferencia_futura', 0x8cb9a0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as transferências futuras: a do jogador, tirar a da oferta e acrescentar a da oferta acertada',
        'the future transfers: the player\'s one, removing the offer\'s one and adding the agreed offer\'s one',
        [('transferencia_futura_do_jogador', 'TRANSFER_MANAGER::get_future_transfer', '0x8cca10', 'transfer_manager.cpp', 'thiscall, ret 8',
          'com a marca 0x10, procura nas ofertas pelo jogador (0x8c5cf0) e nas trocas que o oferecem (0x8c5e40) a de estado 0x13; achando, o registro do vetor +0x920 e 1; senão tira a marca e 0',
          'with the 0x10 flag, searches the offers for the player (0x8c5cf0) and the exchanges offering him (0x8c5e40) for a state 0x13 one; found, the +0x920 array entry and 1; otherwise clears the flag and 0'),
         ('tirar_transferencia_futura', 'TRANSFER_MANAGER::remove_future_transfer', '0x8cbf70', 'transfer_manager.cpp', 'thiscall, ret 4',
          'tira a marca 0x10 do jogador e dos da troca e o registro da oferta do vetor +0x920 (os seguintes descem, 0x8cbf40)',
          'clears the 0x10 flag of the player and the exchange players and removes the offer\'s entry from the +0x920 array (the next ones move down, 0x8cbf40)'),
         ('acrescentar_transferencia_futura', 'TRANSFER_MANAGER::add_future_transfer', '0x8cb9a0', 'transfer_manager.cpp', 'thiscall, ret 8',
          'acrescenta (data, oferta) ao vetor +0x920 e reordena; a oferta vai ao estado 0x13; suspende as listas de alvos pelo jogador e pelos da troca (0x842b00) e liga a marca 0x10 deles; o contrato marcado guarda a oferta',
          'adds (date, offer) to the +0x920 array and sorts it; the offer moves to state 0x13; suspends the shortlists for the player and the exchange players (0x842b00) and sets their 0x10 flag; a flagged contract keeps the offer')])
CAMPOS = [('TRANSFER_MANAGER +0x920 / +0x924 / +0x928: as transferências futuras (12 bytes: data +0, oferta +8), a capacidade e a conta; ponteiro, valor e id; lê e grava',
           'TRANSFER_MANAGER +0x920 / +0x924 / +0x928: the future transfers (12 bytes: date +0, offer +8), the capacity and the count; pointer, value and id; reads and writes'),
          ('TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bit 0x10 (transferência futura); valor; lê e grava',
           'TRANSFER_MANAGER +0x86e: person\'s 8-byte record, bit 0x10 (future transfer); value; reads and writes'),
          ('oferta de transferência +4: a lista da troca; +0xc: o jogador; +0x20: o número da oferta; +0x2c: o estado (0x13); +0x2d: a marca; id e valor; lê e grava +0x2c/+0x2d (acrescentar_transferencia_futura)',
           'transfer offer +4: the exchange list; +0xc: the player; +0x20: the offer number; +0x2c: the state (0x13); +0x2d: the flag; id and value; reads and writes +0x2c/+0x2d (acrescentar_transferencia_futura)'),
          ('contrato +0x49 / +0x4a: a marca e a oferta da transferência futura; valor e id; lê e grava (acrescentar_transferencia_futura)',
           'contract +0x49 / +0x4a: the flag and the future transfer offer; value and id; reads and writes (acrescentar_transferencia_futura)'),
          ('TRANSFER_MANAGER +0x856: as listas de alvos dos clubes (Shortlist_Manager); ponteiro; lê (acrescentar_transferencia_futura)',
           'TRANSFER_MANAGER +0x856: the club shortlists (Shortlist_Manager); pointer; reads (acrescentar_transferencia_futura)')]
INTERESSA = ['CMRTE', 'SGE']
