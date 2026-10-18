"""Documentacao da 169a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 169
R = [('evento_na_frente', 'event_at_front', '0x6d5d00', 'match_events.cpp', 'thiscall', 'põe um evento na frente da fila da narração (recua a posição no anel de 12, tira o repetido) e enfileira pela 0x6d6f70', 'puts an event at the front of the commentary queue (steps the 12-slot ring back, drops the repeat) and queues it via 0x6d6f70'), ('construir_fila_da_narracao', 'construct_commentary_queue', '0x6d5bf0', 'match_events.cpp', 'thiscall', 'o construtor da fila da narração: zera a fila, lê os eventos e, se pedido, toca o loop da torcida', 'the commentary queue constructor: clears the queue, reads the events and, if asked, plays the crowd loop'), ('liberar_tabelas_de_eventos', 'free_event_tables', '0x6d5b50', 'match_events.cpp', 'cdecl', 'libera as tabelas de eventos carregadas e os textos de cada registro', "frees the loaded event tables and each record's texts")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'c_tempo': ('mutação equivalente: o +0x2db0 inicial (125) só é comparado com 20 e 101 e as telas da partida o regravam', 'equivalent mutation: the initial +0x2db0 (125) is only compared with 20 and 101 and the match screens rewrite it'),
    'l_fim': ('mutação equivalente: a 0x6d9e50, que chama a liberação, regrava 0xa1599c (o idioma dos eventos) logo depois', 'equivalent mutation: 0x6d9e50, which calls the freeing, rewrites 0xa1599c (the events language) right after'),
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
fazer(N, 380, 380 + 3, 'o fechamento do motor (3): a fila da narração', 'closing the engine (3): the commentary queue',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'o fechamento do motor (3): a fila da narração', 'closing the engine (3): the commentary queue', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
