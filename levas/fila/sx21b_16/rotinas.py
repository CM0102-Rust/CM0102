# Acrescentar um valor de 4 bytes no fim de um arquivo (utils.cpp, a pasta si\code; leitor sx21b, rodada 5,
# 24/11/2026). So as telas o chamam: gravar uma tatica (formation.cpp 0x5b710b) e gravar um filtro de busca de
# jogadores (player_search.cpp 0x7b3cfb). Tipo TELAS.
# Caminho no jogo: Taticas > Gravar tatica (num arquivo novo); Busca de jogadores > gravar o filtro.
# Convencoes conferidas: 0x90c1a0 cdecl(arquivo, valor) (eax = o que o fwrite devolve). Chamadas: 0x946a22
# fseek(arquivo, 0, SEEK_END); 0x945ce6 fwrite(&valor, 4, 1, arquivo) (a original grava o proprio argumento da pilha).
# Comparador: so a original (grava o arquivo). Sem mutacao: o comparador nao roda a copia.
SAVE = False
TELAS = True
ROTINAS = [
    ('utils_append_u32_to_file', 0x90c1a0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
]
DOCS = ('acrescentar um valor de 4 bytes no fim de um arquivo',
        'appending a four-byte value at the end of a file',
        [('utils_append_u32_to_file', 'utils_append_u32_to_file', '0x90c1a0', 'utils.cpp', 'cdecl',
          'regra: vai ao fim do arquivo e grava os 4 bytes do valor',
          'rule: goes to the end of the file and writes the 4 bytes of the value')])
CAMPOS = []
INTERESSA = []
