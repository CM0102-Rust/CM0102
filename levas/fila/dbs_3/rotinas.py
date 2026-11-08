# Do Database.cpp (o mapa não dá o arquivo; as vizinhas são dele): as qualidades do jogador (0x546ab0: mídia 0x754a30,
# 0x744240, 0x72c1f0, olheiros, busca e transferências 0x8dbef0, que a carreira percorre 750 vezes) e os defeitos
# (0x5470d0: mídia 0x72c1f0 e 0x487c70). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo
# real comparando. Sem SEH; sorteiam (19 e até 19 vezes, 0x90cfc0 pelo endereço do jogo, alguns com n = −50): os
# comparadores voltam o estado do sorteio (STATE_REGIONS) e a cópia roda em without_counting. Sem variável local lida sem
# valor: as locais e os próprios argumentos que as duas usam de rascunho (floats e bytes) são gravados antes de lidos;
# a cópia não escreve nos argumentos. A 0x5470d0 chama a nota 0x6e6340, que pela 0x62ed70 pode sortear só na primeira
# consulta do jogador (memória fria): divergindo, o comparador refaz uma vez a comparação do mesmo estado e devolve o
# estado do sorteio da primeira. As chamadas (0x543540, 0x7c1e30, 0x8cca10, 0x62c3d0, 0x8972c0, 0x896660, 0x5b2ae0,
# 0x6e6340) pelo endereço do jogo. Pouco quentes (mídia, olheiros, busca, transferências).
SAVE = False
ROTINAS = [('forcas_do_jogador', 0x546ab0, [0x83, 0xec, 0x0c, 0x8b, 0x44, 0x24, 0x14]),
           ('fraquezas_do_jogador', 0x5470d0, [0x83, 0xec, 0x0c, 0x8b, 0x44, 0x24, 0x14])]
DOCS = ('as qualidades e os defeitos do jogador',
        'the player strengths and weaknesses',
        [('forcas_do_jogador', 'staff_get_player_strength_flags', '0x546ab0', 'Database.cpp', 'cdecl',
          'os bits das qualidades do jogador: posições naturais, nibble de [0xae2d0c] e 19 sorteios contra bytes, atributos (0x543540) e médias (0x7c1e30)',
          'the player strength bits: natural positions, the [0xae2d0c] nibble and 19 draws against bytes, attributes (0x543540) and averages (0x7c1e30)'),
         ('fraquezas_do_jogador', 'staff_get_player_weakness_flags', '0x5470d0', 'Database.cpp', 'cdecl',
          'os bits dos defeitos do jogador: posições naturais e sorteios contra bytes, atributos, médias, a palavra 0x62c3d0 e a nota de titular (0x8972c0, 0x5b2ae0, 0x6e6340)',
          'the player weakness bits: natural positions and draws against bytes, attributes, averages, the 0x62c3d0 word and the starter score (0x8972c0, 0x5b2ae0, 0x6e6340)')])
CAMPOS = [('jogador (TStaff +0x61) +5, +0xb reputação, +0xf..+0x19 posições, +0x1c, +0x1e, +0x21, +0x25, +0x28, +0x2e, +0x34, +0x37, +0x39, +0x3c, +0x3e, +0x40, +0x44; TStaff +0x5d; [0xae2d0c] + 79 × id + 0xb (nibble); [0xb74350], [0xde1f54], [0xb63d28], [0xde1d08]; lê (forcas_do_jogador, fraquezas_do_jogador)',
           'player (TStaff +0x61) +5, +0xb reputation, +0xf..+0x19 positions, +0x1c, +0x1e, +0x21, +0x25, +0x28, +0x2e, +0x34, +0x37, +0x39, +0x3c, +0x3e, +0x40, +0x44; TStaff +0x5d; [0xae2d0c] + 79 × id + 0xb (nibble); [0xb74350], [0xde1f54], [0xb63d28], [0xde1d08]; reads (forcas_do_jogador, fraquezas_do_jogador)')]
INTERESSA = ['SGE', 'CMRTE']
