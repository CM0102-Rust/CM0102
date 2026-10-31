# As relações do elenco no dia a dia (game.cpp): game_update_squad_relationships 0x5d6d10 e o moral espalhado pela
# liderança 0x5ce0b0, as duas pelo laço do dia do jogo (0x5cda10.., todo dia: chamadas em 0x5cd9f6 e 0x5cdb0a, ao lado
# da 0x5d5ae0, 516 chamadas na carreira até 2002-06-01), e o atrito no vestiário 0x5d7190 (pela 0x5d6d10 nos dias
# múltiplos de 50). Fora de fechar_partida_do_dia e de eventos_do_pos_jogo. Iguais no cm0102-1080.exe. Sem SEH, sem
# x87 e sem local lida antes de gravada. As três sorteiam ou chamam rotinas com efeito (preferências 0x547a10/0x547af0/
# 0x547c10/0x547c50, briga 0x5888a0, queixa 0x587c60, marca 0x867700, moral 0x544710): comparadores só a original,
# a cópia leva os ramos. O atrito (0x5d7190 e o fim do laço da 0x5d6d10) só chega à queixa com dia % 14 + id da pessoa
# = 0 (a pessoa de id 0), como na original.
SAVE = False
ROTINAS = [('relacoes_do_elenco', 0x5d6d10, [0x83, 0xec, 0x08, 0x53, 0x55, 0x56]),
           ('atrito_com_o_tecnico', 0x5d7190, [0x83, 0xec, 0x08, 0x53, 0x55, 0x56]),
           ('moral_pela_lideranca', 0x5ce0b0, [0x83, 0xec, 0x1c, 0xa1, 0x5c, 0x23, 0xae, 0x00])]
DOCS = ('as relações do elenco no dia a dia: as brigas e preferências entre colegas, o atrito com o técnico e o moral espalhado pela liderança',
        'the daily squad relationships: the fights and likes between team-mates, the friction with the manager and the morale spread by leadership',
        [('relacoes_do_elenco', 'game_update_squad_relationships', '0x5d6d10', 'game.cpp', 'cdecl',
          'dez pares sorteados de colegas: quem não gostava deixa de não gostar e pode brigar (0x5888a0); senão passa a não gostar, a gostar ou deixa de gostar; a cada 50 dias, clubes sorteados pelo atrito no vestiário; nos outros dias cada jogador, por sorteios dos atributos, pode chegar ao atrito com o técnico',
          'ten random pairs of team-mates: a disliked one stops being disliked and may fight (0x5888a0); otherwise becomes disliked, liked or stops being liked; every 50 days, random clubs go through the dressing room friction; on the other days each player, by attribute draws, may reach the friction with the manager'),
         ('atrito_com_o_tecnico', 'game_check_club_dressing_room_unrest', '0x5d7190', 'game.cpp', 'cdecl',
          'cada jogador do elenco com vínculo e preferências: o primeiro colega de quem não gosta; com dia % 14 + id = 0, a queixa contra o técnico, o auxiliar ou outro (0x587c60) e a marca no vínculo (0x867700)',
          'each squad player with a contract and preferences: the first team-mate he dislikes; with day % 14 + id = 0, the complaint against the manager, the assistant or another (0x587c60) and the flag on the contract (0x867700)'),
         ('moral_pela_lideranca', 'game_spread_squad_morale_by_leadership', '0x5ce0b0', 'game.cpp', 'cdecl',
          'um clube em sete por dia (não seleção): liderança e liderança × moral do elenco somadas; cada jogador sorteia a partir delas e do próprio moral e o moral sobe ou desce um (0x544710)',
          'one club in seven each day (not national teams): squad leadership and leadership × morale summed; each player draws from them and his own morale and the morale goes up or down by one (0x544710)')])
CAMPOS = [('clube +0xd7: as 50 vagas do elenco; ponteiros; lê (as três)',
           'club +0xd7: the 50 squad slots; pointers; reads (all three)'),
          ('clube +0xcf técnico, +0xd3 auxiliar, +0xff a pessoa do vínculo, +0x80 reputação; ponteiros/valor; lê (relacoes_do_elenco, atrito_com_o_tecnico)',
           'club +0xcf manager, +0xd3 assistant, +0xff the contract person, +0x80 reputation; pointers/value; reads (relacoes_do_elenco, atrito_com_o_tecnico)'),
          ('pessoa +0x18, +0x59..+0x5d atributos mentais; valores; lê (relacoes_do_elenco)',
           'staff +0x18, +0x59..+0x5d mental attributes; values; reads (relacoes_do_elenco)'),
          ('pessoa +0x39 clube do contrato, +0x61 ficha de jogador, +0x65 preferências; ponteiros; lê (as três)',
           'staff +0x39 contract club, +0x61 player data, +0x65 preferences; pointers; reads (all three)'),
          ('jogador (pessoa +0x61) +0x2f liderança, +0xb reputação; valores; lê (moral_pela_lideranca, relacoes_do_elenco)',
           'player (staff +0x61) +0x2f leadership, +0xb reputation; values; reads (moral_pela_lideranca, relacoes_do_elenco)'),
          ('vínculo (0x4cf5f0) +0x44 moral, +0x45 marcas; valores; lê (as três)',
           'contract (0x4cf5f0) +0x44 morale, +0x45 flags; values; reads (all three)'),
          ('[0xae1080] o último contrato achado (0x4cf5a0); ponteiro; grava (relacoes_do_elenco)',
           '[0xae1080] the last contract found (0x4cf5a0); pointer; writes (relacoes_do_elenco)')]
INTERESSA = ['CMRTE']
