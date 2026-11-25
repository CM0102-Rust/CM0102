# Destrutores escalares de copas e competicoes (leitor rx14, rodada 6, 25/11/2026). Item de TELAS: rodam quando o
# jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira
# aberta que tenha essas competicoes carregadas (cada copa nacional pede a liga da sua nacao), carregar outro save,
# jogando comparando.
# Os destrutores escalares (push esi; mov esi, ecx; call ~X; test byte [esp+8], 1; operator delete 0x944c94;
# mov eax, esi; ret 4): o prologo tem 3 bytes antes de um call rel32 e entra pelas RELATIVAS (endereco, 3) do
# rust/src/detours.rs, ver INTEGRAR_ANTES.txt. O destrutor da classe e chamado pelo desvio (delete_through, do
# cx03_3: integrar depois dele). Convencoes conferidas: thiscall(flags) ret 4 -> this; o destrutor da classe
# thiscall, ret 0; operator delete cdecl. Iguais no cm0102-1080.exe (todas inteiras).
# Cup::scalar_deleting_destructor (0x518670) so rodaria numa Cup pura: toda copa derivada tem o seu slot 0.
# Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('delete_african_nations', 0x401200, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_asia_nations', 0x40ea20, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_euro_champ', 0x57aab0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_european_cup', 0x5839e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_inter_amer_cup', 0x632250, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_intertoto_cup', 0x6331a0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_friendly', 0x5c22a0, [0x56, 0x8b, 0xf1, 0xe8, 0x28, 0x00, 0x00, 0x00]),
           ('delete_cup', 0x518670, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bel_fa_cup', 0x41d220, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bel_super', 0x4210c0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_champ_cup', 0x425b30, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_cup', 0x427610, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('destrutores escalares de copas e competições',
        'cup and competition scalar deleting destructors',
        [('delete_african_nations', 'AfricanNations::scalar_deleting_destructor', '0x401200', 'african_nations.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa Africana de Nações e, com o bit 0, apaga o objeto',
          'runs the destructor of the African Nations Cup and, with bit 0, deletes the object'),
         ('delete_asia_nations', 'AsiaNations::scalar_deleting_destructor', '0x40ea20', 'asia_nations.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Ásia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Asian Cup and, with bit 0, deletes the object'),
         ('delete_euro_champ', 'EuroChamp::scalar_deleting_destructor', '0x57aab0', 'euro_champ.cpp', 'thiscall, ret 4',
          'roda o destrutor da Eurocopa e, com o bit 0, apaga o objeto',
          'runs the destructor of the European Championship and, with bit 0, deletes the object'),
         ('delete_european_cup', 'EuropeanCup::scalar_deleting_destructor', '0x5839e0', 'european_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa dos Campeões Europeus e, com o bit 0, apaga o objeto',
          'runs the destructor of the European Cup and, with bit 0, deletes the object'),
         ('delete_inter_amer_cup', 'InterAmerCup::scalar_deleting_destructor', '0x632250', 'inter_amer_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa Interamericana e, com o bit 0, apaga o objeto',
          'runs the destructor of the Inter-American Cup and, with bit 0, deletes the object'),
         ('delete_intertoto_cup', 'IntertotoCup::scalar_deleting_destructor', '0x6331a0', 'intertoto_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa Intertoto e, com o bit 0, apaga o objeto',
          'runs the destructor of the Intertoto Cup and, with bit 0, deletes the object'),
         ('delete_friendly', 'Friendly::scalar_deleting_destructor', '0x5c22a0', 'friendly.cpp', 'thiscall, ret 4',
          'roda o destrutor da competição dos amistosos e, com o bit 0, apaga o objeto',
          'runs the destructor of the friendlies competition and, with bit 0, deletes the object'),
         ('delete_cup', 'Cup::scalar_deleting_destructor', '0x518670', 'Cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da base da copa e, com o bit 0, apaga o objeto',
          'runs the destructor of the cup base and, with bit 0, deletes the object'),
         ('delete_bel_fa_cup', 'BelFaCup::scalar_deleting_destructor', '0x41d220', 'bel_fa_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Bélgica e, com o bit 0, apaga o objeto',
          'runs the destructor of the Belgian Cup and, with bit 0, deletes the object'),
         ('delete_bel_super', 'BelSuper::scalar_deleting_destructor', '0x4210c0', 'bel_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Bélgica e, com o bit 0, apaga o objeto',
          'runs the destructor of the Belgian Super Cup and, with bit 0, deletes the object'),
         ('delete_bra_champ_cup', 'BraChampCup::scalar_deleting_destructor', '0x425b30', 'bra_champ_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa dos Campeões do Brasil e, com o bit 0, apaga o objeto',
          'runs the destructor of the Brazilian Champions Cup and, with bit 0, deletes the object'),
         ('delete_bra_cup', 'BraCup::scalar_deleting_destructor', '0x427610', 'bra_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa do Brasil e, com o bit 0, apaga o objeto',
          'runs the destructor of the Brazilian Cup and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
