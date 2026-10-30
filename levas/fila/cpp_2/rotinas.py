# Os empréstimos e o lugar no elenco (contract_manager.cpp): assinar o contrato de empréstimo
# (CONTRACT_MANAGER::sign_loan_contract 0x4d01e0, pelas 0x8bf5b4 e 0x8c2684 do transfer_manager.cpp: os empréstimos
# combinados), encerrar o empréstimo (CONTRACT_MANAGER::end_loan 0x4d07e0, pela daily_update 0x4c8114 no fim do
# empréstimo e pela 0x8c9d5d na transferência) e trocar o lugar no elenco (CONTRACT_MANAGER::toggle_squad_placement
# 0x4cbd20, pelas 0x4cc0b0/0x4cc0f0 da 244a leva e pela 0x4cb02b depois de cada partida). A 0x4d01e0 tem quadro SEH com
# uma data local (0x54a110) e nenhum destrutor no caminho normal. Sem sorteio, sem leitura de lixo (a local de time A da
# 0x4d07e0 só é lida quando a 0x540a50 a grava). Iguais no cm0102-1080.exe. As três mexem no elenco, nas táticas e nas
# listas (0x53bd10, 0x53c190, 0x896c60, 0x7c5fe0; a 0x4d07e0 também nas disputas 0x556570, que libera itens, e a 0x4d01e0
# realoca o vetor de contratos): os comparadores rodam só a original e as cópias provam pela carreira no modo Rust,
# com os ramos. Peculiaridades mantidas: a 0x4d01e0 sai sem refazer as folhas quando o contrato vai para uma vaga
# livre; sem contrato principal, a 0x4cbd20 lê [0x35] e cai (como a original).
SAVE = False
ROTINAS = [('assinar_contrato_de_emprestimo', 0x4d01e0, [0x6a, 0xff, 0x68, 0x1b, 0x6e, 0x95, 0x00]),
           ('encerrar_o_emprestimo', 0x4d07e0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('trocar_lugar_no_elenco', 0x4cbd20, [0x51, 0x53, 0x8b, 0x5c, 0x24, 0x10])]
DOCS = ('os empréstimos e o lugar no elenco: assinar o contrato de empréstimo, encerrar o empréstimo e trocar o lugar no elenco',
        'loans and squad placement: signing the loan contract, ending the loan and toggling the squad placement',
        [('assinar_contrato_de_emprestimo', 'CONTRACT_MANAGER::sign_loan_contract', '0x4d01e0', 'contract_manager.cpp', 'thiscall, ret 4',
          'passa a pessoa ao clube do empréstimo (listas, táticas, elenco como jogador), marca o reserva no contrato principal e guarda o contrato de empréstimo na vaga dela (a que tem, uma livre ou no fim do vetor); refaz as folhas',
          'moves the person to the loan club (lists, tactics, squad as a player), marks the reserve bit in the main contract and stores the loan contract in her slot (the existing one, a free one or at the end of the array); recalculates the wage bills'),
         ('encerrar_o_emprestimo', 'CONTRACT_MANAGER::end_loan', '0x4d07e0', 'contract_manager.cpp', 'thiscall, ret 4',
          'tira a pessoa do clube do empréstimo (listas, disputas, táticas, elenco, lista de observação), libera o contrato de empréstimo, refaz as folhas e a devolve ao clube dono com a marca de elenco',
          'takes the person out of the loan club (lists, disputes, tactics, squad, shortlist), frees the loan contract, recalculates the wage bills and returns her to the parent club with the squad mark'),
         ('trocar_lugar_no_elenco', 'CONTRACT_MANAGER::toggle_squad_placement', '0x4cbd20', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'troca a pessoa entre o principal e os reservas do clube ou entre o clube e o parceiro (time A/B), com o contrato de empréstimo pesando; 0 sem vaga no destino',
          'moves the person between the club\'s first team and reserves or between the club and its partner (A/B side), taking the loan contract into account; 0 with no room at the destination')])
CAMPOS = [('[0xae2d0c] registro de 79 bytes da pessoa +0, +4: os contratos principal e de empréstimo; índice; lê e grava +4 (as três)',
           '[0xae2d0c] person\'s 79-byte record +0, +4: the main and loan contracts; index; reads and writes +4 (all three)'),
          ('CONTRACT_MANAGER +0, +4, +8: o vetor dos contratos (0x50 bytes), a capacidade e a conta; ponteiro e valores; lê e grava (assinar_contrato_de_emprestimo, encerrar_o_emprestimo)',
           'CONTRACT_MANAGER +0, +4, +8: the contract array (0x50 bytes), the capacity and the count; pointer and values; reads and writes (assinar_contrato_de_emprestimo, encerrar_o_emprestimo)'),
          ('contrato +0x35 bit 0x40: o reserva; valor; lê e grava (as três)',
           'contract +0x35 bit 0x40: reserve; value; reads and writes (all three)'),
          ('[0xae1080] a ficha global do contrato; ponteiro; grava (assinar_contrato_de_emprestimo, encerrar_o_emprestimo)',
           '[0xae1080] the global contract slot; pointer; writes (assinar_contrato_de_emprestimo, encerrar_o_emprestimo)'),
          ('pessoa +0x39 (clube atual), +0x3e (data de entrada), +0x5e (marcas de elenco), +0x6d (situação, 0x10 vira 4); ponteiro e valores; lê e grava (encerrar_o_emprestimo), lê +0x39/+0x5e e devolve +0x3e (assinar_contrato_de_emprestimo)',
           'person +0x39 (current club), +0x3e (join date), +0x5e (squad marks), +0x6d (status, 0x10 becomes 4); pointer and values; reads and writes (encerrar_o_emprestimo), reads +0x39/+0x5e and restores +0x3e (assinar_contrato_de_emprestimo)')]
INTERESSA = ['CMRTE', 'SGE']
