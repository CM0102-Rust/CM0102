"""Documentacao da 326a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, resultado_pt_en

N = 326
R = [('euro_champ_qual_draw_groups', 'EuroChampQual::draw_groups', '0x57dab0', 'euro_champ_qual.cpp', 'thiscall, ret 0', 'O sorteio dos grupos das eliminatórias da Eurocopa: em 1998, a tabela fixa (49 seleções em 9 grupos, os grupos 1, 3, 6 e 8 com seis); nos outros anos, 9 grupos (+0xff + 2) e até seis potes: a ordem dos grupos é embaralhada com 20 trocas sorteadas e cada grupo, na ordem, recebe a próxima seleção enquanto tiver vaga; defeito: a vaga de cada grupo é a do grupo anterior (g − 1), então no formato 1 os grupos de seis ficam 1, 3 e 6', 'The groups draw of the European Championship qualifiers: in 1998, the fixed table (49 national teams in 9 groups, groups 1, 3, 6 and 8 with six); in other years, 9 groups (+0xff + 2) and up to six pots: the group order is shuffled by 20 drawn swaps and each group in turn takes the next national team while it has room; defect: each group room is that of the previous group (g − 1), so in format 1 the groups of six are 1, 3 and 6')]
# the escapes are explained in escapadas.py next to this file, when there are any (18/11/2026)
_e = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'escapadas.py')
ESCAPADAS = {k: (a.rstrip('.'), b.rstrip('.')) for k, (a, b) in (__import__('runpy').run_path(_e)['ESCAPADAS'].items() if os.path.exists(_e) else [])}  # nome: (pt, en)
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia com divergência ou carreira que não chegou à data'
RES_PT, RES_EN = resultado_pt_en(N)
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
PS = prova_save(N)  # leva do save: mutações e prova pelo .qst gravado com e sem as rotinas (cadeia.sh)
VIA_PT = 'pela prova do save' if PS else 'pela carreira até 2002-06-01, confirmando no modo comparação as que escapam'
VIA_EN = 'by the save proof' if PS else 'by the career to 2002-06-01, confirming the escaped ones in comparison mode'
PS_PT = f'Prova do save (o .qst com e sem as rotinas): {PS}. ' if PS else ''
PS_EN = f'Save proof (the .qst with and without the routines): {PS.replace("IGUAL", "EQUAL").replace("DIFERENTE", "DIFFERENT")}. ' if PS else ''
M = (f"`mutacoes/leva{N}.py` ({tot}, {VIA_PT}): {ac} acusadas. {lac_pt} {PS_PT}"
     f"{RES_PT}")
ME = (f"`mutacoes/leva{N}.py` ({tot}, {VIA_EN}): {ac} caught. {lac_en} {PS_EN}"
      f"{RES_EN}")
fazer(N, 3789, 3789 + len(R), 'o sorteio das eliminatórias da Eurocopa', 'the European Championship qualifying draw',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o sorteio das eliminatórias da Eurocopa', 'the European Championship qualifying draw', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[], interessa=[])
