# Os inscritos das copas inglesas: a Copa da Liga, a segunda chave do Troféu da Liga (Auto Windscreens) e os dois clubes
# da Supercopa (eng_cc_cup.cpp, eng_auto_cup.cpp, eng_charity.cpp). Frente das competições (Agente D, 04/11/2026),
# carreira de 8 ligas.
# - 0x56ccc0 (EngCcCup::collect_entrants, 1135 bytes; 1 chamada até 2002-06-01 com as 8 ligas, na atualização do
#   primeiro dia): os 92 clubes das quatro divisões e a classe de cada um. Uma mutação.
# - 0x56b3c0 (EngAutoCup::collect_entrants, 225 bytes; 1 chamada, no construtor): os 28 da segunda chave. Uma mutação.
# - 0x56d780 (EngCharity::select_teams, 559 bytes; 1 chamada, no construtor): o campeão inglês e o da Copa da
#   Inglaterra. Uma mutação.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são thiscall, ret 0 (mov ecx, copa; sem push; devolvem −1, 0 ou 1 em
# eax). Dentro: 0x944e46 (push tamanho; add esp, 4 -> cdecl), 0x945333 (o qsort: push 0x5221c0; push 6; push n; push
# lista; add esp, 0x10 -> cdecl), 0x4afd80 (push lugar; push competição; add esp, 8 -> cdecl) e a caixa (aviso_em).
# Comparadores: lado a lado com o +0x3e, o +0xb2 (a Copa da Liga) e o [0xb67a34]; a original deixa a lista dela em +0xa7
# e a cópia faz a sua, comparada nos bytes inscritos (6 por clube; na Supercopa os 12 só com a devolução 1) e liberada.
# O qsort chama o 0x5221c0 (já em Rust) pelo desvio.
# Determinismo: nada lê pilha, heap ou endereço como valor (o qsort ordena por classe e reputação, como na original).
# Provas (04-05/11/2026, cópia fora do projeto: motor.rs da principal com a 273a publicada e a 274a integrada + os 9
# itens cql_10..12, cqm_10..12, cqn_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO, nenhuma caixa (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01 também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 17dfd306919e4965; todos os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base 426fa70704cbf326): as 27 dos 9 itens ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqm10_copa_da_liga_inscritos', 0x56ccc0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqm10_trofeu_inscritos', 0x56b3c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqm10_supercopa_clubes', 0x56d780, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os inscritos das copas inglesas: a Copa da Liga, a segunda chave do Troféu da Liga e os dois clubes da Supercopa',
        'the English cups\' entrants: the League Cup, the second section of the League Trophy and the two Charity Shield clubs',
        [('cqm10_copa_da_liga_inscritos', 'EngCcCup::collect_entrants', '0x56ccc0', 'eng_cc_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Copa da Liga os 92 clubes das quatro divisões inglesas, na ordem Third, Second, First e Premier (fora das copas europeias), e depois os das copas europeias (contados em +0xb2, no máximo 12: são os que entram na 3ª rodada); a classe de cada um ordena a lista (qsort, a menor primeiro, no empate a menor reputação): 0 na Third e na Second, na First 0x46 − posição para quem caiu da Premier e 0x28 − posição para quem ficou, na Premier 0x78 − posição para quem ficou e 0x64 − posição para quem subiu, 0x78 nas copas europeias; depois da ordem a classe vira 1 (Premier), 2 (First) ou 3',
          'rule: the League Cup takes the 92 clubs of the four English divisions, in the order Third, Second, First and Premier (outside the European cups), then those in European cups (counted in +0xb2, at most 12: they enter in the 3rd round); each one\'s class orders the list (qsort, smallest first, ties by smallest reputation): 0 in the Third and Second, in the First 0x46 − position for clubs relegated from the Premier and 0x28 − position for the others, in the Premier 0x78 − position for clubs that stayed and 0x64 − position for promoted ones, 0x78 for the European clubs; after sorting the class becomes 1 (Premier), 2 (First) or 3'),
         ('cqm10_trofeu_inscritos', 'EngAutoCup::collect_entrants', '0x56b3c0', 'eng_auto_cup.cpp', 'thiscall, ret 0',
          'regra: o Troféu da Liga (Auto Windscreens) tem duas chaves de 28 clubes; a segunda leva os 28 últimos dos 56 escolhidos (o vetor em +0xb2, já ordenado pela latitude e pela reputação: a chave do sul [inferência]), na ordem, classe e situação 0',
          'rule: the League Trophy (Auto Windscreens) has two 28-club sections; the second takes the last 28 of the 56 chosen clubs (the array at +0xb2, already sorted by latitude and reputation: the southern section [inference]), in order, class and status 0'),
         ('cqm10_supercopa_clubes', 'EngCharity::select_teams', '0x56d780', 'eng_charity.cpp', 'thiscall, ret 0',
          'regra: a Supercopa inglesa (Charity Shield) é entre o campeão da Premier e o da Copa da Inglaterra; se o mesmo clube ganhou as duas, entra o vice da Premier; sem algum deles, a caixa de erro',
          'rule: the Charity Shield is between the Premier champion and the FA Cup winner; if the same club won both, the Premier runner-up takes its place; if one is missing, the error box')])
CAMPOS = [('clube +0x57 divisão (competição: +0 id), +0x5b a da temporada anterior, +0x5f a posição nela (byte) [inferência], +0x1db competição europeia (−1 nenhuma); ponteiro/valor; lê (cqm10_copa_da_liga_inscritos)',
           'club +0x57 division (competition: +0 id), +0x5b the previous season\'s one, +0x5f the position in it (byte) [inference], +0x1db European competition (−1 none); pointer/value; reads (cqm10_copa_da_liga_inscritos)'),
          ('copa +0x3e inscritos, +0xa7 lista dos inscritos (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (as três)',
           'cup +0x3e entrants, +0xa7 entrant list (6 bytes: club, +4 class, +5 status); value/pointer; writes (all three)'),
          ('Copa da Liga +0xb2 clubes nas copas europeias (byte, no máximo 12); valor; grava (cqm10_copa_da_liga_inscritos)',
           'League Cup +0xb2 clubs in European cups (byte, at most 12); value; writes (cqm10_copa_da_liga_inscritos)'),
          ('Troféu da Liga +0xb2 vetor dos 56 clubes escolhidos (ponteiros); ponteiro; lê (cqm10_trofeu_inscritos)',
           'League Trophy +0xb2 array of the 56 chosen clubs (pointers); pointer; reads (cqm10_trofeu_inscritos)'),
          ('[0x9cf5bc] Premier, [0x9cf5c0] First, [0x9cf5c4] Second, [0x9cf5c8] Third, [0x9cf74c] Copa da Inglaterra; [0xae23d0] competições do banco (0x6b bytes); [0xae23b4]/[0xae235c] clubes; id/ponteiro; lê',
           '[0x9cf5bc] Premier, [0x9cf5c0] First, [0x9cf5c4] Second, [0x9cf5c8] Third, [0x9cf74c] FA Cup; [0xae23d0] database competitions (0x6b bytes); [0xae23b4]/[0xae235c] clubs; id/pointer; reads')]
INTERESSA = ['CMRTE']
