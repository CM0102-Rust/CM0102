"""Documentacao da 154a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva154.py` (6, pela carreira até 2002-06-01): as 6 escaparam, e os contadores de ramo explicam: a gravação dos eventos roda (\"eventos arquivo com espaco\" acima de 2048, \"eventos arquivo novo\" 2), mas vai para arquivos temporários que não entram nas impressões; a leitura de volta e o passo em rede não rodam na carreira automática. Viram 2 lacunas em `PENDENCIAS.md`; o teste delas é no jogo (rever os lances de uma partida assistida). "
     "Sem comparação própria: mexem em arquivos e só rodam com a tela ou em rede; validadas pela carreira (Rust × original). "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = ("`mutacoes/leva154.py` (6, by the career to 2002-06-01): all 6 escaped, and the branch counters explain it: recording the events runs (\"eventos arquivo com espaco\" above 2048, \"eventos arquivo novo\" 2), but goes to temporary files that are not part of the fingerprints; reading them back and the network step do not run in the automatic career. They become 2 gaps in `PENDENCIAS.en.md`; their test is in the game (replaying the plays of a watched match). "
      "No comparison of their own: they touch files and only run with the screen or on a network; validated by the career (Rust × original). "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(154, 340, 343, 'os eventos gravados e o passo em rede', 'the recorded events and the network step',
      ["| `gravar_eventos()` | 0x6d98b0 | thiscall, ret 8 | match_events.cpp: grava os eventos (+0x720, 0xe bytes cada) num dos dois arquivos \"events1/2.tmp\" (humano ou computador) de CM3_TEMP; cabeçalho versão 1, número, quantos; cheios os dois (2500), recria o mais antigo. Em `desc`: posição, quantos, número, humano |",
       "| `ler_eventos()` | 0x6d9c40 | thiscall, ret 4 | match_events.cpp: lê de volta os eventos descritos em `desc` do arquivo com o número certo |",
       "| `passo_em_rede()` | 0x6b55c0 | thiscall, ret 4 | match_day.cpp: um passo da partida do humano em rede, pelo relógio e pelo placar dos humanos (todos veem cada estado; sons, eventos, passo do motor, fim, intervalo, gols, minuto); registra (0x729ff0) e escolhe a partida do humano do dia |"],
      ["| `gravar_eventos()` | 0x6d98b0 | thiscall, ret 8 | match_events.cpp: records the events (+0x720, 0xe bytes each) in one of the two \"events1/2.tmp\" files (human or computer) in CM3_TEMP; header version 1, number, count; with both full (2500), recreates the oldest. In `desc`: position, count, number, human |",
       "| `ler_eventos()` | 0x6d9c40 | thiscall, ret 4 | match_events.cpp: reads back the events described in `desc` from the file with the right number |",
       "| `passo_em_rede()` | 0x6b55c0 | thiscall, ret 4 | match_day.cpp: one step of the human's match on a network, by the clock and the humans' scoreboard (everyone sees each state; sounds, events, engine step, end, half time, goals, minute); records it (0x729ff0) and picks the day's human match |"],
      M, ME,
      [('gravar_eventos', 'record_events'), ('ler_eventos', 'read_events'), ('passo_em_rede', 'network_step')],
      "'sons_terminados': 'sounds_finished',")
