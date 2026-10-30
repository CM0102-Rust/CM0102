# Os regens (player_regen.cpp): a nota da pessoa como doadora dos dados de não jogador
# (player_regen_score_non_player_candidate 0x7a7150, pela escolha do doador 0x7a6e60 já em Rust, a cada pessoa não
# jogadora: três sorteios, pow 0x945ff0 e _ftol; o mapa dá 145 bytes, são 434), a ordem da lista de aposentadorias
# (player_regen_compare_retirements 0x7ae2a0, o comparador do qsort da PLAYER_REGEN::process_retirements 0x7a59e0 todo
# dia, que para na primeira entrada futura; também das listas iniciais 0x7a5790/0x7a58f0 e da 0x7a8690) e a ordem da
# fila de regens (player_regen_compare_queue_entries 0x7ae240, do qsort no fim da 0x7a59e0; a 0x7a65d0 cria os regens
# das entradas vencidas). Sem SEH. Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira até 2002-06-01.
# Comparadores: a nota lado a lado pelo estado do sorteio (a cópia sem contar); as duas ordens pelo valor.
SAVE = False
ROTINAS = [('nota_do_doador_de_dados', 0x7a7150, [0x83, 0xec, 0x08, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('ordem_das_aposentadorias', 0x7ae2a0, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('ordem_da_fila_de_regens', 0x7ae240, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('os regens: a nota do doador de dados de não jogador, a ordem das aposentadorias e a ordem da fila de regens',
        'regens: the non-player data donor score, the retirement order and the regen queue order',
        [('nota_do_doador_de_dados', 'player_regen_score_non_player_candidate', '0x7a7150', 'player_regen.cpp', 'stdcall, ret 4',
          '−10000 sem idade (31+) ou marcado; senão sorteio pela idade, a habilidade sobre a reputação do clube, sorteio pelo cubo de +0x2f, os atributos e os quadrados de +0x5b e +0x57, e o sorteio final (+75 sem +0xcf no clube; sem clube, + (+0x22)² e dobra contra um sorteio de 10000)',
          '−10000 when under 31 or flagged; else a draw by age, ability over the club reputation, a draw by the cube of +0x2f, the attributes and the squares of +0x5b and +0x57, and the final draw (+75 without club +0xcf; without a club, + (+0x22)² and doubled against a 10000 draw)'),
         ('ordem_das_aposentadorias', 'player_regen_compare_retirements', '0x7ae2a0', 'player_regen.cpp', 'cdecl',
          'comparador do qsort da lista de aposentadorias: pela data que vale (a do anúncio +8 se ainda não passou, senão a da aposentadoria +0x10), ano e dia; no empate pelo id',
          'qsort comparator of the retirement list: by the effective date (the announcement +8 if not past yet, else the retirement +0x10), year and day; on a tie by id'),
         ('ordem_da_fila_de_regens', 'player_regen_compare_queue_entries', '0x7ae240', 'player_regen.cpp', 'cdecl',
          'comparador do qsort da fila de regens: pelo ano (+6), pelo dia (+4) e pelo id (+0)',
          'qsort comparator of the regen queue: by year (+6), day (+4) and id (+0)')])
CAMPOS = [('pessoa +0x18, +0x22, +0x56..+0x5d, +0x69: idade, um byte sem sinal, atributos (bytes com sinal) e a marca que exclui; valor; lê (nota_do_doador_de_dados)',
           'person +0x18, +0x22, +0x56..+0x5d, +0x69: age, an unsigned byte, attributes (signed bytes) and the excluding flag; value; reads (nota_do_doador_de_dados)'),
          ('pessoa +0x39, +0x61: clube e jogador; ponteiro; lê (nota_do_doador_de_dados)',
           'person +0x39, +0x61: club and player; pointer; reads (nota_do_doador_de_dados)'),
          ('jogador +0xb, +0xd, +0x2f: habilidade atual e potencial (com sinal) e um byte com sinal; valor; lê (nota_do_doador_de_dados)',
           'player +0xb, +0xd, +0x2f: current and potential ability (signed) and a signed byte; value; reads (nota_do_doador_de_dados)'),
          ('clube +0x80, +0xcf: reputação e um ponteiro que dá +75 quando falta; valor e ponteiro; lê (nota_do_doador_de_dados)',
           'club +0x80, +0xcf: reputation and a pointer that gives +75 when missing; value and pointer; reads (nota_do_doador_de_dados)'),
          ('[0xae2d0c] registro de 79 bytes por pessoa, +0xb bits 0x30; valor; lê (nota_do_doador_de_dados)',
           '[0xae2d0c] 79-byte record per person, +0xb bits 0x30; value; reads (nota_do_doador_de_dados)'),
          ('aposentadoria (0x1a bytes) +0, +8, +0x10: id, data do anúncio e data da aposentadoria (dia com sinal, ano sem sinal); id e valor; lê (ordem_das_aposentadorias)',
           'retirement (0x1a bytes) +0, +8, +0x10: id, announcement date and retirement date (signed day, unsigned year); id and value; reads (ordem_das_aposentadorias)'),
          ('[0xae2c90], [0xae2c92]: dia e ano do jogo; valor; lê (ordem_das_aposentadorias)',
           '[0xae2c90], [0xae2c92]: game day and year; value; reads (ordem_das_aposentadorias)'),
          ('fila de regens (0xc bytes) +0, +4, +6: id, dia e ano; id e valor; lê (ordem_da_fila_de_regens)',
           'regen queue (0xc bytes) +0, +4, +6: id, day and year; id and value; reads (ordem_da_fila_de_regens)')]
INTERESSA = ['GPF2', 'CMRTE']
