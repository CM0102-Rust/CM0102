# O play-off de acesso e o play-out da Série C1 italiana (os dois grupos) e a passagem de fase da Third Division
# inglesa (Agente B da frente das competições, 04/11/2026). Rodam uma vez cada na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt), no fim da liga (o slot +0x28, chamado pelo dia de liga 0x6847c0):
# - 0x64b9e0 e 0x64fba0 (ItaSerC1a/ItaSerC1b::advance_stage, 143 bytes cada) e 0x578330 (EngThird::advance_stage,
#   143 bytes): a fase atual sobe e, na primeira depois da liga, as fases são criadas;
# - 0x64ba70 e 0x64fc30 (ItaSerC1a::create_playoff_stages / ItaSerC1b::create_play_off_stages, 780 bytes cada, quadro
#   SEH só para o operator new): a Supercopa, o play-off e o play-out.
# Iguais no cm0102-1080.exe (inteiras). As duas da C1 diferem só no fonte, nas linhas das caixas e na ordem de soltar a
# lista e o gabarito no fim (conferido byte a byte pela desmontagem normalizada).
# Convenções conferidas nos pontos de chamada da original:
# - 0x64ba70/0x64fc30/0x5783c0: mov ecx, this (o ecx que entrou) -> thiscall, ret 0;
# - a caixa: aviso_em (o fonte e a linha; a original empurra os dois buffers na ordem inversa, o texto é o mesmo);
# - 0x944e46 (malloc) e 0x944cf1 (operator new): push n; add esp, 4 -> cdecl;
# - a virtual +0x3c (create_schedule): push 0; push &extra; push &rodadas; push fase; mov ecx, esi -> thiscall, ret 0x10;
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 push; mov ecx, novo -> thiscall, ret 0x3c (os clubes, as rodadas, o ano e o
#   extra vão como dupla com lixo nos bits de cima; o construtor só lê a palavra: mov ax, word ptr [esp + ...]);
# - 0x51c800 (Cup::set_up_round): push 0; mov ecx, fase -> thiscall, ret 4;
# - 0x9452ca (free): push; add esp, 4 -> cdecl.
# Atenção: as posições do play-out (16, 13, 15, 14) são gravadas no mesmo registro local das do play-off antes do
# segundo construtor; o `extra` de cada fase é o que a virtual +0x3c devolveu (conferido: [esp + 0x14] = o `extra`,
# [esp + 0x10] = as posições, nas duas chamadas).
# Comparadores: as passagens, lado a lado em +0x30 quando só sobem a fase (na caixa ou ao criar as fases, só a
# original); a criação, só a original (aloca e sorteia), conferindo a vaga do campeão na Supercopa, as duas fases e
# +0x30 = 1.
# Determinismo: as posições e as listas são constantes; o lixo dos bits de cima não vai para a cópia (zero).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('fase_seguinte_da_c1a', 0x64b9e0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('fases_do_fim_da_c1a', 0x64ba70, [0x6a, 0xff, 0x68, 0xec, 0xba, 0x95, 0x00]),
           ('fase_seguinte_da_c1b', 0x64fba0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('fases_do_fim_da_c1b', 0x64fc30, [0x6a, 0xff, 0x68, 0x8c, 0xbb, 0x95, 0x00]),
           ('fase_seguinte_da_third', 0x578330, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('o play-off de acesso e o play-out da Série C1 italiana; a passagem de fase da Third Division inglesa',
 'the promotion play-off and the play-out of the Italian Serie C1; the stage advance of the English Third Division',
 [('fase_seguinte_da_c1a', 'ItaSerC1a::advance_stage', '0x64b9e0', 'ita_ser_c1a.cpp', 'thiscall, ret 0',
   'depois da liga, a fase atual sobe; na primeira, o play-off e o play-out são criados; além da última, a caixa de erro',
   'after the league, the current stage goes up; on the first, the play-off and the play-out are created; past the '
   'last, the error box'),
  ('fases_do_fim_da_c1a', 'ItaSerC1a::create_playoff_stages', '0x64ba70', 'ita_ser_c1a.cpp', 'thiscall, ret 0',
   'a regra da C1/A (18 clubes): o campeão sobe direto e entra na Supercopa da Série C1; 2º × 5º e 3º × 4º jogam o '
   'play-off de acesso (a segunda vaga); 14º × 17º e 15º × 16º jogam o play-out (quem perde cai com o 18º '
   '[inferência]); cada confronto é uma fase de copa com o calendário da própria liga',
   'the C1/A rule (18 clubs): the champion goes up directly and enters the Serie C1 Super Cup; 2nd v 5th and 3rd v 4th '
   'play the promotion play-off (the second place); 14th v 17th and 15th v 16th play the play-out (the losers go down '
   'with the 18th [inference]); each is a cup stage with the league\'s own schedule'),
  ('fase_seguinte_da_c1b', 'ItaSerC1b::advance_stage', '0x64fba0', 'ita_ser_c1b.cpp', 'thiscall, ret 0',
   'a mesma da C1/A', 'the same as C1/A'),
  ('fases_do_fim_da_c1b', 'ItaSerC1b::create_play_off_stages', '0x64fc30', 'ita_ser_c1b.cpp', 'thiscall, ret 0',
   'a mesma regra na C1/B', 'the same rule in C1/B'),
  ('fase_seguinte_da_third', 'EngThird::advance_stage', '0x578330', 'eng_third.cpp', 'thiscall, ret 0',
   'depois da liga da Third Division, a fase atual sobe e, na primeira, o play-off de acesso é criado (0x5783c0)',
   'after the Third Division league, the current stage goes up and, on the first, the promotion play-off is created '
   '(0x5783c0)')])
CAMPOS = [('liga +0x30 fase atual, +0x2c fases, +0xc vetor das fases; valor e ponteiro; lê e grava (as cinco)',
           'league +0x30 current stage, +0x2c stages, +0xc stage vector; value and pointer; reads and writes'),
          ('liga +0xbf vagas do play-off, +0x40 ano, +4 registro da competição; valor; lê (fases_do_fim_da_c1a/c1b)',
           'league +0xbf play-off places, +0x40 year, +4 competition record; value; reads'),
          ('[[0xadadfc] + [0x9cf938] × 4] a Supercopa da Série C1, +0xa7 inscritos (6 bytes; +0 clube); ponteiro; grava '
           'o campeão (fases_do_fim_da_c1a/c1b)',
           '[[0xadadfc] + [0x9cf938] × 4] the Serie C1 Super Cup, +0xa7 entrants (6 bytes; +0 club); pointer; writes the '
           'champion'),
          ('linha da tabela +0 clube; ponteiro; lê (fases_do_fim_da_c1a/c1b)', 'table row +0 club; pointer; reads')]
INTERESSA = []
