# A partida do jogo: a semente do sorteio, as linhas dos arquivos de configuracao e o corte dos caracteres de controle
# (utils.cpp, a pasta si\code; leitor sx21b, rodada 5, 24/11/2026). Rodam na partida do jogo, que a carreira de teste
# faz a cada execucao: a semente (0x5cc6b2, 0x81b99e: a opcao -seed ou o relogio), as linhas de configuracao (o
# auto_setup.cfg, os .cfg de eventos, de clima e das ligas americanas; 20 chamadores) e o corte (game_config e Setup).
# Item de simulacao.
# Convencoes conferidas: as tres cdecl: 0x90d0a0(semente) (eax = o cursor sorteado); 0x90cba0(arquivo, buffer) (1 ou
# 0; tabela de 4 saltos em 0x90cc24 com o indice de bytes em 0x90cc34 para os caracteres 9..0x23); 0x90bb70(texto)
# (sem texto: a caixa da linha 0x18f). Chamadas: 0x946ebb time(0); 0x9469fa srand; 0x946a04 rand; 0x944c9f sprintf;
# 0x9469e0 fgetc. A tabela de sorteio do rng.rs: o cursor em 0xde1f60 (de 0xaa2454 a 0xad4130), o deslocamento em
# 0xde1f5c, o fim em 0xde2798.
# Comparadores: a semente lado a lado (o estado do sorteio com a semente do CRT, o fim da tabela e o texto); a leitura
# das linhas so a original (o arquivo anda); o corte lado a lado no texto.
SAVE = False
ROTINAS = [
    ('utils_seed_random', 0x90d0a0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
    ('utils_read_config_line', 0x90cba0, [0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
    ('utils_trim_control_chars', 0x90bb70, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
]
DOCS = ('a partida do jogo: a semente do sorteio, as linhas de configuração e o corte dos caracteres de controle',
        "the game's start: the random seed, the config lines and trimming the control characters",
        [('utils_seed_random', 'utils_seed_random', '0x90d0a0', 'utils.cpp', 'cdecl',
          'regra: a semente dada (ou o relógio, com 0) no srand, registrada no texto; o cursor da tabela de sorteio num ponto sorteado (rand % 0xc738 entradas), o fim da tabela e o deslocamento (rand % 0xffff)',
          'rule: the given seed (or the clock, for 0) into srand, logged into the text; the random table cursor at a drawn point (rand % 0xc738 entries), the table end and the offset (rand % 0xffff)'),
         ('utils_read_config_line', 'utils_read_config_line', '0x90cba0', 'utils.cpp', 'cdecl',
          'regra: a próxima linha não vazia, sem os brancos do começo; # começa um comentário até o fim da linha',
          'rule: the next non-empty line, without the leading blanks; # starts a comment to the end of the line'),
         ('utils_trim_control_chars', 'utils_trim_control_chars', '0x90bb70', 'utils.cpp', 'cdecl',
          'regra: tira os caracteres abaixo de 0x20 do começo e do fim do texto',
          'rule: removes the characters below 0x20 from the start and the end of the text')])
CAMPOS = []
INTERESSA = []
