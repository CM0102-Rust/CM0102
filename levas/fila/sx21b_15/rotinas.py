# A marca de formato no fim de um arquivo de dados (utils.cpp, a pasta si\code; leitor sx21b, rodada 5, 24/11/2026).
# Roda na partida do jogo (o indice 0x603c89, o arquivo de lingua 0x66ed02 e o banco de dados 0x545025) e nas telas
# (taticas 0x5b5e24/0x5b8654, busca de jogadores 0x7b2ce3). Item de simulacao.
# Convencoes conferidas: 0x90c140 cdecl(arquivo) (eax = a marca ou 0xf3fffff3). Chamadas: 0x946aae ftell, 0x946a22
# fseek(arquivo, 4, SEEK_END) (4 bytes alem do fim, nao antes: num arquivo comum a leitura falha e a resposta e
# 0xf3fffff3, como na original), 0x945bfe fread(&marca, 4, 1), fseek de volta (SEEK_SET). A original le a marca para
# o espaco do proprio argumento na pilha; a copia, para uma local.
# Comparador: a original devolve o arquivo a posicao dele, entao a copia roda depois e os valores sao comparados.
SAVE = False
ROTINAS = [
    ('utils_read_file_format_marker', 0x90c140, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
]
DOCS = ('a marca de formato no fim de um arquivo de dados',
        'the format marker at the end of a data file',
        [('utils_read_file_format_marker', 'utils_read_file_format_marker', '0x90c140', 'utils.cpp', 'cdecl',
          'regra: a marca de 4 bytes lida (0xf1fffff1 ou 0xf2fffff2) ou 0xf3fffff3; a posição do arquivo volta ao que era',
          'rule: the 4-byte marker read (0xf1fffff1 or 0xf2fffff2) or 0xf3fffff3; the file position goes back to where it was')])
CAMPOS = []
INTERESSA = []
