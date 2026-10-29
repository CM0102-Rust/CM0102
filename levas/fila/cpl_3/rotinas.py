# Os registros das transferências (transfer_manager.cpp): guardar a transferência passada
# (TRANSFER_MANAGER::add_past_transfer 0x8cb810, ~950 chamadas na carreira; a cópia 0x8cb940 já em Rust), dar a
# permissão de trabalho (TRANSFER_MANAGER::add_work_permit 0x8c7070, ~100; lida pela 0x8c7260/0x8c7560) e guardar o
# prêmio por jogo no time principal (TRANSFER_MANAGER::add_first_team_appearance_money 0x8cac40, ~14). Trocas: as
# irmãs 0x8caaf0 (percentual da próxima venda) e 0x8cad90 (prêmio por jogo na seleção) rodam 2 e 1 vez na carreira;
# entram a 0x8cb810 e a 0x8c7070. Sem SEH, sem sorteio; caixa só sem memória. Comparadores: a original roda (realoca,
# reordena) e o resultado é conferido com o plano.
SAVE = False
ROTINAS = [('guardar_transferencia_passada', 0x8cb810, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('dar_permissao_de_trabalho', 0x8c7070, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('guardar_premio_por_jogo_no_time', 0x8cac40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os registros das transferências: guardar a transferência passada, dar a permissão de trabalho e guardar o prêmio por jogo no time',
        'transfer records: storing the past transfer, granting the work permit and storing the first-team appearance money',
        [('guardar_transferencia_passada', 'TRANSFER_MANAGER::add_past_transfer', '0x8cb810', 'transfer_manager.cpp', 'thiscall, ret 4',
          'acrescenta a transferência passada (0x25 bytes, 0x8cb940) ao vetor +0x862 (capacidade +0x866, conta +0x86a), que cresce de 100 em 100; devolve o registro novo',
          'appends the past transfer (0x25 bytes, 0x8cb940) to the +0x862 vector (capacity +0x866, count +0x86a), growing by 100; returns the new record'),
         ('dar_permissao_de_trabalho', 'TRANSFER_MANAGER::add_work_permit', '0x8c7070', 'transfer_manager.cpp', 'thiscall, ret 0x14',
          'acrescenta a permissão (14 bytes: pessoa, ano +8, nação +0xa) ao vetor +0x93a (cresce de 30 em 30), liga a marca 0x20 da pessoa em +0x86e e, se pedido, reordena (0x8e55c0)',
          'appends the permit (14 bytes: person, year +8, nation +0xa) to the +0x93a vector (growing by 30), sets the person 0x20 flag at +0x86e and, when asked, sorts (0x8e55c0)'),
         ('guardar_premio_por_jogo_no_time', 'TRANSFER_MANAGER::add_first_team_appearance_money', '0x8cac40', 'transfer_manager.cpp', 'thiscall, ret 4',
          'acrescenta o prêmio por jogo (13 bytes) ao vetor +0x88c (cresce de 50 em 50) e reordena pela pessoa (0x8b4e00)',
          'appends the appearance money (13 bytes) to the +0x88c vector (growing by 50) and sorts by person (0x8b4e00)')])
CAMPOS = [('gerente de transferências +0x862/+0x866/+0x86a: vetor de 0x25 bytes das transferências passadas, capacidade, conta; ponteiro e valores; lê e grava (guardar_transferencia_passada)',
           'transfer manager +0x862/+0x866/+0x86a: 0x25-byte past transfer vector, capacity, count; pointer and values; reads and writes (guardar_transferencia_passada)'),
          ('gerente de transferências +0x93a/+0x942/+0x93e: vetor de 14 bytes das permissões de trabalho (+0 pessoa, +4/+6 zerados, +8 ano, +0xa nação), capacidade, conta; lê e grava (dar_permissao_de_trabalho)',
           'transfer manager +0x93a/+0x942/+0x93e: 14-byte work permit vector (+0 person, +4/+6 zeroed, +8 year, +0xa nation), capacity, count; reads and writes (dar_permissao_de_trabalho)'),
          ('gerente de transferências +0x86e: tabela de 8 bytes por id de pessoa (bit 0x20 = tem permissão de trabalho); valor; grava (dar_permissao_de_trabalho)',
           'transfer manager +0x86e: 8-byte table by person id (bit 0x20 = has work permit); value; writes (dar_permissao_de_trabalho)'),
          ('gerente de transferências +0x88c/+0x890/+0x894: vetor de 13 bytes dos prêmios por jogo no time principal, capacidade, conta; lê e grava (guardar_premio_por_jogo_no_time)',
           'transfer manager +0x88c/+0x890/+0x894: 13-byte first-team appearance money vector, capacity, count; reads and writes (guardar_premio_por_jogo_no_time)')]
INTERESSA = ['SGE', 'CMRTE']
