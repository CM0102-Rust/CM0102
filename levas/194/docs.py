"""Documentacao da 194a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 194
R = [('tirar_reserva_de_tecnico', 'PLAYER_REGEN::remove_spare_non_player_record', '0x7a82b0', 'player_regen.cpp', 'thiscall', 'tira da lista de reserva o registro de não jogador de um id', 'removes a non-player record from the spare list by id'), ('tirar_jogador_do_banco', 'PLAYER_REGEN::remove_player_data_from_pool', '0x7a8470', 'player_regen.cpp', 'thiscall', 'tira do banco de jogadores livres o registro dos dados de jogador', 'removes the player data record from the free player pool'), ('premio_da_liga', 'League::calc_prize_fund', '0x682f70', 'league.cpp', 'thiscall', 'o prêmio total da liga (uma cota a mais por posição acima do último)', 'the league total prize fund (one more share per place above the last)')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'r_marca': ('o campo +8 da vaga que fica livre no fim da lista de reserva nunca é lido antes de ser regravado', 'field +8 of the slot freed at the end of the reserve list is never read before being rewritten'),
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
fazer(N, 455, 455 + 3, 'as listas de reserva do gerador de jogadores e o prêmio da liga', 'the player generator reserve lists and the league prize fund',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'as listas de reserva do gerador de jogadores e o prêmio da liga', 'the player generator reserve lists and the league prize fund', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
