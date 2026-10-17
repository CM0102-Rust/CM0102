"""Documentacao da 148a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva148.py` (18, pela carreira até 2002-06-01) e `mutacoes/leva148b.py` (as 4 que escaparam, refeitas mais fortes): 16 pegas pela carreira e 2 lacunas (`i_ceu`: o limite do céu com chuva não pesa na carreira automática; `i_insatisfeito`: o código do evento 0x13 não muda o resultado; ver `PENDENCIAS.md`). "
     "Sem comparação própria: as três alocam (o narrador e a lista de eventos) ou liberam, e a fita teria de reproduzir as alocações; validadas pela carreira (Rust × original), como o laço do motor. As rotinas que elas chamam seguem comparadas. A tabela de ângulos (0xb67c30), calculada na primeira partida com fpatan, sai do atan do Rust e bate. "
     "Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba")
ME = ("`mutacoes/leva148.py` (18, by the career to 2002-06-01) and `mutacoes/leva148b.py` (the 4 that escaped, redone stronger): 16 caught by the career and 2 gaps (`i_ceu`: the rainy-sky limit does not matter in the automatic career; `i_insatisfeito`: the code of event 0x13 does not change the result; see `PENDENCIAS.en.md`). "
      "No comparison of their own: the three allocate (the commentator and the event list) or free, and the tape would have to reproduce the allocations; validated by the career (Rust × original), like the engine loop. The routines they call stay compared. The angle table (0xb67c30), computed in the first match with fpatan, comes from Rust's atan and matches. "
      "Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba")
fazer(148, 326, 329, 'o motor nasce, prepara a partida e morre', 'the engine is born, prepares the match and dies',
      ["| `construir_motor()` | 0x6b7e50 | thiscall | match_eng.cpp: o construtor do motor (os vetores de jogadores, táticas e linhas táticas pelo construtor de vetores 0x9463af; o rigor 0x6e9460; a lista vazia) |",
       "| `preparar_partida()` | 0x6b7f20 | thiscall, ret 0x10 | match_eng.cpp (6 KB): zera o estado, o clima (0x923010/0x923970/0x923630), os uniformes, o árbitro, o gramado (0x85cba0, +0x1ad/+0x1ae), os elencos (0x6db500), o narrador (dado ou novo), a força dos lados, a marcação, a tabela de ângulos e a lista de eventos de pré-jogo (insatisfação 0x13, interesse 0x1e, forma 5/4, fim de contrato 6); sorteia quem começa |",
       "| `destruir_motor()` | 0x6b97a0 | thiscall | match_eng.cpp: o destrutor (o narrador próprio, a lista, o rigor e os vetores) |"],
      ["| `construir_motor()` | 0x6b7e50 | thiscall | match_eng.cpp: the engine constructor (the player, tactics and tactical line vectors through the runtime vector constructor 0x9463af; the referee strictness 0x6e9460; the empty list) |",
       "| `preparar_partida()` | 0x6b7f20 | thiscall, ret 0x10 | match_eng.cpp (6 KB): clears the state, the weather (0x923010/0x923970/0x923630), the kits, the referee, the pitch (0x85cba0, +0x1ad/+0x1ae), the squads (0x6db500), the commentator (given or new), the side strength, the marking, the angle table and the pre-match event list (unhappiness 0x13, interest 0x1e, form 5/4, contract end 6); draws who kicks off |",
       "| `destruir_motor()` | 0x6b97a0 | thiscall | match_eng.cpp: the destructor (its own commentator, the list, the strictness and the vectors) |"],
      M, ME,
      [('construir_motor', 'build_engine'), ('preparar_partida', 'prepare_match'), ('destruir_motor', 'destroy_engine')],
      "'chutar': 'shoot',")
