# As datas da transferência (transfer_manager.cpp): a data em que a oferta pode se concretizar pelo tipo de prazo
# (transfer_manager_get_offer_completion_date 0x8cee70, pela verificação das restrições da oferta 0x8cfc90 e por
# 0x8c3310 e 0x8da5e0), o primeiro dia que as regras das nações do comprador e do vendedor permitem
# (TRANSFER_MANAGER::find_next_allowed_transfer_date 0x8cf0e0, pela 0x8cee70 nos tipos 1 e 4) e a janela fixa de
# transferências (transfer_manager_get_next_window_dates 0x8d0bb0, no tipo 5). Quadro SEH só das datas locais (sem
# destrutor). Iguais no cm0102-1080.exe. Comparadores lado a lado pela saída, [0xae1080] e as estáticas das datas, e
# pelo motivo devolvido.
SAVE = False
ROTINAS = [('data_de_conclusao_da_oferta', 0x8cee70, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('primeiro_dia_permitido_da_oferta', 0x8cf0e0, [0x6a, 0xff, 0x68, 0xa8, 0x3f, 0x96, 0x00]),
           ('janela_fixa_de_transferencias', 0x8d0bb0, [0x6a, 0xff, 0x68, 0x90, 0x40, 0x96, 0x00])]
DOCS = ('as datas da transferência: a data de conclusão da oferta, o primeiro dia que as regras permitem e a janela fixa de transferências',
        'the transfer dates: the offer\'s completion date, the first day the rules allow and the fixed transfer window',
        [('data_de_conclusao_da_oferta', 'transfer_manager_get_offer_completion_date', '0x8cee70', 'transfer_manager.cpp', 'thiscall, ret 8',
          'pelo tipo de prazo da oferta (+0x30): hoje, o primeiro dia permitido (0x8cf0e0), o fim da temporada do vendedor mais 7 dias, o dia seguinte ao fim do contrato (nunca antes de hoje) ou o início da janela fixa; devolve o motivo',
          'by the offer\'s timing type (+0x30): today, the first allowed day (0x8cf0e0), the seller\'s season end plus 7 days, the day after the contract ends (never before today) or the start of the fixed window; returns the reason'),
         ('primeiro_dia_permitido_da_oferta', 'TRANSFER_MANAGER::find_next_allowed_transfer_date', '0x8cf0e0', 'transfer_manager.cpp', 'thiscall, ret 8',
          'anda dia a dia, de hoje (ou do fim do contrato, no tipo 4) até um ano, até as regras das nações do comprador e do vendedor permitirem (método +0xc); sem dia, devolve o primeiro motivo',
          'steps day by day, from today (or the contract end, in type 4) up to a year, until the rules of the buyer\'s and seller\'s nations allow it (method +0xc); with no day, returns the first reason'),
         ('janela_fixa_de_transferencias', 'transfer_manager_get_next_window_dates', '0x8d0bb0', 'transfer_manager.cpp', 'stdcall, ret 8',
          'o início (hoje, se dentro) e o fim da janela atual ou da próxima: 1º a 31 de julho e 15 de dezembro a 15 de janeiro',
          'the start (today, when inside) and the end of the current or next window: 1 to 31 July and 15 December to 15 January')])
CAMPOS = [('oferta de transferência +0x30: o tipo de prazo (& 0xf: 0 hoje, 1 e 4 pelas regras, 2 fim da temporada, 3 fim do contrato, 5 janela fixa); valor; lê (data_de_conclusao_da_oferta, primeiro_dia_permitido_da_oferta)',
           'transfer offer +0x30: the timing type (& 0xf: 0 today, 1 and 4 by the rules, 2 season end, 3 contract end, 5 fixed window); value; reads (data_de_conclusao_da_oferta, primeiro_dia_permitido_da_oferta)'),
          ('oferta +4: a lista de jogadores na troca (só se há vendedor a consultar); ponteiro; lê (primeiro_dia_permitido_da_oferta)',
           'offer +4: the list of part-exchange players (only whether the seller is consulted); pointer; reads (primeiro_dia_permitido_da_oferta)'),
          ('oferta +8 / +0xc: o clube comprador e a pessoa; id; lê (as duas primeiras)',
           'offer +8 / +0xc: the buying club and the person; id; reads (the first two)'),
          ('contrato (0x4cf5a0) +4: o clube; +0x2d: o fim (dia, ano); +0x35 & 0x3f: o tipo (5 não conta); id e valor; lê',
           'contract (0x4cf5a0) +4: the club; +0x2d: the end (day, year); +0x35 & 0x3f: the type (5 does not count); id and value; reads'),
          ('[0xae1080] o último contrato achado; ponteiro; grava (data_de_conclusao_da_oferta, primeiro_dia_permitido_da_oferta)',
           '[0xae1080] the last contract found; pointer; writes (data_de_conclusao_da_oferta, primeiro_dia_permitido_da_oferta)'),
          ('staff +0x61: a ficha de jogador (sem ela, hoje); ponteiro; lê (primeiro_dia_permitido_da_oferta)',
           'staff +0x61: the player data (without it, today); pointer; reads (primeiro_dia_permitido_da_oferta)'),
          ('clube +0x53: a nação (regras de transferência das 28 em TRANSFER_MANAGER +0x8ac, +0xa a nação); ponteiro; lê',
           'club +0x53: the nation (transfer rules among the 28 at TRANSFER_MANAGER +0x8ac, +0xa the nation); pointer; reads'),
          ('[0xae2c90] a data de hoje (dia do ano, ano); valor; lê (as três)', '[0xae2c90] today\'s date (day of year, year); value; reads (all three)')]
INTERESSA = ['CMRTE']
