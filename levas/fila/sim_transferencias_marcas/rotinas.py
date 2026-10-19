ROTINAS = [('marcar_contrato_acertado', 0x8c5410, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10]),
           ('desmarcar_contrato_acertado', 0x8c5430, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10]),
           ('vaga_da_oferta', 0x8c61e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x04])]
DOCS = ('a marca de contrato acertado e a vaga da oferta',
        'the agreed-contract flag and the offer slot',
        [('marcar_contrato_acertado', 'set_agreed_contract_flag', '0x8c5410', 'transfer_manager.cpp', 'thiscall',
          'liga a marca de contrato acertado (0x40) da pessoa no gerente de transferências',
          'sets the person\'s agreed-contract flag (0x40) in the transfer manager'),
         ('desmarcar_contrato_acertado', 'clear_agreed_contract_flag', '0x8c5430', 'transfer_manager.cpp', 'thiscall',
          'desliga a marca de contrato acertado da pessoa',
          'clears the person\'s agreed-contract flag'),
         ('vaga_da_oferta', 'offer_slot', '0x8c61e0', 'transfer_manager.cpp', 'thiscall',
          'o endereço da vaga de uma oferta (fora da faixa, o começo da tabela)',
          'the address of an offer slot (out of range, the table start)')])
