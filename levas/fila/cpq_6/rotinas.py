# O dinheiro da transferência (transfer_manager.cpp): a compensação pela pessoa (transfer_manager_calc_compensation_fee
# 0x8d0780, pelo preço pedido 0x4d1600, pelo lance da IA 0x8e5d60 e pela compensação da lista 0x84b6f0), pagar a
# transferência e guardar as cláusulas (TRANSFER_MANAGER::pay_fee_and_record_clauses 0x8ca8a0, pela conclusão da
# transferência 0x8c9390) e se o clube vendedor tem de aceitar a oferta (transfer_manager_selling_club_must_accept
# 0x8cd240, pela verificação das restrições 0x8cfc90, pela 0x8c8090 e pelas telas). Quadros SEH só de valores locais
# (sem destrutor). Iguais no cm0102-1080.exe. Comparadores: a compensação e o "deve aceitar" lado a lado (valor,
# motivo, [0xae1080] e as estáticas das datas); pagar só a original (finanças e listas que alocam).
SAVE = False
ROTINAS = [('compensacao_pela_pessoa', 0x8d0780, [0x6a, 0xff, 0x68, 0x3b, 0x40, 0x96, 0x00]),
           ('pagar_transferencia_e_clausulas', 0x8ca8a0, [0x83, 0xec, 0x18, 0x53, 0x55]),
           ('clube_vendedor_deve_aceitar', 0x8cd240, [0x6a, 0xff, 0x68, 0x00, 0x3f, 0x96, 0x00])]
DOCS = ('o dinheiro da transferência: a compensação pela pessoa, pagar a transferência e guardar as cláusulas, e se o clube vendedor tem de aceitar',
        'the transfer money: the compensation for the person, paying the fee and recording the clauses, and whether the selling club must accept',
        [('compensacao_pela_pessoa', 'transfer_manager_calc_compensation_fee', '0x8d0780', 'transfer_manager.cpp', 'stdcall, ret 0xc',
          'comissão técnica de contrato tipo 2: o que falta do contrato (1000 a 1000000); regras novas: a média com o valor do jogador; entre clubes da nação [0x9cf4a8] ou, abaixo de 24 anos, entre nações britânicas: metade do valor (0x4d0c60)',
          'staff on a type 2 contract: what is left of the contract (1000 to 1000000); new rules: the average with the player value; between clubs of nation [0x9cf4a8] or, under 24, between British nations: half the value (0x4d0c60)'),
         ('pagar_transferencia_e_clausulas', 'TRANSFER_MANAGER::pay_fee_and_record_clauses', '0x8ca8a0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'soma o valor aos totais, o comprador paga e o vendedor recebe a primeira parcela, guarda as parcelas que faltam (lista +0x872) e as cláusulas de próxima venda e prêmios por jogo, com as marcas da pessoa',
          'adds the fee to the totals, the buyer pays and the seller receives the first instalment, stores the remaining instalments (list +0x872) and the sell-on and appearance clauses, with the person\'s flags'),
         ('clube_vendedor_deve_aceitar', 'transfer_manager_selling_club_must_accept', '0x8cd240', 'transfer_manager.cpp', 'stdcall, ret 0xc',
          '1 quando o valor cobre a multa de rescisão ativa (motivo 0x1b) ou o vendedor em crise recebe bem mais que vale (motivo 0x1c); sem vendedor, 1',
          '1 when the fee covers an active release clause (reason 0x1b) or the seller in financial trouble gets far more than the value (reason 0x1c); without a seller, 1')])
CAMPOS = [('oferta de transferência +8 / +0xc: o comprador e a pessoa; id; lê (pagar_transferencia_e_clausulas, clube_vendedor_deve_aceitar)',
           'transfer offer +8 / +0xc: the buyer and the person; id; reads (pagar_transferencia_e_clausulas, clube_vendedor_deve_aceitar)'),
          ('oferta +0x1b: o valor; +0x40: as parcelas; +4: a troca; valor e ponteiro; lê',
           'offer +0x1b: the fee; +0x40: the instalments; +4: the part exchange; value and pointer; reads'),
          ('oferta +0x3f: a porcentagem da próxima venda; +0x35/+0x39 e +0x3a/+0x3e: os prêmios por jogo no time e pela seleção; valor; lê (pagar_transferencia_e_clausulas)',
           'offer +0x3f: the sell-on percentage; +0x35/+0x39 and +0x3a/+0x3e: the club and international appearance money; value; reads (pagar_transferencia_e_clausulas)'),
          ('TRANSFER_MANAGER +0x872: a lista das parcelas (13 bytes); +0x86e: o registro de 8 bytes da pessoa (marcas 8, 2, 4); ponteiro e valor; grava (pagar_transferencia_e_clausulas)',
           'TRANSFER_MANAGER +0x872: the instalment list (13 bytes); +0x86e: the person\'s 8-byte record (flags 8, 2, 4); pointer and value; writes (pagar_transferencia_e_clausulas)'),
          ('finanças do clube (0x5a0590) +0x1c: soma ± 5/21 da parcela; +0x165: a crise (1); valor; grava e lê',
           'club finances (0x5a0590) +0x1c: adds ± 5/21 of the instalment; +0x165: the financial trouble flag (1); value; writes and reads'),
          ('contrato (0x4cf5a0) +4 o clube; +0xc o salário; +0x21 a multa de rescisão; +0x2d o fim; +0x35 o tipo; id e valor; lê',
           'contract (0x4cf5a0) +4 the club; +0xc the wage; +0x21 the release fee; +0x2d the end; +0x35 the type; id and value; reads'),
          ('staff +0x61 / +0x69: as fichas de jogador e de comissão técnica; ponteiro; lê (compensacao_pela_pessoa)',
           'staff +0x61 / +0x69: the player and non-player data; pointer; reads (compensacao_pela_pessoa)'),
          ('clube +0x53: a nação; +0x80: a reputação; ponteiro e valor; lê', 'club +0x53: the nation; +0x80: the reputation; pointer and value; reads'),
          ('[0xae233c] as regras novas de transferência (1); valor; lê (compensacao_pela_pessoa)', '[0xae233c] the new transfer rules (1); value; reads (compensacao_pela_pessoa)'),
          ('[0xae1080] o último contrato achado; ponteiro; grava (pagar_transferencia_e_clausulas, clube_vendedor_deve_aceitar)',
           '[0xae1080] the last contract found; pointer; writes (pagar_transferencia_e_clausulas, clube_vendedor_deve_aceitar)')]
INTERESSA = ['CMRTE', 'SGE']
