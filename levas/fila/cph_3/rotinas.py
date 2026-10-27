# Contratos (contract_manager.cpp), 3º item: a cópia do contrato de pessoal (STAFF_CONTRACT::operator= 0x4cc810, 3.141
# chamadas na carreira até 2002-06-01; as datas pela 0x417c10), a moral depois da boa partida
# (contract_manager_raise_match_morale 0x4cb0a0, 153.222; ponto flutuante x87 com o ftol 0x945640; chama a 0x8674e0 e a
# 0x867240, originais) e tirar as conversas de contrato da pessoa (CONTRACT_MANAGER::remove_talks_records 0x4ca1c0,
# 750; qsort 0x945333 com 0x4dc970). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real
# comparando. Sem SEH, sem caixa, sem sorteio, sem local lida sem valor (a data local é gravada pela 0x417c10).
# Comparadores: a cópia lado a lado pelo contrato inteiro (0x50); a moral só a original (a 0x8674e0 e a 0x867240 mexem
# nas queixas, nos vínculos e na transferência); as conversas lado a lado pelo vetor e pela contagem. A cópia do byte
# +0x35 em três passos e do +0x4f em dois termina igual ao de origem (os passos ficam como no original).
SAVE = False
ROTINAS = [('copiar_contrato_de_pessoal', 0x4cc810, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('subir_moral_pela_partida', 0x4cb0a0, [0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tirar_conversas_de_contrato', 0x4ca1c0, [0x51, 0x8b, 0xc1, 0x53, 0x33, 0xdb])]
DOCS = ('a cópia do contrato de pessoal, a moral depois da boa partida e tirar as conversas de contrato',
        'the staff contract copy, the morale after a good game and removing the contract talks',
        [('copiar_contrato_de_pessoal', 'STAFF_CONTRACT::operator=', '0x4cc810', 'contract_manager.cpp', 'thiscall, ret 4',
          'copia o contrato (0x50 bytes) campo a campo, as datas pela 0x417c10 e os campos de bits +0x35 e +0x4f em passos',
          'copies the contract (0x50 bytes) field by field, the dates through 0x417c10 and the bit fields +0x35 and +0x4f in steps'),
         ('subir_moral_pela_partida', 'contract_manager_raise_match_morale', '0x4cb0a0', 'contract_manager.cpp', 'stdcall, ret 0xc',
          'o ganho de moral pela nota contra a reputação do clube (até 10), a moral entre −100 e 100 e abaixo do teto das queixas; apaga as queixas 4 e 0x10 (0x8674e0) e confere a retirada (0x867240)',
          'the morale gain from the rating against the club reputation (up to 10), morale between −100 and 100 and under the grievance cap; clears grievances 4 and 0x10 (0x8674e0) and checks the withdrawal (0x867240)'),
         ('tirar_conversas_de_contrato', 'CONTRACT_MANAGER::remove_talks_records', '0x4ca1c0', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'tira as conversas da pessoa (e do clube e do técnico, quando dados) pondo o último no lugar; reordena',
          'removes the person talks (and of the club and manager, when given) moving the last one into the gap; sorts again')])
CAMPOS = [('contrato de pessoal (0x50 bytes): +0x25/+0x2d datas, +0x35 e +0x4f campos de bits, +0x3d felicidade, +0x3f, +0x44 moral, +0x45 queixas, +0x49; escreve (copiar_contrato_de_pessoal, subir_moral_pela_partida); CONTRACT_MANAGER +0x28/+0x2c conversas (21 bytes: pessoa, clube, técnico); escreve (tirar_conversas_de_contrato)',
           'staff contract (0x50 bytes): +0x25/+0x2d dates, +0x35 and +0x4f bit fields, +0x3d happiness, +0x3f, +0x44 morale, +0x45 grievances, +0x49; writes (copiar_contrato_de_pessoal, subir_moral_pela_partida); CONTRACT_MANAGER +0x28/+0x2c talks (21 bytes: person, club, manager); writes (tirar_conversas_de_contrato)')]
INTERESSA = ['SGE', 'CMRTE']
