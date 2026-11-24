# Os destrutores das três continentais da América e nove destrutores escalares de copas (leitor cx03, rodada 5, 24/11/2026). Item de TELAS: os destrutores rodam quando o jogo e
# desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira aberta que
# tenha essas competicoes carregadas (cada copa nacional pede a liga da sua nacao; as continentais, as ligas do
# continente), carregar outro save, jogando comparando.
# - Merconorte, Libertadores e Mercosul (League): drop_group_cup do cx02_3 sem lista de inscritos
#   (integrar depois dele). Prologo push -1; push imm32 (7 bytes).
# Os destrutores escalares (push esi; mov esi, ecx; call ~X; test byte [esp+8], 1; operator delete 0x944c94;
# mov eax, esi; ret 4): o prologo tem 3 bytes antes de um call rel32 e entra pelas RELATIVAS (endereco, 3) do
# rust/src/detours.rs, ver INTEGRAR_ANTES.txt. O destrutor da classe e chamado pelo desvio (delete_through, aqui).
# Convencoes conferidas: destrutores thiscall, ret 0 (mov ecx -> call pelo destrutor escalar); o free do CRT 0x9452ca
# (cdecl), Comp_Stats::~Comp_Stats 0x49f450 (thiscall), operator delete 0x944c94 (cdecl), Cup::~Cup 0x518690 e
# League::~League 0x682300 (thiscall, ret 0), League::free_table 0x687970 (push 0, thiscall, ret 4), o destrutor
# escalar de cada fase (slot 0 com 1). O quadro SEH so cobre o desenrolar de excecao (nao portado).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_con_merc_cup', 0x4c18a0, [0x6a, 0xff, 0x68, 0xa8, 0x69, 0x95, 0x00]),
           ('drop_conmebol_liber', 0x4c3100, [0x6a, 0xff, 0x68, 0x58, 0x6a, 0x95, 0x00]),
           ('drop_conmebol_merc', 0x4c4870, [0x6a, 0xff, 0x68, 0x08, 0x6b, 0x95, 0x00]),
           ('delete_pol_cup', 0x7c7f50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_pol_lge_cup', 0x7ca6b0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_pol_super', 0x7ccae0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_por_cup', 0x7cdb50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_por_super', 0x7d3000, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_sco_chal_cup', 0x7ecb50, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_sco_fa_cup', 0x7ed4e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_sco_lge_cup', 0x7efbc0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_spa_cup', 0x84d130, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores das três continentais da América e nove destrutores escalares de copas',
        'the destructors of the three American continental cups and nine cup scalar deleting destructors',
        [('drop_con_merc_cup', 'ConMercCup::~ConMercCup', '0x4c18a0', 'con_merc_cup.cpp', 'thiscall',
          'desmonta a Copa Merconorte: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Copa Merconorte: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_conmebol_liber', 'ConmebolLiber::~ConmebolLiber', '0x4c3100', 'conmebol_liber.cpp', 'thiscall',
          'desmonta a Libertadores: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Copa Libertadores: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('drop_conmebol_merc', 'ConmebolMerc::~ConmebolMerc', '0x4c4870', 'conmebol_merc.cpp', 'thiscall',
          'desmonta a Mercosul: libera a tabela e as datas, apaga as fases e o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Copa Mercosur: frees the table and the dates, destroys the stages and their vector, frees the statistics and runs the league destructor'),
         ('delete_pol_cup', 'PolCup::scalar_deleting_destructor', '0x7c7f50', 'pol_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Polônia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Polish Cup and, with bit 0, deletes the object'),
         ('delete_pol_lge_cup', 'PolLgeCup::scalar_deleting_destructor', '0x7ca6b0', 'pol_lge_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Liga polonesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the Polish League Cup and, with bit 0, deletes the object'),
         ('delete_pol_super', 'PolSuper::scalar_deleting_destructor', '0x7ccae0', 'pol_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supercopa da Polônia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Polish Super Cup and, with bit 0, deletes the object'),
         ('delete_por_cup', 'PorCup::scalar_deleting_destructor', '0x7cdb50', 'por_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Taça de Portugal e, com o bit 0, apaga o objeto',
          'runs the destructor of the Portuguese Cup and, with bit 0, deletes the object'),
         ('delete_por_super', 'PorSuper::scalar_deleting_destructor', '0x7d3000', 'por_super.cpp', 'thiscall, ret 4',
          'roda o destrutor da Supertaça de Portugal e, com o bit 0, apaga o objeto',
          'runs the destructor of the Portuguese Super Cup and, with bit 0, deletes the object'),
         ('delete_sco_chal_cup', 'ScoChalCup::scalar_deleting_destructor', '0x7ecb50', 'sco_chal_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Challenge Cup escocesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the Scottish Challenge Cup and, with bit 0, deletes the object'),
         ('delete_sco_fa_cup', 'ScoFaCup::scalar_deleting_destructor', '0x7ed4e0', 'sco_fa_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Escócia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Scottish Cup and, with bit 0, deletes the object'),
         ('delete_sco_lge_cup', 'ScoLgeCup::scalar_deleting_destructor', '0x7efbc0', 'sco_lge_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa da Liga escocesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the Scottish League Cup and, with bit 0, deletes the object'),
         ('delete_spa_cup', 'SpaCup::scalar_deleting_destructor', '0x84d130', 'spa_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa do Rei e, com o bit 0, apaga o objeto',
          'runs the destructor of the Spanish Cup and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
