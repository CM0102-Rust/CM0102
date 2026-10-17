"""Documentacao da 152a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva152.py` (8, pela carreira até 2002-06-01): 5 acusadas e 3 escaparam: a da fila era fraca e, refeita sem a chamada (`mutacoes/leva152b.py`), foi acusada; a das linhas do humano não roda na carreira automática (lacuna nova em `PENDENCIAS.md`); a do contador de terminadas é a lacuna já anotada na 151ª. "
     "Sem comparação própria: o laço do dia cria e libera motores e resumos e mexe na tela; validado pela carreira (Rust × original). "
     "Jogo real comparando até 2002-01-01: 0 divergências (impressão nova 963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: as três iguais, numa referência nova (com o laço do dia desviado, as originais que ele chama rodam na pilha particular e leem outro lixo, igual nos dois modos): 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = ("`mutacoes/leva152.py` (8, by the career to 2002-06-01): 5 caught and 3 escaped: the queue one was weak and, redone without the call (`mutacoes/leva152b.py`), was caught; the humans' lines one does not run in the automatic career (new gap in `PENDENCIAS.en.md`); the finished counter one is the gap already noted in batch 151. "
      "No comparison of its own: the day loop creates and frees engines and summaries and touches the screen; validated by the career (Rust × original). "
      "Real game comparing to 2002-01-01: 0 divergences (new fingerprint 963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: all three equal, on a new reference (with the day loop hooked, the originals it calls run on the private stack and read other garbage, the same in both modes): 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(152, 336, 337, 'o laço do dia de jogos', 'the match day loop',
      ["| `jogar_o_dia()` | 0x6b4360 | thiscall | match_day.cpp (~3,3 KB): joga o dia. Em rede, liga os humanos às partidas deles (placar de 0x11d bytes, 0x765480, 0x5fcff0/0x5fd100) e as joga até o fim; depois cada partida sem motor: com humano em rede anda a atual (0x6b5ce0) e recomeça; senão cria o motor, joga até o fim, cria o resumo, registra e escreve o placar com a porcentagem na barra (0x66f4e0, 0x802f80). Por fim fecha cada partida (0x6b5f00), refaz as táticas guardadas dos humanos e libera os resumos |"],
      ["| `jogar_o_dia()` | 0x6b4360 | thiscall | match_day.cpp (~3.3 KB): plays the day. On a network, ties the humans to their matches (0x11d-byte scoreboard, 0x765480, 0x5fcff0/0x5fd100) and plays them to the end; then each match without an engine: with a human on a network, steps the current one (0x6b5ce0) and restarts; otherwise creates the engine, plays to the end, creates the summary, records it and writes the score with the percentage on the bar (0x66f4e0, 0x802f80). Finally closes each match (0x6b5f00), redoes the humans' stored tactics and frees the summaries |"],
      M, ME,
      [('jogar_o_dia', 'play_the_day')],
      "'fechar_partida_do_dia': 'close_day_match',")
