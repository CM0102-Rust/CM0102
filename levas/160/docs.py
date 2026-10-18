"""Documentacao da 160a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia

R = [('estatistica_do_jogador', '0x7c2150', 'player_stats.cpp, stdcall', 'a estatística `qual` 1..22 do registro (bytes, porcentagens ×100.0f, médias por jogo, as duas notas completadas com menos de 20 jogos ×0.05f), em x87 como a original', 'statistic `qual` 1..22 of the record (bytes, percentages ×100.0f, per-match averages, the two ratings completed with under 20 matches ×0.05f), in x87 like the original'),
     ('media_das_notas', '0x7c1950', 'player_stats.cpp, thiscall', 'a média das cinco notas de 4 bits do registro (zero conta 27), em 16 bits dividida por 4', 'the average of the record\'s five 4-bit ratings (zero counts 27), 16-bit, divided by 4'),
     ('zagueiros_sem_conflito', '0x5b8dc0', 'formation.cpp, thiscall', '0 se alguma vaga com lado aberto é de zagueiro ou volante', '0 if any slot with an open side is a centre-back\'s or defensive midfielder\'s')]
M = ("`mutacoes/leva160.py` (7, pela carreira até 2002-06-01): as 7 acusadas. A estatística em ponto flutuante (divisões e produtos em f64) bate com o x87 da original em toda a carreira. "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = ("`mutacoes/leva160.py` (7, by the career to 2002-06-01): all 7 caught. The floating-point statistic (f64 divisions and products) matches the original's x87 over the whole career. "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(160, 353, 356, 'estatísticas do jogador e zagueiros na formação', 'player statistics and centre-backs in the formation',
      [f"| `{a}()` | {e} | {f.split(', ')[1]} | {f.split(',')[0]}: {d} |" for a, e, f, d, _ in R],
      [f"| `{a}()` | {e} | {f.split(', ')[1]} | {f.split(',')[0]}: {d} |" for a, e, f, _, d in R],
      M, ME,
      [('estatistica_do_jogador', 'player_statistic'), ('media_das_notas', 'ratings_average'), ('zagueiros_sem_conflito', 'centre_backs_without_conflict')],
      "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(160, 'estatísticas do jogador e zagueiros na formação', 'player statistics and centre-backs in the formation', R, 7, 7)
