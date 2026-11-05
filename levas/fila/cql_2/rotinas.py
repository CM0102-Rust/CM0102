# Os números de camisa (squad_manager.cpp), parte 2: dar número a quem chega (0x85a470, ~7,8 mil chamadas até
# 2002-06-01 com 8 ligas: as transferências, os empréstimos e a numeração do elenco) e numerar o elenco de um clube
# (0x859fe0, ~740: a numeração das ligas da 0x859e20 no começo do jogo e nas viradas). Inteiras iguais no 1080.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x85a470: 0x85a3dc push eax (pessoa); push edi (clube); mov ecx, ebp; call -> thiscall(this, clube, pessoa), ret 8.
# - 0x859fe0: 0x859f94 push 1; push clube; mov ecx, ebp -> thiscall(this, clube, manter), ret 8.
# - dentro: 0x541f90 (push 0; push lista; push clube; cdecl de 3), 0x945333 (qsort: push 0x540130; push 4; push 0x32
#   ou 0x1e; push base; cdecl de 4), 0x85a760 (push clube; push pessoa; mov ecx, this -> thiscall de 2), 0x85a850
#   (push n; push clube; push pessoa; mov ecx, this -> thiscall de 3; o n com lixo nos bytes altos, ela lê só o byte),
#   0x600a50 (push 0, 0, 1, 0, clube; add esp, 0x14 -> cdecl de 5), 0x540a50 (cdecl de 3), 0x4cf5f0 (push clube; push
#   pessoa; mov ecx, [0xae1050] -> thiscall de 2), 0x6e2000 (15 push; add esp, 0x3c -> cdecl de 15; a conta de
#   jogadores vai com lixo nos bytes altos, a cópia Rust dela lê só o byte), 0x85a470 (acima).
# Comparadores: lado a lado no +0x3a dos contratos (o do clube e o de empréstimo) das pessoas que a rotina pode
# numerar (0x85a470: a pessoa; 0x859fe0: o elenco do clube e o do clube ligado) e na marca da caixa [0xb67a34]. Na
# 0x85a470, quando a ficha não tem posição boa (todas as notas checadas abaixo de 15), a original lê a lista de
# preferidos de 4 bytes sem valor na pilha: a cópia usa a lista vazia e o comparador deixa só a original nesse caso.
# Determinismo: as ordenações são as da original (qsort 0x945333 com a 0x540130, idade e nome); a escalação 0x6e2000
# não sorteia; nada lê pilha, heap ou endereço como valor na cópia.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('dar_numero_de_camisa', 0x85a470, [0x81, 0xec, 0x08, 0x01, 0x00, 0x00]),
           ('numerar_elenco', 0x859fe0, [0x81, 0xec, 0x90, 0x02, 0x00, 0x00])]
DOCS = ('os números de camisa: dar número a quem chega e numerar o elenco de um clube',
        'squad numbers: give a number to a new arrival and number a club\'s squad',
        [('dar_numero_de_camisa', 'SQUAD_MANAGER::allocate_squad_number', '0x85a470', 'squad_manager.cpp', 'thiscall, ret 8',
          'a regra: quem chega ganha o primeiro número livre da sua posição: goleiro 1; zagueiro 2 (lateral direito), 3 (esquerdo) ou 4 a 6; volante, meia e meia-atacante 6 a 8; atacante 8 a 11 (posição boa = nota 15 ou mais); sem vaga, o primeiro livre de 12 a 50, e por fim o maior livre de 11 para baixo; positivo com técnico humano, negativo sem',
          'the rule: a new arrival gets the first free number for his position: goalkeeper 1; defender 2 (right back), 3 (left) or 4 to 6; defensive, central and attacking midfielder 6 to 8; striker 8 to 11 (good position = rating 15 or more); otherwise the first free from 12 to 50, and lastly the highest free from 11 down; positive with a human manager, negative without'),
         ('numerar_elenco', 'SQUAD_MANAGER::assign_club_squad_numbers', '0x859fe0', 'squad_manager.cpp', 'thiscall, ret 8',
          'a regra: o elenco (com o time B ligado) é ordenado pela escalação de referência (11 titulares e 9 reservas) e o resto pela idade; quem já tem número o guarda se não estiver repetido (sem `manter`, o número de titular, 1 a 11, só fica com os 11 primeiros e o de reserva só com os outros); quem não tem número ganha o da sua ordem se estiver livre; do 22º em diante, sem contrato com o clube ou com contrato juvenil/amador [inferência: tipos 6 e 7], fica sem número; os 40 primeiros ainda sem número ganham um pela regra de quem chega; no fim os números guardados são regravados (positivos com técnico humano, senão −1, −2... na ordem dos números)',
          'the rule: the squad (with the linked B team) is ordered by the reference line-up (11 starters and 9 substitutes) and the rest by age; whoever already has a number keeps it unless it is repeated (without `keep`, a starter\'s number, 1 to 11, only stays with the first 11 and a reserve\'s only with the others); whoever has none gets his order number if free; from the 22nd on, without a contract with the club or with a youth/amateur contract [inference: types 6 and 7], he gets no number; the first 40 still without one get one by the new-arrival rule; at the end the kept numbers are rewritten (positive with a human manager, otherwise −1, −2... in number order)')])
CAMPOS = [('contrato +0x3a número de camisa, +0x4f tipo (nibble alto; 6 e 7 sem número depois do 21º [inferência: juvenil e amador]); valor; lê e grava (as duas, pela 0x85a850)',
           'contract +0x3a squad number, +0x4f type (high nibble; 6 and 7 get no number after the 21st [inference: youth and amateur]); value; reads and writes (both, through 0x85a850)'),
          ('clube +0xd7 elenco (50 pessoas); ponteiro; lê (numerar_elenco)', 'club +0xd7 squad (50 people); pointer; reads (numerar_elenco)'),
          ('ficha de jogador (pessoa +0x61) +0xf goleiro, +0x10 líbero, +0x11 zagueiro, +0x12 volante, +0x13 meia, +0x14 meia-atacante, +0x15 atacante, +0x17 lado direito, +0x18 lado esquerdo (notas 1 a 20); valor; lê (dar_numero_de_camisa)',
           'player record (person +0x61) +0xf goalkeeper, +0x10 sweeper, +0x11 defender, +0x12 defensive midfielder, +0x13 midfielder, +0x14 attacking midfielder, +0x15 striker, +0x17 right side, +0x18 left side (ratings 1 to 20); value; reads (dar_numero_de_camisa)')]
INTERESSA = ['CMRTE', 'SGE', 'CMScout']
