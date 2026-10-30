# Os registros de carreira (staff_records.cpp): a transferência (Staff_Records::add_transfer_records 0x86cd70, a cada
# transferência fechada), a eliminação (add_knocked_out_record 0x86c000, pela copa 0x519890 e outras) e a não
# classificação (add_failed_to_qualify_record 0x86c1d0, 33 chamadores). O SEH das três só cobre a data local
# (0x54a110, sem destrutor). Gravam os históricos no arquivo (FILE_LLIST 0x58e340: não rodam duas vezes); a
# transferência também mexe nos registros de pontos dos técnicos (0x86f5b0: maiores compra e venda, contas e totais em
# 64 bits), na global [0xae1080] e no cursor da lista da troca; as outras duas acabam as convocações da seleção na
# competição guardada (0x76d940/0x774f90). Os bytes da entrada que a original não grava (lixo de pilha) saem zerados
# na cópia: só vão para o arquivo. Iguais no cm0102-1080.exe. Comparadores: a transferência lado a lado (a global, o
# sorteador, a lista da troca e os dois registros; a cópia sem os históricos); as outras duas só a original.
SAVE = False
ROTINAS = [('registrar_a_transferencia', 0x86cd70, [0x6a, 0xff, 0x68, 0x5b, 0x2d, 0x96, 0x00]),
           ('registrar_a_eliminacao', 0x86c000, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('registrar_a_nao_classificacao', 0x86c1d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os registros de carreira: a transferência, a eliminação e a não classificação',
        'career records: the transfer, the knock-out and the failure to qualify',
        [('registrar_a_transferencia', 'Staff_Records::add_transfer_records', '0x86cd70', 'staff_records.cpp', 'thiscall, ret 4',
          'nos registros dos técnicos do comprador e do vendedor: a maior compra ou venda, as contas, os totais em 64 bits e os jogadores da troca; os históricos de compra (0x64), venda (0x65) e transferência (0x66) dos técnicos, da pessoa e dos jogadores da troca',
          'in the buying and selling managers\' records: the biggest signing or sale, the counts, the 64-bit totals and the exchange players; the buy (0x64), sale (0x65) and transfer (0x66) histories of the managers, the person and the exchange players'),
         ('registrar_a_eliminacao', 'Staff_Records::add_knocked_out_record', '0x86c000', 'staff_records.cpp', 'thiscall, ret 0x14',
          'o histórico do técnico (tipo 4, com a fase e a rodada base) e o fim das convocações da seleção na competição guardada',
          'the manager history (type 4, with the round and the base round) and the end of the national team duty in the stored competition'),
         ('registrar_a_nao_classificacao', 'Staff_Records::add_failed_to_qualify_record', '0x86c1d0', 'staff_records.cpp', 'thiscall, ret 0x10',
          'o histórico do técnico (tipo 7, com a fase) e o fim das convocações da seleção na competição guardada',
          'the manager history (type 7, with the round) and the end of the national team duty in the stored competition')])
CAMPOS = [('registro de pontos do técnico (0x86f5b0, 210 bytes): +0..+0x24 maior compra (pessoa, três ids +0x33, data, valor +0x18, troca +0x1c, vendedor, comprador), +0x25..+0x49 maior venda (valor +0x3d), +0xa0/+0xa2 contas, +0xa4/+0xac totais em 64 bits; valores; lê e grava (registrar_a_transferencia)',
           'manager points record (0x86f5b0, 210 bytes): +0..+0x24 biggest signing (person, three +0x33 ids, date, value +0x18, exchange +0x1c, seller, buyer), +0x25..+0x49 biggest sale (value +0x3d), +0xa0/+0xa2 counts, +0xa4/+0xac 64-bit totals; values; reads and writes (registrar_a_transferencia)'),
          ('transferência +4 lista da troca (LLIST: cursor +0xa reposto), +8 comprador, +0xc pessoa, +0x11 tipo; ids e ponteiro; lê (registrar_a_transferencia)',
           'transfer +4 exchange list (LLIST: cursor +0xa reset), +8 buyer, +0xc person, +0x11 type; ids and pointer; reads (registrar_a_transferencia)'),
          ('[0xae1080] o contrato atual; ponteiro; grava (registrar_a_transferencia)',
           '[0xae1080] the current contract; pointer; writes (registrar_a_transferencia)'),
          ('pessoa +0x52: valor; lê (registrar_a_transferencia); +0x39 clube contratado, +0x3d cargo; lê',
           'person +0x52: value; reads (registrar_a_transferencia); +0x39 contracted club, +0x3d job; reads'),
          ('Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (as três)',
           'Staff_Records +0 and +0xc: staff history file and lists; appends (all three)')]
INTERESSA = ['CMRTE', 'SGE']
