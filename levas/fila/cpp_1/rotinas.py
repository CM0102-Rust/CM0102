# As datas de decisão e as ofertas do clube (contract_manager.cpp): a ordem das datas de decisão (compare_decision_dates
# 0x4dc910, o comparador do qsort que a CONTRACT_MANAGER::daily_update 0x4c9de7 usa antes de decidir), decidir as
# datas vencidas (CONTRACT_MANAGER::process_due_decision_dates 0x4cd9c0, pela 0x4c9e15 todo dia: a pessoa decide as
# ofertas de contrato, 0x4ce4e0) e as ofertas de contrato do clube (CONTRACT_MANAGER::get_current_contract_offers
# 0x4ca300, pelo orçamento de transferências Finance::get_transfer_budget 0x598a06). Sem SEH, sem sorteio, sem leitura
# de lixo (a 0x4ca300 tem 0x208 bytes de pilha só para a caixa de erro). Iguais no cm0102-1080.exe. Comparadores: a
# ordem pelo valor; as datas vencidas pelo plano (com decisão do tipo 3, só a original; sem ela, lado a lado pelo
# índice e pelas entradas vencidas); as ofertas só a original (aloca o vetor) com o plano lido da lista antes.
SAVE = False
ROTINAS = [('comparar_datas_de_decisao', 0x4dc910, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('decidir_as_datas_vencidas', 0x4cd9c0, [0x53, 0x55, 0x56, 0x57, 0x8b, 0xf9]),
           ('ofertas_de_contrato_do_clube', 0x4ca300, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as datas de decisão e as ofertas do clube: a ordem das datas, decidir as datas vencidas e as ofertas de contrato do clube',
        'decision dates and club offers: the date order, deciding the due dates and the club\'s contract offers',
        [('comparar_datas_de_decisao', 'compare_decision_dates', '0x4dc910', 'contract_manager.cpp', 'cdecl',
          'a ordem do qsort das datas de decisão: pela data e, no mesmo dia, pela fatia do dia (com sinal)',
          'the qsort order of the decision dates: by date and, on the same day, by the time slice (signed)'),
         ('decidir_as_datas_vencidas', 'CONTRACT_MANAGER::process_due_decision_dates', '0x4cd9c0', 'contract_manager.cpp', 'thiscall, ret 8',
          'consome as datas de decisão vencidas até a data e a fatia dadas: cada uma fica com −1 e, sendo do tipo 3, a pessoa decide as ofertas de contrato',
          'consumes the decision dates due by the given date and slice: each one becomes −1 and, for type 3, the person decides on the contract offers'),
         ('ofertas_de_contrato_do_clube', 'CONTRACT_MANAGER::get_current_contract_offers', '0x4ca300', 'contract_manager.cpp', 'thiscall, ret 8',
          'copia as ofertas de contrato abertas do clube (fora dos estados 5 e 6) para um vetor novo de 111 bytes por oferta e devolve quantas',
          'copies the club\'s open contract offers (outside states 5 and 6) into a new array of 111 bytes per offer and returns how many')])
CAMPOS = [('CONTRACT_MANAGER +0xc, +0x14, +0x18: o vetor das datas de decisão (14 bytes: +0 id da pessoa ou −1, +4 data, +0xc fatia, +0xd tipo), a conta e o próximo índice; ponteiro e valores; lê e grava −1 e o índice (decidir_as_datas_vencidas), lê (comparar_datas_de_decisao)',
           'CONTRACT_MANAGER +0xc, +0x14, +0x18: the decision date array (14 bytes: +0 person id or −1, +4 date, +0xc slice, +0xd type), the count and the next index; pointer and values; reads and writes −1 and the index (decidir_as_datas_vencidas), reads (comparar_datas_de_decisao)'),
          ('CONTRACT_MANAGER +0x34: a lista das ofertas de contrato (111 bytes: +4 id do clube, +0x35 estado nos 6 bits baixos); ponteiro; lê (ofertas_de_contrato_do_clube)',
           'CONTRACT_MANAGER +0x34: the contract offer list (111 bytes: +4 club id, +0x35 state in the low 6 bits); pointer; reads (ofertas_de_contrato_do_clube)')]
INTERESSA = ['CMRTE', 'SGE']
