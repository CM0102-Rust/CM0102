# O cabecalho do registro do programa de treino (training_screens.cpp; leitor tel45, rodada 5, 23/11/2026).
# Convencao conferida na desmontagem: thiscall, sem argumentos, devolve this em eax, ret 0 (0x8bba10..0x8bba2b).
# Chamador unico: 0x8b4e16 (TRAINING_SCHEDULE::TRAINING_SCHEDULE 0x8b4e10, com ecx = programa + 0x18), que o
# training_screens_open_schedule_editor (tel25_2, ja no motor) chama pelo endereco ao abrir o editor do programa.
# Prologo: mov eax, ecx; xor ecx, ecx; mov dword [eax], 0x1011 (10 bytes, fronteira de instrucao). Igual no
# cm0102-1080.exe.
TELAS = True
ROTINAS = [('training_record_init_header', 0x8bba10, [0x8b, 0xc1, 0x33, 0xc9, 0xc7, 0x00, 0x11, 0x10, 0x00, 0x00])]
DOCS = ('o cabeçalho do registro do programa de treino',
        "the training schedule record's header",
        [('training_record_init_header', 'training_record_init_header', '0x8bba10', 'training_screens.cpp', 'thiscall',
          'os valores iniciais do cabeçalho de 0x11 bytes do programa de treino (+0x18): 0x1011, o byte 0xff, dois zeros e 0x21',
          "the defaults of the training schedule's 0x11-byte header (+0x18): 0x1011, the byte 0xff, two zeros and 0x21")])
