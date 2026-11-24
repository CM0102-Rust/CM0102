# Doze destrutores escalares de ligas (leitor cx09, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx02_3 e cx03_3:
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
ROTINAS = [('delete_bra_nat_third', 0x42d520, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_cro_a1', 0x514030, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_cro_a2a', 0x515530, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_cro_a2b', 0x516650, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_den_first', 0x551900, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_den_prm', 0x552850, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_prm', 0x5735b0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_bahia', 0x42f4f0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_central', 0x4306d0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_gaucho', 0x4319e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_goias', 0x432d70, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_reg_minas_gerais', 0x433f70, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('doze destrutores escalares de ligas',
        'twelve league scalar deleting destructors',
        [('delete_bra_nat_third', 'BraNatThird::scalar_deleting_destructor', '0x42d520', 'bra_nat_third.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série C do Brasileiro e, com o bit 0, apaga o objeto',
          'runs the destructor of the Brazilian Serie C and, with bit 0, deletes the object'),
         ('delete_cro_a1', 'CroA1::scalar_deleting_destructor', '0x514030', 'cro_a1.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão da Croácia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Croatian First Division and, with bit 0, deletes the object'),
         ('delete_cro_a2a', 'CroA2a::scalar_deleting_destructor', '0x515530', 'cro_a2a.cpp', 'thiscall, ret 4',
          'roda o destrutor do grupo A da Segunda Divisão da Croácia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Croatian Second Division group A and, with bit 0, deletes the object'),
         ('delete_cro_a2b', 'CroA2b::scalar_deleting_destructor', '0x516650', 'cro_a2b.cpp', 'thiscall, ret 4',
          'roda o destrutor do grupo B da Segunda Divisão da Croácia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Croatian Second Division group B and, with bit 0, deletes the object'),
         ('delete_den_first', 'DenFirst::scalar_deleting_destructor', '0x551900', 'den_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão da Dinamarca e, com o bit 0, apaga o objeto',
          'runs the destructor of the Danish First Division and, with bit 0, deletes the object'),
         ('delete_den_prm', 'DenPrm::scalar_deleting_destructor', '0x552850', 'den_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Superliga da Dinamarca e, com o bit 0, apaga o objeto',
          'runs the destructor of the Danish Superliga and, with bit 0, deletes the object'),
         ('delete_eng_prm', 'EngPrm::scalar_deleting_destructor', '0x5735b0', 'eng_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Premier League e, com o bit 0, apaga o objeto',
          'runs the destructor of the English Premier League and, with bit 0, deletes the object'),
         ('delete_bra_reg_bahia', 'BraRegBahia::scalar_deleting_destructor', '0x42f4f0', 'bra_reg_bahia.cpp', 'thiscall, ret 4',
          'roda o destrutor do Campeonato Baiano e, com o bit 0, apaga o objeto',
          'runs the destructor of the Bahia state championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_central', 'BraRegCentral::scalar_deleting_destructor', '0x4306d0', 'bra_reg_central.cpp', 'thiscall, ret 4',
          'roda o destrutor do estadual do Centro-Oeste e, com o bit 0, apaga o objeto',
          'runs the destructor of the Central regional championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_gaucho', 'BraRegGaucho::scalar_deleting_destructor', '0x4319e0', 'bra_reg_gaucho.cpp', 'thiscall, ret 4',
          'roda o destrutor do Campeonato Gaúcho e, com o bit 0, apaga o objeto',
          'runs the destructor of the Rio Grande do Sul state championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_goias', 'BraRegGoias::scalar_deleting_destructor', '0x432d70', 'bra_reg_goias.cpp', 'thiscall, ret 4',
          'roda o destrutor do Campeonato Goiano e, com o bit 0, apaga o objeto',
          'runs the destructor of the Goias state championship and, with bit 0, deletes the object'),
         ('delete_bra_reg_minas_gerais', 'BraRegMinasGerais::scalar_deleting_destructor', '0x433f70', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 4',
          'roda o destrutor do Campeonato Mineiro e, com o bit 0, apaga o objeto',
          'runs the destructor of the Minas Gerais state championship and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
