# A T2K (library_without_asserts): o ajuste do contorno do ajustador automatico (o autogrid da T2K; leitor rx03,
# rodada 6, 25/11/2026, terminado pela retomada; o mapa em levas/reorganizacao/proximas/TRUETYPE.md). Antes no
# rx03_8/FORA.md.
# Item de TELAS. No jogo nao roda: o ajustador so entra no desenho com T2K_GRID_FIT (cmd & 0x281), e o FONT do jogo
# chama o T2K_RenderGlyph com cmd = 2 (0x5b1c8d). Caminho no jogo: nenhum; a prova e o emulador (rascunho
# rx03b/check_fit.py: a original e a copia no mesmo glifo das fontes do jogo, analisado pelo glyph_hint_run_pass e
# ajustado nos modos 1 e 2, memoria inteira comparada; e o e2e.py com cmd 1 e 3 nas .t2k).
# Convencao conferida: cdecl (glifo, ajustador), chamada pelo glyph_hint_run_pass (rx01_7) pelo endereco.
# Igual no cm0102-1080.exe (inteira). Prologo sem call/jmp relativo (sub esp, 0x70; push ebx; push ebp).
# Comparador: so a original (muda o glifo e o ajustador).
# Depende de o rx01_1, o rx01_5, o rx01_6, o rx01_8 e o rx03_7 (glyph_hint_heap_sift_down_by_coord).
SAVE = False
TELAS = True
ROTINAS = [('glyph_hint_fit_outline_both_axes', 0x93c680, [0x83, 0xec, 0x70, 0x53, 0x55])]
_A = 'library_without_asserts'
DOCS = ('o ajustador automático da T2K: o ajuste do contorno nos dois eixos',
        'the T2K automatic hinter: fitting the outline on both axes',
        [('glyph_hint_fit_outline_both_axes', 'glyph_hint_fit_outline_both_axes', '0x93c680', _A, 'cdecl',
          'regra: ajusta o glifo à grade, primeiro em x e depois em y. Em cada eixo: monta as bordas (as hastes do eixo, os pontos de zona no y, os pontos de borda, o extremo mais baixo e o mais alto e, no x de um glifo do modo 1 que não é algarismo, os dois pontos fantasmas), ordena-as pela coordenada, agrupa as vizinhas a menos de unidades por em / 1000 + 1, dá às candidatas a zona mais próxima (ou nenhuma num glifo liso), funde as do mesmo grupo e as põe na grade em ordem: a primeira presa à zona ou arredondada, cada uma pela largura de haste a partir do par já posto ou entre as anteriores; as de zona refinam a escala, que recomeça da última zona enquanto busca; o par ainda solto vai a partir dela e as do meio são interpoladas; no fim os pontos candidatos entre duas bordas seguidas são interpolados, e depois os não tocados dos dois eixos. No modo 2, um polinômio fixo do número de contornos, de pontos e de candidatos diz se o glifo é liso. Devolve 0',
          'rule: fits the glyph to the grid, x first and then y. On each axis: builds the edges (the axis\' stems, the zone points in y, the edge points, the lowest and highest extremum and, in x for a mode 1 non-figure glyph, the two phantom points), sorts them by coordinate, groups neighbours closer than units per em / 1000 + 1, gives the candidates their nearest zone (or none on a plain glyph), merges those of one group and places them in order: the first snapped to its zone or rounded, each one by the stem width from its placed partner or between the ones before; the zone edges refine the scale, which restarts from the last zone while it searches; the loose partner is placed from it and the ones between are interpolated; at the end the candidate points between two consecutive edges are interpolated, then the untouched ones of both axes. In mode 2 a fixed polynomial of the contour, point and candidate counts says whether the glyph is plain. Returns 0')])
CAMPOS = []
INTERESSA = []
