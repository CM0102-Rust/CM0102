# Quem serve para o cargo e a moral depois da partida ruim (manager_manager.cpp, contract_manager.cpp): o candidato a
# técnico (MANAGER_MANAGER::is_staff_suitable_for_job 0x69f810, pelas vagas 0x69bdf7/0x69bf77 e pela melhor nota de
# candidato 0x69c540 da 250a leva: os vetos e os sim pela reputação, nação, contrato e tempo no cargo, senão o
# interesse 0x844780 contra a nota mínima), o cargo que a pessoa pode assumir (staff_can_take_job 0x4cb990, pela
# tabela de salto 0x4cbaf4; na carreira só pelo regen de não jogador 0x7ab9f2, que pergunta com a pessoa nula; as
# telas de contrato com a pessoa) e baixar a moral pela partida (contract_manager_lower_match_morale 0x4cb250, pela
# CONTRACT_MANAGER::update_after_match 0x4cac5f/0x4cb05b e pela Shortlist_Manager::assess_club_squad: a irmã da
# 0x4cb0a0 da 244a leva). Sem SEH, sem leitura de lixo (a 0x4cb250, sem nação no clube do contrato, lê o byte baixo do
# ponteiro do clube na vaga do argumento como índice de nação: valor dado, a cópia faz o mesmo). Iguais no
# cm0102-1080.exe. x87 em dupla precisão (as constantes do .rdata; a 0x4cb250 limita em 15000 pelo float 0x968da8 e a
# condição dos dias do contrato acima de 400 nunca vale). Comparadores: as duas primeiras pela devolução (só leem); a
# 0x4cb250 só a original (as queixas 0x867700 sorteiam e abrem pedido de transferência) com o plano: a queda da moral
# calculada antes dá a moral +0x44 que ela deixa (as queixas não a gravam).
SAVE = False
ROTINAS = [('serve_para_o_cargo', 0x69f810, [0x55, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('pode_assumir_o_cargo', 0x4cb990, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('baixar_moral_pela_partida', 0x4cb250, [0x83, 0xec, 0x08, 0x56, 0x57])]
DOCS = ('quem serve para o cargo e a moral depois da partida ruim: o candidato a técnico, o cargo que a pessoa pode assumir e baixar a moral pela partida',
        'who fits the job and morale after a poor match: the manager candidate, the job a person can take and lowering morale after the match',
        [('serve_para_o_cargo', 'MANAGER_MANAGER::is_staff_suitable_for_job', '0x69f810', 'manager_manager.cpp', 'thiscall, ret 0x10',
          'se a pessoa é candidata realista ao cargo de técnico do clube: vetos e aceites pela reputação, nação, contrato e tempo no cargo; senão o interesse dela no cargo contra a nota mínima (double)',
          'whether the person is a realistic candidate for the club\'s manager job: vetoes and accepts by reputation, nation, contract and time in the job; otherwise her interest in the job against the minimum score (double)'),
         ('pode_assumir_o_cargo', 'staff_can_take_job', '0x4cb990', 'contract_manager.cpp', 'stdcall, ret 8',
          'se a pessoa serve para o cargo (6 a 15) pela idade, pelo cargo atual e pelos registros de jogador e de não jogador; pessoa nula: 0',
          'whether the person fits the job (6 to 15) by age, current job and the player and non-player records; null person: 0'),
         ('baixar_moral_pela_partida', 'contract_manager_lower_match_morale', '0x4cb250', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'baixa a moral do contrato do jogador depois da partida ruim (pela nota, reputação, tempo no clube, idade e faixa do contrato) e, com a moral muito baixa, pode dar a queixa de querer sair ou de falta de jogos; machucado, emprestado ou aposentando apaga as queixas',
          'lowers the player\'s contract morale after a poor match (by rating, reputation, time at the club, age and contract band) and, with very low morale, may add the wants-to-leave or lack-of-games grievance; injured, on loan or retiring clears the grievances')])
CAMPOS = [('pessoa +0x18 (idade), +0x1a (nação), +0x22, +0x24/+0x28 (seleção e cargo nela), +0x39 (clube), +0x3d (cargo), +0x3e/+0x40 (data de entrada), +0x57, +0x59, +0x61 (jogador), +0x65, +0x69 (não jogador); valores e ponteiros; lê (as três)',
           'person +0x18 (age), +0x1a (nation), +0x22, +0x24/+0x28 (national team and job there), +0x39 (club), +0x3d (job), +0x3e/+0x40 (join date), +0x57, +0x59, +0x61 (player), +0x65, +0x69 (non-player); values and pointers; reads (all three)'),
          ('contrato +0x2d/+0x2f (data), +0x3d, +0x3e, +0x3f, +0x44 (moral), +0x45 (queixas), +0x49, +0x4f (faixa); valores; lê e grava +0x44 (baixar_moral_pela_partida), lê (serve_para_o_cargo)',
           'contract +0x2d/+0x2f (date), +0x3d, +0x3e, +0x3f, +0x44 (morale), +0x45 (grievances), +0x49, +0x4f (band); values; reads and writes +0x44 (baixar_moral_pela_partida), reads (serve_para_o_cargo)'),
          ('clube +0x53 (nação, +0x85 o peso), +0x57 (liga, +0x69 o nível), +0x80 (reputação), +0xcf/+0xd3; lê (serve_para_o_cargo, baixar_moral_pela_partida)',
           'club +0x53 (nation, +0x85 the weight), +0x57 (league, +0x69 the level), +0x80 (reputation), +0xcf/+0xd3; reads (serve_para_o_cargo, baixar_moral_pela_partida)'),
          ('[0xb63d8a] tabela das janelas de transferência (72 bytes por nação-chave, as datas em +0 e +0xf); lê (baixar_moral_pela_partida)',
           '[0xb63d8a] transfer window table (72 bytes per key nation, the dates at +0 and +0xf); reads (baixar_moral_pela_partida)'),
          ('[0xae1080] a ficha global do contrato; ponteiro; grava (baixar_moral_pela_partida)',
           '[0xae1080] the global contract slot; pointer; writes (baixar_moral_pela_partida)')]
INTERESSA = ['CMRTE', 'CMScout']
