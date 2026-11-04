# A Primera espanhola e a Série C2/A italiana (Agente B da frente das competições, 03/11/2026):
# - 0x84fc70 (SpaFirst::create_table, 380 bytes): a divisão de baixo espanhola vira a genérica, a tabela com os 20
#   clubes, o fundo e a TV; uma vez, na criação da carreira de 8 ligas (o construtor: mov ecx, esi; call -> thiscall,
#   ret 0).
# - 0x654180 (ItaSerC2a::update_staff_records_for_status, 528 bytes): os registros pela situação, a virtual +0x44
#   (anunciada pela 0x686670: push a6; push jogo; push a4; push fase; push situação; push clube; mov ecx, liga;
#   call [eax+0x44] -> thiscall, ret 0x18); 40 chamadas na carreira de 8 ligas até 2002-06-01.
# - 0x850880 (SpaFirst::set_club_progress, 63 bytes): a virtual +0x68 da 0x684cd0, a cada linha da tabela da
#   Primera (thiscall, ret 0x14; ~6.000 chamadas).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x944e46 (malloc): push 0x49c; add esp, 4 -> cdecl;
# - 0x687430: push 0; push 0; push clube; push eax (al = a linha); mov ecx, edi -> thiscall, ret 0x10 (a linha limpa:
#   só o byte é lido);
# - 0x682f70: push 0x3d090; mov ecx, edi -> thiscall, ret 4; 0x683010: push 0; push 0x4c4b40; mov ecx, edi ->
#   thiscall, ret 8;
# - 0x868c50: push comp; push clube; mov ecx, [0xdd7f00] -> thiscall, ret 8; 0x869480: push 0x64/0x32; push comp;
#   push clube; mov ecx, [0xdd7f00] -> thiscall, ret 0xc; 0x86bdd0: push 0x28; push 0x82; push 0xa0; push comp; push
#   clube; mov ecx, [0xdd7f00] -> thiscall, ret 0x14; 0x86a1c0: push comp; push clube; mov ecx, [0xdd7f00] ->
#   thiscall, ret 8;
# - 0x4bf850: push 2; push cx (a rodada +0x34, lixo acima); push 0x32; push 0x1e; call; add esp, 0x10 -> cdecl (a
#   palavra limpa: só ela é lida);
# - 0x86c000: push eax (o peso); push edx (dx = +0x30, lixo acima); push eax (ax = +0x32, os bits de cima do peso);
#   push comp; push clube; mov ecx, [0xdd7f00] -> thiscall, ret 0x14 (as palavras limpas: só elas são lidas, pela
#   0x4b0870 e pela entrada do histórico);
# - 0x4a4850 (Comp_Stats::find_club_entry): push clube; mov ecx, [ecx+8] -> thiscall, ret 4.
# Comparadores: 0x84fc70 só a original (aloca, paga, notícia) com o plano (a divisão genérica, as linhas, +0x3e,
# +0xe1, +0x28); 0x654180 só a original (os registros não rodam duas vezes) com o plano do +0x37 da linha; 0x850880
# lado a lado nos 3 bytes da entrada.
# Determinismo: percorre o banco e a tabela na ordem; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('tabela_da_primera', 0x84fc70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('registros_pela_situacao_na_c2a', 0x654180, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('situacao_na_primera', 0x850880, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('a Primera espanhola: a tabela, a TV e a situação nas estatísticas; a Série C2/A: os registros pela situação',
 'La Primera: the table, the TV money and the progress in the stats; Serie C2/A: the records by status',
 [('tabela_da_primera', 'SpaFirst::create_table', '0x84fc70', 'spa_first.cpp', 'thiscall, ret 0',
   'a Primera tem 20 clubes; antes, os clubes da divisão de baixo espanhola passam à divisão de baixo genérica; '
   'fundo de prêmios de 250.000 por posição; a TV paga 5.000.000 a cada clube',
   'La Primera has 20 clubs; first, the clubs of the Spanish lower division move to the generic lower division; '
   'prize fund of 250,000 per place; TV pays 5,000,000 to each club'),
  ('registros_pela_situacao_na_c2a', 'ItaSerC2a::update_staff_records_for_status', '0x654180', 'ita_ser_c2a.cpp',
   'thiscall, ret 0x18',
   'os registros da Série C2/A: na tabela, o título, o acesso (peso 100), a vaga no play-off (peso 40) e o '
   'rebaixamento; no play-off de acesso, quem vence sobe (peso 50) e quem cai fica com o registro da eliminação '
   '(peso pela rodada); no play-out, quem vence se salva e quem perde cai',
   'Serie C2/A records: in the table, the title, promotion (weight 100), the play-off place (weight 40) and '
   'relegation; in the promotion play-off, the winner goes up (weight 50) and the loser gets the knock-out record '
   '(weight by round); in the play-out, the winner stays up and the loser goes down'),
  ('situacao_na_primera', 'SpaFirst::set_club_progress', '0x850880', 'spa_first.cpp', 'thiscall, ret 0x14',
   'a posição atual, a melhor e a pior possíveis do clube nas estatísticas da Primera; as posições 17 e 18 viram 16 '
   'na melhor e 19 na pior [inferência: não formam faixa própria]',
   'the club\'s current, best and worst possible position in La Primera stats; places 17 and 18 become 16 for the '
   'best and 19 for the worst [inference: they are not a band of their own]')])
CAMPOS = [('clube +0x57 competição (id no +0); ponteiro; lê e grava (tabela_da_primera: a divisão de baixo espanhola '
           'vira a genérica, [0xae23d0] + [0x9cf764] × 0x6b)',
           'club +0x57 competition (id at +0); pointer; reads and writes (tabela_da_primera: the Spanish lower '
           'division becomes the generic one, [0xae23d0] + [0x9cf764] × 0x6b)'),
          ('liga +0x3e, +0xb1, +0x28, +0xe1; valor e ponteiro; grava (tabela_da_primera)',
           'league +0x3e, +0xb1, +0x28, +0xe1; value and pointer; writes (tabela_da_primera)'),
          ('linha da tabela +0x37 situação (0 subiu, 3 caiu, 0xfe fora); valor; grava '
           '(registros_pela_situacao_na_c2a)',
           'table row +0x37 status (0 promoted, 3 relegated, 0xfe out); value; writes '
           '(registros_pela_situacao_na_c2a)'),
          ('jogo +0x30 rodada, +0x32 fase, +0x34 rodada para o peso; valor; lê (registros_pela_situacao_na_c2a)',
           'match +0x30 round, +0x32 stage, +0x34 round for the weight; value; reads '
           '(registros_pela_situacao_na_c2a)'),
          ('Comp_Stats entrada do clube +0x73 posição atual, +0x74 melhor, +0x75 pior; valor; grava '
           '(situacao_na_primera)',
           'Comp_Stats club entry +0x73 current place, +0x74 best, +0x75 worst; value; writes (situacao_na_primera)')]
INTERESSA = ['CMRTE']
