# As supercopas e o Jogo das Estrelas: os clubes da Supercopa da Espanha e do Japão e o calendário do Jogo das Estrelas
# da MLS (spa_super.cpp, jap_super.cpp, usa_mls_all_stars.cpp). Frente das competições (Agente D, 04/11/2026), carreira de
# 8 ligas (Espanha, Japão e EUA jogáveis).
# - 0x858aa0 (SpaSuper::select_teams, 557 bytes; 1 chamada até 2002-06-01 com as 8 ligas, no construtor): o campeão da
#   Copa do Rei e o da Liga. Uma mutação. É a rotina da Supercopa inglesa e da francesa (cqm_10, cqm_11), com a troca
#   na primeira vaga (o vice da Copa do Rei) em vez da segunda.
# - 0x666160 (JapSuper::select_teams, 890 bytes; 2 chamadas: o construtor e a virada da temporada japonesa): o campeão
#   da J1 e o da Copa do Imperador, com o vice no lugar de clube sem nação. Uma mutação.
# - 0x90a400 (UsaMlsAllStars::create_round_table, 307 bytes; 2 chamadas): a rodada única do Jogo das Estrelas. Uma
#   mutação.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): 0x858aa0 e 0x666160 são thiscall, ret 0 (mov ecx, copa; sem push; devolvem
# −1, 0 ou 1); 0x90a400 é a virtual +0x3c (push saida; push &extra; push &rodadas; push fase; mov ecx -> thiscall, ret
# 0x10; a fase lida como byte). Dentro: 0x944e46 (cdecl), 0x4afd80 (push lugar; push competição; add esp, 8 -> cdecl),
# 0x521e60 (7 pushes; add esp, 0x1c -> cdecl), 0x521eb0 (9 pushes; add esp, 0x24 -> cdecl) e a caixa (aviso_em).
# Comparadores: os das supercopas lado a lado com o +0x3e e o [0xb67a34] e a lista de cada uma (a da cópia comparada nos
# 12 bytes com a devolução 1 e liberada); o da tabela lado a lado com as saídas e os bytes gravados (+0..+0x10,
# +0x13..+0x22, +0x5c..+0x67), a tabela da cópia liberada; fora da fase −1 só a original.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (04-05/11/2026, cópia fora do projeto: motor.rs da principal com a 273a publicada e a 274a integrada + os 9
# itens cql_10..12, cqm_10..12, cqn_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO, nenhuma caixa (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01 também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 17dfd306919e4965; todos os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base 426fa70704cbf326): as 27 dos 9 itens ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqm12_supercopa_da_espanha_clubes', 0x858aa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqm12_supercopa_do_japao_clubes', 0x666160, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqm12_jogo_das_estrelas_tabela', 0x90a400, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as supercopas e o Jogo das Estrelas: os clubes da Supercopa da Espanha e do Japão e o calendário do Jogo das Estrelas da MLS',
        'the super cups and the All-Star game: the Spanish and Japanese Super Cup clubs and the MLS All-Star game calendar',
        [('cqm12_supercopa_da_espanha_clubes', 'SpaSuper::select_teams', '0x858aa0', 'spa_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Espanha é entre o campeão da Copa do Rei (o mandante da ida [inferência]) e o campeão da Liga; se o mesmo clube ganhou as duas, entra o vice da Copa do Rei; sem algum deles, a caixa de erro',
          'rule: the Spanish Super Cup is between the Copa del Rey winner (home in the first leg [inference]) and the League champion; if the same club won both, the Copa del Rey runner-up takes its place; if one is missing, the error box'),
         ('cqm12_supercopa_do_japao_clubes', 'JapSuper::select_teams', '0x666160', 'jap_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa do Japão é entre o campeão da J1 e o da Copa do Imperador; um campeão sem nação (+0x53 = 0, clube que não existe mais [inferência]) dá a vaga ao vice; se é o mesmo clube, entra o vice da Copa do Imperador e, se ainda coincidir, o vice da J1; sem algum deles, a caixa de erro',
          'rule: the Japanese Super Cup is between the J1 champion and the Emperor\'s Cup winner; a winner with no nation (+0x53 = 0, a club that no longer exists [inference]) gives the place to the runner-up; if it is the same club, the Emperor\'s Cup runner-up comes in and, if they still coincide, the J1 runner-up; if one is missing, the error box'),
         ('cqm12_jogo_das_estrelas_tabela', 'UsaMlsAllStars::create_round_table', '0x90a400', 'usa_mls_all_stars.cpp', 'thiscall, ret 0x10',
          'regra: o Jogo das Estrelas da MLS é um jogo único entre as duas seleções das conferências, sorteado em 15/6 e jogado em 2/8 (levado ao dia da semana 6) na casa do mandante emprestada [inferência: mando 0], sem sorteio de chave (+0xd = 0x20 [inferência]); categoria 8; sem prêmio',
          'rule: the MLS All-Star game is a single match between the two conference selections, drawn on 15/6 and played on 2/8 (moved to weekday 6) at the borrowed home ground [inference: venue 0], with no bracket draw (+0xd = 0x20 [inference]); category 8; no prize')])
CAMPOS = [('clube +0x53 nação (0: sem nação); ponteiro; lê (cqm12_supercopa_do_japao_clubes)', 'club +0x53 nation (0: none); pointer; reads (cqm12_supercopa_do_japao_clubes)'),
          ('copa +0x3e inscritos, +0xa7 lista dos inscritos (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (as duas supercopas)',
           'cup +0x3e entrants, +0xa7 entrant list (6 bytes: club, +4 class, +5 status); value/pointer; writes (both super cups)'),
          ('copa +0x40 ano; rodada de copa (0x68 bytes, ver cql_10); valor; lê e grava (cqm12_jogo_das_estrelas_tabela)',
           'cup +0x40 year; cup round (0x68 bytes, see cql_10); value; reads and writes (cqm12_jogo_das_estrelas_tabela)'),
          ('[0x9cf740] Copa do Rei, [0x9cf624] Liga espanhola, [0x9cf65c] J1, [0x9cf660] Copa do Imperador; [0xae23d0] competições do banco (0x6b bytes); id/ponteiro; lê',
           '[0x9cf740] Copa del Rey, [0x9cf624] Spanish League, [0x9cf65c] J1, [0x9cf660] Emperor\'s Cup; [0xae23d0] database competitions (0x6b bytes); id/pointer; reads')]
INTERESSA = ['CMRTE']
