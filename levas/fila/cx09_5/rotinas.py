# Oito destrutores escalares de ligas (leitor cx09, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx02_3 e cx03_3:
# rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo:
# com uma carreira aberta que tenha as ligas dessas nacoes carregadas (Inglaterra; Belgica, Brasil, Croacia,
# Dinamarca e Finlandia), carregar outro save, jogando comparando.
# Destrutores escalares (push esi; mov esi, ecx; call ~X; test byte [esp+8], 1; operator delete 0x944c94;
# mov eax, esi; ret 4): prologo com 3 bytes antes de um call rel32, entra pelas RELATIVAS (endereco, 3), ver
# INTEGRAR_ANTES.txt; o ~X e chamado pelo desvio (delete_through do cx03_3).
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), Comp_Stats::~Comp_Stats 0x49f450 e
# League::~League 0x682300 (thiscall, ret 0), League::free_table 0x687970 (thiscall, ret 4). Iguais no
# cm0102-1080.exe (todas inteiras). Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('delete_bra_reg_north', 0x435560, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_northeast', 0x436610, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_parana', 0x437620, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_pern', 0x438b10, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_rio', 0x439c60, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_santa', 0x43b350, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_sp', 0x43c4c0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_den_second', 0x554090, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('oito destrutores escalares de ligas',
        'eight league scalar deleting destructors',
        [('delete_bra_reg_north', 'BraRegNorth::scalar_deleting_destructor', '0x435560', 'bra_reg_north.cpp', 'thiscall, ret 4',
          'roda o destrutor do estadual do Norte e, com o bit 0, apaga o objeto',
          'runs the destructor of the North regional championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_northeast', 'BraRegNortheast::scalar_deleting_destructor', '0x436610', 'bra_reg_northeast.cpp', 'thiscall, ret 4',
          'roda o destrutor do estadual do Nordeste e, com o bit 0, apaga o objeto',
          'runs the destructor of the Northeast regional championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_parana', 'BraRegParana::scalar_deleting_destructor', '0x437620', 'bra_reg_parana.cpp', 'thiscall, ret 4',
          'roda o destrutor do Campeonato Paranaense e, com o bit 0, apaga o objeto',
          'runs the destructor of the Parana state championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_pern', 'BraRegPern::scalar_deleting_destructor', '0x438b10', 'bra_reg_pern.cpp', 'thiscall, ret 4',
          'roda o destrutor do Campeonato Pernambucano e, com o bit 0, apaga o objeto',
          'runs the destructor of the Pernambuco state championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_rio', 'BraRegRio::scalar_deleting_destructor', '0x439c60', 'bra_reg_rio.cpp', 'thiscall, ret 4',
          'roda o destrutor do Campeonato Carioca e, com o bit 0, apaga o objeto',
          'runs the destructor of the Rio de Janeiro state championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_santa', 'BraRegSanta::scalar_deleting_destructor', '0x43b350', 'bra_reg_santa.cpp', 'thiscall, ret 4',
          'roda o destrutor do Campeonato Catarinense e, com o bit 0, apaga o objeto',
          'runs the destructor of the Santa Catarina state championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_sp', 'BraRegSp::scalar_deleting_destructor', '0x43c4c0', 'bra_reg_sp.cpp', 'thiscall, ret 4',
          'roda o destrutor do Campeonato Paulista e, com o bit 0, apaga o objeto',
          'runs the destructor of the Sao Paulo state championship and, with bit 0, deletes the object'),
         ('delete_den_second', 'DenSecond::scalar_deleting_destructor', '0x554090', 'den_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Segunda Divisão da Dinamarca e, com o bit 0, apaga o objeto',
          'runs the destructor of the Danish Second Division and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
