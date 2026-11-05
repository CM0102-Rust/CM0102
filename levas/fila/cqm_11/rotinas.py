# Os inscritos das copas francesas: a Copa da França, a Copa da Liga francesa e os dois clubes da Supercopa (fra_cup.cpp,
# fra_lge_cup.cpp, fra_super.cpp). Frente das competições (Agente D, 04/11/2026), carreira de 8 ligas (a França é uma
# delas).
# - 0x5baa50 (FraCup::collect_entrants, 1012 bytes; 1 chamada até 2002-06-01 com as 8 ligas, no construtor): os 222
#   inscritos. Uma mutação.
# - 0x5bd160 (FraLgeCup::collect_entrants, 692 bytes; 1 chamada, no construtor): os 54 inscritos. Uma mutação.
# - 0x5bf770 (FraSuper::select_teams, 559 bytes; 1 chamada, no construtor): o campeão e o da Copa da França. Uma
#   mutação. É a mesma rotina da Supercopa inglesa (cqm_10) com outros ids e linhas.
# Ajudantes do item: cqm11_passada (uma passada pelos clubes com a condição de cada uma), cqm11_divisao, cqm11_inscrever,
# cqm11_colocado e cqm11_comparar.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são thiscall, ret 0 (mov ecx, copa; sem push; devolvem −1, 0 ou 1).
# Dentro: 0x944e46 (push tamanho; add esp, 4 -> cdecl), 0x53b900 (push clube; add esp, 4 -> cdecl; já em Rust), 0x4afd80
# (push lugar; push competição; add esp, 8 -> cdecl) e a caixa (aviso_em).
# Comparadores: lado a lado com o +0x3e e o [0xb67a34]; a original deixa a lista dela em +0xa7 e a cópia faz a sua,
# comparada nos bytes inscritos (6 por clube; na Supercopa os 12 só com a devolução 1) e liberada.
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
ROTINAS = [('cqm11_copa_da_franca_inscritos', 0x5baa50, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqm11_copa_da_liga_inscritos', 0x5bd160, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqm11_supercopa_clubes', 0x5bf770, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os inscritos das copas francesas: a Copa da França, a Copa da Liga francesa e os dois clubes da Supercopa',
        'the French cups\' entrants: the Coupe de France, the French League Cup and the two Super Cup clubs',
        [('cqm11_copa_da_franca_inscritos', 'FraCup::collect_entrants', '0x5baa50', 'fra_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Copa da França 222 clubes, nesta ordem (a ordem decide a rodada de entrada: os 40 primeiros na 6ª rodada, os 164 seguintes na 7ª e os 18 últimos, a Primeira, nos 64 avos): todos os da divisão [0x9cf5d4] (o National [inferência]); os da [0x9cf764] ligados (+0x60) à [0x9cf5d8] (o CFA [inferência]); os franceses da [0x9cf764] sem ligação; os franceses sem divisão que não são seleções — estes três grupos até 184 inscritos em 2001 e 182 nos outros anos; a Segunda e a Primeira; com a lista incompleta, devolve 0',
          'rule: the Coupe de France takes 222 clubs, in this order (the order decides the entry round: the first 40 in the 6th round, the next 164 in the 7th and the last 18, the First Division, in the last 128): all clubs of division [0x9cf5d4] (the National [inference]); those of [0x9cf764] linked (+0x60) to [0x9cf5d8] (the CFA [inference]); the French ones of [0x9cf764] without a link; the French clubs with no division that are not national teams — these three groups up to 184 entrants in 2001 and 182 in other years; the Second and the First Division; with an incomplete list it returns 0'),
         ('cqm11_copa_da_liga_inscritos', 'FraLgeCup::collect_entrants', '0x5bd160', 'fra_lge_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Copa da Liga francesa 54 clubes, nesta ordem (os 16 primeiros na preliminar, os 20 seguintes na 1ª rodada e os 18 últimos na 2ª): os da divisão [0x9cf5d4] que não vieram da [0x9cf5d8] (até 16 em 2001 e 14 nos outros anos), completados até 16 com os da [0x9cf5d4] que vieram da [0x9cf5d8] (os promovidos [inferência]); a Segunda e a Primeira; com a lista incompleta, devolve 0',
          'rule: the French League Cup takes 54 clubs, in this order (the first 16 in the preliminary round, the next 20 in the 1st round and the last 18 in the 2nd): those of division [0x9cf5d4] that did not come from [0x9cf5d8] (up to 16 in 2001 and 14 in other years), topped up to 16 with those of [0x9cf5d4] that came from [0x9cf5d8] (the promoted ones [inference]); the Second and the First Division; with an incomplete list it returns 0'),
         ('cqm11_supercopa_clubes', 'FraSuper::select_teams', '0x5bf770', 'fra_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da França é entre o campeão da Primeira e o da Copa da França; se o mesmo clube ganhou as duas, entra o vice da Primeira; sem algum deles, a caixa de erro',
          'rule: the French Super Cup is between the First Division champion and the Coupe de France winner; if the same club won both, the First Division runner-up takes its place; if one is missing, the error box')])
CAMPOS = [('clube +0x53 nação, +0x57 divisão, +0x5b a da temporada anterior, +0x60 a competição ligada [inferência] (as três: ponteiro para o registro, +0 o id); ponteiro; lê (cqm11_copa_da_franca_inscritos, cqm11_copa_da_liga_inscritos)',
           'club +0x53 nation, +0x57 division, +0x5b the previous season\'s one, +0x60 the linked competition [inference] (all pointers to the record, +0 the id); pointer; reads (cqm11_copa_da_franca_inscritos, cqm11_copa_da_liga_inscritos)'),
          ('copa +0x3e inscritos, +0xa7 lista dos inscritos (6 bytes: clube, +4 classe, +5 situação), +0x40 ano; valor/ponteiro; grava +0x3e e +0xa7 (as três)',
           'cup +0x3e entrants, +0xa7 entrant list (6 bytes: club, +4 class, +5 status), +0x40 year; value/pointer; writes +0x3e and +0xa7 (all three)'),
          ('[0x9cf5cc] Primeira, [0x9cf5d0] Segunda, [0x9cf5d4], [0x9cf5d8], [0x9cf764] divisões francesas, [0x9cf70c] Copa da França, [0x9cf300] a França; [0xae2350] seleções; id/valor; lê',
           '[0x9cf5cc] First, [0x9cf5d0] Second, [0x9cf5d4], [0x9cf5d8], [0x9cf764] French divisions, [0x9cf70c] Coupe de France, [0x9cf300] France; [0xae2350] national teams; id/value; reads')]
INTERESSA = ['CMRTE']
