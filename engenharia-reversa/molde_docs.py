"""Documentacao da %%N%%a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, resultado_pt_en

N = %%N%%
R = %%R%%
ESCAPADAS = {}  # nome: (explicação pt, explicação en), quando houver
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
fazer(N, %%ANTES%%, %%ANTES%% + len(R), %%TPT%%, %%TEN%%,
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, %%TPT%%, %%TEN%%, [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=%%CAMPOS%%, interessa=%%INTERESSA%%)
