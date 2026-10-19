"""Documentacao da 178a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 178
R = [('media_do_jogo', 'game_average', '0x5d5a60', 'game.cpp', 'thiscall', 'a média f64 +0x20/+0x28 ou +0x10/+0x18 (divisor zero: 6.8)', 'the f64 average +0x20/+0x28 or +0x10/+0x18 (zero divisor: 6.8)'), ('valor_do_contrato', 'contract_value', '0x4d0c20', 'contract_manager.cpp', 'thiscall', 'o dword +0xc da ficha de contrato da pessoa', "dword +0xc of the person's contract record"), ('nivel_para_a_selecao', 'national_team_level', '0x76d810', 'national_teams.cpp', 'stdcall', 'o nível 1..3 do jogador para a seleção pela reputação e pelas notas de posição', "the player's level 1..3 for the national team by reputation and position ratings")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'm_padrao': ('o divisor da média (+0x18/+0x28) nunca é zero quando a carreira consulta a média: o 6,8 padrão não aparece', 'the average divisor (+0x18/+0x28) is never zero when the career asks for the average: the default 6.8 never shows'),
}
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia sem a referencia'
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
M = (f"`mutacoes/leva{N}.py` ({tot}, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): {ac} acusadas. {lac_pt} "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = (f"`mutacoes/leva{N}.py` ({tot}, by the career to 2002-06-01, confirming the escaped ones in comparison mode): {ac} caught. {lac_en} "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(N, 407, 407 + 3, 'a média do jogo, o valor do contrato e o nível para a seleção', 'the game average, the contract value and the national team level',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'a média do jogo, o valor do contrato e o nível para a seleção', 'the game average, the contract value and the national team level', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
