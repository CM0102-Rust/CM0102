# As regras das quatro divisões inglesas (o slot set_up_rules de eng_prm.cpp, eng_first.cpp, eng_second.cpp e
# eng_third.cpp; Agente B da frente das competições, 03/11/2026). Cada uma grava os bytes de regra da liga e monta a
# tabela de rodadas da fase principal pela virtual +0x3c (create_schedule). Rodam uma vez cada, na criação da
# carreira (o construtor da liga as chama direto: 0x573406 mov ecx, esi; call 0x574b70 -> thiscall, ret 0; o mesmo
# nas outras três), na carreira de 8 ligas.
# Iguais no cm0102-1080.exe (inteiras). Não chamam nenhuma rotina por endereço: só a virtual +0x3c (create_schedule,
# 0x573660/0x571980/0x5757a0/0x5770e0), como a original: push 0; push &+0x3a; push &+0xa9; push -1; mov ecx, esi;
# call [eax+0x3c] -> thiscall, ret 0x10; o resultado vai para +0xba.
# Comparador: ensaio (a virtual aloca a tabela de rodadas): a original roda inteira, a faixa +0x1c..+0xec volta, a
# cópia grava as regras sem a virtual e cada campo gravado tem de sair igual ao da original; no fim fica a original.
# Determinismo: só constantes, globais de id ([0x9cf5bc]...) e o bit da nação; nada de pilha, heap nem endereço.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('regras_da_premier', 0x574b70, [0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00]),
           ('regras_da_first', 0x572d90, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('regras_da_second', 0x576780, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('regras_da_third', 0x5780c0, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x03])]
DOCS = ('as regras das quatro divisões inglesas: turnos, pontos, acesso, mata-mata, rebaixamento e desempates',
 'the rules of the four English divisions: rounds, points, promotion, play-offs, relegation and tie-breaks',
 [('regras_da_premier', 'EngPrm::set_up_rules', '0x574b70', 'eng_prm.cpp', 'thiscall, ret 0',
   'a Premier (20 clubes): 2 turnos, 3 pontos pela vitória e 1 pelo empate; ninguém sobe, os 3 últimos caem direto '
   'para a First; desempates saldo, gols pró e posição [inferência pelos códigos 1, 2, 3]; monta a tabela de '
   'rodadas',
   'the Premier (20 clubs): 2 rounds, 3 points for a win and 1 for a draw; nobody goes up, the bottom 3 go straight '
   'down to the First; tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; builds '
   'the round schedule'),
  ('regras_da_first', 'EngFirst::set_up_rules', '0x572d90', 'eng_first.cpp', 'thiscall, ret 0',
   'a First (24): os 2 primeiros sobem direto, do 3º ao 6º vão ao mata-mata de acesso, os 3 últimos caem para a '
   'Second; 5 reservas e 3 substituições; 2 turnos, 3 pontos',
   'the First (24): the top 2 go up directly, 3rd to 6th go to the promotion play-offs, the bottom 3 go down to the '
   'Second; 5 substitutes and 3 substitutions; 2 rounds, 3 points'),
  ('regras_da_second', 'EngSecond::set_up_rules', '0x576780', 'eng_second.cpp', 'thiscall, ret 0',
   'a Second (24): 2 sobem direto, do 3º ao 6º ao mata-mata, os 4 últimos caem para a Third',
   'the Second (24): 2 go up directly, 3rd to 6th to the play-offs, the bottom 4 go down to the Third'),
  ('regras_da_third', 'EngThird::set_up_rules', '0x5780c0', 'eng_third.cpp', 'thiscall, ret 0',
   'a Third (24): 3 sobem direto, do 4º ao 7º ao mata-mata, o último cai para a Conference quando a nação tem as '
   'divisões de baixo carregadas [inferência pelo bit 4 da nação] (senão não há liga abaixo)',
   'the Third (24): 3 go up directly, 4th to 7th to the play-offs, the bottom club goes down to the Conference when '
   'the nation has its lower divisions loaded [inference from nation bit 4] (otherwise there is no league below)')])
CAMPOS = [('liga +0x3c turnos; +0xc2/+0xc3 pontos por vitória/empate; +0xc4 categoria das partidas (2 = liga); '
           '+0xc5..+0xc7 desempates; valor; grava (as quatro)',
           'league +0x3c rounds; +0xc2/+0xc3 points per win/draw; +0xc4 match category (2 = league); +0xc5..+0xc7 '
           'tie-breaks; value; writes (all four)'),
          ('liga +0xbe/+0xbf/+0xc0/+0xc1 vagas de acesso direto, mata-mata de acesso, play-off de permanência e '
           'rebaixamento direto; valor; grava (as quatro)',
           'league +0xbe/+0xbf/+0xc0/+0xc1 direct promotion, promotion play-off, relegation play-off and direct '
           'relegation places; value; writes (all four)'),
          ('liga +0x1c/+0x20 a liga de cima/de baixo; id (−1 nenhuma); grava (as quatro)',
           'league +0x1c/+0x20 the league above/below; id (−1 none); writes (all four)'),
          ('liga +0x42 tipo (1 clubes nacionais), +0x49 reservas, +0x4a substituições, +0x52 [sem nome], +0xab/+0xdb '
           'palavras copiadas nas partidas; valor; grava',
           'league +0x42 type (1 national clubs), +0x49 substitutes, +0x4a substitutions, +0x52 [unnamed], '
           '+0xab/+0xdb words copied into matches; value; writes'),
          ('liga +0xba tabela de rodadas, +0xa9 rodadas, +0x3a código da fase; ponteiro e valor; grava pela virtual '
           '+0x3c (as quatro)',
           'league +0xba round schedule, +0xa9 rounds, +0x3a stage code; pointer and value; writes through virtual '
           '+0x3c (all four)'),
          ('nação +0x11c bit 4 as divisões de baixo carregadas [inferência]; valor; lê (regras_da_third)',
           'nation +0x11c bit 4 lower divisions loaded [inference]; value; reads (regras_da_third)')]
INTERESSA = ['CMRTE']
