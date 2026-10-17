"""Documentacao da 155a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva155.py` (6 e 2 refeitas, pela carreira até 2002-06-01): 2 acusadas (o maior peso dos sons e, refeita, a opção 2) e 6 escaparam: os pesos, tipos e nomes dos sons e as opções 0, 1 e 4 só servem ao áudio, à tela e ao passo em rede (lacuna em `PENDENCIAS.md`; no jogo do Lucas a narração e os sons funcionaram). "
     "Sem comparação própria: lê arquivo, aloca os textos e grava a tabela global uma vez na abertura; validada pela carreira (Rust × original). Junto, a leitura dos lances da 154ª (0x6d9c40) ganhou comparação de verdade (lê só arquivo): no jogo do Lucas, 0 divergências. "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = ("`mutacoes/leva155.py` (6 and 2 redone, by the career to 2002-06-01): 2 caught (the sounds' largest weight and, redone, option 2) and 6 escaped: the sounds' weights, types and names and options 0, 1 and 4 only serve the audio, the screen and the network step (gap in `PENDENCIAS.en.md`; in Lucas's game the commentary and sounds worked). "
      "No comparison of its own: it reads a file, allocates the texts and writes the global table once at startup; validated by the career (Rust × original). Alongside, batch 154's read of the plays (0x6d9c40) got a real comparison (it only reads a file): in Lucas's game, 0 divergences. "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(155, 343, 344, 'o leitor dos eventos da partida', "the match events reader",
      ["| `ler_eventos_cfg()` | 0x6d9e50 | cdecl | match_events.cpp (~5,2 KB): lê o `events_<idioma>.cfg` (ou o `events_eng.cfg`) para a tabela dos 677 códigos de evento em [0xb68788] (7 bytes de opções, os sons de 0x110 bytes com peso, tipo, nome e os dois textos `G`/`R`); confere marcas `<X`, eventos sem texto e, com `-verifysounds`, os arquivos de som; no fim converte os pesos e dá o som padrão a quem não tem |"],
      ["| `ler_eventos_cfg()` | 0x6d9e50 | cdecl | match_events.cpp (~5.2 KB): reads the `events_<language>.cfg` (or `events_eng.cfg`) into the table of the 677 event codes at [0xb68788] (7 option bytes, the 0x110-byte sounds with weight, type, name and the two texts `G`/`R`); checks `<X` tokens, events without text and, with `-verifysounds`, the sound files; at the end converts the weights and gives the default sound to those without one |"],
      M, ME,
      [('ler_eventos_cfg', 'read_events_cfg'), ('erro_em', 'error_in')],
      "'passo_em_rede': 'network_step',")
