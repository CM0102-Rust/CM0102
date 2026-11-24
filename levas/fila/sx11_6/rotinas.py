# game_config.cpp, parte 3 (leitor sx11, rodada 5, 23/11/2026): o destrutor e as opções padrão da configuração.
# Usa config_offsets, config_free_sound_player, compare_config_side_by_side (sx11_3) e config_call0, config_life_fns
# (sx11_4): vai depois dos dois.
# ATENÇÃO, PRÓLOGOS COM INSTRUÇÃO RELATIVA: as duas precisam de entrada em RELATIVAS (rust/src/detours.rs) antes de
# integrar: (0x5d7b70, 3) (cmp dword [ecx], 0; jne curto) e (0x5d7bc0, 4) (push ebx; push esi; mov esi, ecx; call
# 0x7a1750). Sem a entrada o trampolim copia o salto cru e o jogo cai: por isso ficam num item à parte.
# Convenções conferidas na desmontagem (thiscall, ret 0):
# - 0x5d7b70 Config::~Config(): com origem não faz nada (eax sem valor); viva: 0x41bad0, o SoundPlayer (stop_all
#   0x84cb10, ~SoundPlayer 0x84caf0, delete, [0xdd7eec] = 0), 0x5e2cc0 e o salto final para 0x66e9c0 (eax dele).
#   Chamada pela game_fatal_cleanup (0x5cc80a) e pela caixa Game Settings (0x825a6e, 0x825acf).
# - 0x5d7bc0 Config::set_defaults() -> 1: 0x7a1750 (al), 0x7a16d0 (eax, com sinal), [+4] = 0x212552a9, set_option
#   0x5d7c60 com 1 ou 2, 0x20000 e 0x40000 ou 0x10000 e 0x80000/0x40000, e 0x1000000/0x2000000 por 0x84c7f0
#   thiscall(0xdd7ee8) -> al. Chamada pelo Config::Config (0x5d72ea) e pelo Reset da caixa (0x825a12).
# Tipo: telas.
MOTOR = True
TELAS = True
ROTINAS = [('config_destruct', 0x5d7b70, [0x83, 0x39, 0x00, 0x75, 0x44]),
           ('config_set_defaults', 0x5d7bc0, [0x53, 0x56, 0x8b, 0xf1, 0xe8, 0x87, 0x9b, 0x1c, 0x00])]
DOCS = ('as preferências do jogo (3): o destrutor e as opções padrão',
        'the game settings (3): the destructor and the default options',
        [('config_destruct', 'Config::~Config', '0x5d7b70', 'game_config.cpp', 'thiscall',
          'a cópia não faz nada; a configuração viva libera as tabelas de fundo, para e libera o tocador de som, fecha a tela e a biblioteca de idiomas',
          'a copy does nothing; the live settings free the background tables, stop and free the sound player, close the display and the language library'),
         ('config_set_defaults', 'Config::set_defaults', '0x5d7bc0', 'game_config.cpp', 'thiscall',
          'as opções padrão 0x212552a9 ajustadas à máquina: desempenho 1 com processador que não é o mais lento e 63 MB de memória, senão 2; processador rápido 0x20000 e 0x40000, os outros 0x10000 e 0x80000 (o mais lento) ou 0x40000; com impressora instalada 0x1000000, sem 0x2000000; devolve 1',
          'the default options 0x212552a9 tuned to the machine: performance 1 with a CPU that is not the slowest and 63 MB of memory, otherwise 2; fast CPU 0x20000 and 0x40000, the others 0x10000 and 0x80000 (the slowest) or 0x40000; with a printer installed 0x1000000, without 0x2000000; returns 1')])
CAMPOS = []
INTERESSA = []
