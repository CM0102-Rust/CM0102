ROTINAS = [('oferta_de_contrato', 0x4cc9e0, [0x53, 0x56, 0x8b, 0xf1, 0x57]),
           ('oferta_acertada', 0x4cca30, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x34]),
           ('oferta_fechada', 0x8d0d90, [0x8b, 0x4c, 0x24, 0x04, 0x8a, 0x41, 0x2c])]
DOCS = ('as ofertas de contrato e a oferta de transferência fechada',
        'the contract offers and the closed transfer offer',
        [('oferta_de_contrato', 'contract_offer', '0x4cc9e0', 'contract_manager.cpp', 'thiscall',
          'a oferta de contrato da pessoa para o clube, na lista do gerente de contratos',
          'the person\'s contract offer for the club, in the contract manager\'s list'),
         ('oferta_acertada', 'agreed_contract_offer', '0x4cca30', 'contract_manager.cpp', 'thiscall',
          'a oferta de contrato acertada (estado 2) da pessoa',
          'the person\'s agreed contract offer (state 2)'),
         ('oferta_fechada', 'transfer_offer_closed', '0x8d0d90', 'transfer_offer.cpp', 'stdcall',
          'a oferta de transferência está encerrada (pelo estado e, no estado 0xf, pela fase)',
          'whether the transfer offer is closed (by its state and, in state 0xf, by its phase)')])
