"""Documentacao da 331a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, resultado_pt_en

N = 331
R = [('club_records_record_type_applies', 'club_records_record_type_applies', '0x449590', 'club_records.cpp', 'thiscall (ecx nao lido), ret 0xc', 'diz se o tipo de recorde (1 a 32) vale para o clube no periodo (1 de todos os tempos, 2 a temporada): os tipos so de clube (posicoes na liga, publico, resultados, jogos e gols na liga, transferencias...) valem para quem nao e selecao; o 27 (convocacoes) so para selecao; o 32 (o jogador da torcida no ano) so para clube e nunca no periodo de todos os tempos; os demais, e um tipo fora de 1..32, valem sempre. Clube nulo: a caixa de erro e 0', "tells whether record type (1 to 32) applies to the club in the period (1 all-time, 2 the season): the club-only types (league positions, attendance, results, league appearances and goals, transfers...) apply to anyone who is not a national team; 27 (caps) only to national teams; 32 (the fans' player of the year) only to clubs and never in the all-time period; the others, and a type outside 1..32, always. A null club: the error box and 0"), ('club_records_end_season_for_nation', 'Club_Records::end_season_for_nation', '0x447880', 'club_records.cpp', 'thiscall, ret 4', 'fecha a temporada de cada clube da nacao-chave: o publico medio da temporada acima de -1 que supera o de todos os tempos (-1 = nenhum) vai para o de todos os tempos com o ano; a nota da temporada ainda em -1,0 e calculada; a nota media da temporada acima de -1,0 que supera a de todos os tempos e copiada com o ano; em cada recorde batido, a noticia entra na fila (clube humano ou selecao com 5 temporadas ou mais, sem repetir); com registro da temporada, o jogador da torcida no ano (tipo 32): quando ha um, a noticia 0x3e8 para o clube de tecnico humano e o registro dos torcedores da pessoa (com o ano e a marca de temporada dividida da nacao); a temporada vai para o arquivo do historico (o ano e o conjunto de registros de 0x38b bytes) e o registro da temporada e zerado (-1); sempre, uma temporada a mais na entrada do clube e os contadores da temporada zerados. Devolve 1; nacao nula: a caixa de erro e 0. Defeito do original: o publico e a nota leem o registro de todos os tempos antes de testar se e nulo (so depois o criam); como a partida cria os dois juntos, a criacao nunca roda', "closes the season of every club of the key nation: a season average attendance above -1 that beats the all-time one (-1 = none) goes to the all-time record with the year; a season rating still at -1.0 is worked out; a season average rating above -1.0 that beats the all-time one is copied with the year; for each broken record the news is queued (a human club or national team with 5 seasons or more, without repeats); with a season record, the fans' player of the year (type 32): when there is one, news item 0x3e8 for a club with a human manager and the person's supporters' record (with the year and the nation's split-season flag); the season goes to the history file (the year and the 0x38b-byte record set) and the season record is reset (-1); always, one more season in the club's entry and the season counters cleared. Returns 1; a null nation: the error box and 0. Defect of the original: the attendance and rating checks read the all-time record before testing it for null (and only then create it); since the match creates both together, the creation never runs")]
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
fazer(N, 3850, 3850 + len(R), 'os registros dos clubes: os tipos de recorde de cada clube e o fim da temporada de uma nacao', "club records: the record types of each club and the end of a nation's season",
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'os registros dos clubes: os tipos de recorde de cada clube e o fim da temporada de uma nacao', "club records: the record types of each club and the end of a nation's season", [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[], interessa=[])
