# O construtor de uma vaga de tatica do tecnico humano (human_manager.cpp; leitor rx15, rodada 6, 25/11/2026).
# Simulacao, carreira ENG (sem LIGAS): roda na criacao do jogo, 5 vezes por registro de tecnico humano (16 registros),
# pelo iterador de vetor do HUMAN_RECORD::HUMAN_RECORD 0x5fb690 (sx12b_2: crt_vector_constructor_iterator(this, 0x1ecd,
# 5, 0x5fb720, 0x5a53c0); a unica referencia a 0x5fb720 e o push em 0x5fb6ad), e na carga do HUMAN_MANAGER.
# - 0x5fb720 HUMAN_TACTIC_SLOT::HUMAN_TACTIC_SLOT thiscall(), ret 0, eax this: push esi; mov esi, ecx; call
#   Formation::Formation 0x5b27c0 (ja em Rust: formation_new, chamada direta); mov eax, esi; pop esi; ret. O resto da
#   vaga (0x1ecd bytes) fica como esta.
# O prologo tem 3 bytes antes de um call rel32: entra pelas RELATIVAS (0x5fb720, 3), ver INTEGRAR_ANTES.txt.
# Depende do sx12b_2 (HUMAN_TACTIC_SLOT_SIZE): integrar junto ou depois dele.
# Igual no cm0102-1080.exe (inteira). Comparador: lado a lado sobre a vaga inteira.
SAVE = False
ROTINAS = [('construct_human_tactic_slot', 0x5fb720, [0x56, 0x8b, 0xf1, 0xe8, 0x98, 0x70, 0xfb, 0xff])]
DOCS = ('o construtor de uma vaga de tática do técnico humano',
        'the human manager tactic slot constructor',
        [('construct_human_tactic_slot', 'HUMAN_TACTIC_SLOT::HUMAN_TACTIC_SLOT', '0x5fb720', 'human_manager.cpp', 'thiscall',
          'monta uma das cinco vagas de tática de um registro de técnico humano: só a formação do começo (o 4-4-2 padrão, "Uninitialised"); o resto da vaga fica como está',
          'builds one of the five tactic slots of a human manager record: only the formation at its start (the default 4-4-2, "Uninitialised"); the rest of the slot is left as it is')])
CAMPOS = []
INTERESSA = []
