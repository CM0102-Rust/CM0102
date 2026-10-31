# A fila da transferência livre (transfer_manager.cpp): liberar os jogadores da fila (TRANSFER_MANAGER::
# process_free_transfer_queue 0x8c18c0, pelo dia a dia das transferências 0x8c5880; quadro SEH das duas LLIST locais,
# construídas e destruídas como na original), a próxima janela aberta da nação do clube (TRANSFER_MANAGER::
# find_next_window_open_date 0x8cf470, pela contratação desejada 0x85fc50, que regrava a data com hoje logo depois;
# quadro SEH só das datas) e o empréstimo negociado (TRANSFER_MANAGER::find_negotiated_loan_contract 0x8c72d0, pela
# tela do empréstimo 0x8f8be0). Iguais no cm0102-1080.exe. Comparadores: só a original na fila (sorteia e libera; a
# cópia leva os ramos); lado a lado pela data de saída; pelo valor.
SAVE = False
ROTINAS = [('fila_da_transferencia_livre', 0x8c18c0, [0x6a, 0xff, 0x68, 0xb0, 0x3b, 0x96, 0x00]),
           ('proxima_janela_aberta', 0x8cf470, [0x6a, 0xff, 0x68, 0xd6, 0x3f, 0x96, 0x00]),
           ('emprestimo_negociado', 0x8c72d0, [0x53, 0x56, 0x8b, 0xf1, 0x57])]
DOCS = ('a fila da transferência livre, a próxima janela aberta e o empréstimo negociado',
        'the free transfer queue, the next open window and the negotiated loan',
        [('fila_da_transferencia_livre', 'TRANSFER_MANAGER::process_free_transfer_queue', '0x8c18c0', 'transfer_manager.cpp', 'thiscall, ret 0',
          'para cada jogador da fila (+0x92c): no clube humano a diretoria pode barrar (0x696940); um sorteio põe o técnico nos desafetos (0x547af0); os registros (0x86d800), as ofertas canceladas (0x8df9b0) e a liberação com a compensação (0x8c1af0); limpa a fila',
          'for each queued player (+0x92c): at a human club the board may block it (0x696940); a draw adds the manager to the disliked staff (0x547af0); the records (0x86d800), the cancelled offers (0x8df9b0) and the release with the compensation (0x8c1af0); clears the queue'),
         ('proxima_janela_aberta', 'TRANSFER_MANAGER::find_next_window_open_date', '0x8cf470', 'transfer_manager.cpp', 'thiscall, ret 8',
          'o primeiro dia, de hoje até um ano depois, com a janela da nação do clube aberta (0x8e8e50; sem regra, hoje); sem achar, a caixa "Error"',
          'the first day, from today up to a year later, with the club nation window open (0x8e8e50; without a rule, today); when none, the "Error" box'),
         ('emprestimo_negociado', 'TRANSFER_MANAGER::find_negotiated_loan_contract', '0x8c72d0', 'transfer_manager.cpp', 'thiscall, ret 8',
          'o item da lista +0x946 com a pessoa e o clube, ou 0',
          'the +0x946 list item with the person and the club, or 0')])
CAMPOS = [('TRANSFER_MANAGER +0x92c: fila da transferência livre (LLIST; item +0 id da pessoa, +4 motivo, +5 compensação); valor; lê e limpa (fila_da_transferencia_livre)',
           'TRANSFER_MANAGER +0x92c: free transfer queue (LLIST; item +0 person id, +4 reason, +5 compensation); value; reads and clears (fila_da_transferencia_livre)'),
          ('TRANSFER_MANAGER +0x8ac: as 28 regras das nações (+0xa nação); ponteiro; lê (proxima_janela_aberta)',
           'TRANSFER_MANAGER +0x8ac: the 28 nation rules (+0xa nation); pointer; reads (proxima_janela_aberta)'),
          ('TRANSFER_MANAGER +0x946: lista dos empréstimos negociados (item +0 pessoa, +4 clube); ponteiro; lê e move o cursor (emprestimo_negociado)',
           'TRANSFER_MANAGER +0x946: negotiated loans list (item +0 person, +4 club); pointer; reads and moves the cursor (emprestimo_negociado)'),
          ('clube +0xcf: técnico; clube +0x53: nação; ponteiro; lê', 'club +0xcf: manager; club +0x53: nation; pointer; reads'),
          ('staff +0x5b: o valor do sorteio do desafeto; valor; lê (fila_da_transferencia_livre)', 'staff +0x5b: the disliked staff draw value; value; reads (fila_da_transferencia_livre)'),
          ('[0xae1080] contrato da vez; ponteiro; grava (fila_da_transferencia_livre)', '[0xae1080] current contract; pointer; writes (fila_da_transferencia_livre)')]
INTERESSA = ['CMRTE']
