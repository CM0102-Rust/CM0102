# As regras das três divisões francesas (o slot set_up_rules de fra_first.cpp, fra_second.cpp e fra_third.cpp;
# Agente B da frente das competições, 03/11/2026). Gravam os bytes de regra (o acesso e o rebaixamento mudam com o ano
# da liga: 2001 foi a última temporada com 18 clubes na primeira) e montam a tabela de rodadas pela virtual +0x3c.
# Rodam uma vez cada, na criação da carreira de 8 ligas (o construtor as chama direto: mov ecx, esi; call ->
# thiscall, ret 0).
# Iguais no cm0102-1080.exe (inteiras). Não chamam rotina por endereço: só a virtual +0x3c (push 0; push &+0x3a;
# push &+0xa9; push -1; mov ecx, esi; call [edx+0x3c] -> thiscall, ret 0x10; o resultado em +0xba).
# Comparador: ensaio (a virtual aloca): a original inteira, a faixa +0x1c..+0xec volta, a cópia grava sem a virtual e
# os campos gravados têm de sair iguais; no fim fica a original.
# Determinismo: constantes, o ano da liga e globais de id; nada de pilha, heap nem endereço.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('regras_da_franca_1', 0x5bc410, [0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00]),
           ('regras_da_franca_2', 0x5bf0a0, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('regras_da_franca_3', 0x5c0700, [0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00])]
DOCS = ('as regras das três divisões francesas: turnos, pontos, acesso e rebaixamento pelo ano (a primeira passa a 20 '
 'clubes em 2002) e desempates',
 'the rules of the three French divisions: rounds, points, promotion and relegation by year (the top flight grows '
 'to 20 clubs in 2002) and tie-breaks',
 [('regras_da_franca_1', 'FraFirst::set_up_rules', '0x5bc410', 'fra_first.cpp', 'thiscall, ret 0',
   'a primeira divisão: 2 turnos, 3 pontos pela vitória e 1 pelo empate; ninguém sobe; caem 2 na temporada 2001 '
   '(18 clubes, a liga cresce para 20) e 3 depois; desempates 1, 2 e 3 [códigos]; 5 reservas e 3 substituições; '
   'ordem das rodadas dividida ao meio (+0xd9 = 0x83)',
   'the top division: 2 rounds, 3 points for a win and 1 for a draw; nobody goes up; 2 go down in the 2001 season '
   '(18 clubs, the league grows to 20) and 3 afterwards; tie-breaks 1, 2 and 3 [codes]; 5 substitutes and 3 '
   'substitutions; round order split in halves (+0xd9 = 0x83)'),
  ('regras_da_franca_2', 'FraSecond::set_up_rules', '0x5bf0a0', 'fra_second.cpp', 'thiscall, ret 0',
   'a segunda: sobem 4 e caem 2 em 2001, depois sobem 3 e caem 3; o resto como a primeira',
   'the second: 4 go up and 2 go down in 2001, then 3 up and 3 down; the rest as the top division'),
  ('regras_da_franca_3', 'FraThird::set_up_rules', '0x5c0700', 'fra_third.cpp', 'thiscall, ret 0',
   'o National: sobem 4 em 2001 e 3 depois, caem 4 (sem liga abaixo no jogo); 3 reservas e 3 substituições',
   'the National: 4 go up in 2001 and 3 afterwards, 4 go down (no league below in the game); 3 substitutes and 3 '
   'substitutions')])
CAMPOS = [('liga +0x40 ano da liga; valor; lê (as três)', 'league +0x40 league year; value; reads (all three)'),
          ('liga +0x3c turnos; +0xc2/+0xc3 pontos; +0xc4 categoria; +0xc5..+0xc7 desempates; +0xbe..+0xc1 vagas; '
           'valor; grava (as três)',
           'league +0x3c rounds; +0xc2/+0xc3 points; +0xc4 category; +0xc5..+0xc7 tie-breaks; +0xbe..+0xc1 places; '
           'value; writes (all three)'),
          ('liga +0xd9 bits de regra = 0x83; valor; grava (as três)', 'league +0xd9 rule bits = 0x83; value; writes '
           '(all three)'),
          ('liga +0x1c/+0x20 a liga de cima/de baixo; id; grava', 'league +0x1c/+0x20 league above/below; id; writes'),
          ('liga +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; valor; grava; +0xa7 = −1 (regras_da_franca_3); +0xba, '
           '+0xa9, +0x3a pela virtual +0x3c',
           'league +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; value; writes; +0xa7 = −1 (regras_da_franca_3); +0xba, '
           '+0xa9, +0x3a through virtual +0x3c')]
INTERESSA = ['CMRTE']
