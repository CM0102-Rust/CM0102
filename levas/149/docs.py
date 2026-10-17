"""Documentacao da 149a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva149.py` (8, pela carreira até 2002-06-01): 7 pegas e 1 lacuna (`e_ficha`: o segundo campo da ficha do técnico não pesa na carreira automática; ver `PENDENCIAS.md`). "
     "Sem comparação própria: a escalação de cada lado chama a automática (0x6dbc50, ainda na original, que aloca a lista e lê a pilha) e o dia de jogos libera motores e listas; validadas pela carreira (Rust × original). As rotinas que elas chamam seguem comparadas. "
     "Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba")
ME = ("`mutacoes/leva149.py` (8, by the career to 2002-06-01): 7 caught and 1 gap (`e_ficha`: the second field of the manager's record does not matter in the automatic career; see `PENDENCIAS.en.md`). "
      "No comparison of their own: each side's selection calls the automatic one (0x6dbc50, still in the original, which allocates the list and reads the stack) and the match day frees engines and lists; validated by the career (Rust × original). The routines they call stay compared. "
      "Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba")
fazer(149, 329, 332, 'a escalação de cada lado e o dia de jogos', "each side's selection and the match day",
      ["| `escalar_lado()` | 0x6db500 | thiscall, ret 0x24 | match_man.cpp: monta a tática de um lado (técnico, jogadores, clube, adversário, partida, mando) e escala: no clube do humano as linhas guardadas (0x5fbe20/0x5fbd00) e a automática 0x6dbc50 modo 1 ou os titulares (0x6dbb80); fora isso a automática modo 2 e a conta dos da casa; os reservas pelo banco e a linha tática do clube. Devolve quantos escalou |",
       "| `encerrar_dia()` | 0x6b4190 | thiscall | match_day.cpp: libera a lista global, os vetores do dia e, de cada partida, o motor (0x6b97a0), o resumo e o objeto +0x5f |",
       "| `escolher_partida_do_dia()` | 0x6b42a0 | thiscall | match_day.cpp: limpa os campos de cada partida do dia e escolhe a do humano (ou a primeira marcada) para a tela (0x717cc0, 0x7fc7e0) |"],
      ["| `escalar_lado()` | 0x6db500 | thiscall, ret 0x24 | match_man.cpp: sets up one side's tactics (manager, players, club, opponent, match, home side) and selects: at the human's club the stored lines (0x5fbe20/0x5fbd00) and the automatic 0x6dbc50 mode 1 or the regulars (0x6dbb80); otherwise the automatic mode 2 and the count of the home players; the substitutes by the bench and the club's tactical line. Returns how many it selected |",
       "| `encerrar_dia()` | 0x6b4190 | thiscall | match_day.cpp: frees the global list, the day's vectors and, for each match, the engine (0x6b97a0), the summary and the +0x5f object |",
       "| `escolher_partida_do_dia()` | 0x6b42a0 | thiscall | match_day.cpp: clears each of the day's matches' fields and picks the human's (or the first marked) for the screen (0x717cc0, 0x7fc7e0) |"],
      M, ME,
      [('escalar_lado', 'select_side'), ('encerrar_dia', 'end_day'), ('escolher_partida_do_dia', 'pick_day_match')],
      "'destruir_motor': 'destroy_engine',")
