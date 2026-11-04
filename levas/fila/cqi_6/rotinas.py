# As regras das divisões italianas (o slot set_up_rules de ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp e
# ita_ser_c1b.cpp; Agente B da frente das competições, 03/11/2026). Gravam os bytes de regra e montam a tabela de
# rodadas pela virtual +0x3c. Rodam uma vez cada, na criação da carreira de 8 ligas (o construtor as chama direto:
# mov ecx, esi; call -> thiscall, ret 0, como na cqi_4).
# Iguais no cm0102-1080.exe (inteiras). Não chamam rotina por endereço: só a virtual +0x3c (push 0; push &+0x3a;
# push &+0xa9; push -1; mov ecx, esi; call [edx+0x3c] -> thiscall, ret 0x10; o resultado em +0xba).
# Comparador: ensaio (a virtual aloca): a original inteira, a faixa +0x1c..+0xec volta, a cópia grava sem a virtual e
# os campos gravados têm de sair iguais; no fim fica a original.
# Determinismo: constantes, globais de id, o bit da nação e o byte +0xd9 como está; nada de pilha, heap nem endereço.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('regras_da_serie_a', 0x641640, [0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00]),
           ('regras_da_serie_b', 0x6468a0, [0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00]),
           ('regras_da_c1a', 0x64b770, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03]),
           ('regras_da_c1b', 0x64f920, [0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00])]
DOCS = ('as regras das divisões italianas: turnos, pontos, acesso, play-off, play-out, rebaixamento e o jogo-desempate',
 'the rules of the Italian divisions: rounds, points, promotion, play-off, play-out, relegation and the tie-break '
 'match',
 [('regras_da_serie_a', 'ItaSerA::set_up_rules', '0x641640', 'ita_ser_a.cpp', 'thiscall, ret 0',
   'a Série A (18 clubes): 2 turnos, 3 pontos pela vitória e 1 pelo empate, ninguém sobe, os 4 últimos caem para a '
   'B; 7 reservas e 3 substituições; desempates 1, 2 e 4 [códigos; inferência: saldo, gols pró, confronto direto]; '
   'o empate em pontos na fronteira de uma vaga vai ao jogo-desempate (+0xe6); ordem fixa das rodadas',
   'Serie A (18 clubs): 2 rounds, 3 points for a win and 1 for a draw, nobody goes up, the bottom 4 go down to B; 7 '
   'substitutes and 3 substitutions; tie-breaks 1, 2 and 4 [codes; inference: goal difference, goals for, head to '
   'head]; a points tie across a place border goes to a tie-break match (+0xe6); fixed round order'),
  ('regras_da_serie_b', 'ItaSerB::set_up_rules', '0x6468a0', 'ita_ser_b.cpp', 'thiscall, ret 0',
   'a Série B (20): os 4 primeiros sobem para a A, os 4 últimos caem para a C1; o resto como a A (jogo-desempate '
   'também)',
   'Serie B (20): the top 4 go up to A, the bottom 4 go down to C1; the rest as in A (tie-break match too)'),
  ('regras_da_c1a', 'ItaSerC1a::set_up_rules', '0x64b770', 'ita_ser_c1a.cpp', 'thiscall, ret 0',
   'a Série C1/A (18): o campeão sobe para a B, do 2º ao 5º vão ao play-off de acesso, do 14º ao 17º ao play-out de '
   'permanência e o último cai; desempates 1 e 2; a liga de baixo gravada é a C1/B (assim no original)',
   'Serie C1/A (18): the champion goes up to B, 2nd to 5th go to the promotion play-off, 14th to 17th to the '
   'relegation play-out and the last goes down; tie-breaks 1 and 2; the league below stored is C1/B (as in the '
   'original)'),
  ('regras_da_c1b', 'ItaSerC1b::set_up_rules', '0x64f920', 'ita_ser_c1b.cpp', 'thiscall, ret 0',
   'a Série C1/B (18): como a C1/A; abaixo a C2/A quando a nação tem as divisões de baixo carregadas [inferência '
   'pelo bit 4 da nação]',
   'Serie C1/B (18): as C1/A; below it C2/A when the nation has its lower divisions loaded [inference from nation '
   'bit 4]')])
CAMPOS = [('liga +0x3c turnos; +0xc2/+0xc3 pontos; +0xc4 categoria das partidas; +0xc5..+0xc8 desempates; valor; '
           'grava (as quatro)',
           'league +0x3c rounds; +0xc2/+0xc3 points; +0xc4 match category; +0xc5..+0xc8 tie-breaks; value; writes '
           '(all four)'),
          ('liga +0xbe/+0xbf/+0xc0/+0xc1 acesso direto, play-off de acesso, play-out, rebaixamento; valor; grava',
           'league +0xbe/+0xbf/+0xc0/+0xc1 direct promotion, promotion play-off, play-out, relegation; value; '
           'writes'),
          ('liga +0x1c/+0x20 a liga de cima/de baixo; id; grava', 'league +0x1c/+0x20 league above/below; id; writes'),
          ('liga +0xd9 bits de regra (0x40 = ordem fixa das rodadas); valor; lê e grava (as quatro)',
           'league +0xd9 rule bits (0x40 = fixed round order); value; reads and writes (all four)'),
          ('liga +0xe6 o empate na fronteira vai ao jogo-desempate; valor; grava (regras_da_serie_a, '
           'regras_da_serie_b)',
           'league +0xe6 a tie across a border goes to a tie-break match; value; writes (regras_da_serie_a, '
           'regras_da_serie_b)'),
          ('liga +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; valor; grava; +0xba, +0xa9, +0x3a pela virtual +0x3c',
           'league +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; value; writes; +0xba, +0xa9, +0x3a through virtual '
           '+0x3c'),
          ('nação +0x11c bit 4 [inferência: divisões de baixo carregadas]; valor; lê (regras_da_c1b)',
           'nation +0x11c bit 4 [inference: lower divisions loaded]; value; reads (regras_da_c1b)')]
INTERESSA = ['CMRTE']
