"""Documentacao da 309a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, resultado_pt_en

N = 309
R = [('conmebol_seeding_load_from_temp_file', 'ConmebolSeeding::load_from_temp_file', '0x4c6970', 'conmebol_seeding.cpp', 'thiscall, ret 0', 'lê CM3_TEMP\\conmebol_seeding.tmp: quantos países (+0x6e), 0xa bytes por país de +0 (o ponteiro da nação +4 refeito pelo id), +0x6f (u16) e, desde a versão de dados 0x3e010, +0x71 (u32; antes, −1); sem o arquivo (jogo novo), 0; erro: a caixa "Error" sem fechar o arquivo', 'reads CM3_TEMP\\conmebol_seeding.tmp: the country count (+0x6e), 0xa bytes per country from +0 (the nation pointer +4 rebuilt from the id), +0x6f (u16) and, from data version 0x3e010 on, +0x71 (u32; before, −1); without the file (a new game), 0; on error the "Error" box without closing the file'), ('uefa_seeding_load_from_temp_file', 'UefaSeeding::load_from_temp_file', '0x905230', 'uefa_seeding.cpp', 'thiscall, ret 0', 'lê CM3_TEMP\\uefa_seeding.tmp: quantos países (+0x438), 0x12 bytes por país de +0 (o ponteiro da nação +4 refeito pelo id), +0x439 (u16), quantos u32 (+0x43f) numa lista nova em +0x43b e os 50 registros de 0xc de +0x440; sem o arquivo (jogo novo), 0; erro: a caixa "Error" sem fechar o arquivo', 'reads CM3_TEMP\\uefa_seeding.tmp: the country count (+0x438), 0x12 bytes per country from +0 (the nation pointer +4 rebuilt from the id), +0x439 (u16), as many u32 as +0x43f into a new list at +0x43b and the 50 records of 0xc from +0x440; without the file (a new game), 0; on error the "Error" box without closing the file'), ('stadium_load_from_temp_file', 'Stadium::load_from_temp_file', '0x85ba40', 'stadium.cpp', 'thiscall, ret 0', 'lê CM3_TEMP\\Stadium.tmp: as reservas (capacidade +8, quantas +4, 12 bytes cada, num vetor novo em +0), os locais (capacidade +0x14, quantos +0x10, a marca +0x18; 16 bytes cada num vetor novo em +0xc, o estádio +0xc refeito pelo id, −1 nulo) e, desde a versão de dados 0x3e029, o local guardado [0xa835a4]; sem o arquivo (jogo novo), 0; erro: a caixa "Error"', 'reads CM3_TEMP\\Stadium.tmp: the bookings (capacity +8, count +4, 12 bytes each, in a new array at +0), the venues (capacity +0x14, count +0x10, the flag +0x18; 16 bytes each in a new array at +0xc, the stadium +0xc rebuilt from its id, −1 null) and, from data version 0x3e029 on, the stashed venue [0xa835a4]; without the file (a new game), 0; on error the "Error" box')]
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
fazer(N, 2687, 2687 + len(R), 'a carga dos temporários do save: as sementes da CONMEBOL e da UEFA e as reservas dos estádios', "loading the saved game's temporary files: the CONMEBOL and UEFA seedings and the stadium bookings",
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'a carga dos temporários do save: as sementes da CONMEBOL e da UEFA e as reservas dos estádios', "loading the saved game's temporary files: the CONMEBOL and UEFA seedings and the stadium bookings", [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[('ConmebolSeeding +0 (+4 o ponteiro da nação refeito), +0x6e, +0x6f, +0x71; UefaSeeding +0 (+4), +0x438, +0x439, +0x43b, +0x43f, +0x440; Stadium +0, +4, +8, +0xc, +0x10, +0x14, +0x18; lê do .tmp e grava', 'ConmebolSeeding +0 (+4 the nation pointer rebuilt), +0x6e, +0x6f, +0x71; UefaSeeding +0 (+4), +0x438, +0x439, +0x43b, +0x43f, +0x440; Stadium +0, +4, +8, +0xc, +0x10, +0x14, +0x18; reads from the .tmp and writes')], interessa=['SGE'])
