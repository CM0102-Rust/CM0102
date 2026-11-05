# Os inscritos das copas do Japão e dos EUA: a Copa da Liga japonesa, a Copa dos EUA e a Copa do Imperador (o mesmo
# papel, collect_entrants; jap_j_cup.cpp, usa_open_cup.cpp, jap_emp_cup.cpp). Frente das competições (Agente D,
# 04/11/2026), carreira de 8 ligas (Japão e EUA jogáveis).
# - 0x665a00 (JapJCup::collect_entrants, 443 bytes; 2 chamadas até 2002-06-01 com as 8 ligas: o construtor e a virada
#   da temporada japonesa): os 28 inscritos, a J1 com classe sorteada. Uma mutação.
# - 0x90b530 (UsaOpenCup::collect_entrants, 600 bytes; 2 chamadas): os 32 inscritos. Uma mutação.
# - 0x6624f0 (JapEmpCup::collect_entrants, 1112 bytes; 2 chamadas): os 80 inscritos. Uma mutação.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são thiscall, ret 0 (mov ecx, copa; sem push; devolvem −1, 0 ou 1).
# Dentro: 0x944e46 (push tamanho; add esp, 4 -> cdecl), 0x945501 (o calloc: push 6; push n; add esp junto -> cdecl),
# 0x90cfc0 (push 0x14; add esp, 4 -> cdecl), 0x4afbd0 (push escolhas; push n; push vetor -> cdecl, 3 argumentos; a conta
# num registrador com lixo nos 16 bits altos, lida como palavra: a cópia passa limpo), 0x945333 (o qsort com 0x522190:
# 4 pushes; add esp, 0x10 -> cdecl) e a caixa (aviso_em).
# Comparadores: lado a lado com o +0x3e, o [0xb67a34] e o estado do sorteador (a cópia sorteia sem contar); a original
# deixa a lista dela em +0xa7 e a cópia faz a sua, comparada nos inscritos gravados e liberada.
# Determinismo: os vetores locais da Copa dos EUA e da Copa do Imperador começam com a sobra da pilha na original; a
# cópia começa com zero. Só fariam diferença com menos clubes do que as escolhas (10 por liga de baixo nos EUA; sem
# clube nenhum no Japão), casos em que a original já mostra a caixa de erro.
# Provas (04-05/11/2026, cópia fora do projeto: motor.rs da principal com a 273a publicada e a 274a integrada + os 9
# itens cql_10..12, cqm_10..12, cqn_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO, nenhuma caixa (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01 também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 17dfd306919e4965; todos os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base 426fa70704cbf326): as 27 dos 9 itens ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqn12_copa_da_liga_inscritos', 0x665a00, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqn12_copa_dos_eua_inscritos', 0x90b530, [0x81, 0xec, 0x98, 0x03, 0x00, 0x00]),
           ('cqn12_copa_do_imperador_inscritos', 0x6624f0, [0x81, 0xec, 0xc8, 0x04, 0x00, 0x00])]
DOCS = ('os inscritos das copas do Japão e dos EUA: a Copa da Liga japonesa, a Copa dos EUA e a Copa do Imperador',
        'the Japanese and US cups\' entrants: the J League Cup, the US Open Cup and the Emperor\'s Cup',
        [('cqn12_copa_da_liga_inscritos', 'JapJCup::collect_entrants', '0x665a00', 'jap_j_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Copa da Liga japonesa 28 clubes: os da J2, classe 0, e os da J1, cada um com uma classe sorteada de 0 a 19 (a semente); as 16 inscrições a partir da 11ª vão ao qsort pela classe (no empate a reputação); com a lista incompleta, devolve 0',
          'rule: the J League Cup takes 28 clubs: the J2 ones, class 0, and the J1 ones, each with a class drawn from 0 to 19 (the seed); the 16 entries from the 11th on are sorted by class (ties by reputation); with an incomplete list it returns 0'),
         ('cqn12_copa_dos_eua_inscritos', 'UsaOpenCup::collect_entrants', '0x90b530', 'usa_open_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Copa dos EUA 32 clubes: 10 de cada uma das duas ligas de baixo ([0x9cf594] e [0x9cf598], até 50 olhados em cada uma, ordenados pela reputação e embaralhados entre os melhores), com a classe 2, e todos os da MLS com a classe 1',
          'rule: the US Open Cup takes 32 clubs: 10 from each of the two lower leagues ([0x9cf594] and [0x9cf598], up to 50 looked at in each, sorted by reputation and shuffled among the best), with class 2, and all MLS clubs with class 1'),
         ('cqn12_copa_do_imperador_inscritos', 'JapEmpCup::collect_entrants', '0x6624f0', 'jap_emp_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Copa do Imperador 80 clubes: 47 das divisões regionais ([0x9cf678], ordenadas pela reputação e embaralhadas), 2 da liga universitária ([0x9cf674] [inferência]), 3 da liga de futebol ([0x9cf670] [inferência]) — os sorteados dos clubes de outra divisão japonesa (até 6) não entram —, os da J2 (classe 0) e os da J1 com a classe = a posição da temporada anterior; as 16 últimas inscrições vão ao qsort pela classe; faltando clubes nas divisões de baixo, a caixa de erro',
          'rule: the Emperor\'s Cup takes 80 clubs: 47 from the regional divisions ([0x9cf678], sorted by reputation and shuffled), 2 from the university league ([0x9cf674] [inference]), 3 from the football league ([0x9cf670] [inference]) — those drawn from other Japanese divisions (up to 6) do not enter —, the J2 clubs (class 0) and the J1 clubs with class = last season\'s position; the last 16 entries are sorted by class; if the lower divisions lack clubs, the error box')])
CAMPOS = [('clube +0x53 nação, +0x57 divisão (ponteiro para a competição; +0 o id), +0x5f a posição da temporada anterior (byte) [inferência]; ponteiro/valor; lê',
           'club +0x53 nation, +0x57 division (pointer to the competition; +0 the id), +0x5f the previous season\'s position (byte) [inference]; pointer/value; reads'),
          ('copa +0x3e inscritos, +0xa7 lista dos inscritos (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (as três)',
           'cup +0x3e entrants, +0xa7 entrant list (6 bytes: club, +4 class, +5 status); value/pointer; writes (all three)'),
          ('[0x9cf768] J2, [0x9cf65c] J1, [0x9cf678]/[0x9cf674]/[0x9cf670] divisões japonesas de baixo, [0x9cf36c] o Japão, [0x9cf590] MLS, [0x9cf594]/[0x9cf598] ligas americanas de baixo; [0xae23d0] competições do banco; id/ponteiro; lê',
           '[0x9cf768] J2, [0x9cf65c] J1, [0x9cf678]/[0x9cf674]/[0x9cf670] lower Japanese divisions, [0x9cf36c] Japan, [0x9cf590] MLS, [0x9cf594]/[0x9cf598] lower US leagues; [0xae23d0] database competitions; id/pointer; reads')]
INTERESSA = ['CMRTE', 'GPF2']
