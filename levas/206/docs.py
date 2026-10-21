"""Documentacao da 206a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 206
R = [('ler_registro_da_partida', 'Match_Stats::read_match_record', '0x729600', 'match_stats.cpp', 'thiscall', 'lê um registro de estatística da partida (0xd7a bytes) do match_stats_matches.tmp pelo índice', 'reads one match statistics record (0xd7a bytes) from match_stats_matches.tmp by index'), ('registrar_partida', 'Match_Stats::record_match', '0x7293f0', 'match_stats.cpp', 'thiscall', 'preenche o registro de uma partida que terminou e o grava, salvo os jogos excluídos', 'fills the record of a finished match and writes it, except for excluded matches'), ('gravar_registro_da_partida', 'Match_Stats::write_match_record', '0x729820', 'match_stats.cpp', 'thiscall', 'grava os eventos e o registro na vaga do jogo (ou na vaga do cursor do balde, que anda em círculo) e atualiza o índice', "writes the events and the record into the fixture's slot (or the bucket cursor's slot, which wraps around) and updates the index")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'l_tamanho': ('ler o registro da partida (0x729600) só roda quando se abre o relatório de uma partida jogada; a carreira automática não abre telas', 'reading the match record (0x729600) only runs when the report of a played match is opened; the automatic career opens no screens'),
    'r_excluido': ('na carreira o jogo nunca é o da competição [0x9cf684], então o "ou" decide na primeira metade e a marca +0x4c do registro nunca é lida', 'in the career the fixture is never the [0x9cf684] competition one, so the "or" is decided by its first half and the record +0x4c flag is never read'),
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
fazer(N, 491, 491 + 3, 'as estatísticas da partida: ler e gravar o registro no arquivo temporário e registrar a partida', 'the match statistics: reading and writing the record in the temporary file and recording the match',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'as estatísticas da partida: ler e gravar o registro no arquivo temporário e registrar a partida', 'the match statistics: reading and writing the record in the temporary file and recording the match', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
