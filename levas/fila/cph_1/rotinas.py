# Contrato de pessoal (staff_contracts.cpp), 1º item: os dias até o fim do contrato (STAFF_CONTRACT::days_to_end_date
# 0x85fa00, 9.161.204 chamadas na carreira até 2002-06-01: QUENTE, pela 0x54cdb0 já em Rust), limpar todas as queixas
# (clear_all_grievances 0x867520, 3.671; 0x549c90 e, com o argumento, 0x867240) e conferir a retirada do pedido de
# transferência (check_transfer_request_withdrawal 0x867240, 201.381; 0x4cf5a0, 0x866f60, 0x8c5450, 0x4d9460 original,
# 0x549d60 e ela mesma, recursiva, a cada queixa apagada, com o +0x45 de antes como contexto). Simulação: as mutações
# pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem sorteio, sem local lida
# sem valor. Comparadores: os dias pelo valor (pura); as queixas lado a lado (+0x45 e o registro de opiniões, 79
# bytes) sem o argumento e só a original com ele; a retirada lado a lado ([0xae1080] e o contrato) quando a decisão da
# cópia é não retirar, só a original quando retira (a 0x4d9460 mexe na transferência). A decisão chama a 0x8c5450,
# que pode apagar a marca 0x40 do gerente (idempotente: a original acha a marca já apagada e dá o mesmo 0).
SAVE = False
ROTINAS = [('dias_ate_o_fim_do_contrato', 0x85fa00, [0xa1, 0x94, 0x2c, 0xae, 0x00]),
           ('limpar_todas_as_queixas', 0x867520, [0x56, 0x8b, 0xf1, 0x8b, 0x06]),
           ('conferir_retirada_do_pedido', 0x867240, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x06])]
DOCS = ('o contrato de pessoal: os dias até o fim, limpar todas as queixas e conferir a retirada do pedido de transferência',
        'the staff contract: the days to the end date, clearing all grievances and checking the transfer request withdrawal',
        [('dias_ate_o_fim_do_contrato', 'STAFF_CONTRACT::days_to_end_date', '0x85fa00', 'staff_contracts.cpp', 'thiscall',
          'os dias da data do jogo até o fim do contrato (+0x2d), pela 0x54cdb0',
          'the days from the game date to the contract end date (+0x2d), through 0x54cdb0'),
         ('limpar_todas_as_queixas', 'STAFF_CONTRACT::clear_all_grievances', '0x867520', 'staff_contracts.cpp', 'thiscall, ret 4',
          'zera as queixas (+0x45), tira os vínculos de opinião (0x549c90) e, pedido, confere a retirada (0x867240)',
          'clears the grievances (+0x45), the opinion links (0x549c90) and, when asked, checks the withdrawal (0x867240)'),
         ('conferir_retirada_do_pedido', 'STAFF_CONTRACT::check_transfer_request_withdrawal', '0x867240', 'staff_contracts.cpp', 'thiscall, ret 4',
          'jogador do clube do contrato com pedido de transferência, sem critério da comissão e sem contrato acertado: retira o pedido (0x4d9460), a moral vira +0x5b + +0x59 + 5, +0x3e = 0, +0x3d ao menos 15, e apaga cada queixa conferindo de novo',
          'a player of the contract club with a transfer request, no staff criterion and no agreed contract: withdraws the request (0x4d9460), morale becomes +0x5b + +0x59 + 5, +0x3e = 0, +0x3d at least 15, and clears each grievance checking again')])
CAMPOS = [('contrato de pessoal (0x50 bytes): +0 pessoa, +4 clube, +0x2d data do fim, +0x3d felicidade, +0x3e, +0x44 moral, +0x45 queixas (bits), +0x49, +0x4e bit 3 pedido de transferência, +0x4f nibble alto; lê e grava (limpar_todas_as_queixas, conferir_retirada_do_pedido); lê (dias_ate_o_fim_do_contrato)',
           'staff contract (0x50 bytes): +0 person, +4 club, +0x2d end date, +0x3d happiness, +0x3e, +0x44 morale, +0x45 grievances (bits), +0x49, +0x4e bit 3 transfer request, +0x4f high nibble; reads and writes (limpar_todas_as_queixas, conferir_retirada_do_pedido); reads (dias_ate_o_fim_do_contrato)')]
INTERESSA = ['SGE', 'CMRTE']
