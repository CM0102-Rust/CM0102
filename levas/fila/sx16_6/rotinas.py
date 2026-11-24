# os.cpp (leitor sx16, rodada 5, 24/11/2026): as medidas da maquina (memoria, processador, disco).
# Convencoes conferidas na desmontagem (cdecl; as funcoes do Windows pela tabela de importacao do exe, stdcall):
# - 0x7a16d0 () -> eax: GlobalMemoryStatus [0x9670c4] (so dwLength = 0x20 antes); dwTotalPhys com sinal / 2^20 (cdq,
#   and 0xfffff, sar 0x14). Chamada por Config::set_defaults 0x5d7bc0 (0x5d7bcb).
# - 0x7a1710 () -> eax: (dwAvailPhys + dwAvailPageFile) >> 20 (shr). Chamada ~140 vezes: a criacao do jogo
#   (create_game 0x828110 e o carregamento 0x82b..0x82c.., uma por modulo), o turno do humano (0x5cdebc), o menu
#   (0x76218f), o inicio de jogo novo (0x81ce13...).
# - 0x7a1750 () -> al: 0x90bde0 utils_build_file_path cdecl("CM3_DATA", "cpuinf32.dll"); sprintf(0xde1f64, "%s");
#   LoadLibraryA [0x9670b4]; GetProcAddress [0x9670b8] "cpuspeed" (sem teste); cpuspeed cdecl(&local, 0) -> ponteiro
#   (+0, +4, +8 a velocidade); FreeLibrary [0x9670bc]; GetSystemInfo [0x9670c0] (dwProcessorType +0x18). Chamada so
#   por Config::set_defaults (0x5d7bc4).
# - 0x7a1820 (caminho) -> eax: GetVersionExA [0x9670c8] (0x94 bytes); KERNEL32.DLL e GetDiskFreeSpaceExA, ou
#   GetDiskFreeSpaceA [0x9670b0] no Windows 95 antigo; as caixas "Error" do os.cpp (linhas 0x13d, 0x14b, 0x158) =
#   show_error_box. Chamada ~140 vezes, as mesmas da memoria livre (create_game e o carregamento).
# Tipo: simulacao (as duas de memoria livre e disco rodam na criacao do jogo de toda carreira; as de memoria total e
# processador so no Config::set_defaults, quando a configuracao e criada ou o Reset da caixa Game Settings).
# Comparadores: a memoria total igual; a memoria livre, o disco e a classe do processador podem mudar entre duas
# leituras: a original, a copia e a original de novo, e a copia tem de dar uma das duas.
# Defeito do original: o GetProcAddress de "cpuspeed" nao e testado (uma cpuinf32.dll sem a funcao derruba o jogo).
SAVE = False
ROTINAS = [('os_get_total_physical_memory_mb', 0x7a16d0, [0x83, 0xec, 0x20, 0xb8, 0x20, 0x00, 0x00, 0x00]),
           ('os_get_free_memory_mb', 0x7a1710, [0x83, 0xec, 0x20, 0xb8, 0x20, 0x00, 0x00, 0x00]),
           ('os_get_cpu_speed_class', 0x7a1750, [0x83, 0xec, 0x44, 0x53, 0x55]),
           ('os_get_free_disk_space_mb', 0x7a1820, [0x81, 0xec, 0xbc, 0x02, 0x00, 0x00])]
DOCS = ('as medidas da máquina: memória, processador e disco',
        "the machine checks: memory, CPU and disk",
        [('os_get_total_physical_memory_mb', 'os_get_total_physical_memory_mb', '0x7a16d0', 'os.cpp', 'cdecl',
          'a memória física total em MB (GlobalMemoryStatus)',
          'the total physical memory in MB (GlobalMemoryStatus)'),
         ('os_get_free_memory_mb', 'os_get_free_memory_mb', '0x7a1710', 'os.cpp', 'cdecl',
          'a memória livre em MB: a física livre mais o arquivo de paginação livre',
          'the free memory in MB: the free physical memory plus the free page file'),
         ('os_get_cpu_speed_class', 'os_get_cpu_speed_class', '0x7a1750', 'os.cpp', 'cdecl',
          'a classe de velocidade do processador: a velocidade pela cpuinf32.dll dos dados (150 MHz sem ela); um 586 acima de 300 MHz é rápido (0), até 200 lento (2), entre eles 1; outro processador, lento',
          'the CPU speed class: the speed from the data folder\'s cpuinf32.dll (150 MHz without it); a 586 above 300 MHz is fast (0), up to 200 slow (2), in between 1; any other processor, slow'),
         ('os_get_free_disk_space_mb', 'os_get_free_disk_space_mb', '0x7a1820', 'os.cpp', 'cdecl',
          'o espaço livre em disco de um caminho em MB: GetDiskFreeSpaceExA (4096 com 4 GB ou mais) ou, no Windows 95 antigo, GetDiskFreeSpaceA; 150 quando não consegue medir (com a caixa de erro do os.cpp), 0 noutra plataforma',
          'the free disk space of a path in MB: GetDiskFreeSpaceExA (4096 with 4 GB or more) or, on the old Windows 95, GetDiskFreeSpaceA; 150 when it cannot measure (with the os.cpp error box), 0 on another platform')])
