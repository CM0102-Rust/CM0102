"""Documentacao da 217a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, REF_JOGO, CARREIRA_PT, CARREIRA_EN

N = 217
R = [('montar_estatisticas_da_partida', 'Match_Stats::Match_Stats', '0x729000', 'match_stats.cpp', 'thiscall', 'confere o CD do CM01/02 e monta as estatísticas: carrega do save ou cria o índice, os baldes e o arquivo temporário zerado', 'checks for the CM01/02 CD and builds the statistics: loads from the save or creates the index, the buckets and the zeroed temporary file'), ('desmontar_estatisticas_da_partida', 'Match_Stats::~Match_Stats', '0x7293c0', 'match_stats.cpp', 'thiscall', 'libera o índice dos registros e zera o ponteiro e a quantidade', 'frees the record index and clears its pointer and count')]
ESCAPADAS = {}  # nome: (explicação pt, explicação en), quando houver
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia sem a referencia'
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
M = (f"`mutacoes/leva{N}.py` ({tot}, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): {ac} acusadas. {lac_pt} "
     f"Jogo real comparando até 2002-01-01: 0 divergências ({REF_JOGO}). Carreiras Rust ×2 e original até 2002-06-01: {CARREIRA_PT}")
ME = (f"`mutacoes/leva{N}.py` ({tot}, by the career to 2002-06-01, confirming the escaped ones in comparison mode): {ac} caught. {lac_en} "
      f"Real game comparing to 2002-01-01: 0 divergences ({REF_JOGO}). Rust ×2 and original careers to 2002-06-01: {CARREIRA_EN}")
fazer(N, 555, 555 + len(R), 'o fim das estatísticas da partida: o construtor (com a conferência do CD) e o destrutor', 'the end of the match statistics: the constructor (with the CD check) and the destructor',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'o fim das estatísticas da partida: o construtor (com a conferência do CD) e o destrutor', 'the end of the match statistics: the constructor (with the CD check) and the destructor', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[('Match_Stats +0: índice dos registros (n × 12 bytes); ponteiro; grava (montar_estatisticas_da_partida); lê, libera e grava 0 (desmontar_estatisticas_da_partida)', 'Match_Stats +0: record index (n × 12 bytes); pointer; writes (montar_estatisticas_da_partida); reads, frees and writes 0 (desmontar_estatisticas_da_partida)'), ('Match_Stats +4: capacidade do índice (n); valor; grava (montar_estatisticas_da_partida); grava 0 (desmontar_estatisticas_da_partida)', 'Match_Stats +4: index capacity (n); value; writes (montar_estatisticas_da_partida); writes 0 (desmontar_estatisticas_da_partida)'), ('Match_Stats +8..+0x14: cursores dos quatro baldes (k × n / 4); valor; grava (montar_estatisticas_da_partida)', 'Match_Stats +8..+0x14: cursors of the four buckets (k × n / 4); value; writes (montar_estatisticas_da_partida)'), ('Match_Stats +0x18: sem nome (zerado depois de gravar o arquivo temporário); valor; grava 0 (montar_estatisticas_da_partida)', 'Match_Stats +0x18: unnamed (cleared after writing the temporary file); value; writes 0 (montar_estatisticas_da_partida)'), ('Match_Stats +0x1c: quantidade de registros (n); valor; grava (montar_estatisticas_da_partida)', 'Match_Stats +0x1c: record count (n); value; writes (montar_estatisticas_da_partida)'), ('match_stats_matches.tmp (CM3_TEMP): n registros de estatística de 0xd7a bytes zerados; arquivo; grava (montar_estatisticas_da_partida)', 'match_stats_matches.tmp (CM3_TEMP): n zeroed 0xd7a-byte statistics records; file; writes (montar_estatisticas_da_partida)'), ('[0x729089]/[0x7290cb] bytes do código da conferência do CD (nop/xor no exe de trabalho); valor; lê (montar_estatisticas_da_partida)', '[0x729089]/[0x7290cb] code bytes of the CD check (nop/xor in the working exe); value; reads (montar_estatisticas_da_partida)')], interessa=['SGE'])
