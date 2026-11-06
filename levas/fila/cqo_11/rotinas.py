# Os inscritos das copas italianas: a Copa da Itália, a Copa da Série C e os dois clubes da Supercopa (o papel de
# collect_entrants; ita_cup.cpp, ita_c_cup.cpp, ita_super.cpp). Frente das competições (Agente D, 05/11/2026), carreira
# de 8 ligas (a Itália jogável).
# - 0x63faf0 (ItaCup::select_entrants, 1039 bytes; 1 chamada até 2002-06-01 com as 8 ligas, no construtor): os 48
#   inscritos, os 32 dos grupos repartidos em potes sorteados. Uma mutação.
# - 0x63dfe0 (ItaCCup::select_entrants, 653 bytes; 1 chamada): os 90 inscritos da Copa da Série C, pela longitude e
#   embaralhados em dezenas. Uma mutação.
# - 0x65fa60 (ItaSuper::init_team_list, 559 bytes; 1 chamada): o campeão da Série A e o da Copa da Itália. Uma mutação.
# Reaproveita do motor.rs os ajudantes das supercopas da 276a (cqm12_lista, cqm12_colocado, cqm12_vaga, cqm12_fim,
# cqm12_comparar): a Supercopa da Itália é a da Espanha com a troca na segunda vaga.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são thiscall, ret 0 (mov ecx, copa; call; sem push), chamadas pelo
# construtor e pela virada da temporada. Dentro: 0x944e46 (push tamanho; add esp, 4 -> cdecl), 0x90cfc0 (push 8; add
# esp, 4 -> cdecl), 0x945333 (o qsort: 4 pushes; add esp, 0x10 -> cdecl; com 0x640590 e 0x53fe50, originais, chamadas
# pelo qsort), 0x4afb40 (push n; push vetor; add esp junto -> cdecl, 2 argumentos; já em Rust), 0x4afd80 (pela
# cqm12_colocado: push lugar; push competição; add esp, 8 -> cdecl) e a caixa (aviso_em).
# Comparadores: lado a lado com o +0xf2 (ou o +0x3e), o [0xb67a34] e o estado do sorteador (a cópia sorteia sem
# contar); a original deixa a lista dela em +0xee (ou +0xa7), a cópia faz a sua, comparada nos inscritos que gravou e
# liberada.
# Determinismo: o vetor local dos 32 da Copa da Itália é preenchido inteiro antes de ser lido (as passadas só param com
# 32); a lista da Copa da Série C é lida nos 80 primeiros, gravados pela passada (90 clubes na C1 e na C2).
# Provas (05-06/11/2026, cópia fora do projeto: motor.rs da principal com a 277a publicada e a 278a integrada + os 12
# itens cqo_10..13, cqp_10..13, cqq_10..13, 39 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 (parou em 2001-273): 0 linhas DIVERGENCIA fora
#   da cqo11_copa_c_inscritos (8 dezenas embaralhadas, não 9: corrigida e conferida de novo até 2001-01-05, 0
#   divergências), 0 EXCECAO, nenhuma caixa;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a; os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base f29c85195c2a534f): as três ACUSADAS (os potes da
#   Copa da Itália em 2001-223, as dezenas da Série C em 2001-001, o vice da Série A na Supercopa em 2001-231).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqo11_copa_da_italia_inscritos', 0x63faf0, [0x81, 0xec, 0x8c, 0x02, 0x00, 0x00]),
           ('cqo11_copa_c_inscritos', 0x63dfe0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqo11_supercopa_clubes', 0x65fa60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os inscritos das copas italianas: a Copa da Itália, a Copa da Série C e os dois clubes da Supercopa',
        'the Italian cups\' entrants: the Coppa Italia, the Serie C Cup and the two Super Cup clubs',
        [('cqo11_copa_da_italia_inscritos', 'ItaCup::select_entrants', '0x63faf0', 'ita_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Copa da Itália 48 clubes italianos. Os 32 da fase de grupos: os da Série B da temporada, os que vieram da Série B em 4º, 5º ou do 18º para baixo e os que vieram da C1 em 4º, 5º ou 6º (posições contadas de 0 [inferência]); faltando, os da C1 pela posição (6º, 7º...). Os 32 são ordenados (qsort 0x640590) e repartidos em 4 potes de 8, e cada uma das 8 voltas sorteia um clube de cada pote. Na 2ª rodada entram os 3 primeiros da Série B anterior (os que subiram) e os da Série A do 10º ao 15º; por último os 9 primeiros da Série A, até completar 48 (os que passam disso ficam de fora pela ordem da tabela)',
          'rule: 48 Italian clubs enter the Coppa Italia. The 32 of the group stage: the season\'s Serie B clubs, those that came from Serie B in 4th, 5th or from 18th down and those that came from C1 in 4th, 5th or 6th (positions counted from 0 [inference]); if short, C1 clubs by position (6th, 7th...). The 32 are sorted (qsort 0x640590) and split into 4 pots of 8, and each of the 8 rounds draws one club from each pot. In the 2nd round enter the top 3 of last season\'s Serie B (the promoted) and the Serie A clubs from 10th to 15th; last, the top 9 of Serie A, up to 48 (the rest are left out by table order)'),
         ('cqo11_copa_c_inscritos', 'ItaCCup::select_entrants', '0x63dfe0', 'ita_c_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Copa da Série C os 90 clubes italianos da C1 e da C2 (as duas chaves da C1 e as três da C2). Os que vieram da Série B (rebaixados) e os que vieram da C1 em 4º, 5º ou 6º vão para as últimas vagas (a partir da 81ª); os 80 primeiros são embaralhados, ordenados pela longitude da cidade e cada uma das suas 8 dezenas embaralhada (as 10 últimas vagas ficam na ordem): os grupos são regionais [inferência]',
          'rule: the Serie C Cup takes the 90 Italian clubs of C1 and C2 (the two C1 groups and the three C2 ones). Those that came down from Serie B and those that came from C1 in 4th, 5th or 6th go to the last places (from the 81st); the first 80 are shuffled, sorted by the city\'s longitude and each of their 8 tens shuffled (the last 10 places stay in order): the groups are regional [inference]'),
         ('cqo11_supercopa_clubes', 'ItaSuper::init_team_list', '0x65fa60', 'ita_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Itália é entre o campeão da Série A (o mandante) e o campeão da Copa da Itália; se o mesmo clube ganhou as duas, entra o vice da Copa',
          'rule: the Italian Super Cup is between the Serie A champion (the home side) and the Coppa Italia winner; if the same club won both, the Cup runner-up plays')])
CAMPOS = [('clube +0x53 nação, +0x57 divisão (ponteiro para a competição; +0 o id), +0x5b divisão da temporada anterior [inferência], +0x5f posição na temporada anterior (byte com sinal, de 0) [inferência]; ponteiro/valor; lê',
           'club +0x53 nation, +0x57 division (pointer to the competition; +0 the id), +0x5b previous season\'s division [inference], +0x5f previous season\'s position (signed byte, from 0) [inference]; pointer/value; reads'),
          ('copa +0xee lista dos inscritos (ponteiros de clube), +0xf2 máximo (dword); ponteiro/valor; grava (Copa da Itália, Copa da Série C)',
           'cup +0xee entrant list (club pointers), +0xf2 maximum (dword); pointer/value; writes (Coppa Italia, Serie C Cup)'),
          ('copa +0x3e inscritos, +0xa7 lista (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (Supercopa)',
           'cup +0x3e entrants, +0xa7 list (6 bytes: club, +4 class, +5 status); value/pointer; writes (Super Cup)'),
          ('[0x9cf360] a Itália, [0x9cf570] Série A, [0x9cf574] Série B, [0x9cf578]/[0x9cf57c] C1, [0x9cf580]/[0x9cf584]/[0x9cf588] C2, [0x9cf71c] Copa da Itália; [0xae23b4] clubes, [0xae235c]/[0xae2350] contas; id/ponteiro; lê',
           '[0x9cf360] Italy, [0x9cf570] Serie A, [0x9cf574] Serie B, [0x9cf578]/[0x9cf57c] C1, [0x9cf580]/[0x9cf584]/[0x9cf588] C2, [0x9cf71c] Coppa Italia; [0xae23b4] clubs, [0xae235c]/[0xae2350] counts; id/pointer; reads')]
INTERESSA = ['CMRTE', 'GPF2']
