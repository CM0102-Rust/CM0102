# A tabela das três divisões nacionais do Brasil (o slot create_table de bra_nat_first.cpp, bra_nat_second.cpp e
# bra_nat_third.cpp; agente C da frente das competições, 04/11/2026). Cada uma roda duas vezes na carreira de 8 ligas
# até 2002-06-01 (docs/cobertura_competicoes.txt: o construtor e a virada da temporada as chamam direto:
# 0x429910/0x42a738 mov ecx, esi; call 0x42a480 -> thiscall, ret 0; idem 0x42bf00/0x42ccbc e 0x42d382/0x42dfc6).
# - 0x42a480 BraNatFirst::create_table (281 bytes): 28 clubes (+0x57 = [0x9cf9ec]) e a TV 150.000 a cada um.
# - 0x42ca70 BraNatSecond::create_table (253 bytes, o molde dos estaduais com o +0x57): 28 clubes ([0x9cf9e0]).
# - 0x42dc90 BraNatThird::create_table (447 bytes): até 56 clubes da Série C numa lista (+0xee), embaralhada; os 7
#   primeiros do sorteio na tabela (7 linhas).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl; 0x9452ca (free): push p; call; add esp, 4 -> cdecl;
# - 0x687430 (League::init_table_row): push 0; push 0; push clube; push eax/edi (a linha); mov ecx, edi/esi ->
#   thiscall, ret 0x10 (na A e na B a linha vai no al com os bits de cima do id da competição: a cópia passa o byte
#   limpo, a 0x687430 só lê o char);
# - 0x683010 (League::pay_tv_rights): push 0; push 0x249f0; mov ecx, edi -> thiscall, ret 8;
# - 0x4afb40 (comp_util_shuffle_pointer_array, já em Rust): push eax (movsx ax, bl: a conta no word; os bits de cima
#   são sobra que ela não lê, movsx esi, word [esp+0x10]); push lista; add esp, 8 -> cdecl;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original (aloca a tabela; a A paga a TV com notícia; a C sorteia), conferindo o plano feito antes:
# A e B: os clubes da divisão na ordem do banco nas linhas, +0x3e e a devolução; C: a lista com os clubes do plano em
# alguma ordem, as 7 linhas com os 7 primeiros da lista, +0x3e = 7 e a devolução 1.
# Determinismo: percorre o banco na ordem; o sorteio é o do jogo (a 0x4afb40); nada de pilha, heap ou endereço como
# valor (o comparador ordena ponteiros só para conferir o conjunto).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cql7_tabela_da_serie_a', 0x42a480, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cql7_tabela_da_serie_b', 0x42ca70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cql7_tabela_da_serie_c', 0x42dc90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela das três divisões nacionais do Brasil: os clubes da Série A e da Série B e o sorteio da Série C',
        'the table of the three Brazilian national divisions: the Series A and Series B clubs and the Series C draw',
        [('cql7_tabela_da_serie_a', 'BraNatFirst::create_table', '0x42a480', 'bra_nat_first.cpp', 'thiscall, ret 0',
          'a Série A tem 28 clubes: os da divisão nacional da Série A, na ordem do banco; cheia, a TV paga 150.000 a '
          'cada clube (sem fundo de prêmios)',
          'Series A has 28 clubs: those whose national division is Series A, in database order; when full, TV pays '
          '150,000 to each club (no prize fund)'),
         ('cql7_tabela_da_serie_b', 'BraNatSecond::create_table', '0x42ca70', 'bra_nat_second.cpp', 'thiscall, ret 0',
          'a Série B tem 28 clubes, os da divisão na ordem do banco; sem dinheiro da TV nem fundo de prêmios',
          'Series B has 28 clubs, those of the division in database order; no TV money nor prize fund'),
         ('cql7_tabela_da_serie_c', 'BraNatThird::create_table', '0x42dc90', 'bra_nat_third.cpp', 'thiscall, ret 0',
          'a Série C recebe até 56 clubes (os da divisão, na ordem do banco), que são sorteados (3 trocas por clube); '
          'os 7 primeiros do sorteio formam a primeira tabela, de 7 clubes, e a lista sorteada fica guardada para os '
          'outros grupos [inferência: 8 grupos de 7 na primeira fase]',
          'Series C takes up to 56 clubs (those of the division, in database order), which are drawn (3 swaps per '
          'club); the first 7 of the draw make the first table, of 7 clubs, and the drawn list is kept for the other '
          'groups [inference: 8 groups of 7 in the first stage]')])
CAMPOS = [('clube +0x57: divisão nacional (registro de competição); ponteiro; lê (as três)',
           'club +0x57: national division (competition record); pointer; reads (all three)'),
          ('[0x9cf9ec] / [0x9cf9e0]: id da Série A / da Série B; id; lê', '[0x9cf9ec] / [0x9cf9e0]: Series A / Series B id; id; reads'),
          ('liga +4: o registro da própria competição; ponteiro; lê (Série C)', 'league +4: the competition own record; pointer; reads (Series C)'),
          ('liga +0x3e: número de times da tabela; valor; grava (28, 28, 7)', 'league +0x3e: number of teams in the table; value; writes (28, 28, 7)'),
          ('liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; grava', 'league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; writes'),
          ('Série C +0xee: a lista sorteada dos participantes (56 ponteiros); ponteiro; libera e grava',
           'Series C +0xee: the drawn list of entrants (56 pointers); pointer; frees and writes')]
INTERESSA = ['CMRTE']
