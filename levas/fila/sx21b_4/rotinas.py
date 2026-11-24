# Os construtores do gestor de jogadores virtuais, do gestor de treino e de uma programacao de treino
# (virtual_staff.cpp, training_manager.cpp; leitor sx21b, rodada 5, 24/11/2026). Rodam na criacao do jogo (a carreira
# de teste cria o jogo do zero a cada execucao): os dois gestores uma vez cada, a programacao 167 vezes (o iterador de
# vetor do construtor do gestor de treino) e nas telas de treino (0x8b989b).
# Convencoes conferidas: 0x90d140 thiscall ret 0 (devolve this; quadro SEH); 0x8b0ec0 thiscall(caminho) ret 4
# (devolve this; quadro SEH); 0x8b4e10 thiscall ret 0 (devolve this). 0x9463af eh vector constructor iterator
# stdcall(this, 0x9b, 0xa7, 0x8b4e10, 0x5a53c0); 0x8b10d0 load_training_dat thiscall(caminho); 0x8b3c50
# init_player_entries e 0x8b3d40 set_default_schedules (Rust, pelo endereco); 0x8bba10 a iniciacao do cabecalho do
# arquivo (+0x18 da programacao).
# A checagem do CD (wait_for_game_cd, comum aos dois construtores): o cm0102-1080 a desliga trocando o jne do tipo de
# unidade (0x90d1a6 e 0x8b0f34, nop) e o teste do arquivo achado (0x90d1e9 e 0x8b0f76, xor); a copia le esses bytes,
# como a do historico de clubes. O laco percorre as unidades com um byte com sinal (simplificacao: so passa de 32
# unidades).
# Comparadores: o gestor virtual, a copia num objeto de rascunho comparado nos 0x6a bytes; o gestor de treino so a
# original (aloca e carrega arquivos) com o plano conferido no jogo novo; a programacao lado a lado nos 0x9b bytes.
SAVE = False
ROTINAS = [
    ('construct_virtual_staff', 0x90d140, [0x6a, 0xff, 0x68, 0x5e, 0x55, 0x96, 0x00]),
    ('construct_training_manager', 0x8b0ec0, [0x6a, 0xff, 0x68, 0x5c, 0x38, 0x96, 0x00]),
    ('construct_training_schedule', 0x8b4e10, [0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x18]),
]
DOCS = ('os construtores do gestor de jogadores virtuais, do gestor de treino e de uma programação de treino',
        'the constructors of the virtual staff manager, the training manager and a training schedule',
        [('construct_virtual_staff', 'construct_virtual_staff', '0x90d140', 'virtual_staff.cpp', 'thiscall, ret 0',
          'regra: a lista de partidas vazia, o CD conferido, os mínimos de cada grupo de posição (clube: 4, 4, 4, 8, 4, 8, 4, 8; seleção B: 6, 4, 4, 12, 4, 8, 4, 8; clube de fora das ligas: a metade) e as 17 vagas de clube visto vazias',
          'rule: the fixture list empty, the CD checked, the minimum of each position group (club: 4, 4, 4, 8, 4, 8, 4, 8; B national team: 6, 4, 4, 12, 4, 8, 4, 8; club outside the leagues: half) and the 17 viewed club slots empty'),
         ('construct_training_manager', 'construct_training_manager', '0x8b0ec0', 'training_manager.cpp', 'thiscall, ret 4',
          'regra: as 167 programações construídas, o CD conferido; com o training.dat carregado do jogo salvo, só isso; num jogo novo, as entradas de treino dos jogadores, as 96 vagas de treinador vazias, as programações padrão e a semana 0',
          "rule: the 167 schedules built, the CD checked; with the saved game's training.dat loaded, nothing else; in a new game, the players' training entries, the 96 coach slots empty, the default schedules and week 0"),
         ('construct_training_schedule', 'construct_training_schedule', '0x8b4e10', 'training_manager.cpp', 'thiscall, ret 0',
          'regra: o cabeçalho de arquivo (assinatura 0x200cb, versão 1) e os indicadores zerados',
          'rule: the file header (magic 0x200cb, version 1) and the flags cleared')])
CAMPOS = []
INTERESSA = []
