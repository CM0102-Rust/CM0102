# A competição: todas as fases completas, a lista dos clubes e o progresso do clube (comp.cpp):
# - 0x48ce10 (Comp::all_stages_complete, 74 bytes, método +0xc das competições): só na competição do topo, ela e cada
#   uma da cadeia de baixo completas (método +0x10 com 1, 1). [inferência: pelo fim da temporada das competições; a
#   contagem pelos contadores da cópia.]
# - 0x48e1c0 (Comp::collect_club_list, 400 bytes, método +0x58): o vetor de até 200 clubes distintos de todas as fases
#   (0x944e46(0x320)); devolve a conta. Passageira.
# - 0x48cc70 (Comp::find_club_progress, 411 bytes): o id da fase, a situação e a rodada a que o clube chegou, da última
#   fase para a primeira. Passageira (telas e textos).
# Iguais no cm0102-1080.exe (as três inteiras). Chamam originais não portadas: 0x944e46 (malloc), a caixa e os métodos
# +0x10 das fases; o resto já em Rust (0x6826d0).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x48ce10: o método +0xc das vtables das competições (0x9712f0, 0x96a1d0...: mov ecx, comp; sem push)
#   -> thiscall, ret 0; devolve no al.
# - quem chama a 0x48e1c0: o método +0x58 (push x; push &lista; mov ecx, comp) -> thiscall, ret 8; devolve a conta no ax.
# - quem chama a 0x48cc70: (push &situação; push &rodada; push &fase; push clube; mov ecx, comp) -> thiscall, ret 0x10.
# - dentro: o método +0x10 push 1; push 1; mov ecx, fase -> thiscall, ret 8; 0x944e46 push 0x320; add esp, 4 -> cdecl;
#   0x6826d0 push i; mov ecx, fase -> thiscall, ret 4.
# Comparadores: a 0x48ce10 só a original (o método +0x10 marca a liga completa, grava recordes e paga prêmios: não roda
# duas vezes); a 0x48e1c0 lado a lado (a conta e os clubes das duas listas, a da cópia liberada); a 0x48cc70 lado a lado
# nas três saídas.
# Determinismo: nada lê pilha, heap ou endereço como valor.
SAVE = False
ROTINAS = [('competicao_todas_completas', 0x48ce10, [0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x1c, 0xff]),
           ('competicao_lista_de_clubes', 0x48e1c0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('competicao_progresso_do_clube', 0x48cc70, [0x83, 0xec, 0x0c, 0x53, 0x55])]
DOCS = ('a competição: todas as fases completas, a lista dos clubes e o progresso do clube',
        'the competition: all stages complete, the club list and the club progress',
        [('competicao_todas_completas', 'Comp::all_stages_complete', '0x48ce10', 'comp.cpp', 'thiscall, ret 0',
          'na competição do topo, se ela e todas as da cadeia de baixo estão completas',
          'in the top competition, whether it and every one down the chain are complete'),
         ('competicao_lista_de_clubes', 'Comp::collect_club_list', '0x48e1c0', 'comp.cpp', 'thiscall, ret 8',
          'o vetor dos clubes distintos de todas as fases (até 200) e a conta',
          'the vector of the distinct clubs of all stages (up to 200) and the count'),
         ('competicao_progresso_do_clube', 'Comp::find_club_progress', '0x48cc70', 'comp.cpp', 'thiscall, ret 0x10',
          'o id da fase, a situação e a rodada a que o clube chegou, da última fase para a primeira',
          'the stage id, the status and the round the club reached, from the last stage back')])
CAMPOS = [('competição +0xc as fases, +0x1c a de cima (índice), +0x20 a de baixo (índice em [0xadadfc]), +0x30 última fase, +0x3a id, +0x3e times, +0x43 tipo (2 liga), +0xa3 rodadas (0x68 bytes: +7 rodada, +0x1a confrontos), +0xa7 times (6 bytes: +0 clube, +5 situação), +0xab rodada atual (char), +0xad confrontos (0x35 bytes: +0/+2 os times), +0xb1 tabela (0x3b bytes: +0 clube, +0x37 situação); valor e ponteiro; lê (competicao_todas_completas, competicao_lista_de_clubes, competicao_progresso_do_clube)',
           'competition +0xc the stages, +0x1c the one above (index), +0x20 the one below (index into [0xadadfc]), +0x30 last stage, +0x3a id, +0x3e teams, +0x43 kind (2 league), +0xa3 rounds (0x68 bytes: +7 round, +0x1a ties), +0xa7 teams (6 bytes: +0 club, +5 status), +0xab current round (char), +0xad ties (0x35 bytes: +0/+2 the teams), +0xb1 table (0x3b bytes: +0 club, +0x37 status); value and pointer; reads (competicao_todas_completas, competicao_lista_de_clubes, competicao_progresso_do_clube)')]
INTERESSA = ['CMRTE']
