# Arquivos e pastas: o arquivo existe, a pasta existe e a pasta do usuario (utils.cpp, a pasta si\code; leitor sx21b,
# rodada 5, 24/11/2026). Rodam na partida do jogo, que a carreira de teste faz a cada execucao: utils_file_exists na
# carga do banco de dados e do jogo (Database 0x53595a.., Setup 0x81c076..), utils_is_directory no Setup (0x81b8a2) e
# na pasta do usuario, utils_get_user_folder no game_config (0x5d7597) e no human_manager (0x5fb8d8..). Item de
# simulacao.
# Convencoes conferidas: as tres cdecl: 0x90c120(caminho) (1/0: _access 0x9478d8 com modo 0); 0x90bf90(caminho) (1/0;
# sem caminho ou vazio: a caixa, linhas 0x29d e 0x2a4; 0x944e46 malloc, 0x9475b5 _stat com o modo em +6 da struct,
# 0x9452ca free); 0x90bea0(nome, saida) (1/0; 0x946da0 _getcwd(buf, 0x104), 0x945b13 _makepath(saida, 0, cwd, nome,
# 0), 0x945bd2 _mkdir, a caixa da linha 0x28c).
# Defeito do original: utils_is_directory aloca o tamanho do caminho mais 1 e, depois de uma letra de unidade ("C:"),
# acrescenta a barra um byte alem do bloco.
# Comparadores: pelo valor (utils_is_directory sem caminho ou vazio: so a original); a pasta do usuario: a original
# primeiro (pode criar a pasta) e, se deu 1, a copia sobre o mesmo buffer, com o caminho comparado.
# Sem mutacao: as tres decidem se o jogo acha os seus arquivos e pastas na partida; uma mutacao trava a carreira de
# teste no modo Rust (a triagem das mutacoes) ou abre a caixa de erro. A prova e o comparador na carreira comparando.
SAVE = False
ROTINAS = [
    ('utils_file_exists', 0x90c120, [0x8b, 0x44, 0x24, 0x04, 0x6a, 0x00]),
    ('utils_is_directory', 0x90bf90, [0x81, 0xec, 0x24, 0x02, 0x00, 0x00]),
    ('utils_get_user_folder', 0x90bea0, [0x81, 0xec, 0x04, 0x03, 0x00, 0x00]),
]
DOCS = ('arquivos e pastas: o arquivo existe, a pasta existe e a pasta do usuário',
        "files and folders: a file exists, a folder exists and the user's folder",
        [('utils_file_exists', 'utils_file_exists', '0x90c120', 'utils.cpp', 'cdecl',
          'regra: 1 quando o _access com modo 0 acha o arquivo',
          'rule: 1 when _access with mode 0 finds the file'),
         ('utils_is_directory', 'utils_is_directory', '0x90bf90', 'utils.cpp', 'cdecl',
          'regra: 1 quando o _stat acha o caminho com o bit de pasta (sem a barra final; com barra depois de uma letra de unidade)',
          'rule: 1 when _stat finds the path with the folder bit (without a final backslash; with one after a drive letter)'),
         ('utils_get_user_folder', 'utils_get_user_folder', '0x90bea0', 'utils.cpp', 'cdecl',
          'regra: a pasta debaixo da pasta atual; 1 se já existe ou foi criada, senão a caixa e 0',
          'rule: the folder under the current folder; 1 when it exists or was created, otherwise the box and 0')])
CAMPOS = []
INTERESSA = []
