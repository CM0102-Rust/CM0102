"""Documentacao da 156a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva156.py` (3, pela carreira até 2002-06-01): as 3 escaparam; os contadores de ramo (\"texto do evento\", \"partida em rede confirmada\") ficam em zero: as duas só rodam com a tela da partida (lacuna em `PENDENCIAS.md`). "
     "O texto do evento ganhou comparação de verdade (só escreve nos buffers de texto): no `JOGAR_COMPARANDO`, cada linha de narração é montada pela original e pela cópia e conferida. A confirmação em rede não tem comparação (cria e libera motor e resumo). "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = ("`mutacoes/leva156.py` (3, by the career to 2002-06-01): all 3 escaped; the branch counters (\"texto do evento\", \"partida em rede confirmada\") stay at zero: both only run with the match screen (gap in `PENDENCIAS.en.md`). "
      "The event text got a real comparison (it only writes to the text buffers): in `JOGAR_COMPARANDO`, each commentary line is built by the original and the copy and checked. The network confirmation has no comparison (it creates and frees engine and summary). "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(156, 344, 346, 'o texto dos eventos e o começo da partida em rede', 'the event text and the start of the network match',
      ["| `texto_do_evento()` | 0x6d8d10 | stdcall, ret 0x10 | match_events.cpp: o texto `R` do som do evento com as marcas trocadas (`<P1>` jogador, `<T1>`/`<T2>` clube, `<C>` competição, `<F>` fase, `<D>`/`<N>` número, `<rf>`/`<Rf>` árbitro), prefixo e sufixo `{...}` (0x6701f0) e maiúsculas; escreve em [0xb67ea0] |",
       "| `confirmar_partida_em_rede()` | 0x6b5260 | thiscall, ret 4 | match_day.cpp: cada humano confirma a partida em rede; com todos, cria o motor e o prepara para ver, ou joga até o fim e registra |"],
      ["| `texto_do_evento()` | 0x6d8d10 | stdcall, ret 0x10 | match_events.cpp: the event sound's `R` text with the tokens replaced (`<P1>` player, `<T1>`/`<T2>` club, `<C>` competition, `<F>` stage, `<D>`/`<N>` number, `<rf>`/`<Rf>` referee), `{...}` prefix and suffix (0x6701f0) and upper case; writes to [0xb67ea0] |",
       "| `confirmar_partida_em_rede()` | 0x6b5260 | thiscall, ret 4 | match_day.cpp: each human confirms the network match; with all of them, creates the engine and prepares it for watching, or plays it to the end and records it |"],
      M, ME,
      [('texto_do_evento', 'event_text'), ('confirmar_partida_em_rede', 'confirm_network_match')],
      "'ler_eventos_cfg': 'read_events_cfg',")
