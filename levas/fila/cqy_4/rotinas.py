# A árvore dos confrontos de uma fase de copa feita com árvore (cup_stage.cpp; leitor cqy, 12/11/2026). Roda 2 vezes na
# carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt): o Cup_Stage::Cup_Stage com a árvore (o 12º
# argumento) é o da Liga dos Campeões depois dos grupos (european_cup.rs, a fase em .stages() + 0x30), em março de 2002.
# É a irmã do Cup::map_fixture_tree (0x518790, já em Rust: map_fixture_tree, cqd_7) com quatro diferenças, todas
# conferidas na desmontagem: (1) sem memória, a caixa comum das linhas 0x201/0x20b e −1 (não a caixa fatal); (2) sem
# season_set_up devolve 1 antes de zerar +0xab/+0xac; (3) os confrontos de cada rodada vão em ordem (sem o
# embaralhamento) e o lado é o primeiro livre (casa, depois fora; sem a moeda); (4) os classificados de cada rodada vão
# para o confronto q/2 (pelo par, sem sorteio nem moeda) e o lado é o primeiro livre. O resto (a vaga pelo par com o
# bit 0x200 da rodada seguinte ou sorteada, a vaga duas rodadas adiante, as caixas fatais 0x294 e 0x2bf, a troca de
# mando pelo estádio da sede, o número do primeiro confronto de cada rodada) é o mesmo molde.
# - 0x5241e0 Cup_Stage::map_fixture_tree (cup_stage.cpp), igual no cm0102-1080.exe (inteira).
# Convenções conferidas: a rotina (do Cup_Stage::Cup_Stage 0x522e00: mov ecx, esi; call -> thiscall, ret 0; devolve 1
# ou −1); dentro: malloc 0x944e46 (push; add esp, 4 -> cdecl), utils_random 0x90cfc0 (push n; add esp, 4 -> cdecl), a
# caixa (show_error_at, fonte 0x9b5944) e a fatal (fatal_error_at: a caixa, 0x5cc7d0, 0x5a8f60 e exit(−1) 0x945934).
# Comparador: só a original (aloca e sorteia), com o plano depois (cup_stage_tree_mismatch): cada confronto das
# rodadas antes de m − 1 é o que a vaga dele aponta de volta (rodada r + 1 ou r + 2, o confronto e a rodada ligados), e
# cada classificado de cada rodada está em exatamente um confronto dela.
# Determinismo: o sorteio da vaga (utils_random) na mesma ordem da original.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cup_stage_map_fixture_tree', 0x5241e0, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00])]
DOCS = ('a árvore dos confrontos de uma fase de copa: os confrontos de cada rodada, as ligações com a rodada seguinte e os classificados nos primeiros confrontos',
        'the fixture tree of a cup stage: the ties of each round, the links to the next round and the entrants in the first ties',
        [('cup_stage_map_fixture_tree', 'Cup_Stage::map_fixture_tree', '0x5241e0', 'cup_stage.cpp', 'thiscall, ret 0',
          'regra: a fase de copa com árvore (o mata-mata da Liga dos Campeões) monta os confrontos de cada rodada; o vencedor de cada confronto vai para a vaga do par na rodada seguinte (p/2, quando a rodada seguinte manda pelo par) ou para uma vaga sorteada, no primeiro lado livre (casa, depois fora; sem vaga, duas rodadas adiante); os classificados entram aos pares, o 1º e o 2º no primeiro confronto, o 3º e o 4º no segundo e assim por diante, o primeiro de cada par em casa; com sede fixa, o time cujo estádio é a sede joga em casa; no fim, o número do primeiro confronto de cada rodada, de trás para a frente',
          'rule: the cup stage with a tree (the Champions League knockout) builds the ties of each round; each tie winner goes to the pair\'s slot in the next round (p/2, when the next round goes by pairs) or to a drawn slot, on the first free side (home, then away; with no slot, two rounds ahead); the entrants go in by pairs, the 1st and 2nd in the first tie, the 3rd and 4th in the second and so on, the first of each pair at home; with a fixed venue, the team whose stadium is the venue plays at home; at the end, the number of the first tie of each round, from back to front')])
CAMPOS = []
INTERESSA = []
