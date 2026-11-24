# Utilidades: trocar o conteudo de dois buffers, copiar um arquivo e o texto da versao (utils.cpp, a pasta si\code;
# leitor sx21b, rodada 5, 24/11/2026). utils_swap_bytes nas ordenacoes da formacao, das taticas e das listas da
# interface (0x5b30xx, 0x89749f, 0x7fc6xx; 25 chamadores); utils_copy_file na carga do banco de dados (0x5286b1) e
# das estatisticas (0x7c5516); utils_get_version_string em toda caixa de erro (e so nelas: na carreira limpa nao
# roda). Item de simulacao.
# Convencoes conferidas: as tres cdecl: 0x90ba90(a, b, n) (sem a ou b: a caixa, linhas 0x16b e 0x172); 0x90cc50(de,
# para) (1/0; 0x945b00 fopen, 0x946a22 fseek, 0x946aae ftell, 0x944e46 malloc, 0x945bfe fread, 0x945ce6 fwrite,
# 0x9452ca free, 0x945a09 fclose); 0x90d130() (eax = 0xad4394, "3.9.68"; prologo de 6 bytes, mov eax, imm32).
# Defeito do original: utils_copy_file entra em laco sem fim se o arquivo de origem render menos que o tamanho medido.
# Comparadores: os dois buffers lado a lado; a copia de arquivo so a original (grava o arquivo); a versao pelo valor.
SAVE = False
ROTINAS = [
    ('utils_swap_bytes', 0x90ba90, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('utils_copy_file', 0x90cc50, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x01, 0x00, 0x00]),
    ('utils_get_version_string', 0x90d130, [0xb8, 0x94, 0x43, 0xad, 0x00]),
]
DOCS = ('utilidades: trocar dois buffers, copiar um arquivo e o texto da versão',
        'utilities: swapping two buffers, copying a file and the version text',
        [('utils_swap_bytes', 'utils_swap_bytes', '0x90ba90', 'utils.cpp', 'cdecl',
          'regra: troca os n bytes dos dois buffers; buffer nulo mostra a caixa',
          'rule: swaps the n bytes of the two buffers; a null buffer shows the box'),
         ('utils_copy_file', 'utils_copy_file', '0x90cc50', 'utils.cpp', 'cdecl',
          'regra: copia o arquivo em blocos do tamanho dele (metade de cada vez, até 256 bytes, se faltar memória); 1 se os dois abriram',
          'rule: copies the file in blocks of its size (halved down to 256 bytes when memory is short); 1 when both opened'),
         ('utils_get_version_string', 'utils_get_version_string', '0x90d130', 'utils.cpp', 'cdecl',
          'regra: o texto "3.9.68"',
          'rule: the text "3.9.68"')])
CAMPOS = []
INTERESSA = []
