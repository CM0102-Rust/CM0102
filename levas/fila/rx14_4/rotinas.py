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
ROTINAS = [('delete_ita_c_cup', 0x63ce90, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ita_cup', 0x63eb50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ita_super', 0x65f830, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_jap_emp_cup', 0x661f00, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_jap_j_cup', 0x665550, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_jap_super', 0x665f30, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_kor_fa_cup', 0x66a630, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_kor_league_cup', 0x66c140, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('destrutores escalares de copas e competições',
        'cup and competition scalar deleting destructors',
        [('delete_ita_c_cup', 'ItaCCup::scalar_deleting_destructor', '0x63ce90', 'ita_c_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Itália da Série C e, com o bit 0, apaga o objeto',
          'runs the destructor of the Serie C Italian Cup and, with bit 0, deletes the object'),
         ('delete_ita_cup', 'ItaCup::scalar_deleting_destructor', '0x63eb50', 'ita_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Itália e, com o bit 0, apaga o objeto',
          'runs the destructor of the Italian Cup and, with bit 0, deletes the object'),
         ('delete_ita_super', 'ItaSuper::scalar_deleting_destructor', '0x65f830', 'ita_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Itália e, com o bit 0, apaga o objeto',
          'runs the destructor of the Italian Super Cup and, with bit 0, deletes the object'),
         ('delete_jap_emp_cup', 'JapEmpCup::scalar_deleting_destructor', '0x661f00', 'jap_emp_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa do Imperador e, com o bit 0, apaga o objeto',
          "runs the destructor of the Emperor's Cup and, with bit 0, deletes the object"),
         ('delete_jap_j_cup', 'JapJCup::scalar_deleting_destructor', '0x665550', 'jap_j_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da J. League e, com o bit 0, apaga o objeto',
          'runs the destructor of the J League Cup and, with bit 0, deletes the object'),
         ('delete_jap_super', 'JapSuper::scalar_deleting_destructor', '0x665f30', 'jap_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa do Japão e, com o bit 0, apaga o objeto',
          'runs the destructor of the Japanese Super Cup and, with bit 0, deletes the object'),
         ('delete_kor_fa_cup', 'KorFaCup::scalar_deleting_destructor', '0x66a630', 'kor_fa_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Coreia (FA Cup) e, com o bit 0, apaga o objeto',
          'runs the destructor of the Korean FA Cup and, with bit 0, deletes the object'),
         ('delete_kor_league_cup', 'KorLeagueCup::scalar_deleting_destructor', '0x66c140', 'kor_league_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Liga coreana e, com o bit 0, apaga o objeto',
          'runs the destructor of the Korean League Cup and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
