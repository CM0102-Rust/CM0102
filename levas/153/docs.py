"""Documentacao da 153a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva153.py` (8 e 1 refeita, pela carreira até 2002-06-01): 4 acusadas e 4 escaparam; pelos contadores de ramo, viram 3 lacunas em `PENDENCIAS.md`: a 1ª volta do dia não escolhe nada na carreira automática (2 mutações), nenhuma partida chega marcada -2 à montagem (refeita forte em `mutacoes/leva153b.py`, escapou também) e os sons dos eventos só rodam com a tela. "
     "Sem comparação própria: a montagem aloca os vetores do dia e chama rotinas ainda originais; o lado é só leitura e os sons são tela; validadas pela carreira (Rust × original), com contadores de ramo. "
     "Jogo real comparando até 2002-01-01: 0 divergências (impressão 963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e. "
     "Testes rodados com outro projeto da máquina (CMRTE) aberto ao mesmo tempo, depois do ajuste da trava do `mutacoes.py` (só os exe desta pasta): tempos iguais aos das levas anteriores")
ME = ("`mutacoes/leva153.py` (8 and 1 redone, by the career to 2002-06-01): 4 caught and 4 escaped; by the branch counters, they become 3 gaps in `PENDENCIAS.en.md`: the day's 1st round picks nothing in the automatic career (2 mutations), no match reaches the setup marked -2 (redone strong in `mutacoes/leva153b.py`, escaped too) and the event sounds only run with the screen. "
      "No comparison of their own: the setup allocates the day's vectors and calls routines still in the original; the side is read-only and the sounds are screen; validated by the career (Rust × original), with branch counters. "
      "Real game comparing to 2002-01-01: 0 divergences (fingerprint 963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e. "
      "Tests run with another project on the machine (CMRTE) open at the same time, after adjusting the `mutacoes.py` lock (only this folder's exes): times equal to the previous batches")
fazer(153, 337, 340, 'a montagem do dia de jogos', 'the match day setup',
      ["| `montar_o_dia()` | 0x6b3c10 | thiscall, ret 8 | match_day.cpp: monta o dia (data, modo 0..2): zera os vetores, junta as partidas pendentes de cada lista (0x5ae6a0), registra as já decididas (0x923630/0x923cc0), agrupa por lista e por competição (qsort 0x6b6510), aponta cada partida à sua competição (+0x63), cria a lista global [0xb67dc8] e calcula o interesse do dia (0x6b5040). Devolve o dia |",
       "| `lado_do_jogador()` | 0x714d30 | thiscall | match_pl.cpp: o lado do jogador (+0x27) |",
       "| `sons_terminados()` | 0x6d8c70 | thiscall, ret 4 | match_events.cpp: os sons dos 16 eventos marcados já tocaram e passaram da janela (relógio 0x90c510; para os canais que acabaram, 0x802a50)? |"],
      ["| `montar_o_dia()` | 0x6b3c10 | thiscall, ret 8 | match_day.cpp: sets up the day (date, mode 0..2): zeroes the vectors, gathers the pending matches of each list (0x5ae6a0), records the already decided ones (0x923630/0x923cc0), groups by list and by competition (qsort 0x6b6510), points each match to its competition (+0x63), creates the global list [0xb67dc8] and works out the day's interest (0x6b5040). Returns the day |",
       "| `lado_do_jogador()` | 0x714d30 | thiscall | match_pl.cpp: the player's side (+0x27) |",
       "| `sons_terminados()` | 0x6d8c70 | thiscall, ret 4 | match_events.cpp: have the sounds of the 16 marked events played and passed their window (clock 0x90c510; stops the finished channels, 0x802a50)? |"],
      M, ME,
      [('montar_o_dia', 'set_up_the_day'), ('lado_do_jogador', 'player_side'), ('sons_terminados', 'sounds_finished')],
      "'jogar_o_dia': 'play_the_day',")
