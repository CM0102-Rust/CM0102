# Os destrutores de três estaduais e da Segunda Divisão da Dinamarca, e oito destrutores escalares de ligas (leitor cx09, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx02_3 e cx03_3:
# rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo:
# com uma carreira aberta que tenha as ligas dessas nacoes carregadas (Inglaterra; Belgica, Brasil, Croacia,
# Dinamarca e Finlandia), carregar outro save, jogando comparando.
# Destrutores (thiscall, ret 0; prologo push -1; push imm32, 7 bytes; o quadro SEH so cobre o desenrolar de excecao):
# - forma das ligas com fases (8): a vtable, League::free_table 0x687970 (push 0), free(+0xba) e zera, o slot 0 das
#   fases 0..+0x30 (push 1), free(+0xc) e zera, Comp_Stats::~Comp_Stats 0x49f450 e operator delete, League::~League
#   0x682300: o drop_group_cup do cx02_3 sem lista de inscritos. A Serie C (0x42d540) libera antes o +0xee.
# - forma sem fases (CroA1, CroA2a, CroA2b, DenFirst, DenPrm, EngPrm): sem o laco das fases (drop_plain_league, cx09_1).
# - forma dos estaduais e da DenSecond: o vetor das fases e liberado sem teste de nulo e nao e zerado
#   (drop_regional_league, cx09_2).
# Destrutores escalares (push esi; mov esi, ecx; call ~X; test byte [esp+8], 1; operator delete 0x944c94;
# mov eax, esi; ret 4): prologo com 3 bytes antes de um call rel32, entra pelas RELATIVAS (endereco, 3), ver
# INTEGRAR_ANTES.txt; o ~X e chamado pelo desvio (delete_through do cx03_3).
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), Comp_Stats::~Comp_Stats 0x49f450 e
# League::~League 0x682300 (thiscall, ret 0), League::free_table 0x687970 (thiscall, ret 4). Iguais no
# cm0102-1080.exe (todas inteiras). Comparadores: so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('drop_bra_reg_rio', 0x439c80, [0x6a, 0xff, 0x68, 0x38, 0x4a, 0x95, 0x00]),
           ('drop_bra_reg_santa', 0x43b370, [0x6a, 0xff, 0x68, 0xc8, 0x4a, 0x95, 0x00]),
           ('drop_bra_reg_sp', 0x43c4e0, [0x6a, 0xff, 0x68, 0x38, 0x4b, 0x95, 0x00]),
           ('drop_den_second', 0x5540b0, [0x6a, 0xff, 0x68, 0x98, 0x7b, 0x95, 0x00]),
           ('delete_bel_third', 0x421920, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_nat_first', 0x429a70, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_bra_nat_second', 0x42c060, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_conf', 0x56dd80, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_first', 0x5718a0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_second', 0x5756c0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_eng_third', 0x577000, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_fin_first', 0x5930e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores de três estaduais e da Segunda Divisão da Dinamarca, e oito destrutores escalares de ligas',
        'the destructors of three state championships and the Danish Second Division, and eight league scalar deleting destructors',
        [('drop_bra_reg_rio', 'BraRegRio::~BraRegRio', '0x439c80', 'bra_reg_rio.cpp', 'thiscall',
          'desmonta o Campeonato Carioca: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Rio de Janeiro state championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_santa', 'BraRegSanta::~BraRegSanta', '0x43b370', 'bra_reg_santa.cpp', 'thiscall',
          'desmonta o Campeonato Catarinense: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Santa Catarina state championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_bra_reg_sp', 'BraRegSp::~BraRegSp', '0x43c4e0', 'bra_reg_sp.cpp', 'thiscall',
          'desmonta o Campeonato Paulista: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Sao Paulo state championship: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('drop_den_second', 'DenSecond::~DenSecond', '0x5540b0', 'den_second.cpp', 'thiscall',
          'desmonta a Segunda Divisão da Dinamarca: libera a tabela e as datas, apaga as fases e libera o vetor delas sem zerá-lo, libera as estatísticas e roda o destrutor da liga',
          'destroys the Danish Second Division: frees the table and the dates, destroys the stages and frees their vector without clearing it, frees the statistics and runs the league destructor'),
         ('delete_bel_third', 'BelThird::scalar_deleting_destructor', '0x421920', 'bel_third.cpp', 'thiscall, ret 4',
          'roda o destrutor da Terceira Divisão da Bélgica e, com o bit 0, apaga o objeto',
          'runs the destructor of the Belgian Third Division and, with bit 0, deletes the object'),
         ('delete_bra_nat_first', 'BraNatFirst::scalar_deleting_destructor', '0x429a70', 'bra_nat_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série A do Brasileiro e, com o bit 0, apaga o objeto',
          'runs the destructor of the Brazilian Serie A and, with bit 0, deletes the object'),
         ('delete_bra_nat_second', 'BraNatSecond::scalar_deleting_destructor', '0x42c060', 'bra_nat_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série B do Brasileiro e, com o bit 0, apaga o objeto',
          'runs the destructor of the Brazilian Serie B and, with bit 0, deletes the object'),
         ('delete_eng_conf', 'EngConf::scalar_deleting_destructor', '0x56dd80', 'eng_conf.cpp', 'thiscall, ret 4',
          'roda o destrutor da Conference inglesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the Football Conference and, with bit 0, deletes the object'),
         ('delete_eng_first', 'EngFirst::scalar_deleting_destructor', '0x5718a0', 'eng_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da First Division inglesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the English First Division and, with bit 0, deletes the object'),
         ('delete_eng_second', 'EngSecond::scalar_deleting_destructor', '0x5756c0', 'eng_second.cpp', 'thiscall, ret 4',
          'roda o destrutor da Second Division inglesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the English Second Division and, with bit 0, deletes the object'),
         ('delete_eng_third', 'EngThird::scalar_deleting_destructor', '0x577000', 'eng_third.cpp', 'thiscall, ret 4',
          'roda o destrutor da Third Division inglesa e, com o bit 0, apaga o objeto',
          'runs the destructor of the English Third Division and, with bit 0, deletes the object'),
         ('delete_fin_first', 'FinFirst::scalar_deleting_destructor', '0x5930e0', 'fin_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão da Finlândia e, com o bit 0, apaga o objeto',
          'runs the destructor of the Finnish First Division and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
