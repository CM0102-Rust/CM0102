# A liga: as expectativas da temporada, o rebaixamento de um clube e a cadeia de promoções (league.cpp):
# - 0x68aa80 (League::set_club_season_expectations, 72 bytes): com o gerenciador dos técnicos, as expectativas da
#   temporada de cada clube da tabela da liga (0x6a3770, com a notícia). [inferência: no começo da temporada; a contagem
#   pelo contador "cqh_5 expectativa do clube".]
# - 0x6831a0 (League::relegate_club_to_division, 99 bytes; this não é lido): o clube desce (+0x57 = a divisão, +0x5b =
#   a de antes), as cláusulas de rebaixamento (0x4cd030) e, se a divisão não é jogada e o clube tem técnico humano que
#   decide (0x600a50), o técnico sai (0x69b790). Do fim da temporada: passageira.
# - 0x689c20 (League::process_promotion_relegation_chain, 83 bytes): na liga do topo, as posições finais (método
#   +0xa4) dela e de cada liga de baixo da cadeia (+0x20) e as trocas entre a de cima e a de baixo (0x689c80). Do fim da
#   temporada (depois de 2002-06-01): passageira.
# Iguais no cm0102-1080.exe (as três inteiras). Todas as chamadas já em Rust (0x6a3770, 0x4cd030, 0x600a50, 0x69b790,
# 0x689c80), fora os métodos virtuais +0xa4 das ligas.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x68aa80: [inferência] pelo método da liga (mov ecx, liga; sem push) -> thiscall, ret 0 (ela termina em
#   ret e lê só o ecx).
# - quem chama a 0x6831a0: (push x; push divisão; push clube; mov ecx, liga) -> thiscall, ret 0xc.
# - quem chama a 0x689c20: (push aos_pares; mov ecx, liga) -> thiscall, ret 4.
# - dentro: 0x6a3770 push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 8; 0x4cd030 push antiga; push clube; mov
#   ecx, [0xae1050] -> thiscall, ret 8; 0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl;
#   0x69b790 push 0; push 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 0x10; o método +0xa4 mov ecx, liga
#   -> thiscall; 0x689c80 push −1; push −1; push aos_pares; push 1; push abaixo; push acima; mov ecx, this -> thiscall,
#   ret 0x18.
# Comparadores: a 0x68aa80 e a 0x689c20 só a original (as expectativas, as posições finais e as trocas não rodam duas
# vezes; as cópias levam os ramos); a 0x6831a0 só a original, conferindo +0x57 e +0x5b do clube.
# Determinismo: nada lê pilha, heap ou endereço como valor.
SAVE = False
ROTINAS = [('liga_expectativas_da_temporada', 0x68aa80, [0xa1, 0x44, 0x7a, 0xb6, 0x00]),
           ('liga_rebaixar_clube', 0x6831a0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('liga_cadeia_de_promocoes', 0x689c20, [0x53, 0x8b, 0xd9, 0x57, 0x8b, 0x43, 0x1c])]
DOCS = ('a liga: as expectativas da temporada, o rebaixamento de um clube e a cadeia de promoções',
        'the league: the season expectations, a club relegation and the promotion chain',
        [('liga_expectativas_da_temporada', 'League::set_club_season_expectations', '0x68aa80', 'league.cpp', 'thiscall, ret 0',
          'as expectativas da temporada de cada clube da tabela, pelo gerenciador dos técnicos, com a notícia',
          'the season expectations of each club in the table, through the manager manager, with the news'),
         ('liga_rebaixar_clube', 'League::relegate_club_to_division', '0x6831a0', 'league.cpp', 'thiscall, ret 0xc',
          'o clube desce para a divisão (guardando a anterior), as cláusulas de rebaixamento e a saída do técnico humano quando a divisão não é jogada',
          'the club goes down to the division (keeping the previous one), the relegation clauses and the human manager leaving when the division is not played'),
         ('liga_cadeia_de_promocoes', 'League::process_promotion_relegation_chain', '0x689c20', 'league.cpp', 'thiscall, ret 4',
          'na liga do topo, as posições finais dela e das ligas de baixo e as trocas de rebaixados e promovidos ao longo da cadeia',
          'in the top league, its final positions and those of the lower leagues and the swaps of relegated and promoted clubs along the chain')])
CAMPOS = [('liga +0x1c a de cima (índice; < 0 no topo), +0x20 a de baixo (índice em [0xadadfc]), +0x3e clubes, +0xb1 tabela (linhas de 0x3b bytes, +0 clube); valor e ponteiro; lê (liga_expectativas_da_temporada, liga_cadeia_de_promocoes)',
           'league +0x1c the one above (index; < 0 at the top), +0x20 the one below (index into [0xadadfc]), +0x3e clubs, +0xb1 table (0x3b-byte rows, +0 club); value and pointer; reads (liga_expectativas_da_temporada, liga_cadeia_de_promocoes)'),
          ('clube +0x57 competição, +0x5b competição anterior; ponteiro; lê e grava (liga_rebaixar_clube)',
           'club +0x57 competition, +0x5b previous competition; pointer; reads and writes (liga_rebaixar_clube)')]
INTERESSA = ['CMRTE']
