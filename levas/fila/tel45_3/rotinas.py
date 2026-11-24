# Apagar um programa de treino (training_screens.cpp; leitor tel45, rodada 5, 23/11/2026).
# Convencao conferida na desmontagem: thiscall(flags) ret 4, devolve this (0x8ba920..0x8ba93b): chama o destrutor
# vazio 0x5a53c0 (um ret) e, com o bit 0 dos flags, operator delete 0x944c94 cdecl. Chamador unico: 0x8ba8fb (o
# tratador do editor do programa, 0x8b97e0 e vizinhos; o tel25_2 ja no motor o chama pelo endereco com flags 1).
# PROLOGO COM call rel32: push esi; mov esi, ecx; call 0x5a53c0 (8 bytes, o call na posicao 3). PRECISA de
# (0x8ba920, 3) em RELATIVAS do rust/src/detours.rs ANTES de integrar (ja listado no INTEGRACAO.md das telas); sem
# isso o trampolim refaz o call relativo no lugar errado e a original cai. Item separado para sair sozinho se for o caso.
# Igual no cm0102-1080.exe.
TELAS = True
ROTINAS = [('training_schedule_scalar_deleting_destructor', 0x8ba920, [0x56, 0x8b, 0xf1, 0xe8, 0x98, 0xaa, 0xce, 0xff])]
DOCS = ('apagar um programa de treino',
        'deleting a training schedule',
        [('training_schedule_scalar_deleting_destructor', 'TRAINING_SCHEDULE::scalar_deleting_destructor', '0x8ba920', 'training_screens.cpp',
          'thiscall, ret 4',
          'o destrutor do programa de treino (vazio) e, com o bit 0 dos flags, a memória liberada; devolve o programa',
          "the training schedule's destructor (empty) and, with bit 0 of the flags, the memory freed; returns the schedule")])
