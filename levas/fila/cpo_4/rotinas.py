# As saídas dos clubes (manager_manager.cpp): as saídas pendentes dos técnicos (MANAGER_MANAGER::process_pending_departures
# 0x69b650, pela 0x68f130: a demissão 0x69b790 e a vaga 0x69ba50 de cada clube da lista), tirar o jogador do elenco
# (MANAGER_MANAGER::remove_player_from_squad 0x690730: 18 chamadas, as transferências e as saídas do clube) e tirar a
# pessoa do cargo no clube (MANAGER_MANAGER::remove_staff_from_club 0x690890: 27 chamadas, demissões, contratações,
# aposentadorias; com as duas tabelas de salto 0x690af8 e 0x690b34). Sem SEH, sem sorteio, sem leitura de lixo; a
# 0x690890 lê o byte baixo do próprio argumento da pessoa (`[esp+0x10]`) nas funções sem vaga, e a 0x69b650 empurra os
# dois bytes do item nos registradores inteiros (os bits de cima do id e de 145 × id): a cópia faz o mesmo. Iguais no
# cm0102-1080.exe. Comparadores: só a original (demitem, contratam, mexem na mídia, nas táticas e nas listas); a 0x690730
# e a 0x690890 conferem o plano (a função e o clube da pessoa, ela fora das 50 vagas; os campos e as vagas do clube
# pelo corpo da cópia sobre cópias dos registros).
SAVE = False
ROTINAS = [('saidas_pendentes_dos_tecnicos', 0x69b650, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('tirar_jogador_do_elenco_do_clube', 0x690730, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('tirar_pessoa_do_cargo_no_clube', 0x690890, [0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00])]
DOCS = ('as saídas dos clubes: as saídas pendentes dos técnicos, tirar o jogador do elenco e tirar a pessoa do cargo',
        'leaving clubs: the managers\' pending departures, removing the player from the squad and removing the person from the job',
        [('saidas_pendentes_dos_tecnicos', 'MANAGER_MANAGER::process_pending_departures', '0x69b650', 'manager_manager.cpp', 'thiscall, ret 0',
          'percorre a lista das saídas pendentes e, em cada clube com técnico, faz a saída dele e, se o clube abre vaga, a vaga; tira cada item da lista',
          'walks the pending departure list and, at each club with a manager, carries out his departure and, when the club opens a vacancy, the vacancy; removes each item from the list'),
         ('tirar_jogador_do_elenco_do_clube', 'MANAGER_MANAGER::remove_player_from_squad', '0x690730', 'manager_manager.cpp', 'thiscall, ret 8',
          'tira o jogador do clube (ou da seleção): a função de jogador-técnico volta à de técnico (senão 0xb e ele sai), das listas, da mídia, das táticas e das 50 vagas do elenco; o empréstimo fica com o estado 4',
          'takes the player out of the club (or the national team): a player-manager job goes back to the staff job (else 0xb and he leaves), out of the lists, the media, the tactics and the 50 squad slots; the loan gets state 4'),
         ('tirar_pessoa_do_cargo_no_clube', 'MANAGER_MANAGER::remove_staff_from_club', '0x690890', 'manager_manager.cpp', 'thiscall, ret 8',
          'tira o contrato e a pessoa da vaga do cargo no clube ou na seleção (presidente, diretor, técnico, auxiliar, treinador, olheiro, fisioterapeuta; as funções de jogador viram 0xb), da mídia e, se humana, das listas do humano',
          'removes the contract and the person from the job slot at the club or national team (chairman, director, manager, assistant, coach, scout, physio; the playing jobs become 0xb), from the media and, if human, from the human\'s lists')])
CAMPOS = [('MANAGER_MANAGER +4: a lista das saídas pendentes (6 bytes: dois bytes e o id do clube em +2); ponteiro; lê e grava (saidas_pendentes_dos_tecnicos)',
           'MANAGER_MANAGER +4: the pending departure list (6 bytes: two bytes and the club id at +2); pointer; reads and writes (saidas_pendentes_dos_tecnicos)'),
          ('pessoa +0x39, +0x3d: clube e função no clube; ponteiro e valor; lê e grava (tirar_jogador_do_elenco_do_clube, tirar_pessoa_do_cargo_no_clube)',
           'person +0x39, +0x3d: club and club job; pointer and value; reads and writes (tirar_jogador_do_elenco_do_clube, tirar_pessoa_do_cargo_no_clube)'),
          ('pessoa +0x24, +0x28: seleção e função na seleção; ponteiro e valor; lê e grava (tirar_jogador_do_elenco_do_clube, tirar_pessoa_do_cargo_no_clube)',
           'person +0x24, +0x28: national team and its job; pointer and value; reads and writes (tirar_jogador_do_elenco_do_clube, tirar_pessoa_do_cargo_no_clube)'),
          ('clube +0xbf, +0xc3 (3), +0xcf, +0xd3: presidente, diretores, técnico, auxiliar; ponteiro; lê e grava 0 (tirar_pessoa_do_cargo_no_clube)',
           'club +0xbf, +0xc3 (3), +0xcf, +0xd3: chairman, directors, manager, assistant; pointer; reads and writes 0 (tirar_pessoa_do_cargo_no_clube)'),
          ('clube +0x19f (5), +0x1b3 (7), +0x1cf (3): treinadores, olheiros, fisioterapeutas; ponteiro; lê e grava 0 (tirar_pessoa_do_cargo_no_clube)',
           'club +0x19f (5), +0x1b3 (7), +0x1cf (3): coaches, scouts, physios; pointer; reads and writes 0 (tirar_pessoa_do_cargo_no_clube)'),
          ('clube +0xd7 (50): as vagas do elenco; ponteiro; lê e grava 0 (tirar_jogador_do_elenco_do_clube)',
           'club +0xd7 (50): the squad slots; pointer; reads and writes 0 (tirar_jogador_do_elenco_do_clube)'),
          ('contrato de empréstimo (0x4cf5a0) +0x4e: estado; valor; grava 4 (tirar_jogador_do_elenco_do_clube)',
           'loan contract (0x4cf5a0) +0x4e: state; value; writes 4 (tirar_jogador_do_elenco_do_clube)')]
INTERESSA = ['CMRTE', 'SGE']
