"""Documentacao da 335a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, resultado_pt_en

N = 335
R = [('nation_menu_split_construct', 'NATION_MENU_SPLIT::NATION_MENU_SPLIT', '0x66a280', 'key_nation.cpp', 'thiscall, ret 0xc', 'monta a divisao do menu de nacoes: conta as nacoes da tabela das nacoes-chave escolhidas para jogar (so as marcadas com o bit 2, se pedido) e, na ordem dada, a nacao do meio ((contagem + 1) / 2) da a letra onde comeca a segunda parte: a primeira letra do nome curto ou do nome', 'builds the nation menu split: counts the key nation table nations chosen to be played (only those with bit 2, if asked) and, in the given order, the middle nation ((count + 1) / 2) gives the letter where the second part starts: the first letter of its short name or name'), ('nation_menu_split_format_header', 'NATION_MENU_SPLIT::format_header', '0x66a360', 'key_nation.cpp', 'thiscall, ret 0xc', 'escreve o cabecalho de uma parte do menu: "Nacoes A-{letra}" na primeira, "Nacoes {letra seguinte}-Z" na segunda; texto de menos de 20 bytes: a caixa "Error" e nada', 'writes the header of one menu part: "Nations A-{letter}" for the first, "Nations {next letter}-Z" for the second; a text under 20 bytes: the "Error" box and nothing'), ('nation_menu_split_in_second_part', 'NATION_MENU_SPLIT::in_second_part', '0x66a430', 'key_nation.cpp', 'thiscall, ret 4', 'diz se a nacao vai para a segunda parte: a primeira letra do nome curto (ou do nome) depois da letra da divisao', 'tells whether the nation goes in the second part: the first letter of its short name (or name) after the split letter')]
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
fazer(N, 3918, 3918 + len(R), 'o menu de nacoes partido em duas partes', 'the nation menu split in two parts',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o menu de nacoes partido em duas partes', 'the nation menu split in two parts', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[], interessa=[])
