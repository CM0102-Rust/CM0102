# Os destrutores da Serie C2 italiana, grupos A e B (ita_ser_c2a.cpp, ita_ser_c2b.cpp; leitor cx11, rodada 5,
# 24/11/2026). Item de TELAS, como os destrutores do cx10_4: rodam quando o jogo e desmontado (carregar outro save com
# um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira aberta que tenha a Italia com as divisoes
# inferiores (base NORMAL ou MAXIMA na criacao; com a base MINIMA a Serie C2 nao existe), carregar outro save, jogando
# comparando.
# - 0x6530e0 ItaSerC2a::~ItaSerC2a e 0x6575a0 ItaSerC2b::~ItaSerC2b thiscall() ret 0: o mesmo corpo do
#   ItaSerC1a::~ItaSerC1a (conferido instrucao a instrucao; so a vtable e o manipulador do SEH mudam): a vtable 0x96c69c
#   / 0x96c750, League::free_table 0x687970 (push 0), free(+0xba) e zera, as fases 0..+0x30 pela virtual 0 com 1, o
#   vetor das fases solto sem teste nem zerar, Comp_Stats::~Comp_Stats 0x49f450 e operator delete, League::~League
#   0x682300: drop_stage_league_unchecked do cx10_4. Prologo push -1; push imm32 (7 bytes).
# - 0x6530c0 e 0x657580, os destrutores escalares: push esi; mov esi, ecx; call rel32 ~X; operator delete 0x944c94
#   (cdecl) com o bit 0; thiscall(flags) ret 4 -> this: delete_through do cx03_3 e o prologo pelas RELATIVAS
#   (0x6530c0, 3) e (0x657580, 3) (ver INTEGRAR_ANTES.txt).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (liberam memoria).
# Depende do cx10_4 (drop_stage_league_unchecked) e do cx03_3 (delete_through), e por eles do cx01_14, cx04_1 e cx10_1.
SAVE = False
TELAS = True
ROTINAS = [('drop_ita_ser_c2a', 0x6530e0, [0x6a, 0xff, 0x68, 0xf8, 0xbb, 0x95, 0x00]),
           ('delete_ita_ser_c2a', 0x6530c0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_ita_ser_c2b', 0x6575a0, [0x6a, 0xff, 0x68, 0x98, 0xbc, 0x95, 0x00]),
           ('delete_ita_ser_c2b', 0x657580, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('os destrutores da Série C2 italiana, grupos A e B',
        'the destructors of the Italian Serie C2 groups A and B',
        [('drop_ita_ser_c2a', 'ItaSerC2a::~ItaSerC2a', '0x6530e0', 'ita_ser_c2a.cpp', 'thiscall, ret 0',
          'desmonta a Série C2/A italiana: libera a tabela, a tabela de rodadas, as fases até a atual, o vetor das fases (sem testar nem zerar) e as estatísticas e roda o destrutor da liga',
          'destroys the Italian Serie C2/A: frees the table, the round table, the stages up to the current one, the stage vector (unchecked and not cleared) and the statistics and runs the league destructor'),
         ('delete_ita_ser_c2a', 'ItaSerC2a::scalar_deleting_destructor', '0x6530c0', 'ita_ser_c2a.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série C2/A italiana e, com o bit 0, libera o objeto',
          'runs the destructor of the Italian Serie C2/A and, with bit 0, frees the object'),
         ('drop_ita_ser_c2b', 'ItaSerC2b::~ItaSerC2b', '0x6575a0', 'ita_ser_c2b.cpp', 'thiscall, ret 0',
          'desmonta a Série C2/B italiana, do mesmo jeito que a C2/A',
          'destroys the Italian Serie C2/B, the same way as C2/A'),
         ('delete_ita_ser_c2b', 'ItaSerC2b::scalar_deleting_destructor', '0x657580', 'ita_ser_c2b.cpp', 'thiscall, ret 4',
          'roda o destrutor da Série C2/B italiana e, com o bit 0, libera o objeto',
          'runs the destructor of the Italian Serie C2/B and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
