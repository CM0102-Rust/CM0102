# Os limites da escalação (transfer_manager.cpp): a tabela de 42 bytes das restrições da partida
# (TRANSFER_MANAGER::fill_squad_restrictions 0x8cf6b0, pelo limite de escalação 0x8cf7e0 e pela tela do clube 0x4849f0),
# o limite do banco do dia (transfer_manager_check_matchday_squad_limit 0x8cf880, pela escalação automática 0x6dbc50 e
# pela tela da escalação 0x725f70; thiscall de fato: o `this` em ecx vai direto à 0x8cf7e0) e quem precisa de permissão
# de trabalho (TRANSFER_MANAGER::player_needs_work_permit 0x8c7560, pela escalação automática, pelos contratos e pelas
# telas). Sem SEH; iguais no cm0102-1080.exe. Comparadores: lado a lado pela tabela; lado a lado pela saída e pelo
# aviso, com a devolução; pelo valor (jogador nulo: a caixa, só a original).
SAVE = False
ROTINAS = [('restricoes_da_partida', 0x8cf6b0, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x18]),
           ('limite_do_banco_do_dia', 0x8cf880, [0x83, 0xec, 0x0c, 0x55, 0x8b, 0x6c, 0x24, 0x18]),
           ('precisa_de_permissao_de_trabalho', 0x8c7560, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('os limites da escalação: a tabela de restrições da partida, o limite do banco do dia e quem precisa de permissão de trabalho',
        'the selection limits: the fixture restriction table, the matchday squad limit and who needs a work permit',
        [('restricoes_da_partida', 'TRANSFER_MANAGER::fill_squad_restrictions', '0x8cf6b0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'enche a tabela de 42 bytes com 0xff e, pelo alcance das regras da partida (0x8d2fa0), deixa a regra da nação ou do continente enchê-la (virtual +0x1c); com o pedido, fica só o primeiro limite (+5/+3, +2/+0 ou +0x16/+0x17)',
          'fills the 42-byte table with 0xff and, by the fixture rules scope (0x8d2fa0), lets the nation or continent rule fill it (virtual +0x1c); when asked, keeps only the first limit (+5/+3, +2/+0 or +0x16/+0x17)'),
         ('limite_do_banco_do_dia', 'transfer_manager_check_matchday_squad_limit', '0x8cf880', 'transfer_manager.cpp', 'thiscall (this em ecx), ret 0x18',
          'conta, pelas vagas do banco do dia (jogadores dados ou a tática 0x8968f0), os não comunitários, os estrangeiros ou os acima da idade do limite da partida (0x8cf7e0); passando, 1 e o aviso',
          'counts, over the matchday squad slots (given players or the tactics 0x8968f0), the non-EU, foreign or over-age players of the fixture limit (0x8cf7e0); when over, 1 and the message'),
         ('precisa_de_permissao_de_trabalho', 'TRANSFER_MANAGER::player_needs_work_permit', '0x8c7560', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          '1 se o clube é de nação com permissão de trabalho (+0x76 = 0xe), o jogador não é da UE e não tem a permissão dada (contrato do tipo 2, marca 0x20, busca no vetor +0x93a)',
          '1 when the club nation uses work permits (+0x76 = 0xe), the player is not EU and holds no granted permit (type 2 contract, 0x20 flag, search in the +0x93a array)')])
CAMPOS = [('TRANSFER_MANAGER +0x8ac: as 28 regras de transferência das nações (+0xa nação, +0xe continente, +0x12 tipo, virtual +0x1c enche a tabela); ponteiro; lê (restricoes_da_partida)',
           'TRANSFER_MANAGER +0x8ac: the 28 national transfer rules (+0xa nation, +0xe continent, +0x12 type, virtual +0x1c fills the table); pointer; reads (restricoes_da_partida)'),
          ('tabela de restrições (42 bytes): +0/+2 estrangeiros, +3/+5 não comunitários, +0x16 idade e +0x17 quantidade; valor; grava (restricoes_da_partida)',
           'restriction table (42 bytes): +0/+2 foreign, +3/+5 non-EU, +0x16 age and +0x17 count; value; writes (restricoes_da_partida)'),
          ('partida +0x1c/+0x20: mandante e visitante; +0x40/+0x41: lado do computador; +0x4d & 0xf: reservas além dos 11; ponteiro e valor; lê (limite_do_banco_do_dia)',
           'fixture +0x1c/+0x20: home and away; +0x40/+0x41: computer side; +0x4d & 0xf: substitutes beyond the 11; pointer and value; reads (limite_do_banco_do_dia)'),
          ('limite de escalação (6 bytes): +0 tipo, +1 idade, +2 titulares, +3 reservas, +4 total, +5 quantidade; valor; grava (limite_do_banco_do_dia)',
           'selection limit (6 bytes): +0 type, +1 age, +2 starters, +3 substitutes, +4 total, +5 count; value; writes (limite_do_banco_do_dia)'),
          ('staff +0x18: idade; valor; lê (limite_do_banco_do_dia)', 'staff +0x18: age; value; reads (limite_do_banco_do_dia)'),
          ('clube +0x53: a nação; nação +0x76: a marca 0xe (permissão de trabalho); ponteiro e valor; lê', 'club +0x53: the nation; nation +0x76: the 0xe marker (work permit); pointer and value; reads'),
          ('contrato +0x35 & 0x3f: o tipo (2); valor; lê (precisa_de_permissao_de_trabalho)', 'contract +0x35 & 0x3f: the type (2); value; reads (precisa_de_permissao_de_trabalho)'),
          ('TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bit 0x20; +0x93a/+0x93e: as permissões (14 bytes) e a conta; valor; lê (precisa_de_permissao_de_trabalho)',
           'TRANSFER_MANAGER +0x86e: person\'s 8-byte record, bit 0x20; +0x93a/+0x93e: the permits (14 bytes) and the count; value; reads (precisa_de_permissao_de_trabalho)')]
INTERESSA = ['CMRTE']
