# O gestor de treino nas telas de treino: programacoes dos jogadores, treinadores e treino de posicao
# (training_manager.cpp; leitor sx21b, rodada 5, 24/11/2026). 0 chamadas na carreira de teste (o mundo ate 2002-08-01,
# cobertura_mundo.txt): so o tecnico humano mexe no treino. Tipo TELAS.
# Caminho no jogo (com o proprio clube): Treino > Programacoes: mudar um jogador de programacao (set_player_schedule,
# collect_squad_schedules lista o elenco e o time B), mudar o nivel de uma categoria (set_category_level); Treino >
# Treinadores: abrir e confirmar a distribuicao (get_coach_assignments_copy, set_coach_assignments); numa programacao
# com treino de posicao ou de lado, mudar o jogador para ela (update_position_training, set_position_training_rate,
# add_position_training e a comparacao por id, que o qsort e o bsearch dos itens usam).
# Convencoes conferidas: 0x8b2250 thiscall(jogador, vaga byte) ret 8; 0x8b23b0 thiscall(tecnico, tabela) ret 8;
# 0x8b3410 thiscall(clube, saida 50x5) ret 8; 0x8b35a0 thiscall(tecnico, saida) ret 8; 0x8b42d0 thiscall(jogador)
# ret 4; 0x8b4450 thiscall(item, comecar) ret 8; 0x8b49b0 thiscall(item) ret 4; 0x8b4e00 cdecl(a, b) (eax = [a] -
# [b]); 0x8b4f30 thiscall(categoria i8, nivel) ret 8. Chamadas: 0x8b1750 update_player_training(jog, 0); 0x8b39f0
# get_player_schedule; 0x8b4f60 get_training_load; 0x62c660 Injury::set_training_load ([0xb63d28]; jog, carga);
# 0x8b4cd0 get_coach_assignments; 0x8b2880 update_coach_assignments(clube, tecnico); 0x540a50 club_get_linked_b_team
# cdecl(clube, 0, 1); 0x541f90 database_collect_club_squad cdecl(clube, &lista, 0); 0x8b4b20 find_position_training(jog,
# &primeiro); 0x5b7d20 formation_get_position_mask cdecl(ficha) (ax); 0x8b4770 get_position_aptitude(pessoa, mascara)
# (al); 0x945333 qsort; 0x944e46 malloc; 0x945667 realloc.
# Defeito do original: update_position_training guarda o ponteiro e a contagem dos itens do jogador antes da primeira
# passada; quando ela acrescenta um item (realloc e qsort), a segunda passada (lados) percorre o ponteiro velho.
# Comparadores: lado a lado na lista do elenco, na copia da tabela de treinadores, no item (taxa), na programacao
# (nivel) e pelo valor (comparacao); so a original nas que mudam o treino, os itens e a tabela (set_player_schedule
# confere a vaga gravada).
SAVE = False
TELAS = True
ROTINAS = [
    ('set_player_schedule', 0x8b2250, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('set_coach_assignments', 0x8b23b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('collect_squad_schedules', 0x8b3410, [0x81, 0xec, 0xd4, 0x02, 0x00, 0x00]),
    ('get_coach_assignments_copy', 0x8b35a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('update_position_training', 0x8b42d0, [0x83, 0xec, 0x18, 0x53, 0x55]),
    ('set_position_training_rate', 0x8b4450, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
    ('add_position_training', 0x8b49b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('training_compare_entries_by_staff_id', 0x8b4e00, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
    ('set_category_level', 0x8b4f30, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
]
DOCS = ('o gestor de treino nas telas: programações dos jogadores, treinadores e treino de posição',
        "the training manager on the screens: players' schedules, coaches and position training",
        [('set_player_schedule', 'set_player_schedule', '0x8b2250', 'training_manager.cpp', 'thiscall, ret 8',
          'regra: vaga nova: o treino do jogador é posto em dia e a entrada leva a vaga; a carga da programação vai para o controle de lesões; o treino de posição é refeito; os itens parados (taxa 0) saem e a lista é reordenada',
          "rule: a new slot: the player's training is caught up and the entry takes the slot; the schedule's load goes to the injury control; the position training is redone; stopped items (rate 0) leave and the list is sorted again"),
         ('set_coach_assignments', 'set_coach_assignments', '0x8b23b0', 'training_manager.cpp', 'thiscall, ret 8',
          'regra: a tabela de 54 bytes editada vai para a vaga do técnico humano e é conferida contra os treinadores do clube',
          "rule: the edited 54-byte table goes into the human manager's slot and is checked against the club's coaches"),
         ('collect_squad_schedules', 'collect_squad_schedules', '0x8b3410', 'training_manager.cpp', 'thiscall, ret 8',
          'regra: os 50 do elenco: cada jogador de verdade do clube ou do time B com a vaga de programação dele; os outros, vazios',
          'rule: the 50 of the squad: each real player of the club or of its B team with his schedule slot; the others empty'),
         ('get_coach_assignments_copy', 'get_coach_assignments_copy', '0x8b35a0', 'training_manager.cpp', 'thiscall, ret 8',
          'regra: a tabela de 54 bytes do técnico humano copiada para edição',
          "rule: the human manager's 54-byte table copied for editing"),
         ('update_position_training', 'update_position_training', '0x8b42d0', 'training_manager.cpp', 'thiscall, ret 4',
          'regra: a posição e o lado da programação: o item igual volta a treinar, os outros param, e sem item igual nasce um novo',
          "rule: the schedule's position and side: the matching item trains again, the others stop, and with none matching a new one is made"),
         ('set_position_training_rate', 'set_position_training_rate', '0x8b4450', 'training_manager.cpp', 'thiscall, ret 8',
          'regra: a dificuldade da posição para o jogador (20 a 130), mais a idade (+50 até 16 anos ... -60 a partir de 23) e duas vezes a aptidão, menos 20: acima de 150 a taxa 5, ... até 80 (parando, negativa: -80 a -5)',
          "rule: the position's difficulty for the player (20 to 130), plus the age (+50 up to 16 ... -60 from 23) and twice the aptitude, less 20: above 150 rate 5, ... up to 80 (stopping, negative: -80 to -5)"),
         ('add_position_training', 'add_position_training', '0x8b49b0', 'training_manager.cpp', 'thiscall, ret 4',
          'regra: se o jogador ainda não joga na posição, o item entra na lista (que cresce de 10 em 10), a lista é reordenada e a entrada dele marcada',
          "rule: when the player cannot play the position yet, the item joins the list (growing by 10), the list is sorted and his entry marked"),
         ('training_compare_entries_by_staff_id', 'training_compare_entries_by_staff_id', '0x8b4e00', 'training_manager.cpp', 'cdecl',
          'regra: a diferença dos ids de staff dos dois itens',
          "rule: the difference of the two items' staff ids"),
         ('set_category_level', 'set_category_level', '0x8b4f30', 'training_manager.cpp', 'thiscall, ret 8',
          'regra: o nível de uma das cinco categorias da programação, que deixa de estar como gravada',
          "rule: the level of one of the schedule's five categories, which is no longer as saved")])
CAMPOS = []
INTERESSA = []
