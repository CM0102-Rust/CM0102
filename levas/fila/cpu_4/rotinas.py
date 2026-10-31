# As cláusulas do contrato (staff_contracts.cpp): a cláusula de rescisão ativa (STAFF_CONTRACT::has_active_release_clause
# 0x867b50, pelo dia dos contratos CONTRACT_MANAGER::daily_update 0x4c7ac0, pelo valor do jogador
# contract_manager_calc_player_value 0x4d0c60 e pelo preço pedido 0x4d2070; também a resposta prevista do vendedor
# 0x84af00 e o vendedor que tem de aceitar 0x8cd240, já em Rust, e o lance TRANSFER_OFFER::build_bid 0x8e5d60), o
# período protegido da FIFA (STAFF_CONTRACT::check_fifa_protected_period 0x868210, pelo dia dos contratos, pelo lance e
# pela troca com compensação 0x84b6f0; quadro SEH só da data local, sem destrutor) e o tipo de bônus permitido
# (staff_contract_is_bonus_type_allowed 0x867aa0, pela tela do contrato; passageira). Chamadas originais pelo endereço:
# 0x54cc30 (a duração entre datas) e 0x53ab90 (a idade na data). Iguais no cm0102-1080.exe. Comparadores: pela
# devolução; o período também pelas duas saídas (a cópia grava em locais que partem do valor de antes).
SAVE = False
ROTINAS = [('clausula_de_rescisao_ativa', 0x867b50, [0x8a, 0x51, 0x1c, 0xb0, 0x02]),
           ('periodo_protegido', 0x868210, [0x6a, 0xff, 0x68, 0x08, 0x2b, 0x96, 0x00]),
           ('tipo_de_bonus_permitido', 0x867aa0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x61])]
DOCS = ('as cláusulas do contrato: a cláusula de rescisão ativa, o período protegido da FIFA e o tipo de bônus permitido',
        'the contract clauses: the active release clause, the FIFA protected period and the allowed bonus type',
        [('clausula_de_rescisao_ativa', 'STAFF_CONTRACT::has_active_release_clause', '0x867b50', 'staff_contracts.cpp', 'thiscall',
          '1 com uma das três cláusulas de rescisão ativa no contrato',
          '1 when one of the three release clauses of the contract is active'),
         ('periodo_protegido', 'STAFF_CONTRACT::check_fifa_protected_period', '0x868210', 'staff_contracts.cpp', 'thiscall',
          'se o contrato ainda está no período protegido das regras novas da FIFA: 3 anos abaixo de 28 anos de idade na assinatura, senão 2',
          'whether the contract is still in the protected period of the new FIFA rules: 3 years under age 28 at signing, otherwise 2'),
         ('tipo_de_bonus_permitido', 'staff_contract_is_bonus_type_allowed', '0x867aa0', 'staff_contracts.cpp', 'cdecl',
          'se o tipo de bônus vale para o jogador: o tipo 2 só para goleiros, os outros só para quem não é goleiro',
          'whether the bonus type applies to the player: type 2 only for goalkeepers, the others only for non-goalkeepers')])
CAMPOS = [('contrato +0x1c, +0x1d, +0x1f: cláusulas de rescisão; valor; lê (clausula_de_rescisao_ativa)',
           'contract +0x1c, +0x1d, +0x1f: release clauses; value; reads (clausula_de_rescisao_ativa)'),
          ('contrato +0: id da pessoa; id; +0x25: data do contrato; valor; lê (periodo_protegido)',
           'contract +0: person id; id; +0x25: contract date; value; reads (periodo_protegido)'),
          ('[0xae233c] regras novas da FIFA ligadas; [0xae2330] data das regras; valor; lê (periodo_protegido)',
           '[0xae233c] new FIFA rules on; [0xae2330] rules date; value; reads (periodo_protegido)'),
          ('pessoa +0x61: ficha de jogador; ponteiro; ficha +0xf: posição (0x14 goleiro); lê (tipo_de_bonus_permitido)',
           'person +0x61: player record; pointer; record +0xf: position (0x14 goalkeeper); reads (tipo_de_bonus_permitido)')]
INTERESSA = ['CMRTE', 'SGE']
