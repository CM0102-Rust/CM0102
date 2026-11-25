# Destrutores escalares de copas e competicoes (leitor rx14, rodada 6, 25/11/2026). Item de TELAS: rodam quando o
# jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira
# aberta que tenha essas competicoes carregadas (cada copa nacional pede a liga da sua nacao), carregar outro save,
# jogando comparando.
# Os destrutores escalares (push esi; mov esi, ecx; call ~X; test byte [esp+8], 1; operator delete 0x944c94;
# mov eax, esi; ret 4): o prologo tem 3 bytes antes de um call rel32 e entra pelas RELATIVAS (endereco, 3) do
# rust/src/detours.rs, ver INTEGRAR_ANTES.txt. O destrutor da classe e chamado pelo desvio (delete_through, do
# cx03_3: integrar depois dele). Convencoes conferidas: thiscall(flags) ret 4 -> this; o destrutor da classe
# thiscall, ret 0; operator delete cdecl. Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('delete_cro_cup', 0x517650, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_den_cup', 0x550b80, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_auto_cup', 0x56aca0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_cc_cup', 0x56c480, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_charity', 0x56d550, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_fa_cup', 0x56f230, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_fa_trophy', 0x570dd0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_fin_cup', 0x592420, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_fra_cup', 0x5ba300, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_fra_lge_cup', 0x5bcb50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_fra_super', 0x5bf4e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ger_cup', 0x5d8df0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('destrutores escalares de copas e competições',
        'cup and competition scalar deleting destructors',
        [('delete_cro_cup', 'CroCup::scalar_deleting_destructor', '0x517650', 'cro_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Croácia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Croatian Cup and, with bit 0, deletes the object'),
         ('delete_den_cup', 'DenCup::scalar_deleting_destructor', '0x550b80', 'den_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Dinamarca e, com o bit 0, apaga o objeto',
          'runs the destructor of the Danish Cup and, with bit 0, deletes the object'),
         ('delete_eng_auto_cup', 'EngAutoCup::scalar_deleting_destructor', '0x56aca0', 'eng_auto_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor do Troféu da Football League (Auto Windscreens Shield) e, com o bit 0, apaga o objeto',
          'runs the destructor of the Football League Trophy and, with bit 0, deletes the object'),
         ('delete_eng_cc_cup', 'EngCcCup::scalar_deleting_destructor', '0x56c480', 'eng_cc_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Liga inglesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the English League Cup and, with bit 0, deletes the object'),
         ('delete_eng_charity', 'EngCharity::scalar_deleting_destructor', '0x56d550', 'eng_charity.cpp', 'thiscall, ret 4',
          'roda o destrutor da Charity Shield e, com o bit 0, apaga o objeto',
          'runs the destructor of the Charity Shield and, with bit 0, deletes the object'),
         ('delete_eng_fa_cup', 'EngFaCup::scalar_deleting_destructor', '0x56f230', 'eng_fa_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Inglaterra (FA Cup) e, com o bit 0, apaga o objeto',
          'runs the destructor of the FA Cup and, with bit 0, deletes the object'),
         ('delete_eng_fa_trophy', 'EngFaTrophy::scalar_deleting_destructor', '0x570dd0', 'eng_fa_trophy.cpp', 'thiscall, ret 4',
          'roda o destrutor do FA Trophy e, com o bit 0, apaga o objeto',
          'runs the destructor of the FA Trophy and, with bit 0, deletes the object'),
         ('delete_fin_cup', 'FinCup::scalar_deleting_destructor', '0x592420', 'fin_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Finlândia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Finnish Cup and, with bit 0, deletes the object'),
         ('delete_fra_cup', 'FraCup::scalar_deleting_destructor', '0x5ba300', 'fra_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da França e, com o bit 0, apaga o objeto',
          'runs the destructor of the French Cup and, with bit 0, deletes the object'),
         ('delete_fra_lge_cup', 'FraLgeCup::scalar_deleting_destructor', '0x5bcb50', 'fra_lge_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Liga francesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the French League Cup and, with bit 0, deletes the object'),
         ('delete_fra_super', 'FraSuper::scalar_deleting_destructor', '0x5bf4e0', 'fra_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da França e, com o bit 0, apaga o objeto',
          'runs the destructor of the French Super Cup and, with bit 0, deletes the object'),
         ('delete_ger_cup', 'GerCup::scalar_deleting_destructor', '0x5d8df0', 'ger_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Alemanha e, com o bit 0, apaga o objeto',
          'runs the destructor of the German Cup and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
