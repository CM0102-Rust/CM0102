"""Documentacao da 162a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 162
R = [('gravar_vaga_tatica', 'set_tactic_slot', '0x896a90', 'tactics.cpp', 'thiscall', 'grava a pessoa na vaga 0..19 da linha do clube (0x8982b0) e apaga as instruções pendentes desse clube', "stores the person in slot 0..19 of the club row (0x8982b0) and clears the club's pending instructions"), ('pessoa_da_vaga_tatica', 'tactic_slot_person', '0x8968f0', 'tactics.cpp', 'thiscall', 'a pessoa da vaga; sem ficha ou de outra nação numa seleção, a vaga vira −1', "the slot's person; without a record or from another nation in a national team, the slot becomes −1"), ('gravar_formacao_tatica', 'set_tactic_formation', '0x8967d0', 'tactics.cpp', 'thiscall', 'grava a formação na linha do clube', 'stores the formation in the club row')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'g_pendente': ('as instruções pendentes (+0x5f0) só existem com técnico humano', 'pending instructions (+0x5f0) only exist with a human manager'),
    'p_ficha': ('na carreira automática as vagas da tabela tática estão vazias (−1): nem a condição invertida (`p_ficha_b`, `mutacoes/leva162b.py`) é acusada; a parte com pessoa só roda na tática do técnico humano',
                'in the automatic career the tactics table slots are empty (−1): not even the inverted condition (`p_ficha_b`, `mutacoes/leva162b.py`) is caught; the part with a person only runs on the human manager\'s tactics'),
    'p_nacao': ('idem (`p_nacao_b` invertida também escapa)', 'likewise (the inverted `p_nacao_b` also escapes)'),
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
fazer(N, 359, 359 + 3, 'as vagas da tabela tática', 'the tactics table slots',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'as vagas da tabela tática', 'the tactics table slots', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
