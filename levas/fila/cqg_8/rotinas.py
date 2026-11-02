# A formação lida do arquivo .pct (formation.cpp) e a troca da ordem dos bytes (cash.cpp), pedido de 02/11 (agente C,
# família das pessoas: formation.cpp):
# - 0x5b5d10 (Formation::load_from_disk, 3056 bytes, 876 instruções): lê a formação (senha, criador, máscara, posições
#   gerais e extras, ordem, as casas por fase e faixa, instruções do time e dos jogadores, marcação), acerta as
#   instruções das versões antigas e refaz as posições das 11 vagas pela casa guardada (0x5b3c70 e 0x5b3560, já em
#   Rust). 45 chamadas na carreira automática até 2002-06-01 (cobertura de 02/11): as formações da IA e as
#   pré-definidas na criação das táticas (0x893dc0, com `ofuscado` 1). As mutações estão nela.
# - 0x43f2d0 (swap_bytes_16_array, 40 bytes) e 0x43f300 (swap_bytes_32_array, 69 bytes), cash.cpp: passageiras (só nos
#   arquivos de ordem de bytes trocada; não rodam na carreira).
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x5b5d10: thiscall (mov ecx, [ebx + 0x5f1] = a formação; push 1; push ebp; push caminho em 0x893f72..0x893f8e), 3
#   (ret 0xc: caminho, quieto, ofuscado). Dentro: 0x945b00 fopen cdecl (add esp, 8), 2 (caminho, "rb" 0x98f928);
#   0x90c140 cdecl, 1 (o arquivo; o add esp vem junto do fread); 0x945bfe fread cdecl (add esp, 0x10), 4; 0x43f2d0 /
#   0x43f300 / 0x43f350 cdecl (add esp, 8), 2; 0x5b59a0 thiscall (mov ecx, ebp), 0; 0x5b3c70 thiscall (mov ecx, ebp;
#   push &casa; push 0 (faixa); push 0; push vaga), 4; 0x5b3560 thiscall (mov ecx, ebp; push casa; push 0; push 0; push
#   vaga), 4; 0x944cff _splitpath cdecl (add esp, 0x14), 5 (caminho, 0, 0, a formação, 0); 0x5b7160 thiscall (mov ecx,
#   ebp), 0; 0x945a09 fclose cdecl (add esp, 4), 1; a caixa (aviso_em, formation.cpp 0x9cc3dc).
#   0x43f2d0: cdecl (add esp, 8 em 0x5b61e3), 2; 0x43f300: cdecl (add esp, 8 em 0x5b6620), 2.
# Leitura de pilha: nas versões abaixo de 0x98ec5e a original copia o vetor local das instruções dos jogadores mesmo
#   quando a leitura falha (sobra de pilha só num arquivo truncado); a cópia usa zeros nesse caso. Os textos das
#   mensagens de erro (sprintf num buffer local que não sai dali) não são montados pela cópia.
# Comparadores: os três lado a lado. 0x5b5d10 na formação (0x18e3 bytes) e na devolução (o arquivo é aberto e lido
# pelas duas); 0x43f2d0 e 0x43f300 no vetor.
SAVE = False
ROTINAS = [('ler_formacao_do_disco', 0x5b5d10, [0x81, 0xec, 0x08, 0x04, 0x00, 0x00]),
           ('trocar_bytes_das_palavras', 0x43f2d0, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0xc8]),
           ('trocar_bytes_dos_dwords', 0x43f300, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0xc8])]
DOCS = ('a formação lida do arquivo .pct (as táticas da IA e as do humano) e a troca da ordem dos bytes de palavras e dwords',
        'the formation read from the .pct file (the AI and the human tactics) and the byte order swap of words and dwords',
        [('ler_formacao_do_disco', 'Formation::load_from_disk', '0x5b5d10', 'formation.cpp', 'thiscall',
          'lê a formação de um arquivo .pct (o identificador e a versão, a senha invertida, o criador, as posições gerais e extras, as casas por fase e faixa, as instruções do time e dos jogadores, a marcação), acerta as instruções das versões antigas, refaz as posições das 11 vagas pelas casas e monta o mapa das casas',
          'reads a formation from a .pct file (the identifier and version, the inverted password, the creator, the general and extra positions, the coordinates per phase and zone, the team and player instructions, the marking), fixes the instructions of the old versions, redoes the positions of the 11 slots from the coordinates and builds the coordinate map'),
         ('trocar_bytes_das_palavras', 'swap_bytes_16_array', '0x43f2d0', 'cash.cpp', 'cdecl',
          'troca os dois bytes de cada palavra de um vetor',
          'swaps the two bytes of each word of an array'),
         ('trocar_bytes_dos_dwords', 'swap_bytes_32_array', '0x43f300', 'cash.cpp', 'cdecl',
          'inverte os quatro bytes de cada dword de um vetor',
          'reverses the four bytes of each dword of an array')])
CAMPOS = [('formação +0 o nome (do arquivo), +0x33 senha (50, invertida), +0x65 criador (200), +0x12d posições gerais (11 palavras), +0x143 ordem (11), +0x14e posições extras (11 palavras), +0x164 casas (11 vagas × 2 fases × 3 × 4 faixas, palavras coluna e linha), +0x584 instruções do time (0x2000, 0x4000, 0x20000), +0x5b4 marcação (11), +0x187b máscara, +0x187f/+0x1883/+0x1887 marcas, +0x188b instruções dos jogadores (11 × 8); valor; grava (ler_formacao_do_disco)',
           'formation +0 the name (from the file), +0x33 password (50, inverted), +0x65 creator (200), +0x12d general positions (11 words), +0x143 order (11), +0x14e extra positions (11 words), +0x164 coordinates (11 slots × 2 phases × 3 × 4 zones, column and row words), +0x584 team instructions (0x2000, 0x4000, 0x20000), +0x5b4 marking (11), +0x187b mask, +0x187f/+0x1883/+0x1887 flags, +0x188b player instructions (11 × 8); value; writes (ler_formacao_do_disco)')]
INTERESSA = ['SGE']
