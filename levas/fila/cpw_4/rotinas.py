# A oferta de contrato (contract_manager.cpp): acrescentar ou atualizar a oferta de contrato de um clube a uma pessoa
# (CONTRACT_MANAGER::add_contract_offer 0x4cc490, 3032 chamadas ate 2002-06-01 na cobertura de 17/10: a lista +0x34, o
# contador de ofertas da pessoa no registro de 79 bytes, os termos oferecidos na oferta de transferencia e a decisao
# agendada), tirar a oferta de contrato pela vaga da transferencia (remove_contract_offer_for_transfer 0x4ccb30, pelas
# ofertas retiradas e desfeitas) e o menor salario pedido (contract_manager_lowest_wage_demand 0x4ce450, pela tela do
# contrato). Chamadas originais pelo endereco: 0x85e410 (o contrato local), 0x85fc50 (o desejado), 0x68d020/0x68d0d0/
# 0x68d1f0 (a lista), 0x944cf1, 0x7a81a0; as outras ja em Rust (0x54c5b0, 0x54a110, 0x417c10, 0x4cc810, 0x68d1b0,
# 0x8c61e0, 0x8c1f80, 0x8c2060, 0x4cd230, 0x8c5430). Iguais no cm0102-1080.exe. Comparadores: a oferta pelo ensaio (a
# copia sem mexer na lista e sem as chamadas do fim; atualizada: a oferta, o contador e o cursor iguais; nova: o no que
# a original acrescenta igual ao registro da copia, e o contador); tirar so a original com o plano (o contador desce
# se a oferta estava la); o menor salario lado a lado.
SAVE = False
ROTINAS = [('acrescentar_oferta_de_contrato', 0x4cc490, [0x6a, 0xff, 0x68, 0x29, 0x6d, 0x95, 0x00]),
           ('tirar_oferta_de_contrato_da_vaga', 0x4ccb30, [0x53, 0x55, 0x56, 0x57, 0x8b, 0xf9]),
           ('menor_salario_pedido', 0x4ce450, [0x6a, 0xff, 0x68, 0x88, 0x6d, 0x95, 0x00])]
DOCS = ('a oferta de contrato acrescentada ou atualizada, a oferta de contrato tirada pela vaga e o menor salário pedido',
        'the contract offer added or updated, the contract offer removed by transfer slot and the lowest wage demand',
        [('acrescentar_oferta_de_contrato', 'add_contract_offer', '0x4cc490', 'contract_manager.cpp', 'thiscall',
          'atualiza a oferta do mesmo clube à pessoa ou acrescenta uma nova à lista (o contador de ofertas sobe); marca os termos oferecidos na oferta de transferência e, se pedido, agenda a decisão',
          'updates the same club\'s offer to the person or adds a new one to the list (the offer count goes up); marks the terms as offered on the transfer offer and, if asked, schedules the decision'),
         ('tirar_oferta_de_contrato_da_vaga', 'remove_contract_offer_for_transfer', '0x4ccb30', 'contract_manager.cpp', 'thiscall',
          'tira da lista a oferta de contrato da pessoa ligada à vaga da transferência (desliga a marca de contrato acertado) e baixa o contador de ofertas',
          'removes the person\'s contract offer linked to the transfer slot from the list (clearing the agreed-contract flag) and lowers the offer count'),
         ('menor_salario_pedido', 'contract_manager_lowest_wage_demand', '0x4ce450', 'contract_manager.cpp', 'stdcall',
          'o menor dos salários desejados com a situação 1 e 2 no clube, no máximo 10 milhões',
          'the lower of the desired wages for squad status 1 and 2 at the club, at most 10 million')])
CAMPOS = [('[0xae2d0c] + 79 × id +8: contador de ofertas de contrato da pessoa (general.dat); valor; lê e grava (acrescentar_oferta_de_contrato, tirar_oferta_de_contrato_da_vaga)',
           '[0xae2d0c] + 79 × id +8: person\'s contract offer count (general.dat); value; reads and writes (acrescentar_oferta_de_contrato, tirar_oferta_de_contrato_da_vaga)'),
          ('CONTRACT_MANAGER +0x34: lista das ofertas de contrato (registros de 0x6f bytes); ponteiro; lê e grava (acrescentar_oferta_de_contrato, tirar_oferta_de_contrato_da_vaga)',
           'CONTRACT_MANAGER +0x34: list of contract offers (0x6f-byte records); pointer; reads and writes (acrescentar_oferta_de_contrato, tirar_oferta_de_contrato_da_vaga)'),
          ('oferta de contrato +0..+0x50: os termos (STAFF_CONTRACT); +0x50: 2 acertada; +0x51: data; +0x59: vaga da transferência; +0x5d: valor; +0x61: marca; +0x62: fora do tipo 6; +0x63..+0x6e: o pedido de 0x7a81a0; valor; grava e lê',
           'contract offer +0..+0x50: the terms (STAFF_CONTRACT); +0x50: 2 agreed; +0x51: date; +0x59: transfer slot; +0x5d: value; +0x61: flag; +0x62: not type 6; +0x63..+0x6e: the 0x7a81a0 request; value; writes and reads'),
          ('oferta de transferência +0xc: jogador; id; lê (acrescentar_oferta_de_contrato)', 'transfer offer +0xc: player; id; reads (acrescentar_oferta_de_contrato)'),
          ('[0xae1080] o último contrato pedido; ponteiro; grava (menor_salario_pedido, pela 0x85fc50)',
           '[0xae1080] the last contract asked for; pointer; writes (menor_salario_pedido, via 0x85fc50)')]
INTERESSA = ['CMRTE', 'SGE']
