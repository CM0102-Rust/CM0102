# As seleções e o regen na disciplina (national_teams.cpp, discipline.cpp): a competição atual da nação
# (NationalTeams::set_current_competition 0x775220, 182 chamadas na carreira até 2002-06-01, das eliminatórias e finais
# 0x57c710/0x580040/0x9111d0..; a fase final grande em curso fica na frente e a outra vai para a fila; avisa o
# gerenciador dos técnicos 0x6a3770), os clubes com jogo na data (NationalTeams::mark_clubs_with_fixtures 0x7775c0,
# 204, do dia das seleções 0x76b100 e da remarcação 0x7777e0: a tabela de um byte por clube, 1/2 nos jogos de liga e
# 3 nos clubes de nação marcada) e tirar o jogador regenerado da disciplina (Discipline::remove_player 0x556830, 248,
# do regen 0x7abac0). A 0x53b3d0, 0x5ae6a0, 0x5c6710, 0x68d1b0, 0x55e1d0 e 0x556d00 já em Rust; 0x6a3770, 0x7e82e0
# (sem criar, só lê), 0x68d1f0 e calloc originais. Sem SEH, sem sorteio; o único lixo é o alto do dword da fatia que a
# 0x7775c0 empurra (a 0x5abf10 só lê o byte: a cópia passa a fatia limpa). Caixas "Error" pela aviso_em (nação ou
# competição nula; sem memória). Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo
# real comparando. Comparadores: a 0x775220 pelo plano (o aviso não roda duas vezes); a 0x7775c0 lado a lado na tabela
# (na primeira, com o calloc, só a original); a 0x556830 pelo plano (solta suspensões e tira itens da lista).
SAVE = False
ROTINAS = [('definir_competicao_da_selecao', 0x775220, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('marcar_clubes_com_jogo', 0x7775c0, [0xa1, 0x50, 0x23, 0xae, 0x00]),
           ('tirar_jogador_da_disciplina', 0x556830, [0x83, 0xec, 0x08, 0x53, 0x55])]
DOCS = ('as seleções: a competição atual da nação e os clubes com jogo na data; a disciplina: tirar o jogador regenerado',
        "national teams: the nation's current competition and the clubs playing on a date; discipline: removing a regenerated player",
        [('definir_competicao_da_selecao', 'NationalTeams::set_current_competition', '0x775220', 'national_teams.cpp', 'thiscall, ret 8',
          'a competição vira a atual da nação; com uma fase final grande em curso, a nova vai para a fila; com a nova sendo fase final grande, a atual vai para a fila; avisa o gerenciador dos técnicos (0x6a3770)',
          "makes the competition the nation's current one; with a major finals under way, the new one goes to the queue; when the new one is a major finals, the current one goes to the queue; notifies the manager handler (0x6a3770)"),
         ('marcar_clubes_com_jogo', 'NationalTeams::mark_clubs_with_fixtures', '0x7775c0', 'national_teams.cpp', 'thiscall, ret 4',
          'na tabela de um byte por clube, marca os clubes dos jogos não amistosos da data: 1 jogo da liga do clube, 2 outro, 3 clube de nação marcada (entrada sem nação)',
          'in the one-byte-per-club table, marks the clubs of the non-friendly fixtures on the date: 1 the club\'s league game, 2 another, 3 a club of a flagged nation (entry with no nation)'),
         ('tirar_jogador_da_disciplina', 'Discipline::remove_player', '0x556830', 'discipline.cpp', 'thiscall, ret 4',
          'apaga o histórico de suspensões, solta as suspensões e libera as 4 vagas, tira as suspensões guardadas da lista e deixa o registro de 29 bytes livre (0xff, conta 0)',
          'clears the ban history, releases the bans and frees the 4 slots, drops the held bans from the list and leaves the 29-byte record free (0xff, count 0)')])
CAMPOS = [('seleções +4: registros de 110 bytes por nação (+4 a competição atual, −1 sem); id; lê e grava (definir_competicao_da_selecao)',
           'national teams +4: 110-byte records per nation (+4 the current competition, −1 none); id; reads and writes (definir_competicao_da_selecao)'),
          ('seleções +8: a competição na fila por nação (4 bytes); id; grava (definir_competicao_da_selecao)',
           'national teams +8: the queued competition per nation (4 bytes); id; writes (definir_competicao_da_selecao)'),
          ('[0xae23d0] competições (107 bytes, +0 id); [0x9cf78c..0x9cf964] as fases finais grandes; id; lê',
           '[0xae23d0] competitions (107 bytes, +0 id); [0x9cf78c..0x9cf964] the major finals; id; reads'),
          ('seleções +0x24: tabela de um byte por clube (0, 1 jogo da liga, 2 outro, 3 nação marcada; n = [0xae235c] − 2 × [0xae2350]); ponteiro e valor; grava (marcar_clubes_com_jogo)',
           'national teams +0x24: one byte per club (0, 1 league game, 2 other, 3 flagged nation; n = [0xae235c] − 2 × [0xae2350]); pointer and value; writes (marcar_clubes_com_jogo)'),
          ('0xb63df0: 0x21 entradas de 0x48 bytes (+0 nação, +0x14 grupo das listas); nação +0x11c marcada; jogo +0x14 competição, +0x1c/+0x20 clubes; clube +0x53 nação, +0x57 liga, +0x60; lê (marcar_clubes_com_jogo)',
           '0xb63df0: 0x21 entries of 0x48 bytes (+0 nation, +0x14 list group); nation +0x11c flagged; fixture +0x14 competition, +0x1c/+0x20 clubs; club +0x53 nation, +0x57 league, +0x60; reads (marcar_clubes_com_jogo)'),
          ('disciplina +0x14: registro de 29 bytes da pessoa (4 vagas, +0x1c guardadas); grava 0xff e conta 0; +0x1c lista das guardadas; tira (tirar_jogador_da_disciplina)',
           "discipline +0x14: the person's 29-byte record (4 slots, +0x1c held count); writes 0xff and count 0; +0x1c held-ban list; removes (tirar_jogador_da_disciplina)")]
INTERESSA = ['CMRTE', 'SGE', 'GPF2']
