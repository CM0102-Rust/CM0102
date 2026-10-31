# Os programas de treino padrao: os sete programas do computador e os dez de cada tecnico humano montados no comeco do
# jogo (TRAINING_MANAGER::set_default_schedules 0x8b3d40, pelo construtor 0x8b0ec0 num jogo novo, sem training.dat), o
# tipo do programa (TRAINING_SCHEDULE::set_type 0x8b4e50, 167 vezes no comeco e pelas telas) e o programa do humano por
# vaga (TRAINING_MANAGER::get_manager_schedule 0x8b3930, pela 0x8b3d40 e pelas telas). O computador treina pela vaga 1
# (geral) e pela 6 (goleiros) conforme a ficha de treino (0x8b39f0): os niveis desses dois programas entram no progresso
# de todo jogador do computador em cada atualizacao do treino (0x8b1750). Chamadas originais pelo endereco: 0x8b2230 (os
# nomes padrao, 0x8b50f0) e a caixa de erro (aviso_em); o resto nesta leva. Iguais no cm0102-1080.exe. Trocas: as
# rotinas do treino de posicao (0x8b42d0, 0x8b4450, 0x8b4770, 0x8b49b0) so rodam com a lista do treino de posicao, que
# so as telas enchem: ficam de fora. Simulacao: as mutacoes pela carreira ate 2002-06-01 (a montagem roda no comeco de
# cada carreira nova). Comparadores: a 0x8b3d40 e a 0x8b4e50 lado a lado (so gravam nos programas); a 0x8b3930 pela
# devolucao, sem os casos de erro.
SAVE = False
ROTINAS = [('programas_padrao', 0x8b3d40, [0x81, 0xec, 0x0c, 0x0c, 0x00, 0x00]),
           ('tipo_do_programa', 0x8b4e50, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('programa_do_humano', 0x8b3930, [0x8b, 0x54, 0x24, 0x04, 0xa1, 0x64, 0x23, 0xae, 0x00])]
DOCS = ('os programas de treino padrão: os sete do computador e os dez de cada técnico humano no começo do jogo, o tipo do programa e o programa do humano por vaga',
        "the default training schedules: the computer's seven and each human manager's ten at the start of the game, the schedule type and the human manager's schedule by slot",
        [('programas_padrao', 'TRAINING_MANAGER::set_default_schedules', '0x8b3d40', 'training_manager.cpp', 'thiscall',
          'monta os sete programas do computador (o tipo k na vaga k) e os dez programas de cada um dos 16 técnicos humanos, e dá os nomes padrão',
          "sets up the computer's seven schedules (type k in slot k) and the ten schedules of each of the 16 human managers, then gives the default names"),
         ('tipo_do_programa', 'TRAINING_SCHEDULE::set_type', '0x8b4e50', 'training_manager.cpp', 'thiscall, ret 8',
          'grava a vaga e o tipo do programa e os cinco níveis das categorias pela mistura do tipo (geral, uma categoria forte, goleiros ou sem treino)',
          "stores the schedule's slot and type and its five category levels from the type's mix (general, one strong category, goalkeepers or no training)"),
         ('programa_do_humano', 'TRAINING_MANAGER::get_manager_schedule', '0x8b3930', 'training_manager.cpp', 'thiscall, ret 8',
          'o programa da vaga 0 a 9 (fora disso, 1) de um técnico humano; fora dos 16 humanos, a caixa de erro e 0',
          "a human manager's schedule for slot 0 to 9 (otherwise 1); outside the 16 human managers, the error box and 0")])
CAMPOS = [('TRAINING_MANAGER +0 + 0x9b × vaga: os 167 programas (7 do computador, depois 10 por humano); valor; grava (programas_padrao), lê o endereço (programa_do_humano)',
           'TRAINING_MANAGER +0 + 0x9b × slot: the 167 schedules (7 for the computer, then 10 per human); value; writes (programas_padrao), reads the address (programa_do_humano)'),
          ('TRAINING_SCHEDULE +0: vaga (dword com sinal); +4..+8: níveis das cinco categorias (0, 10, 25, 50); +0x15: tipo (−1 a 7); valor; grava (tipo_do_programa)',
           'TRAINING_SCHEDULE +0: slot (signed dword); +4..+8: levels of the five categories (0, 10, 25, 50); +0x15: type (−1 to 7); value; writes (tipo_do_programa)'),
          ('pessoa +0: id (os 16 últimos de [0xae2364] são os técnicos humanos; registro de 0x6e bytes em [0xae23bc]); id; lê (programas_padrao, programa_do_humano)',
           'person +0: id (the last 16 of [0xae2364] are the human managers; 0x6e-byte record at [0xae23bc]); id; reads (programas_padrao, programa_do_humano)')]
INTERESSA = ['CMRTE']
