# Os caminhos de arquivo do jogo: a pasta de uma chave (CM3_DATA, CM3_TEMP...), o caminho inteiro e a copia limitada
# a 256 bytes (utils.cpp, a pasta si\code; leitor sx21b, rodada 5, 24/11/2026). Rodam desde a partida do jogo (os
# dados, as preferencias) e na carreira (os arquivos temporarios das copas e do historico: utils_build_file_path tem
# dezenas de chamadores). Item de simulacao.
# Convencoes conferidas: as tres cdecl; 0x90bde0(chave, nome) devolve o buffer 0xde279c; 0x90bc50(chave) devolve a
# pasta ou 0; 0x90cf20(caminho, saida). Chamadas: 0x945a60 strstr(linha de comando 0xae2c98, "-ignoreenv");
# 0x947480 getenv; 0x951c50 stricmp; 0x9466d0 strncpy (os ultimos 8 caracteres de um caminho longo).
# Comparadores: a pasta pelo valor; o buffer do caminho (0x104 bytes) e a saida de 256 lado a lado.
SAVE = False
ROTINAS = [
    ('utils_get_folder_for_key', 0x90bc50, [0x56, 0x57, 0x68, 0x90, 0x41, 0xad, 0x00]),
    ('utils_build_file_path', 0x90bde0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x01, 0x00, 0x00]),
    ('utils_copy_path_limited', 0x90cf20, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
]
DOCS = ('os caminhos de arquivo do jogo: a pasta de uma chave, o caminho inteiro e a cópia limitada',
        "the game's file paths: a key's folder, the full path and the limited copy",
        [('utils_get_folder_for_key', 'utils_get_folder_for_key', '0x90bc50', 'utils.cpp', 'cdecl',
          'regra: a variável de ambiente da chave (salvo com -ignoreenv na linha de comando); senão a pasta padrão: data para SI_DATA, CM3_DATA e CM3_TACTICS; a do jogo para CM3_TEMP, CM3_DEBUG e CM3_PREFS; training, history, pictures, sounds, help e qstart para as outras; chave desconhecida, nada',
          'rule: the environment variable of the key (unless -ignoreenv is on the command line); otherwise the default folder: data for SI_DATA, CM3_DATA and CM3_TACTICS; the game folder for CM3_TEMP, CM3_DEBUG and CM3_PREFS; training, history, pictures, sounds, help and qstart for the others; an unknown key, none'),
         ('utils_build_file_path', 'utils_build_file_path', '0x90bde0', 'utils.cpp', 'cdecl',
          'regra: a pasta da chave e uma barra invertida (se a pasta não for vazia) e o nome, no buffer comum',
          "rule: the key's folder and a backslash (when the folder is not empty) and the name, in the shared buffer"),
         ('utils_copy_path_limited', 'utils_copy_path_limited', '0x90cf20', 'utils.cpp', 'cdecl',
          'regra: o caminho inteiro se tiver menos de 256 caracteres; senão sem os espaços e, chegando a 255, cortado com os últimos 8 caracteres no fim',
          'rule: the whole path when under 256 characters; otherwise without spaces and, reaching 255, cut with the last 8 characters at the end')])
CAMPOS = []
INTERESSA = []
