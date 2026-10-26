"""Documentacao da 238a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, REF_JOGO, CARREIRA_PT, CARREIRA_EN

N = 238
R = [('definir_chaves_fixas', 'set_key_ids', '0x601a40', 'index.cpp', 'cdecl', 'as chaves fixas (cores, continentes, nações, competições, prêmios, clubes) em -1 e depois a linha da base pelo nome (pelo eng.lng ou pela base); as que ficam sem valor vão à caixa "Unable to find the %s index" (7 fatais pela nação); no cm0102-1080.exe pela entrada 0x603834, com o -1 nas faixas inteiras', 'the fixed keys (colours, continents, nations, competitions, awards, clubs) set to -1 and then to the database row by name (from eng.lng or from the database); the ones left without a value go to the "Unable to find the %s index" box (7 fatal by nation); in cm0102-1080.exe through the 0x603834 entry, with -1 over the whole ranges')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'dbw_continentes': ('as chaves de continente (0x9cfa08..0x9cfa1c) não decidem nada na criação do .qst nem na carga até 2001-08-28; Ásia e Europa trocadas não mudaram o save', 'the continent keys (0x9cfa08..0x9cfa1c) decide nothing in the .qst creation nor in the load to 2001-08-28; Asia and Europe swapped did not change the save'),
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
fazer(N, 770, 770 + len(R), 'as chaves fixas do jogo: os ids pela base, as cores e os continentes, e o relato das que faltam', "the game's fixed keys: ids from the database, colours and continents, and the report of the missing ones",
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'as chaves fixas do jogo: os ids pela base, as cores e os continentes, e o relato das que faltam', "the game's fixed keys: ids from the database, colours and continents, and the report of the missing ones", [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[('[0x9cf178..0x9d0527], [0xb63c9c..0xb63d03]: as chaves fixas (-1 e a linha da base); grava (definir_chaves_fixas)', '[0x9cf178..0x9d0527], [0xb63c9c..0xb63d03]: the fixed keys (-1 and the database row); writes (definir_chaves_fixas)'), ('[0x9b5984] idioma; CM3_TEMP\\eng.lng (8 contagens e as tabelas); [0xae23fc] lista de clubes; lê (definir_chaves_fixas)', '[0x9b5984] language; CM3_TEMP\\eng.lng (8 counts and the tables); [0xae23fc] club list; reads (definir_chaves_fixas)')], interessa=['SGE', 'CMRTE', 'CMScout'])
