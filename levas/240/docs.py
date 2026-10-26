"""Documentacao da 240a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, REF_JOGO, CARREIRA_PT, CARREIRA_EN

N = 240
R = [('adjetivo_da_nacao', 'nation_get_nationality_adjective', '0x6181d0', 'index.cpp', 'cdecl', 'o adjetivo da nação no texto (tamanho >= 50): com gênero 0 ou fora dos idiomas 9, 4, 5, 10 e 6 o nome da nação; nesses, a forma da 0x545a60 e as cadeias de 937 nações (geradas do exe por adjetivos_nacao.py, textos pelo endereço), com o nome se o texto ficar vazio (menos nos idiomas 4 e 5); devolve 1 (0 com nação ou texto nulo ou tamanho curto)', "the nation's adjective into the text (size >= 50): with gender 0 or outside languages 9, 4, 5, 10 and 6 the nation's name; in those, the form from 0x545a60 and the chains of 937 nations (generated from the exe by adjetivos_nacao.py, texts by address), with the name if the text stays empty (except in languages 4 and 5); returns 1 (0 with a null nation or text or a short size)")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'dbx_forma': ('a forma 5 (min(5) → min(4)) só aparece nas tabelas de italiano e espanhol, que o jogo em inglês não usa; escapou também no jogo real comparando (0 divergências)', 'form 5 (min(5) → min(4)) only appears in the Italian and Spanish tables, which the English game does not use; also escaped the real game comparing (0 divergences)'),
    'dbx_vazio': ('o nome no fim só muda algo nos idiomas 4 e 5 (italiano, espanhol); escapou também no jogo real comparando (0 divergências)', 'the name at the end only changes something in languages 4 and 5 (Italian, Spanish); also escaped the real game comparing (0 divergences)'),
}
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia sem a referencia'
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
PS = prova_save(N)  # leva do save: mutações e prova pelo .qst gravado com e sem as rotinas (cadeia.sh)
VIA_PT = 'pela prova do save' if PS else 'pela carreira até 2002-06-01, confirmando no modo comparação as que escapam'
VIA_EN = 'by the save proof' if PS else 'by the career to 2002-06-01, confirming the escaped ones in comparison mode'
PS_PT = f'Prova do save (o .qst com e sem as rotinas): {PS}. ' if PS else ''
PS_EN = f'Save proof (the .qst with and without the routines): {PS.replace("IGUAL", "EQUAL").replace("DIFERENTE", "DIFFERENT")}. ' if PS else ''
M = (f"`mutacoes/leva{N}.py` ({tot}, {VIA_PT}): {ac} acusadas. {lac_pt} {PS_PT}"
     f"Jogo real comparando até 2002-01-01: 0 divergências ({REF_JOGO}). Carreiras Rust ×2 e original até 2002-06-01: {CARREIRA_PT}")
ME = (f"`mutacoes/leva{N}.py` ({tot}, {VIA_EN}): {ac} caught. {lac_en} {PS_EN}"
      f"Real game comparing to 2002-01-01: 0 divergences ({REF_JOGO}). Rust ×2 and original careers to 2002-06-01: {CARREIRA_EN}")
fazer(N, 772, 772 + len(R), 'o adjetivo de nacionalidade pelo idioma e pela forma', 'the nationality adjective by language and form',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o adjetivo de nacionalidade pelo idioma e pela forma', 'the nationality adjective by language and form', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[('TNation +0 id, +0x57 nome; [0x9cf178..0x9d0527] chaves das nações; [0x9eb2dc] idioma; lê (adjetivo_da_nacao)', 'TNation +0 id, +0x57 name; [0x9cf178..0x9d0527] nation keys; [0x9eb2dc] language; reads (adjetivo_da_nacao)')], interessa=['SGE', 'CMRTE', 'CMScout'])
