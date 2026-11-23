"""Documentacao da 325a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, resultado_pt_en

N = 325
R = [('plan_preseason_tournaments_and_tours', 'Friendly::plan_preseason_tournaments_and_tours', '0x5c3040', 'friendly.cpp', 'thiscall, ret 0xc', 'Na pré-temporada de cada liga escolhida (temporada ainda não começada e a 52 semanas do fim): a partir de 2002, um torneio por nação: entre os clubes da IA sem lista de datas e fora da Intertoto, o maior sorteio da faixa de reputação é a sede (e um sorteio de 10 tem de dar diferente de 0), que recebe três adversários de amistoso (até 50 tentativas cada, sem lista de datas), todos com todos na lista de adversários, numa copa ou minliga (meio a meio); defeito: o contador do laço não é iniciado antes do primeiro torneio e o limite é o byte baixo da reputação da nação. Todo ano, as excursões: cada clube da IA sem adversários, com liga, com até 4 amistosos e que passa num sorteio de 3, com uma nação de excursão, enfrenta clubes dela até ter 3 ou 4 amistosos (faixa = reputação / 500 + sorteio, × 0,8 a cada adversário, 10 abaixo de 1) e, com mais de um adversário, a excursão é marcada', 'In the pre-season of each selected league (season not started yet and within 52 weeks of the end): from 2002 on, one tournament per nation: among the AI clubs without a date list and outside the Intertoto, the highest draw of the reputation band hosts (and a draw of 10 must be nonzero), taking three friendly opponents (up to 50 tries each, without a date list), all against all in the opponents lists, in a cup or a mini league (half and half); defect: the loop counter is not initialised before the first tournament and the bound is the low byte of the nation reputation. Every year, the tours: each AI club without opponents, with a league, with at most 4 friendlies and passing a draw of 3, given a tour nation, meets its clubs until it has 3 or 4 friendlies (band = reputation / 500 + draw, × 0.8 after each opponent, 10 below 1) and, with more than one opponent, the tour is scheduled'), ('construct_tactics', 'Tactics::Tactics', '0x893dc0', 'tactics.cpp', 'thiscall, ret 4', 'Confere o CD (o primeiro drive de CD com cm0102.exe; sem ele, a caixa "Insert CD" e de novo); zera os 16 espaços de desfazer; com o save, as táticas vêm dele. Num jogo novo: as 28 formações da IA e as 17 pré-definidas vêm dos arquivos de CM3_TACTICS, na ordem (442_wide.pct duas vezes), cada uma no próximo espaço enquanto houver espaço (o arquivo que falha não conta; sem espaço, a caixa de erro da linha dele; faltando formação no fim, a caixa 0x92 ou 0xb4), e as pré-definidas ganham o nome de exibição ("4-4-2", "3-5-2 Defensive"...); uma linha de escalação vazia por clube e mais 32 (0x91 bytes, tudo −1 depois dos 2 primeiros); as estatísticas por tática (45 de 0x28 bytes, zeradas com o índice) e 9 constantes em +0xc..+0x2c', 'Checks the CD (the first CD drive holding cm0102.exe; without it, the "Insert CD" box and again); zeroes the 16 undo slots; with the save, the tactics come from it. In a new game: the 28 AI formations and the 17 preset ones come from the CM3_TACTICS files, in order (442_wide.pct twice), each into the next slot while there is room (a file that fails does not count; without room, the box of its line; formations missing at the end, the box at 0x92 or 0xb4), and the presets get their display name ("4-4-2", "3-5-2 Defensive"...); an empty selection row per club plus 32 (0x91 bytes, all −1 after the first 2); the per-tactic statistics (45 of 0x28 bytes, zeroed with the index) and 9 constants at +0xc..+0x2c')]
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
fazer(N, 3787, 3787 + len(R), 'os torneios e as excursões de pré-temporada da IA; a criação das táticas', 'the AI pre-season tournaments and tours; building the tactics store',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'os torneios e as excursões de pré-temporada da IA; a criação das táticas', 'the AI pre-season tournaments and tours; building the tactics store', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[], interessa=[])
