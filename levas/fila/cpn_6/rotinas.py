# A lista de observação do clube (shortlist_manager.cpp): o estatuto no elenco que o clube dá à pessoa
# (Shortlist_Manager::get_squad_status_for_club 0x849f00, 6215 chamadas na carreira até 2002-06-01, pelos contratos e
# transferências), a estimativa do clube humano (shortlist_manager_estimate_squad_status 0x849d60, portada junto; só
# para o clube humano: não roda na carreira automática) e soltar os alvos das 40 vagas que não são do clube
# (shortlist_manager_release_club_slot_targets 0x844660, 14905 chamadas). Sem SEH, sem sorteio, sem caixa. A 0x849d60
# compara a razão das reputações num float (fst dword), como a original. A 0x844660 passa ao 0x843e10 o tipo no byte
# baixo do endereço da vaga (setne al), como a original. Iguais no cm0102-1080.exe.
# Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Comparadores: lado a lado pelo estatuto,
# pelo cargo de saída e por [0xae1080]; a estimativa pelo valor; soltar os alvos só a original (a 0x843e10 muda a lista).
SAVE = False
ROTINAS = [('estatuto_no_elenco_pelo_clube', 0x849f00, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('estimar_estatuto_no_elenco', 0x849d60, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('liberar_alvos_da_lista_do_clube', 0x844660, [0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x14])]
DOCS = ('a lista de observação do clube: o estatuto no elenco que o clube dá à pessoa, a estimativa do clube humano e soltar os alvos que não são do clube',
        'the club shortlist: the squad status the club gives the person, the human club estimate and releasing the targets that are not the club\'s',
        [('estatuto_no_elenco_pelo_clube', 'Shortlist_Manager::get_squad_status_for_club', '0x849f00', 'shortlist_manager.cpp', 'thiscall, ret 0xc',
          'o estatuto no elenco que o clube dá à pessoa: pela entrada dela na lista do clube (bits 1..4 do +0x24, cargo do +0), pelo contrato com o clube (nibble alto do +0x4f), pela estimativa do clube humano ou 7',
          'the squad status the club gives the person: from her entry in the club shortlist (bits 1..4 of +0x24, job from +0), from the contract with the club (high nibble of +0x4f), from the human club estimate, or 7'),
         ('estimar_estatuto_no_elenco', 'shortlist_manager_estimate_squad_status', '0x849d60', 'shortlist_manager.cpp', 'stdcall, ret 8',
          'a estimativa do estatuto que o clube humano dá: a razão entre a reputação da pessoa vista pelo técnico (1,5 × na lista do humano) e a do clube, e a idade',
          'the estimate of the status the human club gives: the ratio between the person\'s reputation seen by the manager (1.5 × when on the human shortlist) and the club\'s, and the age'),
         ('liberar_alvos_da_lista_do_clube', 'shortlist_manager_release_club_slot_targets', '0x844660', 'shortlist_manager.cpp', 'thiscall, ret 8',
          'nas 40 vagas do clube, o alvo de id inválido vira −1 e o alvo que não é do clube (contrato nem +0x39) e sem +0x17 recebe o evento 3 da lista (0x843e10)',
          'in the club\'s 40 slots, a target with an invalid id becomes −1 and a target not at the club (contract nor +0x39) and without +0x17 gets shortlist event 3 (0x843e10)')])
CAMPOS = [('entrada da lista (40 × 0x25 bytes) +0: o cargo (tipo, 11 = não jogador); valor; lê (estatuto_no_elenco_pelo_clube, liberar_alvos_da_lista_do_clube)',
           'shortlist slot (40 × 0x25 bytes) +0: the job (type, 11 = non-player); value; reads (estatuto_no_elenco_pelo_clube, liberar_alvos_da_lista_do_clube)'),
          ('entrada da lista +0xf: o alvo; id de pessoa; lê e grava −1 se acima de [0xae2364] − 1 (liberar_alvos_da_lista_do_clube)',
           'shortlist slot +0xf: the target; person id; reads and writes −1 when above [0xae2364] − 1 (liberar_alvos_da_lista_do_clube)'),
          ('entrada da lista +0x17: o segundo alvo (−1 vazio); id; lê (liberar_alvos_da_lista_do_clube)',
           'shortlist slot +0x17: the second target (−1 empty); id; reads (liberar_alvos_da_lista_do_clube)'),
          ('entrada da lista +0x24: bits 1..4 o estatuto no elenco; valor; lê (estatuto_no_elenco_pelo_clube)',
           'shortlist slot +0x24: bits 1..4 the squad status; value; reads (estatuto_no_elenco_pelo_clube)'),
          ('staff +0x18: a idade; valor; lê (estimar_estatuto_no_elenco)', 'staff +0x18: the age; value; reads (estimar_estatuto_no_elenco)'),
          ('staff +0x39: o clube; ponteiro; lê (estatuto_no_elenco_pelo_clube, liberar_alvos_da_lista_do_clube)',
           'staff +0x39: the club; pointer; reads (estatuto_no_elenco_pelo_clube, liberar_alvos_da_lista_do_clube)'),
          ('staff +0x3d: o cargo; valor; lê (estatuto_no_elenco_pelo_clube)', 'staff +0x3d: the job; value; reads (estatuto_no_elenco_pelo_clube)'),
          ('staff +0x61/+0x69: os dados de jogador e de não jogador; ponteiro; lê (estatuto_no_elenco_pelo_clube)',
           'staff +0x61/+0x69: the player and non-player data; pointer; reads (estatuto_no_elenco_pelo_clube)'),
          ('contrato +4: o clube; id; lê; +0x35 (6 bits) o tipo (5 não conta); valor; lê (liberar_alvos_da_lista_do_clube); +0x4f nibble alto o estatuto no elenco; valor; lê (estatuto_no_elenco_pelo_clube)',
           'contract +4: the club; id; reads; +0x35 (6 bits) the type (5 does not count); value; reads (liberar_alvos_da_lista_do_clube); +0x4f high nibble the squad status; value; reads (estatuto_no_elenco_pelo_clube)'),
          ('clube +0x80: a reputação; valor; lê (estimar_estatuto_no_elenco)', 'club +0x80: the reputation; value; reads (estimar_estatuto_no_elenco)'),
          ('[0xae1080] o último contrato buscado; ponteiro; grava (estatuto_no_elenco_pelo_clube, liberar_alvos_da_lista_do_clube)',
           '[0xae1080] the last contract looked up; pointer; writes (estatuto_no_elenco_pelo_clube, liberar_alvos_da_lista_do_clube)')]
INTERESSA = ['CMRTE', 'CMScout']
