"""Documentacao da 239a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, REF_JOGO, CARREIRA_PT, CARREIRA_EN

N = 239
R = [('inicializar_jogadores', 'database_player_initialisation', '0x535a80', 'Database.cpp', 'cdecl', 'prepara cada pessoa do jogo novo: nomes que faltam, datas, atributos de pessoa, nação; do jogador as habilidades, o potencial, a atual, a idade, as posições e os atributos que faltam (pelo jogador mais parecido entre 250 sorteados); do não jogador as reputações, os atributos (o molde entre 250), o cargo e os nomes famosos; depois os vetores relativos à habilidade e as normalizações; barra "Player Initialisation"', 'prepares every person of the new game: missing names, dates, staff attributes, nation; for the player the abilities, potential, current ability, age, positions and missing attributes (from the most similar of 250 random players); for the non-player the reputations, attributes (template among 250), job and the famous names; then the ability-relative vectors and the normalisations; "Player Initialisation" bar')]
ESCAPADAS = {}  # nome: (explicação pt, explicação en), quando houver
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
fazer(N, 771, 771 + len(R), 'a criação do jogo: inicializar os jogadores e as pessoas', 'game creation: initialising the players and the staff',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'a criação do jogo: inicializar os jogadores e as pessoas', 'game creation: initialising the players and the staff', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[('TStaff +4/+8/+0xc nomes, +0x10 nascimento, +0x18 idade, +0x1a nação, +0x31/+0x3e/+0x46 datas, +0x39 clube, +0x3d cargo, +0x56..+0x5d atributos; grava (inicializar_jogadores)', 'TStaff +4/+8/+0xc names, +0x10 birth date, +0x18 age, +0x1a nation, +0x31/+0x3e/+0x46 dates, +0x39 club, +0x3d job, +0x56..+0x5d attributes; writes (inicializar_jogadores)'), ('TPlayer +5 atual, +7 potencial, +9/+0xb/+0xd habilidades (×50), +0x45, posições, lados e atributos; grava (inicializar_jogadores)', 'TPlayer +5 current, +7 potential, +9/+0xb/+0xd abilities (×50), +0x45, positions, sides and attributes; writes (inicializar_jogadores)'), ('TNonPlayer +4/+6 habilidades, +8/+0xa/+0xc reputações (×50), atributos, +0x23..+0x42 zerados, +0x43; grava (inicializar_jogadores)', 'TNonPlayer +4/+6 abilities, +8/+0xa/+0xc reputations (×50), attributes, +0x23..+0x42 cleared, +0x43; writes (inicializar_jogadores)'), ('tabela de 79 bytes por pessoa [0xae2d0c]: +9 = 0x9c (jogador), +0xb &= 0xf0; grava (inicializar_jogadores)', '79-byte per-person table [0xae2d0c]: +9 = 0x9c (player), +0xb &= 0xf0; writes (inicializar_jogadores)'), ('vetores dos moldes 0xae2044/0xae2114 (bandeira 0x9b59b8) e 0xae18bc/0xae2238 (0x9b59b9); grava (inicializar_jogadores)', 'template vectors 0xae2044/0xae2114 (flag 0x9b59b8) and 0xae18bc/0xae2238 (0x9b59b9); writes (inicializar_jogadores)')], interessa=['SGE', 'CMRTE'])
