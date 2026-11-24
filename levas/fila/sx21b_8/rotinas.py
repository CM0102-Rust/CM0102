# Uma programacao de treino nas telas: os jogadores dela, o treino de posicao e de lado, o nome padrao, o arquivo e
# os nomes das categorias e intensidades (training_schedule.cpp; leitor sx21b, rodada 5, 24/11/2026). 0 chamadas na
# carreira de teste: telas de treino do tecnico humano e o dialogo de arquivos. Tipo TELAS.
# Caminho no jogo (com o proprio clube): Treino > Programacoes: a lista de jogadores de cada programacao (Visao geral,
# Sem treino com os lesionados: count_players), os nomes das categorias e das intensidades nas linhas da programacao
# (get_category_name, get_intensity_name), restaurar o nome padrao (set_default_name), marcar treino de posicao e de
# lado (set_position_training, set_side_training); Gravar e Carregar programacao (save_to_file, load_from_file; o
# dialogo de arquivos filtra com check_file e read_file_header).
# Convencoes conferidas: 0x8b5000 thiscall(lista 50x5, saida LLIST, comparacao do qsort) ret 0xc (al); 0x8b5090 e
# 0x8b50c0 thiscall(bits) ret 4 (eax 1); 0x8b50f0 thiscall ret 0; 0x8b5220 e 0x8b53d0 thiscall(caminho) ret 4;
# 0x8b5520 e 0x8b5690 cdecl(categoria ou nivel i8, buffer, tamanho); 0x8b57e0 cdecl(caminho) (1 ou 2); 0x8b5820
# cdecl(caminho, saida 0x6a). Chamadas: 0x945333 qsort; 0x68d080 clear; 0x68d0d0 add_tail (5 bytes); 0x62c400
# Injury::get_player_injury_status ([0xb63d28]; pessoa) (2 = lesionado); 0x66f4e0 langlib_sprintf(buf, texto,
# 0xad9c64); 0x9466d0 strncpy (11); 0x945b00 fopen, 0x945bfe fread, 0x945ce6 fwrite, 0x945a09 fclose; 0x944cff
# _splitpath(caminho, 0, 0, +0x2e, 0).
# Defeito do original: set_default_name com um tipo fora de 0..8 e 0xff copia o buffer local como a pilha o deixou
# (a copia escreve um nome vazio; o comparador so roda a original nesse caso).
# Comparadores: lado a lado na lista e na contagem, na programacao, no nome, no buffer de texto e no cabecalho; com
# lista de saida (nos novos) e na gravacao do arquivo, so a original com a contagem e os indicadores conferidos.
SAVE = False
TELAS = True
ROTINAS = [
    ('count_players', 0x8b5000, [0x51, 0x8b, 0x44, 0x24, 0x10]),
    ('set_position_training', 0x8b5090, [0x66, 0x8b, 0x41, 0x16, 0x8b, 0x54, 0x24, 0x04]),
    ('set_side_training', 0x8b50c0, [0x66, 0x8b, 0x41, 0x16, 0x8b, 0x54, 0x24, 0x04]),
    ('set_default_name', 0x8b50f0, [0x83, 0xec, 0x64, 0x56, 0x8b, 0xf1]),
    ('load_from_file', 0x8b5220, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x6c]),
    ('save_to_file', 0x8b53d0, [0x55, 0x8b, 0x6c, 0x24, 0x08]),
    ('training_schedule_get_category_name', 0x8b5520, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('training_schedule_get_intensity_name', 0x8b5690, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('training_schedule_check_file', 0x8b57e0, [0x8b, 0x4c, 0x24, 0x04, 0x83, 0xec, 0x6c]),
    ('training_schedule_read_file_header', 0x8b5820, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
]
DOCS = ('uma programação de treino nas telas: os jogadores dela, os bits de posição e de lado, o nome padrão, o arquivo e os nomes',
        "a training schedule on the screens: its players, the position and side bits, the default name, the file and the names",
        [('count_players', 'count_players', '0x8b5000', 'training_schedule.cpp', 'thiscall, ret 0xc',
          'regra: os jogadores da lista na vaga da programação; na Visão geral, todos; em Sem treino, os lesionados; listados quando há lista de saída',
          "rule: the list's players on the schedule's slot; for the Overview, all; for No Training, the injured; listed when there is an output list"),
         ('set_position_training', 'set_position_training', '0x8b5090', 'training_schedule.cpp', 'thiscall, ret 4',
          'regra: os bits de posição (fora de 0xdf80) da palavra de treino trocados; deixa de estar como gravada',
          "rule: the position bits (outside 0xdf80) of the training word replaced; no longer as saved"),
         ('set_side_training', 'set_side_training', '0x8b50c0', 'training_schedule.cpp', 'thiscall, ret 4',
          'regra: os bits de lado (fora de 0xb57f) trocados; deixa de estar como gravada',
          "rule: the side bits (outside 0xb57f) replaced; no longer as saved"),
         ('set_default_name', 'set_default_name', '0x8b50f0', 'training_schedule.cpp', 'thiscall, ret 0',
          'regra: o nome traduzido do tipo (Visão geral, Sem treino, Geral, Físico, Finalização, Técnica, Tática, Goleiros, Sem uso), 11 caracteres; o tipo 8 não muda o nome',
          "rule: the type's translated name (Overview, No Training, General, Fitness, Shooting, Skill, Tactics, Gk, Unused), 11 characters; type 8 leaves the name"),
         ('load_from_file', 'load_from_file', '0x8b5220', 'training_schedule.cpp', 'thiscall, ret 4',
          'regra: o cabeçalho (assinatura 0x200cb, versão 1), os cinco níveis, o nome, o tipo, a palavra de treino e o resto; o nome do arquivo e os dois indicadores de gravada',
          "rule: the header (magic 0x200cb, version 1), the five levels, the name, the type, the training word and the rest; the file's name and both saved flags"),
         ('save_to_file', 'save_to_file', '0x8b53d0', 'training_schedule.cpp', 'thiscall, ret 4',
          'regra: grava os mesmos campos no arquivo e marca a programação como gravada',
          'rule: writes the same fields into the file and marks the schedule as saved'),
         ('training_schedule_get_category_name', 'training_schedule_get_category_name', '0x8b5520', 'training_schedule.cpp', 'cdecl',
          'regra: Físico, Tática, Finalização, Técnica ou Goleiros; buffer abaixo de 20 bytes ou outra categoria: a caixa e 0',
          'rule: Fitness, Tactics, Shooting, Skills or Goalkeeping; a buffer under 20 bytes or another category: the box and 0'),
         ('training_schedule_get_intensity_name', 'training_schedule_get_intensity_name', '0x8b5690', 'training_schedule.cpp', 'cdecl',
          'regra: 0 Nenhuma, 10 Leve, 25 Média, 50 Intensa; outro nível: a caixa e 0',
          'rule: 0 None, 10 Light, 25 Medium, 50 Intensive; another level: the box and 0'),
         ('training_schedule_check_file', 'training_schedule_check_file', '0x8b57e0', 'training_schedule.cpp', 'cdecl',
          'regra: 1 para um arquivo de programação válido, senão 2',
          'rule: 1 for a valid schedule file, otherwise 2'),
         ('training_schedule_read_file_header', 'training_schedule_read_file_header', '0x8b5820', 'training_schedule.cpp', 'cdecl',
          'regra: o cabeçalho de 0x6a bytes lido; 1 com a assinatura 0x200cb e a versão 1',
          'rule: the 0x6a-byte header read; 1 with magic 0x200cb and version 1')])
CAMPOS = []
INTERESSA = []
