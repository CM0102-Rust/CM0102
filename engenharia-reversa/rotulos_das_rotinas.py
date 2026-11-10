r"""The hooked routines are named once, in English (10/11/2026, Lucas: "everything we can move to English, we move,
even if it touches several tools"). Each line of the `routines!` table in motor.rs carried a Portuguese label for the
log (`=> table_a, compare_table_a, "motor_tabela_a";`) that repeated what the Rust function's name already says. The
label goes: the macro takes the name from the function (`stringify!`), so the log, SEM_ROTINAS and the tools all use
`table_a`. The old -> new map is written to docs/NOMES_ROTULOS.csv, so old logs and batch notes stay readable. The
hand-written hook names of the simulation and the gates (`"motor_porta_62ed70 (gravacao)"`) get English text too.
Usage: python rotulos_das_rotinas.py [--gravar]
[pt] Os rótulos em português da tabela das rotinas desviadas saem: o nome vem da própria função Rust.
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MOTOR = os.path.join(R, 'rust', 'src', 'motor.rs')
CSV = os.path.join(R, 'docs', 'NOMES_ROTULOS.csv')

LINHA = re.compile(r'(=>\s*)([\w:]+)(\s*,\s*)([\w:]+)\s*,\s*"([^"]+)"\s*;')
MACRO_A = '($( $p:expr => $r:path, $c:path, $n:literal; )*) => {'
MACRO_B = '($( $p:expr => $r:path, $c:path; )*) => {'
NOMES_A = 'const NAMES: [&str; N] = [$($n),*];'
NOMES_B = 'const NAMES: [&str; N] = [$(stringify!($r)),*];'

GANCHOS = {
    'RAM livre (fixa na simulacao)': 'free RAM (fixed in the simulation)',
    'RAM total (fixa na simulacao)': 'total RAM (fixed in the simulation)',
    'aleatorio': 'random',
    'clock (relogio da simulacao)': 'clock (the simulation clock)',
    'diag queixa 0x867700': 'diag complaint 0x867700',
    'disco livre (fixo na simulacao)': 'free disk (fixed in the simulation)',
    'ferias depois do -load (CM_FERIAS)': 'holiday after -load (CM_FERIAS)',
    'qsort (rastro)': 'qsort (trace)',
    'rand (observado na simulacao)': 'rand (watched in the simulation)',
    'time (relogio fixo da simulacao)': 'time (fixed clock of the simulation)',
}


def main():
    grava = '--gravar' in sys.argv
    t = open(MOTOR, encoding='utf-8').read()
    mapa = []

    def f(m):
        nome = m.group(2).split('::')[-1]
        mapa.append((m.group(5), nome))
        return '%s%s%s%s;' % (m.group(1), m.group(2), m.group(3), m.group(4))
    novo = LINHA.sub(f, t)
    assert novo.count(MACRO_A) == 1 and novo.count(NOMES_A) == 1
    novo = novo.replace(MACRO_A, MACRO_B).replace(NOMES_A, NOMES_B)
    novo = novo.replace('Prologue { .. } => rust_fn, comparer_fn, "label";', 'Prologue { .. } => rust_fn, comparer_fn;')
    novo = novo.replace('and NAMES (the label in the log)', 'and NAMES (the Rust function\'s name, the label in the log)')
    print('%d rótulos' % len(mapa))
    mapa = [(a, b) for a, b in mapa if b != 'rust_fn']  # the example in the macro's doc comment
    nomes = [n for _, n in mapa]
    assert len(nomes) == len(set(nomes)), 'nomes repetidos'
    if not grava:
        return
    open(MOTOR, 'w', encoding='utf-8').write(novo)
    with open(CSV, 'w', encoding='utf-8', newline='\n') as fo:
        fo.write('old_label,name\n')
        for a, b in mapa:
            fo.write('%s,%s\n' % (a, b))
    # hand-written hook names
    for raiz, _, fs in os.walk(os.path.join(R, 'rust', 'src')):
        for fl in fs:
            if fl.endswith('.rs'):
                p = os.path.join(raiz, fl)
                s = open(p, encoding='utf-8').read()
                s2 = re.sub(r'"motor_porta_([0-9a-f]+) \(gravacao\)"', r'"gate_\1 (save)"', s)
                for a, b in GANCHOS.items():
                    s2 = s2.replace('detours::hook("%s"' % a, 'detours::hook("%s"' % b)
                if s2 != s:
                    open(p, 'w', encoding='utf-8').write(s2)


if __name__ == '__main__':
    main()
