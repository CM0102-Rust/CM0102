# A força dos treinadores no treino: o fator do treinador por categoria (TRAINING_MANAGER::get_coaching_factor
# 0x8b41d0, cinco vezes por jogador em cada atualização do treino, TRAINING_MANAGER::update_player_training 0x8b1750,
# que roda na escalação de cada partida 0x6eb045, na lesão 0x62d312 e nas telas), a comissão de treino do técnico
# humano (TRAINING_MANAGER::get_coach_assignments 0x8b4cd0, pelo fator e pelas 0x8b23b0, 0x8b2880 e 0x8b35a0) e a carga
# do programa (TRAINING_SCHEDULE::get_training_load 0x8b4f60, pela 0x8b1750 no clube humano, pela set_player_schedule
# 0x8b2250 e pela reassign_squad_schedules 0x8b2470, esta já em Rust). Chamadas originais pelo endereço: 0x945640 (_ftol) e
# a caixa de erro (aviso_em); o resto já em Rust ou nesta leva. Iguais no cm0102-1080.exe. Trocas: das candidatas da
# família, a update_player_training (0x8b1750, 2,7 KB com o progresso sorteado) fica para outra leva; as de tela e de
# arquivo (0x8b10d0, 0x8b5220, 0x8b53d0, 0x8b5520, 0x8b5690) ficam de fora. Simulação: as mutações pela carreira até
# 2002-06-01. Comparadores: as três pela devolução (só leem), sem os casos de erro da comissão.
SAVE = False
ROTINAS = [('fator_do_treinador', 0x8b41d0, [0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x1c]),
           ('comissao_do_humano', 0x8b4cd0, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('carga_do_programa', 0x8b4f60, [0x51, 0x53, 0x56, 0x57, 0x8b, 0xf9])]
DOCS = ('a força dos treinadores no treino: o fator do treinador por categoria, a comissão de treino do técnico humano e a carga do programa de treino',
        "the coaches' strength in training: the coaching factor by category, the human manager's coach assignments and the training schedule load",
        [('fator_do_treinador', 'TRAINING_MANAGER::get_coaching_factor', '0x8b41d0', 'training_manager.cpp', 'thiscall, ret 8',
          'o fator dos treinadores numa categoria de treino: 100 sem técnico ou com técnico do computador; no clube humano, a média das notas da comissão (dividida pelo divisor da quantidade) × 40/7, no mínimo 50',
          'the coaching factor of a training category: 100 without a manager or with a computer manager; at a human club, the average rating of the coach assignments (divided by the divisor for their number) × 40/7, at least 50'),
         ('comissao_do_humano', 'TRAINING_MANAGER::get_coach_assignments', '0x8b4cd0', 'training_manager.cpp', 'thiscall, ret 4',
          'a comissão de treino (seis vagas de 9 bytes) de um dos 16 técnicos humanos; fora deles, a caixa de erro e 0',
          'the coach assignment table (six 9-byte slots) of one of the 16 human managers; otherwise the error box and 0'),
         ('carga_do_programa', 'TRAINING_SCHEDULE::get_training_load', '0x8b4f60', 'training_manager.cpp', 'thiscall',
          'a carga do programa de treino pelos cinco níveis das categorias, com pesos fixos, até 100',
          'the training schedule load from its five category levels with fixed weights, up to 100')])
CAMPOS = [('clube +0xcf: técnico; ponteiro; lê (fator_do_treinador)',
           'club +0xcf: manager; pointer; reads (fator_do_treinador)'),
          ('pessoa +0: id (os 16 últimos de [0xae2364] são os técnicos humanos); id; lê (fator_do_treinador, comissao_do_humano)',
           'person +0: id (the last 16 of [0xae2364] are the human managers); id; reads (fator_do_treinador, comissao_do_humano)'),
          ('TRAINING_MANAGER +0x652d + 54 × humano: comissão de treino, seis vagas de 9 bytes (+0 id do treinador, −1 vazia; +4 + categoria: nota de 1 a 20); id e valor; lê (fator_do_treinador, comissao_do_humano)',
           'TRAINING_MANAGER +0x652d + 54 × human: coach assignments, six 9-byte slots (+0 coach id, −1 empty; +4 + category: 1 to 20 rating); id and value; reads (fator_do_treinador, comissao_do_humano)'),
          ('TRAINING_SCHEDULE +4..+8: níveis das cinco categorias (0, 10, 25, 50); valor; lê (carga_do_programa)',
           'TRAINING_SCHEDULE +4..+8: levels of the five categories (0, 10, 25, 50); value; reads (carga_do_programa)')]
INTERESSA = ['CMRTE']
