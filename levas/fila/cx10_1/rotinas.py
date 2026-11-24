# Os destrutores da Primeira Divisao finlandesa e das ligas francesas (com as duas competicoes-reserva da Primeira
# Divisao francesa) e o da tabela das financas (fin_prm.cpp, fra_cfa.cpp, fra_first.cpp, fra_lower.cpp; leitor cx10,
# rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx08_1: rodam quando o jogo e desmontado (carregar outro
# save com um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira aberta que tenha a Finlandia e a
# Franca carregadas (a tabela das financas existe em toda carreira), carregar outro save, jogando comparando.
# - 0x595420 FinPrm::~FinPrm thiscall() ret 0: a liga sem fases (drop_plain_league, aqui definido; o mesmo corpo do
#   BelFirst::~BelFirst do cx08_1, conferido instrucao a instrucao): a vtable 0x96a4c0, League::free_table 0x687970
#   (push 0), free(+0xba) e zera, as estatisticas (Comp_Stats::~Comp_Stats 0x49f450 e operator delete), League::~League
#   0x682300. Prologo push -1; push imm32 (7 bytes).
# - 0x59a5a0 FinanceTable::~FinanceTable thiscall() ret 0: com o vetor de registros, o vector destructor iterator do CRT
#   0x9462bb (stdcall, 4 argumentos: o vetor, 0x167, a contagem do dword anterior, Finance::~Finance 0x5a8f60, um corpo
#   vazio: a copia nao o chama) e o operator delete 0x944c94 do bloco - 4; o ponteiro fica. Prologo mov eax, [ecx];
#   test eax, eax; je rel8 (6 bytes): RELATIVAS (0x59a5a0, 4), o salto curto refeito no trampolim.
# - 0x5bb580 FraCfaStubComp::~FraCfaStubComp e 0x5bb5b0 FraLowerStubComp::~FraLowerStubComp thiscall() ret 0: a vtable
#   (0x96a994, 0x96a904) e jmp Comp::~Comp 0x48c650 (a copia chama pelo desvio, como o drop_cup do cx01_14). Prologo
#   mov dword [ecx], imm32 (6 bytes).
# - 0x5bb5e0 FraFirst::~FraFirst thiscall() ret 0: o corpo da liga sem fases e, depois das estatisticas, as duas
#   competicoes-reserva (+0xee, +0xf2) pelo slot 0 com 1, sem zerar. Prologo push -1; push imm32.
# - 0x595400, 0x5b9440, 0x5bb560, 0x5bb590, 0x5bb5c0, 0x5bd850: os destrutores escalares (push esi; mov esi, ecx; call
#   rel32 ~X; operator delete 0x944c94 cdecl com o bit 0), thiscall(flags) ret 4 -> this: o corpo delete_through do
#   cx03_3 e o prologo pelas RELATIVAS (endereco, 3); o ~FraCfa 0x5b9460 e o ~FraLower 0x5bd870 ja sao Rust.
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), os destrutores thiscall, ret 0 (mov ecx ->
# call pelo destrutor escalar). O quadro SEH so cobre o desenrolar de excecao. Iguais no cm0102-1080.exe (inteiras).
# Comparadores: so a original (liberam memoria).
# Depende do cx01_14 (delete_comp_stats) e do cx03_3 (delete_through): integrar depois deles.
SAVE = False
TELAS = True
ROTINAS = [('drop_fin_prm', 0x595420, [0x6a, 0xff, 0x68, 0xe8, 0x8f, 0x95, 0x00]),
           ('delete_fin_prm', 0x595400, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_finance_table', 0x59a5a0, [0x8b, 0x01, 0x85, 0xc0, 0x74, 0x22]),
           ('delete_fra_cfa', 0x5b9440, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_fra_cfa_stub_comp', 0x5bb580, [0xc7, 0x01, 0x94, 0xa9, 0x96, 0x00]),
           ('delete_fra_cfa_stub_comp', 0x5bb560, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_fra_lower_stub_comp', 0x5bb5b0, [0xc7, 0x01, 0x04, 0xa9, 0x96, 0x00]),
           ('delete_fra_lower_stub_comp', 0x5bb590, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_fra_first', 0x5bb5e0, [0x6a, 0xff, 0x68, 0x98, 0x99, 0x95, 0x00]),
           ('delete_fra_first', 0x5bb5c0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_fra_lower', 0x5bd850, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores da Primeira Divisão finlandesa, das ligas francesas e da tabela das finanças',
        'the destructors of the Finnish Premier Division, the French leagues and the finance table',
        [('drop_fin_prm', 'FinPrm::~FinPrm', '0x595420', 'fin_prm.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão finlandesa, uma liga sem fases: libera a tabela, a tabela de rodadas e as estatísticas e roda o destrutor da liga',
          'destroys the Finnish Premier Division, a league with no stages: frees the table, the round table and the statistics and runs the league destructor'),
         ('delete_fin_prm', 'FinPrm::scalar_deleting_destructor', '0x595400', 'fin_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão finlandesa e, com o bit 0, libera o objeto',
          'runs the Finnish Premier Division destructor and, with bit 0, frees the object'),
         ('drop_finance_table', 'FinanceTable::~FinanceTable', '0x59a5a0', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0',
          'desmonta a tabela das finanças: libera o vetor dos registros de finanças dos clubes (o destrutor de cada registro é vazio)',
          'destroys the finance table: frees the array of the clubs\' finance records (each record\'s destructor is empty)'),
         ('delete_fra_cfa', 'FraCfa::scalar_deleting_destructor', '0x5b9440', 'fra_cfa.cpp', 'thiscall, ret 4',
          'roda o destrutor da liga amadora francesa e, com o bit 0, libera o objeto',
          'runs the French amateur league destructor and, with bit 0, frees the object'),
         ('drop_fra_cfa_stub_comp', 'FraCfaStubComp::~FraCfaStubComp', '0x5bb580', 'fra_first.cpp', 'thiscall, ret 0',
          'desmonta a competição-reserva da liga amadora da Primeira Divisão francesa: a tabela virtual dela e o destrutor da competição',
          'destroys the French First Division\'s amateur league placeholder comp: its vtable and the competition destructor'),
         ('delete_fra_cfa_stub_comp', 'FraCfaStubComp::scalar_deleting_destructor', '0x5bb560', 'fra_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da competição-reserva amadora e, com o bit 0, libera o objeto',
          'runs the amateur placeholder comp destructor and, with bit 0, frees the object'),
         ('drop_fra_lower_stub_comp', 'FraLowerStubComp::~FraLowerStubComp', '0x5bb5b0', 'fra_first.cpp', 'thiscall, ret 0',
          'desmonta a competição-reserva das divisões inferiores da Primeira Divisão francesa: a tabela virtual dela e o destrutor da competição',
          'destroys the French First Division\'s lower division placeholder comp: its vtable and the competition destructor'),
         ('delete_fra_lower_stub_comp', 'FraLowerStubComp::scalar_deleting_destructor', '0x5bb590', 'fra_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da competição-reserva das divisões inferiores e, com o bit 0, libera o objeto',
          'runs the lower division placeholder comp destructor and, with bit 0, frees the object'),
         ('drop_fra_first', 'FraFirst::~FraFirst', '0x5bb5e0', 'fra_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão francesa: libera a tabela, a tabela de rodadas e as estatísticas, apaga as duas competições-reserva e roda o destrutor da liga',
          'destroys the French First Division: frees the table, the round table and the statistics, deletes the two placeholder comps and runs the league destructor'),
         ('delete_fra_first', 'FraFirst::scalar_deleting_destructor', '0x5bb5c0', 'fra_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão francesa e, com o bit 0, libera o objeto',
          'runs the French First Division destructor and, with bit 0, frees the object'),
         ('delete_fra_lower', 'FraLower::scalar_deleting_destructor', '0x5bd850', 'fra_lower.cpp', 'thiscall, ret 4',
          'roda o destrutor das divisões inferiores francesas e, com o bit 0, libera o objeto',
          'runs the French lower leagues destructor and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
