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
ROTINAS = [('delete_ger_lge_cup', 0x5dc2b0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_gre_cup', 0x5e8af0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_gre_super', 0x5ec200, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_hol_cup', 0x5f35e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_hol_super', 0x5f89e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ire_chal_cup', 0x634270, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ire_leinster_cup', 0x636670, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ire_lge_cup', 0x637020, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ire_munster_cup', 0x638970, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ire_pres_cup', 0x639440, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ire_super_cup', 0x63ae80, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_ita_c1_super', 0x63c7a0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('destrutores escalares de copas e competições',
        'cup and competition scalar deleting destructors',
        [('delete_ger_lge_cup', 'GerLgeCup::scalar_deleting_destructor', '0x5dc2b0', 'ger_lge_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Liga alemã e, com o bit 0, apaga o objeto',
          'runs the destructor of the German League Cup and, with bit 0, deletes the object'),
         ('delete_gre_cup', 'GreCup::scalar_deleting_destructor', '0x5e8af0', 'gre_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Grécia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Greek Cup and, with bit 0, deletes the object'),
         ('delete_gre_super', 'GreSuper::scalar_deleting_destructor', '0x5ec200', 'gre_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Grécia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Greek Super Cup and, with bit 0, deletes the object'),
         ('delete_hol_cup', 'HolCup::scalar_deleting_destructor', '0x5f35e0', 'hol_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Holanda e, com o bit 0, apaga o objeto',
          'runs the destructor of the Dutch Cup and, with bit 0, deletes the object'),
         ('delete_hol_super', 'HolSuper::scalar_deleting_destructor', '0x5f89e0', 'hol_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Holanda e, com o bit 0, apaga o objeto',
          'runs the destructor of the Dutch Super Cup and, with bit 0, deletes the object'),
         ('delete_ire_chal_cup', 'IreChalCup::scalar_deleting_destructor', '0x634270', 'ire_chal_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Challenge Cup irlandesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the Irish Challenge Cup and, with bit 0, deletes the object'),
         ('delete_ire_leinster_cup', 'IreLeinsterCup::scalar_deleting_destructor', '0x636670', 'ire_leinster_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Leinster Cup e, com o bit 0, apaga o objeto',
          'runs the destructor of the Leinster Cup and, with bit 0, deletes the object'),
         ('delete_ire_lge_cup', 'IreLgeCup::scalar_deleting_destructor', '0x637020', 'ire_lge_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Liga irlandesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the Irish League Cup and, with bit 0, deletes the object'),
         ('delete_ire_munster_cup', 'IreMunsterCup::scalar_deleting_destructor', '0x638970', 'ire_munster_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Munster Cup e, com o bit 0, apaga o objeto',
          'runs the destructor of the Munster Cup and, with bit 0, deletes the object'),
         ('delete_ire_pres_cup', 'IrePresCup::scalar_deleting_destructor', '0x639440', 'ire_pres_cup.cpp', 'thiscall, ret 4',
          "roda o destrutor da President's Cup irlandesa e, com o bit 0, apaga o objeto",
          "runs the destructor of the Irish President's Cup and, with bit 0, deletes the object"),
         ('delete_ire_super_cup', 'IreSuperCup::scalar_deleting_destructor', '0x63ae80', 'ire_super_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa irlandesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the Irish Super Cup and, with bit 0, deletes the object'),
         ('delete_ita_c1_super', 'ItaC1Super::scalar_deleting_destructor', '0x63c7a0', 'ita_c1_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Série C1 e, com o bit 0, apaga o objeto',
          'runs the destructor of the Serie C1 Super Cup and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
