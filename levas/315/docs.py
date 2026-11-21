"""Documentacao da 315a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, resultado_pt_en

N = 315
R = [('inter_amer_cup_start_new_season', 'InterAmerCup::start_new_season', '0x632910', 'inter_amer_cup.cpp', 'thiscall, ret 0', 'a virada da Copa Interamericana: o molde das viradas de copa (tabelas liberadas, temporada arquivada, ano + 1, os clubes, a tabela de rodadas e a árvore com as caixas nas linhas 0x164, 0x16b e 0x172, e as posições previstas)', "the Inter-American Cup's turnover: the cup turnover template (tables freed, season archived, year + 1, the clubs, the round table and the tree with the boxes at lines 0x164, 0x16b and 0x172, and the predicted positions)"), ('oceania_club_champ_start_new_season', 'OceaniaClubChamp::start_new_season', '0x797d60', 'oceania_club_champ.cpp', 'thiscall, ret 0', 'o Campeonato de Clubes da Oceania é a cada 2 anos: na virada a tabela e as datas das rodadas liberadas, a temporada arquivada, as fases destruídas, o ano + 2, a fase atual −1, o sorteio por fazer, a sede alternada entre a Austrália e a Nova Zelândia e as regras de novo', 'the Oceania Club Championship is every 2 years: at the turnover the table and the round dates freed, the season archived, the stages destroyed, the year + 2, the current stage −1, the draw still to do, the host alternated between Australia and New Zealand and the rules again'), ('alternate_oceania_club_champ_host', 'HostCountry::alternate_oceania_club_champ_host', '0x5face0', 'host_country.cpp', 'thiscall, ret 0', 'a sede do Campeonato de Clubes da Oceania alterna a cada edição: em cada entrada dele na tabela de sedes, a Austrália vira a Nova Zelândia e qualquer outra sede vira a Austrália', 'the Oceania Club Championship host alternates every edition: in each of its entries in the host table, Australia becomes New Zealand and any other host becomes Australia')]
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
fazer(N, 2767, 2767 + len(R), 'as viradas da Copa Interamericana e do Campeonato de Clubes da Oceania e a sede alternada da Oceania', 'the turnovers of the Inter-American Cup and the Oceania Club Championship and the alternating Oceania host',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'as viradas da Copa Interamericana e do Campeonato de Clubes da Oceania e a sede alternada da Oceania', 'the turnovers of the Inter-American Cup and the Oceania Club Championship and the alternating Oceania host', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[], interessa=[])
