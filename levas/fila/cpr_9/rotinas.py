# A busca de novas aposentadorias (PLAYER_REGEN::find_new_retirements 0x7a5790, nos dias da 0x5d6250 pela 0x5cccd0),
# a saída dos cargos de quem se aposenta sem ser jogador (player_regen_retire_from_non_playing_staff 0x7aaeb0, pela
# 0x7a59e0 na data da aposentadoria) e o registro da liberação (Staff_Records::add_free_release_records 0x86d800, pelas
# transferências 0x8c2180). A 0x7a5790 e a 0x86d800 têm SEH só da data local (destrutor vazio 0x5a53c0). A 0x7a5790
# chama a 0x7a7310 (no item cpr_8, pelo endereço), a 0x7a8050, a 0x7a8510 e o qsort; a 0x7aaeb0 chama 0x69b790,
# 0x544820, 0x600a50 (Rust) e 0x69ba50, 0x8c4950, 0x8b2880, 0x7f7270 (originais, de outras famílias); a 0x86d800 usa
# os ajudantes do histórico da 253ª (cpq4_*). Trocas: das candidatas da família, as de notícia (0x4d6d70, 0x4d9f80,
# 0x7aa340, 0x7a92e0, 0x62dca0...) ficam de fora pela lição da notícia montada na pilha; a 0x7c1e30 (get_stat) lê dois
# floats sem valor quando falta o registro da categoria (lixo de pilha): fora; a 0x7c0d70 e a 0x868830 só são
# chamadas pela fechar_partida_do_dia (SO_NA_COMPARACAO): fora. Iguais no cm0102-1080.exe. Simulação: as mutações
# pela carreira até 2002-06-01. Comparadores: a busca e o registro só a original (acrescentam; o histórico vai para o
# arquivo); a saída dos cargos lado a lado pela pessoa e pelos dois clubes quando não chama rotina com efeito.
SAVE = False
ROTINAS = [('novas_aposentadorias', 0x7a5790, [0x6a, 0xff, 0x68, 0x58, 0xf1, 0x95, 0x00]),
           ('sair_dos_cargos', 0x7aaeb0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('registrar_a_liberacao', 0x86d800, [0x6a, 0xff, 0x68, 0x7b, 0x2d, 0x96, 0x00])]
DOCS = ('a busca de novas aposentadorias, a saída dos cargos de quem se aposenta e o registro da liberação',
        'the search for new retirements, leaving the jobs on retirement and the free release record',
        [('novas_aposentadorias', 'PLAYER_REGEN::find_new_retirements', '0x7a5790', 'player_regen.cpp', 'thiscall',
          'testa cada pessoa elegível (fora os jovens de competição do jogo, os marcados, os já na lista e os de contrato marcado) pela idade (0x7a7310) ou pelo desemprego (0x7a8050), acrescenta quem se aposenta e ordena a lista',
          'tests every eligible person (except the young ones in game competitions, the flagged, those already listed and those with a flagged contract) by age (0x7a7310) or unemployment (0x7a8050), adds those who retire and sorts the list'),
         ('sair_dos_cargos', 'player_regen_retire_from_non_playing_staff', '0x7aaeb0', 'player_regen.cpp', 'stdcall, ret 4',
          'tira a pessoa das vagas do cargo no clube e na seleção (presidente, diretores, técnico com a vaga aberta, auxiliar, treinadores, olheiros, fisioterapeutas) e zera o clube, a seleção e os tipos de cargo',
          'removes the person from the job slots at the club and national team (chairman, directors, manager with the vacancy opened, assistant, coaches, scouts, physios) and clears the club, the national team and the job types'),
         ('registrar_a_liberacao', 'Staff_Records::add_free_release_records', '0x86d800', 'staff_records.cpp', 'thiscall, ret 4',
          'os históricos da liberação sem custo: o do técnico do clube (tipo 0x68) e o da pessoa (tipo 0x67)',
          'the free release histories: the club manager one (type 0x68) and the person one (type 0x67)')])
CAMPOS = [('pessoa +0x18: idade; valor; lê (novas_aposentadorias)',
           'person +0x18: age; value; reads (novas_aposentadorias)'),
          ('pessoa +0x39 e +0x24: clube e seleção; ponteiro; lê e grava 0 (sair_dos_cargos); lê (novas_aposentadorias)',
           'person +0x39 and +0x24: club and national team; pointer; reads and writes 0 (sair_dos_cargos); reads (novas_aposentadorias)'),
          ('pessoa +0x3d e +0x28: tipos do cargo no clube e na seleção; valor; lê e grava 0 (sair_dos_cargos)',
           'person +0x3d and +0x28: club and national job types; value; reads and writes 0 (sair_dos_cargos)'),
          ('clube +0xbf, +0xc3 (3), +0xcf, +0xd3, +0x19f (5), +0x1b3 (7), +0x1cf (3): presidente, diretores, técnico, auxiliar, treinadores, olheiros, fisioterapeutas; ponteiro; lê e grava 0 (sair_dos_cargos)',
           'club +0xbf, +0xc3 (3), +0xcf, +0xd3, +0x19f (5), +0x1b3 (7), +0x1cf (3): chairman, directors, manager, assistant, coaches, scouts, physios; pointer; reads and writes 0 (sair_dos_cargos)'),
          ('clube +0x57 e [[0xadadfc] + id × 4]: competição e se ela roda no jogo; ponteiro e valor; lê (novas_aposentadorias)',
           'club +0x57 and [[0xadadfc] + id × 4]: competition and whether it runs in the game; pointer and value; reads (novas_aposentadorias)'),
          ('contrato +0x49: marca que exclui da busca; valor; lê (novas_aposentadorias)',
           'contract +0x49: flag that excludes from the search; value; reads (novas_aposentadorias)'),
          ('[0xae2d0c] registro de 79 bytes por pessoa, +0xb bits 0x30; valor; lê (novas_aposentadorias)',
           '[0xae2d0c] 79-byte record per person, +0xb bits 0x30; value; reads (novas_aposentadorias)'),
          ('PLAYER_REGEN +4 lista de aposentadorias (26 bytes, id em +0), +8 conta; ponteiro e valor; lê, acrescenta e ordena (novas_aposentadorias)',
           'PLAYER_REGEN +4 retirement list (26 bytes, id at +0), +8 count; pointer and value; reads, appends and sorts (novas_aposentadorias)'),
          ('[0xae1080] o contrato atual; ponteiro; grava (registrar_a_liberacao)',
           '[0xae1080] the current contract; pointer; writes (registrar_a_liberacao)'),
          ('Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (registrar_a_liberacao)',
           'Staff_Records +0 and +0xc: staff history file and lists; appends (registrar_a_liberacao)')]
INTERESSA = ['GPF2', 'CMRTE']
