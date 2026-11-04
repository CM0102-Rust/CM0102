# As regras das divisões espanholas (o slot set_up_rules de spa_first.cpp, spa_second.cpp e spa_lower.cpp; Agente B
# da frente das competições, 03/11/2026). Gravam os bytes de regra e montam a tabela de rodadas pela virtual +0x3c.
# Rodam uma vez cada, na criação da carreira de 8 ligas (o construtor as chama direto: mov ecx, esi; call ->
# thiscall, ret 0).
# Iguais no cm0102-1080.exe (inteiras). Não chamam rotina por endereço: só a virtual +0x3c (push 0; push &+0x3a;
# push &+0xa9; push -1; mov ecx, esi; call [edx+0x3c] -> thiscall, ret 0x10; o resultado em +0xba).
# Comparador: ensaio (a virtual aloca): a original inteira, a faixa +0x1c..+0xec volta, a cópia grava sem a virtual e
# os campos da lista têm de sair iguais (o +0xa9 = 38 da Primera fica fora: a virtual o reescreve); no fim fica a
# original.
# Determinismo: constantes, globais de id e o bit da nação; nada de pilha, heap nem endereço.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('regras_da_primera', 0x84fdf0, [0x53, 0x56, 0x8b, 0xf1, 0x57]),
           ('regras_da_segunda', 0x855cc0, [0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00]),
           ('regras_da_divisao_de_baixo', 0x853790, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00])]
DOCS = ('as regras das divisões espanholas: turnos, pontos, acesso, rebaixamento e o confronto direto primeiro nos '
 'desempates',
 'the rules of the Spanish divisions: rounds, points, promotion, relegation and head to head first in the '
 'tie-breaks',
 [('regras_da_primera', 'SpaFirst::set_up_rules', '0x84fdf0', 'spa_first.cpp', 'thiscall, ret 0',
   'a Primera (20 clubes, 38 rodadas): 2 turnos, 3 pontos pela vitória e 1 pelo empate; ninguém sobe, os 3 últimos '
   'caem para a Segunda; desempates 4, 1, 2 e 3 [códigos; inferência: confronto direto, saldo, gols pró, vitórias]; '
   '7 reservas e 3 substituições; ordem fixa das rodadas',
   'La Primera (20 clubs, 38 rounds): 2 rounds, 3 points for a win and 1 for a draw; nobody goes up, the bottom 3 go '
   'down to the Segunda; tie-breaks 4, 1, 2 and 3 [codes; inference: head to head, goal difference, goals for, '
   'wins]; 7 substitutes and 3 substitutions; fixed round order'),
  ('regras_da_segunda', 'SpaSecond::set_up_rules', '0x855cc0', 'spa_second.cpp', 'thiscall, ret 0',
   'a Segunda (22): os 3 primeiros sobem direto, os 4 últimos caem para a Segunda B quando ela está carregada '
   '[inferência pelo bit 4 da nação]; os times B não sobem (marcados pela tabela, 0x855b40)',
   'La Segunda (22): the top 3 go up directly, the bottom 4 go down to Segunda B when it is loaded [inference from '
   'nation bit 4]; B teams cannot go up (marked by the table, 0x855b40)'),
  ('regras_da_divisao_de_baixo', 'SpaLower::set_up_rules', '0x853790', 'spa_lower.cpp', 'thiscall, ret 0',
   'a divisão de baixo (22: times B e os melhores clubes menores pela reputação): 2 sobem direto e 2 vão ao play-off '
   'de acesso, 4 caem; a liga de cima gravada é a Primera (assim no original)',
   'the lower division (22: B teams and the best minor clubs by reputation): 2 go up directly and 2 go to the '
   'promotion play-off, 4 go down; the league above stored is La Primera (as in the original)')])
CAMPOS = [('liga +0x3c turnos; +0xc2/+0xc3 pontos; +0xc4 categoria; +0xc5..+0xc8 desempates; +0xbe..+0xc1 vagas; '
           'valor; grava (as três)',
           'league +0x3c rounds; +0xc2/+0xc3 points; +0xc4 category; +0xc5..+0xc8 tie-breaks; +0xbe..+0xc1 places; '
           'value; writes (all three)'),
          ('liga +0xd9 bits de regra = 0x43; valor; grava (as três)', 'league +0xd9 rule bits = 0x43; value; writes '
           '(all three)'),
          ('liga +0x1c/+0x20 a liga de cima/de baixo; id; grava', 'league +0x1c/+0x20 league above/below; id; writes'),
          ('liga +0xa9 rodadas (38 na Primera, antes da virtual); valor; grava (regras_da_primera)',
           'league +0xa9 rounds (38 in La Primera, before the virtual); value; writes (regras_da_primera)'),
          ('liga +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; valor; grava; +0xba, +0x3a pela virtual +0x3c',
           'league +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; value; writes; +0xba, +0x3a through virtual +0x3c'),
          ('nação +0x11c bit 4 [inferência: divisões de baixo carregadas]; valor; lê (regras_da_segunda)',
           'nation +0x11c bit 4 [inference: lower divisions loaded]; value; reads (regras_da_segunda)')]
INTERESSA = ['CMRTE']
