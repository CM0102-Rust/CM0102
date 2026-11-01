# O dia da liga, a criação dos jogos da temporada e a promoção e o rebaixamento (league.cpp):
# - 0x6847c0 (League::update_for_date, 1289 bytes, quadro SEH sem destrutor no caminho normal): o dia de cada liga (as
#   fases, o fim da temporada, os recordes, os jogos para a TV quatro semanas adiante, o prazo das inscrições e a data
#   +0xd1), pelas competições: 46522 chamadas na carreira automática até 2002-06-01 (docs/cobertura_simulacao.txt, 17/10).
# - 0x6835c0 (League::create_fixtures, 2336 bytes, quadro SEH sem destrutor no caminho normal): os jogos da temporada
#   (o rodízio da grade ou a lista própria do ano) no calendário, com a ordem dos clubes sorteada: 44 chamadas.
# - 0x689c80 (league_promote_and_relegate, 937 bytes): troca os rebaixados e os promovidos entre duas ligas, no fim da
#   temporada (depois de 2002-06-01): passageira.
# Iguais no cm0102-1080.exe (as três inteiras). Chamam originais não portadas: 0x442e80, 0x4a19f0, 0x4a2450, 0x859e20,
# 0x85c260, 0x9462bb, 0x4cced0, 0x69b790 e os métodos virtuais da liga e das fases; o resto já em Rust.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x6847c0: 0x4027a5 (push completo; push &data; mov ecx, esi) e outras 26 -> thiscall, ret 8 (ela
#   termina em ret 8; devolve no al).
# - quem chama a 0x6835c0: 0x4026c2 (mov ecx, esi; sem push) e outras 205 -> thiscall, ret 0.
# - quem chama a 0x689c80: 0x689c60 (push -1; push ebp; push 1; push esi; push edi; mov ecx, ebx) e 0x41ef03 (mov ecx,
#   esi) -> 6 argumentos na pilha, ret 0x18, e o ecx de quem chama, que ela guarda e repassa à 0x6830b0 (que não o lê:
#   mov ecx, [0xdd7ef0] no começo): thiscall(ecx, ...) na cópia, para guardar o ecx.
# - dentro da 0x6847c0: 0x669470 push competição; add esp, 4 -> cdecl; 0x683ee0 push 0; mov ecx, esi -> thiscall, ret 4;
#   os métodos da fase (mov ecx, fase; push completo; push &data; call [vt+0x14]) e da liga (vt+0x94 sem push; vt+0x10
#   push 0; push 0; vt+0x28 e vt+0x30 sem push) -> thiscall; o método 0x10 da fase push 1; push 0 -> thiscall; 0x442e80
#   push liga; mov ecx, [0xad9da8] -> thiscall, ret 4; 0x4b0700 push id; add esp, 4 -> cdecl; 0x447430 push 1 (ou 0);
#   push liga; mov ecx, [0xad9dac] -> thiscall, ret 8; a caixa (aviso_em); 0x4a19f0 / 0x4a2450 mov ecx, [liga+8] ->
#   thiscall, ret 0; 0x54c770 / 0x54c5b0 push n; push saída; lea ecx, data -> thiscall, ret 8; 0x685640 push &data; mov
#   ecx, esi -> thiscall, ret 4; 0x549ef0 push ano; push dia; lea ecx -> thiscall, ret 8; 0x859e20 push 0; push 0; push
#   liga; mov ecx, [0xdd7ef0] -> thiscall, ret 0xc; 0x417c10 push &data; push saída; lea ecx, [liga+0xd1] -> thiscall,
#   ret 8.
# - dentro da 0x6835c0: 0x54a110 lea ecx -> thiscall, ret 0; 0x944cf1 push tamanho; add esp, 4 -> cdecl; a caixa
#   (aviso_em) e 0x944c94 push grade; add esp -> cdecl; 0x6844b0 push n; push grade (mov ecx, esi antes, mas ela começa
#   com mov ecx, [esp+8]) -> stdcall, ret 8; 0x686a70 push n; mov ecx, esi -> thiscall, ret 4; o método 0x7c da liga
#   push &jogo; mov ecx, esi -> thiscall; 0x549ef0 e 0x417c10 como acima; 0x68a030 push +0xd9; push +0xa9; push +0x3c;
#   push +0x3e; push id; push &passo; push rodada; add esp, 0x1c -> cdecl; 0x85c260 push +0x24; push &jogo; push [data
#   +0x3d]; mov ecx, [0xdd7ef4] -> thiscall, ret 0xc; 0x5aa680 push 1; push &jogo; mov ecx, [0xae2a58] -> thiscall, ret
#   8; 0x9462bb push 0x5a53c0; push conta; push 0x10; push lista (sem add esp) -> stdcall; 0x685640 como acima.
# - dentro da 0x689c80: 0x4cced0 push competição; push clube; mov ecx, [0xae1050] -> thiscall, ret 8; 0x6830b0 push 0;
#   push competição; push clube; mov ecx, [ecx guardado] -> thiscall, ret 0xc; 0x4cd030 push antiga; push clube; mov ecx,
#   [0xae1050] -> thiscall, ret 8; 0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl; 0x69b790
#   push 0; push 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 0x10; a caixa (aviso_em).
# Comparadores: as três só a original (os métodos virtuais, o calendário, as notícias e os contratos não rodam duas
# vezes; a criação sorteia a ordem dos clubes), conferindo o que cada uma garante: o dia devolve 0 só com a liga parada
# ou sem clubes e, completo, deixa +0xd1 com a data; a criação libera a lista própria do ano.
# Determinismo: o registro do jogo da criação (0x4f bytes na pilha) tem bytes que a original não grava (+4..+7, +0x18,
# +0x2e, +0x39, +0x4d e os bits altos de +0x4e, sobra da pilha dela, salvo o que o método 0x7c e a 0x85c260 preencham):
# a cópia os deixa 0, um valor que o jogo produz, sem depender de endereço. Nada mais lê pilha ou heap como valor.
SAVE = False
ROTINAS = [('liga_dia', 0x6847c0, [0x6a, 0xff, 0x68, 0x21, 0xca, 0x95, 0x00]),
           ('liga_criar_tabela', 0x6835c0, [0x6a, 0xff, 0x68, 0x81, 0xc9, 0x95, 0x00]),
           ('liga_promover_e_rebaixar', 0x689c80, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('a liga: o dia da liga (fases, fim da temporada, TV, prazo das inscrições), a criação dos jogos da temporada e a promoção e o rebaixamento',
        'the league: the daily update (stages, season end, TV, registration deadline), the creation of the season fixtures and promotion and relegation',
        [('liga_dia', 'League::update_for_date', '0x6847c0', 'league.cpp', 'thiscall, ret 8',
          'o dia da liga: a notícia da tabela no dia certo, o dia de cada fase, o evento da data +0xc9, o fim da temporada (fase seguinte, histórico, recordes), os jogos para a TV quatro semanas adiante, o prazo das inscrições 5/20/14 dias antes da primeira data e a data +0xd1',
          'the league day: the fixture list news on its day, each stage\'s day, the +0xc9 date event, the season end (next stage, history, records), the TV fixtures four weeks ahead, the registration deadline 5/20/14 days before the first date and the +0xd1 date'),
         ('liga_criar_tabela', 'League::create_fixtures', '0x6835c0', 'league.cpp', 'thiscall, ret 0',
          'os jogos da temporada: a grade do rodízio (com a ordem dos clubes sorteada) ou a lista própria do ano, o mando pela paridade do turno e do ano, o estádio, a data e a fatia de cada rodada, e o calendário; depois as datas da TV',
          'the season fixtures: the round robin grid (with the club order drawn) or the year\'s own list, home and away by the parity of the half and the year, the stadium, date and slot of each round, and the calendar; then the TV dates'),
         ('liga_promover_e_rebaixar', 'league_promote_and_relegate', '0x689c80', 'league.cpp', 'thiscall (ecx repassado), ret 0x18',
          'as cláusulas de não promoção, os rebaixados de cima descem (cláusulas de rebaixamento, técnico humano demitido numa competição sem registro) e os promovidos de baixo sobem, aos pares ou todos; a caixa se as contas diferem das esperadas',
          'the non-promotion clauses, the relegated clubs go down (relegation clauses, human manager sacked in a competition with no active record) and the promoted clubs go up, in pairs or all; the box if the counts differ from the expected ones')])
CAMPOS = [('liga +0x2c fases, +0x30 fase atual, +0xc as fases, +0x3c turnos, +0x3e times, +0x40 ano, +0x44 fase (0xff primeira), +0x45 clubes, +0x4c fim feito, +0x52 bit 2 inscrições, +0xa7 rodadas, +0xa9 datas, +0xad, +0xb1 tabela (0x3b bytes: +0 clube, +0x37 situação, +0x38 marca; +0x11 em andamento), +0xb5 parada, +0xba tabela das datas (0x41 bytes), +0xc9 data do evento, +0xd1 última data, +0xdd fim da temporada, +0xe6, +0xea lista própria; valor e ponteiro; lê e grava (liga_dia, liga_criar_tabela, liga_promover_e_rebaixar)',
           'league +0x2c stages, +0x30 current stage, +0xc the stages, +0x3c rounds per pair, +0x3e teams, +0x40 year, +0x44 stage (0xff first), +0x45 clubs, +0x4c end done, +0x52 bit 2 registration, +0xa7 rounds, +0xa9 dates, +0xad, +0xb1 table (0x3b bytes: +0 club, +0x37 status, +0x38 flag; +0x11 in progress), +0xb5 stopped, +0xba date table (0x41 bytes), +0xc9 event date, +0xd1 last date, +0xdd season over, +0xe6, +0xea own list; value and pointer; reads and writes (liga_dia, liga_criar_tabela, liga_promover_e_rebaixar)'),
          ('partida (0x4f bytes) +0 competição, +8, +0xc/+0x10 ids, +0x14 competição, +0x1c/+0x20 clubes, +0x24/+0x26 última data, +0x28 ano, +0x2a dia, +0x2c ano da liga, +0x34 rodada, +0x3f fatia, +0x40/+0x41 marcas, +0x42 fase, +0x4e bit 3 última rodada; valor e ponteiro; grava (liga_criar_tabela)',
           'fixture (0x4f bytes) +0 competition, +8, +0xc/+0x10 ids, +0x14 competition, +0x1c/+0x20 clubs, +0x24/+0x26 last date, +0x28 year, +0x2a day, +0x2c league year, +0x34 round, +0x3f slot, +0x40/+0x41 flags, +0x42 stage, +0x4e bit 3 last round; value and pointer; writes (liga_criar_tabela)'),
          ('clube +0x57 competição, +0x5b competição anterior, +0x69 nação; ponteiro; lê e grava (liga_promover_e_rebaixar), lê (liga_criar_tabela)',
           'club +0x57 competition, +0x5b previous competition, +0x69 nation; pointer; reads and writes (liga_promover_e_rebaixar), reads (liga_criar_tabela)')]
INTERESSA = ['CMRTE', 'GPF2']
