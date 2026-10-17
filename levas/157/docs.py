"""Documentacao da 157a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva157.py` (2, pela carreira até 2002-06-01): as 2 escaparam; o contador \"narracao no painel\" fica em zero: a rotina só roda com a tela da partida (lacuna em `PENDENCIAS.md`; o teste é assistir partidas no `JOGAR.bat`). "
     "Sem comparação própria: desenha no painel e toca som. Com ela, o motor de partidas sem as telas fica 100% em Rust. "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = ("`mutacoes/leva157.py` (2, by the career to 2002-06-01): both escaped; the counter \"narracao no painel\" stays at zero: the routine only runs with the match screen (gap in `PENDENCIAS.en.md`; the test is watching matches in `JOGAR.bat`). "
      "No comparison of its own: it draws on the panel and plays sound. With it, the match engine without the screens is 100% in Rust. "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(157, 346, 347, 'a narração no painel', 'the commentary on the panel',
      ["| `narrar_no_painel()` | 0x6d7b00 | thiscall, ret 0x20 | match_events.cpp (~4,4 KB): repõe o som da torcida, monta o texto `G` do evento (como a 0x6d8d10) numa linha com as cores do clube (`<O>` as do outro, `<Y>`/`<R>` as padrão), quebra ou corta com \"...\" pela largura (0x5e5ad0, 0x90cb20), desenha no painel (0x801040…0x800580), toca o som do evento (0x8025e0) e marca a hora e a duração (+0x2d6c, +0x2dac) |"],
      ["| `narrar_no_painel()` | 0x6d7b00 | thiscall, ret 0x20 | match_events.cpp (~4.4 KB): restores the crowd sound, builds the event's `G` text (like 0x6d8d10) into a line with the club colours (`<O>` the other's, `<Y>`/`<R>` the default ones), wraps or cuts with \"...\" by the width (0x5e5ad0, 0x90cb20), draws on the panel (0x801040…0x800580), plays the event sound (0x8025e0) and marks the time and duration (+0x2d6c, +0x2dac) |"],
      M, ME,
      [('narrar_no_painel', 'commentate_on_panel')],
      "'confirmar_partida_em_rede': 'confirm_network_match',")
