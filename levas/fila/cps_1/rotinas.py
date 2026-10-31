# O pedido de transferência e os cargos (contract_manager.cpp): o pedido de transferência do jogador
# (CONTRACT_MANAGER::handle_transfer_request 0x4d8610, pelas queixas do contrato 0x867560 e 0x867700; quadro SEH só da
# notícia do clube humano, que fica com a original), os cargos com contrato (is_contract_staff_job 0x4cbb20, pela tela
# da comissão 0x870650) e quem não é da diretoria (staff_is_not_board_member 0x4cb960, pela mesma tela). Iguais no
# cm0102-1080.exe. Comparadores: só a original no pedido (sorteia e põe na lista; a cópia leva os ramos); pelo valor
# nos outros dois (só rodam na tela).
SAVE = False
ROTINAS = [('pedido_de_transferencia', 0x4d8610, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cargo_com_contrato', 0x4cbb20, [0x8a, 0x44, 0x24, 0x04, 0x3c, 0x0b]),
           ('fora_da_diretoria', 0x4cb960, [0x8b, 0x4c, 0x24, 0x04, 0x8a, 0x41, 0x3d])]
DOCS = ('o pedido de transferência do jogador, os cargos com contrato e quem não é da diretoria',
        'the player\'s transfer request, the jobs with a contract and who is not a board member',
        [('pedido_de_transferencia', 'CONTRACT_MANAGER::handle_transfer_request', '0x4d8610', 'contract_manager.cpp', 'thiscall, ret 4',
          'no clube do computador: se o valor (0x4d0c60) × 1,5 passa do preço pedido (0x4d1600), põe o jogador na lista (+0x4e, moral +12, 0x842c00, 0x4d71b0); senão um sorteio dá a queixa 0x10000, a moral cai 12 e a força do pedido (+0x44) sobe; no clube humano, a notícia (com a original)',
          'at a computer club: when the value (0x4d0c60) × 1.5 beats the asking price (0x4d1600), lists the player (+0x4e, morale +12, 0x842c00, 0x4d71b0); otherwise a draw gives the 0x10000 grievance, morale drops 12 and the request strength (+0x44) grows; at a human club, the news item (by the original)'),
         ('cargo_com_contrato', 'is_contract_staff_job', '0x4cbb20', 'contract_manager.cpp', 'stdcall, ret 4',
          '1 para os cargos 0, 6 a 0xd e 0xf (os que têm contrato pelo gerente de contratos)',
          '1 for jobs 0, 6 to 0xd and 0xf (those holding a contract manager contract)'),
         ('fora_da_diretoria', 'staff_is_not_board_member', '0x4cb960', 'contract_manager.cpp', 'stdcall, ret 4',
          '0 se o cargo (+0x3d) é de diretoria (1 a 4) num clube (+0x39); senão 1',
          '0 when the job (+0x3d) is a board job (1 to 4) at a club (+0x39); otherwise 1')])
CAMPOS = [('contrato +0 / +4: pessoa e clube; id; lê (pedido_de_transferencia)', 'contract +0 / +4: person and club; id; reads (pedido_de_transferencia)'),
          ('contrato +0x44: força do pedido de transferência (com sinal, ±100); valor; lê e grava (pedido_de_transferencia)',
           'contract +0x44: transfer request strength (signed, ±100); value; reads and writes (pedido_de_transferencia)'),
          ('contrato +0x4e: bit 2 tirado e bit 8 (na lista a pedido) ligado; valor; grava (pedido_de_transferencia)',
           'contract +0x4e: bit 2 cleared and bit 8 (listed on request) set; value; writes (pedido_de_transferencia)'),
          ('contrato +0x4f & 0xf0: status no elenco (0x10 = 1); valor; lê (pedido_de_transferencia)',
           'contract +0x4f & 0xf0: squad status (0x10 = 1); value; reads (pedido_de_transferencia)'),
          ('staff +0x59: ânimo; staff +0x61: ficha de jogador; valor e ponteiro; lê (pedido_de_transferencia)',
           'staff +0x59: happiness; staff +0x61: player record; value and pointer; reads (pedido_de_transferencia)'),
          ('staff +0x39 / +0x3d: clube e cargo; ponteiro e valor; lê (fora_da_diretoria)', 'staff +0x39 / +0x3d: club and job; pointer and value; reads (fora_da_diretoria)')]
INTERESSA = ['CMRTE']
