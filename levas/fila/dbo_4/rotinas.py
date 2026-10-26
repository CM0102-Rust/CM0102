# Três consultas do Database.cpp que o olheiro usa no relatório de um clube (PLAYER_SEARCH::run_scout_day 0x7b3e20,
# o único chamador; as respostas vão como parâmetros da notícia do relatório, 0x785790): o clube tem atacante rápido
# e dois meias criativos, e a faixa de força do elenco (0..6) vista pelo clube do olheiro. Leitura pura do elenco com
# a lesão pela INJURY 0x62c400; a faixa ordena um vetor local pelo qsort da CRT 0x945333 com a comparação 0x549c30
# (já em Rust; chamada pelo endereço do jogo) e só lê as casas que escreveu (as primeiras min(n − 1, 16) de n). Sem
# SEH, sem sorteio, sem caixa; os contadores e a soma são zerados na entrada.
SAVE = False
ROTINAS = [('clube_tem_atacante_rapido', 0x549770, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('clube_tem_meias_criativos', 0x549940, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('faixa_de_forca_do_elenco', 0x549a30, [0x81, 0xec, 0xcc, 0x00, 0x00, 0x00])]
DOCS = ('o olheiro: o clube tem atacante rápido e dois meias criativos, e a faixa de força do elenco',
        'the scout: the club has a fast striker and two creative midfielders, and the squad strength band',
        [('clube_tem_atacante_rapido', 'club_has_fast_striker', '0x549770', 'Database.cpp', 'cdecl',
          '1 se algum atacante (+0x15 = 20) sem lesão grave passa do limite de habilidade (reputação/75 + 25, de 40 a 135) com o atributo +0x36 acima de reputação/500 + 5 (no máximo 17)',
          '1 when some striker (+0x15 = 20) without serious injury is above the ability limit (reputation/75 + 25, 40 to 135) with attribute +0x36 above reputation/500 + 5 (at most 17)'),
         ('clube_tem_meias_criativos', 'club_has_two_creative_midfielders', '0x549940', 'Database.cpp', 'cdecl',
          '1 se mais de 1 meia sem lesão grave passa do limite com os atributos +0x43 e +0x37 acima de reputação/500 + 5 (no máximo 14)',
          '1 when more than 1 midfielder without serious injury is above the limit with attributes +0x43 and +0x37 above reputation/500 + 5 (at most 14)'),
         ('faixa_de_forca_do_elenco', 'club_squad_strength_band', '0x549a30', 'Database.cpp', 'cdecl',
          'a faixa 0..6 pela média da habilidade dos melhores do elenco (até 16, qsort com 0x549c30) ou, sem eles, pela reputação; ajustada pela reputação do clube do olheiro e, na mesma competição, pela diferença de reputação e pela tabela [0xae2d14]; clube nulo: 3',
          'the 0..6 band from the average ability of the best in the squad (up to 16, qsort with 0x549c30) or, without them, from the reputation; adjusted by the scout club reputation and, in the same competition, by the reputation gap and the [0xae2d14] table; null club: 3')])
CAMPOS = [('TClub +0x57: competição; ponteiro; lê (faixa_de_forca_do_elenco, compara os dois clubes)', 'TClub +0x57: competition; pointer; reads (faixa_de_forca_do_elenco, compares both clubs)'),
          ('TClub +0x80: reputação; curto com sinal; lê (as três)', 'TClub +0x80: reputation; signed short; reads (all three)'),
          ('TClub +0xd7 (50): elenco; ponteiros de pessoa; lê (as três)', 'TClub +0xd7 (50): squad; person pointers; reads (all three)'),
          ('TPlayer +0x05: habilidade atual; curto; +0x12..+0x15: posições; bytes; +0x36, +0x37, +0x43: atributos; bytes com sinal; lê', 'TPlayer +0x05: current ability; short; +0x12..+0x15: positions; bytes; +0x36, +0x37, +0x43: attributes; signed bytes; reads'),
          ('[0xae2d14]: tabela de 6 bytes por id de clube, +4 byte com sinal; lê (faixa_de_forca_do_elenco)', '[0xae2d14]: 6-byte table per club id, +4 signed byte; reads (faixa_de_forca_do_elenco)')]
INTERESSA = ['SGE', 'CMRTE']
