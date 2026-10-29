# Contratos (contract_manager.cpp): a vaga do vetor de contratos (CONTRACT_MANAGER::get_free_staff_contract_space
# 0x4c79a0, pela criação dos contratos 0x4c7560 e pela 0x4c7950), dar o contrato à pessoa nova
# (CONTRACT_MANAGER::add_staff_contract 0x4c7950, pela contratação de técnico 0x68f9c0 e pelo regen que assina
# 0x7a6f10; 0x85e960 já em Rust) e marcar a data da decisão (CONTRACT_MANAGER::add_decision_date 0x4cd6d0, pelas
# propostas 0x4cd230 e 0x4ce4e0; 0x54c8e0 já em Rust). Simulação: as mutações pela carreira e a confirmação no jogo
# real comparando. Sem SEH, sem sorteio; caixa só sem memória. Comparadores: as três realocam (e a 0x4c7950 gera o
# contrato), então o plano é montado antes, só a original roda e o resultado é conferido.
SAVE = False
ROTINAS = [('vaga_livre_de_contrato', 0x4c79a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('acrescentar_contrato_da_pessoa', 0x4c7950, [0x56, 0x8b, 0xf1, 0x6a, 0x01]),
           ('acrescentar_data_de_decisao', 0x4cd6d0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('os contratos da comissão e as datas de decisão: achar a vaga de contrato, dar o contrato à pessoa nova e marcar a data da decisão',
        'staff contracts and decision dates: finding the contract slot, giving the new person a contract and scheduling the decision date',
        [('vaga_livre_de_contrato', 'CONTRACT_MANAGER::get_free_staff_contract_space', '0x4c79a0', 'contract_manager.cpp', 'thiscall, ret 4',
          'a vaga do vetor de contratos de 0x50 bytes: a primeira vazia (id −1) ou, só no fim, a nova (cresce de 200 em 200; sem memória, caixa e −1)',
          'the slot in the 0x50-byte contract vector: the first empty one (id −1) or, append only, a new one (grows by 200; without memory, error box and −1)'),
         ('acrescentar_contrato_da_pessoa', 'CONTRACT_MANAGER::add_staff_contract', '0x4c7950', 'contract_manager.cpp', 'thiscall, ret 8',
          'a vaga do fim para a pessoa nova (técnico contratado, regen): liga a pessoa à vaga em [0xae2d0c], põe o id e gera o contrato inicial pelo valor (0x85e960)',
          'the appended slot for a new person (hired manager, regen): links the person to the slot in [0xae2d0c], stores the id and generates the initial contract from the value (0x85e960)'),
         ('acrescentar_data_de_decisao', 'CONTRACT_MANAGER::add_decision_date', '0x4cd6d0', 'contract_manager.cpp', 'thiscall, ret 0x10',
          'acrescenta (pessoa, data, fatia do dia, tipo) ao vetor de 14 bytes das datas de decisão (cresce de 100 em 100); o tipo 1 leva antes lembretes de tipo 2 pela moral do contrato',
          'appends (person, date, time slice, type) to the 14-byte decision date vector (grows by 100); type 1 first adds type 2 reminders from the contract morale')])
CAMPOS = [('contract manager +0/+4/+8: vetor de contratos de 0x50 bytes, capacidade, conta; ponteiro e valores; lê e grava (vaga_livre_de_contrato, acrescentar_contrato_da_pessoa)',
           'contract manager +0/+4/+8: 0x50-byte contract vector, capacity, count; pointer and values; reads and writes (vaga_livre_de_contrato, acrescentar_contrato_da_pessoa)'),
          ('contrato +0: id da pessoa (−1 vaga vazia); id; lê e grava (vaga_livre_de_contrato, acrescentar_contrato_da_pessoa)',
           'contract +0: person id (−1 empty slot); id; reads and writes (vaga_livre_de_contrato, acrescentar_contrato_da_pessoa)'),
          ('[0xae2d0c] + id × 0x4f, +0: índice do contrato da pessoa; índice; lê e grava (acrescentar_contrato_da_pessoa), lê (acrescentar_data_de_decisao)',
           '[0xae2d0c] + id × 0x4f, +0: index of the person\'s contract; index; reads and writes (acrescentar_contrato_da_pessoa), reads (acrescentar_data_de_decisao)'),
          ('contrato +0x44: moral; valor; lê (acrescentar_data_de_decisao, os lembretes do tipo 1)',
           'contract +0x44: morale; value; reads (acrescentar_data_de_decisao, the type 1 reminders)'),
          ('pessoa +0x61: registro de jogador; ponteiro; lê (acrescentar_data_de_decisao)',
           'staff +0x61: player record; pointer; reads (acrescentar_data_de_decisao)'),
          ('contract manager +0xc/+0x10/+0x14: vetor de 14 bytes das datas de decisão (+0 id, +4 data, +0xc fatia, +0xd tipo), capacidade, conta; lê e grava (acrescentar_data_de_decisao)',
           'contract manager +0xc/+0x10/+0x14: 14-byte decision date vector (+0 id, +4 date, +0xc time slice, +0xd type), capacity, count; reads and writes (acrescentar_data_de_decisao)'),
          ('[0xae2364] número de pessoas; valor; lê (acrescentar_data_de_decisao)',
           '[0xae2364] number of staff; value; reads (acrescentar_data_de_decisao)')]
INTERESSA = ['SGE', 'CMRTE']
