# As permissões de trabalho do começo do jogo (transfer_manager.cpp): dar as permissões ou a segunda nacionalidade pela
# carreira de cada estrangeiro (TRANSFER_MANAGER::init_work_permits 0x8c6e60, pela criação do gerente de transferências
# 0x8bba30; quadro SEH só da data local, de destrutor vazio), a contagem da cláusula do jogador
# (TRANSFER_MANAGER::get_clause_count 0x8c3e50, pela tela do jogador 0x884ea2) e o prêmio por jogo do jogador
# (TRANSFER_MANAGER::find_first_team_appearance_money 0x8d0ea0, pela mesma tela). Iguais no cm0102-1080.exe.
# Comparadores: só a original na primeira (cresce o vetor das permissões; a cópia leva os ramos); pelo valor; lado a
# lado pelo byte da marca, com a devolução.
SAVE = False
ROTINAS = [('iniciar_permissoes_de_trabalho', 0x8c6e60, [0x6a, 0xff, 0x68, 0x68, 0x3d, 0x96, 0x00]),
           ('contagem_da_clausula', 0x8c3e50, [0x8a, 0x44, 0x24, 0x08, 0x56]),
           ('premio_por_jogo_do_jogador', 0x8d0ea0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('as permissões de trabalho do começo do jogo, a contagem da cláusula e o prêmio por jogo do jogador',
        'the work permits at game start, the clause count and the player\'s appearance money',
        [('iniciar_permissoes_de_trabalho', 'TRANSFER_MANAGER::init_work_permits', '0x8c6e60', 'transfer_manager.cpp', 'thiscall, ret 0',
          'marca 0x3fff/0x1fff pelos jogos na seleção; ao jogador que precisa de permissão (0x8c7560), pelo ano mais antigo na nação do clube (carreira 0x7c3e40): menos de 5 anos, a permissão (0x8c7070); senão a segunda nacionalidade (+0x1e); ordena as permissões',
          'marks 0x3fff/0x1fff by international caps; for a player needing a permit (0x8c7560), by the earliest year in the club nation (career 0x7c3e40): under 5 years, the permit (0x8c7070); otherwise the second nationality (+0x1e); sorts the permits'),
         ('contagem_da_clausula', 'TRANSFER_MANAGER::get_clause_count', '0x8c3e50', 'transfer_manager.cpp', 'thiscall, ret 8',
          'tipo 4: jogos pela seleção desde o acordo (+0x22 menos o byte +7 do registro da pessoa); senão os jogos pelo clube (+0x3b do contrato)',
          'type 4: international caps since the deal (+0x22 minus byte +7 of the person record); otherwise club appearances (+0x3b of the contract)'),
         ('premio_por_jogo_do_jogador', 'TRANSFER_MANAGER::find_first_team_appearance_money', '0x8d0ea0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'com o bit 2 da pessoa, busca o prêmio por jogo no vetor +0x88c; sem achar, tira o bit',
          'with the person bit 2, searches the appearance money array +0x88c; when not found, clears the bit')])
CAMPOS = [('TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa: +0 bit 2 (prêmio por jogo), palavra +3 (0x3fff/0x1fff), byte +7 (jogos na seleção no acordo); valor; lê e grava',
           'TRANSFER_MANAGER +0x86e: person 8-byte record: +0 bit 2 (appearance money), word +3 (0x3fff/0x1fff), byte +7 (caps at the deal); value; reads and writes'),
          ('TRANSFER_MANAGER +0x88c/+0x894: prêmios por jogo (13 bytes) e a conta; valor; lê (premio_por_jogo_do_jogador)',
           'TRANSFER_MANAGER +0x88c/+0x894: appearance money records (13 bytes) and the count; value; reads (premio_por_jogo_do_jogador)'),
          ('TRANSFER_MANAGER +0x93a/+0x93e: permissões de trabalho e a conta; valor; ordena (iniciar_permissoes_de_trabalho)',
           'TRANSFER_MANAGER +0x93a/+0x93e: work permits and the count; value; sorts (iniciar_permissoes_de_trabalho)'),
          ('staff +0x22: jogos pela seleção; valor; lê', 'staff +0x22: international caps; value; reads'),
          ('staff +0x1e: segunda nacionalidade; ponteiro; grava (iniciar_permissoes_de_trabalho)', 'staff +0x1e: second nationality; pointer; writes (iniciar_permissoes_de_trabalho)'),
          ('staff +0x39/+0x3d/+0x3e/+0x40: clube, cargo, data e ano de chegada; lê (iniciar_permissoes_de_trabalho)',
           'staff +0x39/+0x3d/+0x3e/+0x40: club, job, arrival date and year; reads (iniciar_permissoes_de_trabalho)'),
          ('contrato +0x3b: jogos pelo clube; valor; lê (contagem_da_clausula)', 'contract +0x3b: club appearances; value; reads (contagem_da_clausula)'),
          ('[0xae2c92] ano de hoje; [0xdd6ec8] data do começo; valor; lê', '[0xae2c92] current year; [0xdd6ec8] start date; value; reads')]
INTERESSA = ['CMRTE', 'SGE']
