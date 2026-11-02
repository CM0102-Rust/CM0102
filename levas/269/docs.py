"""Documentacao da 269a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, resultado_pt_en

N = 269
R = [('liga_jogos_para_a_tv', 'League::move_fixtures_for_tv', '0x685640', 'league.cpp', 'thiscall, ret 4', 'nas datas da tabela da TV, cada vaga leva o jogo de maior nota (sorteio pelas reputações, zerado com jogo de um dos clubes 2..5 dias antes ou depois; 100/50 nas vagas de seleção) para o dia da semana e a fatia da vaga, com a notícia, o estádio e o calendário', 'on the TV table dates, each slot takes the highest-rated fixture (a draw by reputation, zeroed when either club plays 2..5 days before or after; 100/50 in national team slots) to the weekday and time slot of the slot, with the news, the stadium and the calendar'), ('liga_noticia_dos_grupos', 'League::send_group_draw_news', '0x684230', 'league.cpp', 'thiscall, ret 0', 'a notícia 0x15 do grupo (a competição, a primeira data, os adversários) a cada clube de técnico humano', 'group news 0x15 (the competition, the first date, the opponents) to each human-managed club'), ('liga_noticia_da_tabela', 'League::send_fixture_list_news', '0x683ee0', 'league.cpp', 'thiscall, ret 4', 'a notícia 0xd com os três primeiros jogos (data, adversário, mando) a cada clube de técnico humano ainda não avisado, e o mesmo nas fases seguintes', 'news 0xd with the first three fixtures (date, opponent, venue) to each human-managed club not yet told, and the same in the following stages')]
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
fazer(N, 1595, 1595 + len(R), 'a liga: os jogos de maior nota levados ao dia e à fatia da TV, a notícia do sorteio dos grupos e a notícia dos primeiros jogos', 'the league: the highest-rated fixtures moved to the TV day and slot, the group draw news and the opening fixtures news',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'a liga: os jogos de maior nota levados ao dia e à fatia da TV, a notícia do sorteio dos grupos e a notícia dos primeiros jogos', 'the league: the highest-rated fixtures moved to the TV day and slot, the group draw news and the opening fixtures news', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[('liga +0xba (tabela das datas da TV, 0x41 bytes por entrada; conta +0xa9) +0 dia, +2 ano − o da liga, +4 fatia, +5.. vagas de 7 bytes (+0 estádio, +4 dia da semana, +5 fatia da TV, +6 tipo: 0xff = vazia); valor; lê e grava (liga_jogos_para_a_tv: a vaga usada fica 0xff, a ordem pela 0x945333), lê (liga_noticia_dos_grupos, liga_noticia_da_tabela)', 'league +0xba (TV date table, 0x41 bytes per entry; count +0xa9) +0 day, +2 year − the league one, +4 slot, +5.. 7-byte slots (+0 stadium, +4 weekday, +5 TV slot, +6 type: 0xff = empty); value; reads and writes (liga_jogos_para_a_tv: the used slot becomes 0xff, the order by 0x945333), reads (liga_noticia_dos_grupos, liga_noticia_da_tabela)'), ('liga +0x40 ano, +0x44 tipo/fase (0xff primeira fase), +0x3e times, +0xb1 times de 0x3b bytes (+0 clube), +0x24, +0x3a, +0xc/+0x30 as fases (+0x43); ponteiro e valor; lê (liga_jogos_para_a_tv, liga_noticia_dos_grupos, liga_noticia_da_tabela)', 'league +0x40 year, +0x44 type/stage (0xff first stage), +0x3e teams, +0xb1 0x3b-byte teams (+0 club), +0x24, +0x3a, +0xc/+0x30 the stages (+0x43); pointer and value; reads (liga_jogos_para_a_tv, liga_noticia_dos_grupos, liga_noticia_da_tabela)'), ('partida (0x4f bytes no calendário) +0xc/+0x10 ids, +0x18 estádio, +0x1c/+0x20 clubes, +0x28 ano, +0x2a dia, +0x3b tipo, +0x3f fatia, +0x42 tipo da liga; valor; lê e grava (liga_jogos_para_a_tv), lê (liga_noticia_da_tabela)', 'fixture (0x4f bytes in the calendar) +0xc/+0x10 ids, +0x18 stadium, +0x1c/+0x20 clubs, +0x28 year, +0x2a day, +0x3b type, +0x3f slot, +0x42 league type; value; reads and writes (liga_jogos_para_a_tv), reads (liga_noticia_da_tabela)'), ('clube +0x80 reputação, +0x53 nação; valor; lê (liga_jogos_para_a_tv)', 'club +0x80 reputation, +0x53 nation; value; reads (liga_jogos_para_a_tv)')], interessa=['CMRTE'])
