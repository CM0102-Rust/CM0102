# As ligas de fundo francesas: a CFA e a divisão de baixo (Agente B da frente das competições, 04/11/2026). Rodam na
# criação da carreira de 8 ligas (docs/cobertura_competicoes.txt: 0x5b91c0 e 0x5bd600 1 vez cada, os destrutores 1
# vez cada; a 0x5ba0a0 não está na lista, mas os dois construtores a chamam direto: o contador "cqn_4 liga de fundo
# francesa" prova):
# - 0x5ba0a0 (FraCfa::set_up_rules, 141 bytes): as regras das duas;
# - 0x5b91c0 (FraCfa::FraCfa, 596 bytes) e 0x5bd600 (FraLower::FraLower, 578 bytes), quadro SEH só no caminho de
#   exceção: os clubes, o sorteio e os grupos;
# - 0x5b9460 (FraCfa::~FraCfa) e 0x5bd870 (FraLower::~FraLower), 176 bytes cada, iguais fora a vtable.
# Iguais no cm0102-1080.exe (inteiras).
# Convenções conferidas nos pontos de chamada da original:
# - 0x682200 (League::League) e 0x682300 (League::~League): mov ecx, esi -> thiscall, ret 0;
# - 0x5ba0a0: mov ecx, esi -> thiscall, ret 0;
# - a virtual +0x3c (create_schedule): push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, esi -> thiscall, ret 0x10;
# - 0x4afb40 (comp_util_shuffle_pointer_array): push quantos (edi, a dupla); push lista; add esp, 8 -> cdecl; sorteia;
# - as tabelas dos grupos (0x5b9fa0, 0x5be230, a 275ª): push lista; mov ecx, esi -> thiscall, ret 4;
# - 0x6835c0 (League::create_fixtures): mov ecx, esi -> thiscall, ret 0;
# - 0x687970 (League::free_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x9452ca (free): push; add esp, 4 -> cdecl;
# - a virtual 0 das fases (o destrutor que solta): push 1; mov ecx, fase -> thiscall, ret 4;
# - a caixa: aviso_em.
# Comparadores: as regras e os construtores alocam (o calendário, as tabelas) e os construtores sorteiam: só a
# original, conferindo os campos das regras e a devolução, a vtable, o ano, a disciplina, o registro e a liga sem
# tabela no fim; os destrutores soltam: só a original.
# Determinismo: a lista dos clubes é um vetor zerado (a original usa um registro local de 72/40 que a carreira enche
# inteiro antes de os grupos lerem); nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('regras_da_cfa', 0x5ba0a0, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('construir_cfa', 0x5b91c0, [0x6a, 0xff, 0x68, 0xab, 0x98, 0x95, 0x00]),
           ('construir_divisao_de_baixo_francesa', 0x5bd600, [0x6a, 0xff, 0x68, 0x3b, 0x9a, 0x95, 0x00]),
           ('destruir_cfa', 0x5b9460, [0x6a, 0xff, 0x68, 0xc8, 0x98, 0x95, 0x00]),
           ('destruir_divisao_de_baixo_francesa', 0x5bd870, [0x6a, 0xff, 0x68, 0x58, 0x9a, 0x95, 0x00])]
DOCS = ('as ligas de fundo francesas: a CFA e a divisão de baixo (os construtores, as regras e os destrutores)',
 'the French background leagues: the CFA and the lower division (constructors, rules and destructors)',
 [('regras_da_cfa', 'FraCfa::set_up_rules', '0x5ba0a0', 'fra_cfa.cpp', 'thiscall, ret 0',
   'as regras das duas ligas de fundo francesas: turno e returno, 3 pontos por vitória e 1 por empate, os desempates '
   '4, 1, 2 [inferência: a posição, o saldo, os gols], nenhuma vaga de acesso, play-off nem rebaixamento, os bits de '
   'regra 0x43 (o campeão, as vagas de cima de acesso e a ordem fixa das rodadas); a liga de cima fica com o id de [0x9cf624] (o mesmo global que '
   'a Segunda espanhola usa para a Primera) [inferência: um engano do original, sem efeito porque não há acesso]',
   'the rules of the two French background leagues: home and away, 3 points a win and 1 a draw, tie-breaks 4, 1, 2 '
   '[inference: position, goal difference, goals], no promotion, play-off or relegation places, rule bits 0x43 (a '
   'champion, top places as promotion and fixed round order); the league above gets the id at [0x9cf624] (the same global the Spanish Segunda uses '
   'for the Primera) [inference: an original slip, harmless since there is no promotion]'),
  ('construir_cfa', 'FraCfa::FraCfa', '0x5b91c0', 'fra_cfa.cpp', 'thiscall, ret 4',
   'a CFA (a quarta divisão francesa, de fundo): até 72 clubes, todos os da primeira e da segunda divisões '
   '[inferência: os times reserva] e até 34 da divisão genérica cuja competição de reserva é a CFA, sorteados em 4 '
   'grupos de 18; cada grupo ganha a tabela e os jogos (no calendário do jogo) e a tabela é solta',
   'the CFA (the French fourth level, background): up to 72 clubs, all of the first and second divisions '
   '[inference: the reserve sides] and up to 34 of the generic division whose reserve competition is the CFA, drawn '
   'into 4 groups of 18; each group gets the table and the fixtures (in the game calendar) and the table is freed'),
  ('construir_divisao_de_baixo_francesa', 'FraLower::FraLower', '0x5bd600', 'fra_lower.cpp', 'thiscall, ret 4',
   'a divisão de baixo francesa (de fundo): até 40 clubes, os do National e até 20 franceses da divisão genérica sem '
   'competição de reserva, sorteados em 2 grupos de 20',
   'the French lower division (background): up to 40 clubs, those of the National and up to 20 French clubs of the '
   'generic division with no reserve competition, drawn into 2 groups of 20'),
  ('destruir_cfa', 'FraCfa::~FraCfa', '0x5b9460', 'fra_cfa.cpp', 'thiscall, ret 0',
   'solta a tabela, o calendário, as fases e o vetor delas e destrói a liga',
   'frees the table, the schedule, the stages and their vector and destroys the league'),
  ('destruir_divisao_de_baixo_francesa', 'FraLower::~FraLower', '0x5bd870', 'fra_lower.cpp', 'thiscall, ret 0',
   'o mesmo para a divisão de baixo', 'the same for the lower division')])
CAMPOS = [('liga +0x3c turnos, +0xc2/+0xc3 pontos, +0xc4 categoria das partidas, +0xc5..+0xc7 desempates, +0xbe..+0xc1 '
           'vagas, +0x1c/+0x20 ligas de cima e de baixo, +0xd9 bits de regra, +0xba calendário, +0x42 tipo; valor; '
           'grava (regras_da_cfa)',
           'league +0x3c rounds, +0xc2/+0xc3 points, +0xc4 match category, +0xc5..+0xc7 tie-breaks, +0xbe..+0xc1 '
           'places, +0x1c/+0x20 leagues above and below, +0xd9 rule bits, +0xba schedule, +0x42 type; value; writes'),
          ('clube +0x57 competição, +0x53 nação, +0x60 competição de reserva; ponteiro; lê (os construtores)',
           'club +0x57 competition, +0x53 nation, +0x60 reserve competition; pointer; reads'),
          ('[0xae2350] [inferência: clubes criados depois]; valor; lê (os construtores)',
           '[0xae2350] [inference: clubs created later]; value; reads')]
INTERESSA = ['CMRTE']
