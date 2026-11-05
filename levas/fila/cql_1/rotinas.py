# Os números de camisa (squad_manager.cpp, o SQUAD_MANAGER em [0xdd7ef0]), parte 1: ler o número de um jogador no
# clube (0x85a760, ~545 mil chamadas até 2002-06-01 com 8 ligas), se o clube usa números (0x85add0, ~2,8 milhões) e
# gravar o número (0x85a850, ~63 mil) (docs/cobertura_competicoes.txt). As três inteiras iguais no cm0102-1080.exe.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - as três: thiscall com ecx = [0xdd7ef0] (ex.: 0x85a4e8 push ebp; push eax; mov ecx, ebx; call 0x85a760 -> ret 8;
#   0x8c952d e 0x85a2ff para as outras: push 0; push A; mov ecx, [0xdd7ef0] -> ret 8; push n; push clube; push pessoa;
#   mov ecx, ebp; call 0x85a850 -> ret 0xc). Nenhuma lê o this.
# - dentro: 0x4cf5a0 e 0x4cf6d0 (mov ecx, [0xae1050]; push pessoa -> thiscall de 1, devolvem o contrato); 0x53b620
#   (push edx com dl = +0x3d; add esp, 4 -> cdecl de 1, só o byte); 0x540a50 (push 1; push 0; push clube; add esp, 0xc
#   -> cdecl de 3); a caixa de erro (aviso_em: 0x944cff, 0x90d130, 0x944c9f, 0x5e8290 e [0xb67a34] = 0).
# Comparadores: 0x85a760 e 0x85add0 só leem (a devolução das duas; a 0x85add0 sem clube só a original, pela caixa);
# 0x85a850 lado a lado no +0x3a dos dois contratos da pessoa (achados antes; número fora de −50..50, só a original).
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('numero_de_camisa', 0x85a760, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('clube_usa_numeros', 0x85add0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gravar_numero_de_camisa', 0x85a850, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os números de camisa: o número de um jogador no clube, se o clube usa números e gravar o número',
        'squad numbers: a player\'s number at a club, whether the club uses numbers, and setting the number',
        [('numero_de_camisa', 'SQUAD_MANAGER::get_squad_number', '0x85a760', 'squad_manager.cpp', 'thiscall, ret 8 (this não lido)',
          'a regra: o número de camisa fica no contrato (+0x3a), positivo nos clubes de técnico humano e negativo nos outros; vale o contrato do clube (ou do clube principal [inferência: time B]) e, sem ele, o de empréstimo; quem não está na tabela de pessoas com contrato tem o número na ficha de jogador',
          'the rule: the squad number lives in the contract (+0x3a), positive at human-managed clubs and negative elsewhere; the club\'s contract counts (or the parent club\'s [inference: B team]) and, failing it, the loan contract; a person outside the contracted-people table has the number in the player record'),
         ('clube_usa_numeros', 'SQUAD_MANAGER::club_uses_squad_numbers', '0x85add0', 'squad_manager.cpp', 'thiscall, ret 8 (this não lido)',
          'a regra: um clube usa números fixos de camisa quando a liga da sua divisão tem o bit 2 das opções (+0x52); com uma segunda competição (ex.: uma copa), ela também precisa do bit',
          'the rule: a club uses fixed squad numbers when its division\'s league has bit 2 of its options (+0x52); with a second competition (e.g. a cup), it needs the bit too'),
         ('gravar_numero_de_camisa', 'SQUAD_MANAGER::set_squad_number', '0x85a850', 'squad_manager.cpp', 'thiscall, ret 0xc (this não lido)',
          'a regra: os números vão de −50 a 50 (fora disso, a caixa de erro); grava no contrato que vale no clube, ou no de empréstimo',
          'the rule: numbers go from −50 to 50 (otherwise the error box); written to the contract valid at the club, or to the loan contract')])
CAMPOS = [('contrato +4 clube, +0x3a número de camisa (char; positivo com técnico humano, negativo sem); id e valor; lê (numero_de_camisa), grava +0x3a (gravar_numero_de_camisa)',
           'contract +4 club, +0x3a squad number (char; positive with a human manager, negative without); id and value; reads (numero_de_camisa), writes +0x3a (gravar_numero_de_camisa)'),
          ('pessoa +0 id, +0x3d tipo (0x53b620: jogador), +0x61 ficha de jogador (+4 o número de quem não tem contrato); id, valor e ponteiro; lê (numero_de_camisa)',
           'person +0 id, +0x3d type (0x53b620: player), +0x61 player record (+4 the number of someone without a contract); id, value and pointer; reads (numero_de_camisa)'),
          ('clube +0 id, +0x57 divisão (dword +0 o id da competição); id e ponteiro; lê (clube_usa_numeros, numero_de_camisa)',
           'club +0 id, +0x57 division (dword +0 the competition id); id and pointer; reads (clube_usa_numeros, numero_de_camisa)'),
          ('competição +0x52 opções (bit 2: números de camisa); valor; lê (clube_usa_numeros); [0xadadfc] as competições pelo id; [0xae2364] [inferência: o número de pessoas com contrato]',
           'competition +0x52 options (bit 2: squad numbers); value; reads (clube_usa_numeros); [0xadadfc] competitions by id; [0xae2364] [inference: the number of contracted people]')]
INTERESSA = ['CMRTE', 'SGE', 'CMScout']
