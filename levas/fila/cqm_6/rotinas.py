# A divisão de baixo espanhola (o construtor e a tabela) e a tabela da Segunda espanhola (Agente B da frente das
# competições, 04/11/2026). Rodam uma vez cada na criação da carreira de 8 ligas (docs/cobertura_competicoes.txt):
# - 0x851020 (SpaLower::SpaLower, 367 bytes, quadro SEH só no caminho de exceção), chamado pela Primera (0x84e130);
# - 0x855b40 (SpaSecond::create_table, 381 bytes), chamada pelo construtor da Segunda (0x8539a0);
# - 0x8533d0 (SpaLower::create_table, 947 bytes), chamada pelo construtor acima; sorteia (0x4afbd0).
# Iguais no cm0102-1080.exe (inteiras).
# Convenções conferidas nos pontos de chamada da original:
# - 0x682200 (League::League): mov ecx, esi -> thiscall, ret 0;
# - 0x687b10: push 1; mov ecx, esi -> thiscall, ret 4;
# - 0x853790 (SpaLower::set_up_rules, a 275ª), 0x8533d0 e 0x6835c0: mov ecx, esi -> thiscall, ret 0;
# - 0x944e46 (malloc): push 0x512; add esp, 4 -> cdecl;
# - 0x687430 (League::init_table_row): push 0; push 0; push clube; push linha (Segunda: a dupla [esp + 0x10] com o byte
#   da linha e 3 bytes de pilha sem valor; divisão de baixo: eax com al = a linha e lixo em cima; a 0x687430 só lê o
#   byte); mov ecx, this -> thiscall, ret 0x10;
# - 0x540a50 (club_get_linked_b_team): push 1; push &eh_a; push clube; add esp, 0xc -> cdecl (grava eh_a sempre que
#   devolve um clube, conferido no porte; a original só lê eh_a nesse caso);
# - 0x682f70 (League::calc_prize_fund): push 50000; mov ecx, edi -> thiscall, ret 4;
# - 0x4afbd0 (comp_util_sort_and_shuffle_top_clubs): push faltam (cx, palavra; lixo em cima); push quantos; push lista;
#   add esp, 0xc -> cdecl;
# - a caixa: aviso_em.
# Comparadores: os três alocam (e a tabela de baixo sorteia): só a original, conferindo o construtor (a vtable, o ano,
# a disciplina, o registro), a tabela da Segunda (os 22 da Segunda na ordem do banco e a marca dos times B) e as
# primeiras linhas da de baixo (os times B da Primera e da Segunda). A comparação de 8 ligas achou (05/11) que a
# segunda passada zera o +0x60 dos times B da Primera já postos na tabela (a original faz assim; a cópia repete): o
# comparador não confere mais o +0x60.
# Determinismo: a lista dos candidatos é montada num vetor de 100 zerado (a original usa um registro local de 100 e só
# lê os `quantos` primeiros); nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construir_divisao_de_baixo_espanhola', 0x851020, [0x6a, 0xff, 0x68, 0x0b, 0x27, 0x96, 0x00]),
           ('tabela_da_segunda', 0x855b40, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('tabela_da_divisao_de_baixo_espanhola', 0x8533d0, [0x81, 0xec, 0x9c, 0x03, 0x00, 0x00])]
DOCS = ('a divisão de baixo espanhola (o construtor e a tabela) e a tabela da Segunda espanhola',
 'the Spanish lower division (constructor and table) and the Spanish Segunda table',
 [('construir_divisao_de_baixo_espanhola', 'SpaLower::SpaLower', '0x851020', 'spa_lower.cpp', 'thiscall, ret 4',
   'a divisão de baixo espanhola é uma liga de fundo sem disciplina (0xff), na competição de id [0x9cf6b0]; sem estado '
   'salvo, as regras, a tabela e os jogos (sem estatísticas nem posições previstas)',
   'the Spanish lower division is a background league with no discipline (0xff), in the competition of id '
   '[0x9cf6b0]; with no saved state, the rules, the table and the fixtures (no stats nor predicted positions)'),
  ('tabela_da_segunda', 'SpaSecond::create_table', '0x855b40', 'spa_second.cpp', 'thiscall, ret 0',
   'a Segunda tem 22 clubes (os da competição, na ordem do banco); o time B de um clube da Primera fica marcado como '
   'não pode subir (destino −4); o fundo de prêmios parte de 50.000',
   'the Segunda has 22 clubs (the competition\'s, in database order); the B team of a Primera club is marked as not '
   'promotable (fate −4); the prize fund starts from 50,000'),
  ('tabela_da_divisao_de_baixo_espanhola', 'SpaLower::create_table', '0x8533d0', 'spa_lower.cpp', 'thiscall, ret 0',
   'a divisão de baixo tem 22 clubes: primeiro os times B espanhóis cujo clube de cima está na Primera, depois os da '
   'Segunda (na ordem do banco); o resto vem dos clubes da Segunda B [inferência: as competições [0x9cf62c..0x9cf638]] '
   'e da divisão genérica espanhola que não são times B, ordenados pela reputação e com os melhores embaralhados; '
   'cada clube posto na tabela guarda a competição em +0x60, mas a passada dos times B da Segunda zera o +0x60 de todo '
   'time espanhol que não entra nela, inclusive os da Primera já postos [observado na comparação]',
   'the lower division has 22 clubs: first the Spanish B teams whose parent club is in the Primera, then those of the '
   'Segunda (database order); the rest come from the Segunda B clubs [inference: competitions [0x9cf62c..0x9cf638]] '
   'and the Spanish generic division that are not B teams, ordered by reputation with the best shuffled; each club '
   'placed keeps the competition at +0x60, but the Segunda B-team pass zeroes +0x60 of every Spanish side not entering '
   'it, including the Primera ones already placed [observed in the comparison]')])
CAMPOS = [('linha da tabela +0 clube, +0x37 destino (−4 = time B que não sobe); ponteiro e valor; grava (as duas tabelas)',
           'table row +0 club, +0x37 fate (−4 = B team that cannot go up); pointer and value; writes'),
          ('clube +0x57 competição (id no +0), +0x53 nação; ponteiro; lê e grava (a divisão de baixo muda +0x57 dos seus '
           'clubes para a genérica)',
           'club +0x57 competition (id at +0), +0x53 nation; pointer; reads and writes (the lower division moves its '
           'clubs\' +0x57 to the generic one)'),
          ('clube +0x60 a competição de reserva [inferência: ClubReserveDivision]; ponteiro; grava '
           '(tabela_da_divisao_de_baixo_espanhola)',
           'club +0x60 the reserve competition [inference: ClubReserveDivision]; pointer; writes'),
          ('liga +0x28 fundo de prêmios, +0xe1 marca; valor; grava (tabela_da_segunda)',
           'league +0x28 prize fund, +0xe1 mark; value; writes'),
          ('[0xae2350] [inferência: clubes criados depois]; valor; lê (tabela_da_divisao_de_baixo_espanhola)',
           '[0xae2350] [inference: clubs created later]; value; reads')]
INTERESSA = ['CMRTE', 'SGE']
