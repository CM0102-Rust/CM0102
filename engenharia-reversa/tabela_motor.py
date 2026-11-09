r"""A tabela das rotinas desviadas do motor.rs, num lugar só (etapa 4 da reorganização, 09/11/2026; o leitor de fora,
rodada 9: PROLOGOS, a lista `rust`, a lista `comp` e os nomes eram quatro tabelas de 2.092 entradas alinhadas só pela
posição). Agora cada rotina é uma linha da macro `routines!`:

    Prologue { address: 0x6c38c0, bytes: &[..] } => side_lookup_a, compare_side_lookup_a, "motor_tabela_a";

e a macro gera PROLOGOS, `rust_table()`, `compare_table()` e NAMES. Este módulo é também o leitor único da tabela
para as ferramentas: `rotinas(texto_do_motor_rs)` devolve [(endereço, bytes, rotina, comparador, nome)].

Uso: python tabela_motor.py --converter [--gravar]   (uma vez: as quatro tabelas viram a macro)
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MOTOR = os.path.join(R, 'rust', 'src', 'motor.rs')
ENTRADA = re.compile(r'Prologue \{ address: (0x[0-9a-f]+), bytes: &\[([^\]]*)\] \} => ([\w:]+), ([\w:]+), "([^"]*)";')


def rotinas(s=None):
    """[(endereço, [bytes], rotina, comparador, nome)] da macro `routines!` do motor.rs, na ordem da tabela."""
    s = s if s is not None else open(MOTOR, encoding='utf-8').read()
    i = s.index('routines! {')
    fim = s.index('\n}\n', i)
    return [(int(a, 16), [int(x, 16) for x in b.split(',') if x.strip()], r, c, n) for a, b, r, c, n in ENTRADA.findall(s[i:fim])]


def nomes_rust(s=None):
    return [x[2] for x in rotinas(s)]


def converter():
    s = open(MOTOR, encoding='utf-8', newline='').read()
    nl = '\r\n' if '\r\n' in s else '\n'
    t = s.replace('\r\n', '\n')
    i0 = t.index('const PROLOGOS: [Prologue; N] = [\n')
    i1 = t.index('\n];\nconst N: usize = ', i0)
    corpo = t[i0 + len('const PROLOGOS: [Prologue; N] = [\n'):i1]
    linhas = corpo.split('\n')
    entradas, comentarios = [], []
    for l in linhas:
        m = re.match(r'\s*(Prologue \{ address: 0x[0-9a-f]+, bytes: &\[[^\]]*\] \}),\s*$', l)
        if m:
            entradas.append((list(comentarios), m.group(1)))
            comentarios = []
        elif l.strip():
            comentarios.append(l.rstrip())
    fim_comentarios = comentarios
    j0 = t.index('    let rust: [usize; N] = [\n')
    j1 = t.index('\n    ];\n', j0)
    rust = re.findall(r'^\s*([\w:]+) as \*const \(\) as usize,\s*$', t[j0:j1], re.M)
    k0 = t.index('    let comp: [usize; N] = [\n')
    k1 = t.index('\n    ];\n', k0)
    comp = re.findall(r'^\s*([\w:]+) as \*const \(\) as usize,\s*$', t[k0:k1], re.M)
    n0 = t.index('    let nomes = [\n')
    n1 = t.index('\n    ];\n', n0)
    nomes = re.findall(r'"([^"]*)"', t[n0:n1])
    N = int(re.search(r'\nconst N: usize = (\d+);', t).group(1))
    print('PROLOGOS %d, rust %d, comp %d, nomes %d, N %d' % (len(entradas), len(rust), len(comp), len(nomes), N))
    assert len(entradas) == len(rust) == len(comp) == len(nomes) == N, 'as tabelas não têm o mesmo tamanho'
    macro = [
        '/// Builds the tables of the hooked routines from one line per routine (the four used to be separate arrays aligned',
        '/// only by position): `Prologue { .. } => rust_fn, comparer_fn, "label";`. Generates PROLOGOS (the address and the',
        '/// expected first bytes, checked before hooking), `rust_table()` and `compare_table()` (the Rust copy and the comparer',
        '/// of each routine, as addresses) and NAMES (the label in the log). Read by the tools through',
        '/// engenharia-reversa/tabela_motor.py.',
        '///',
        '/// [pt] As tabelas das rotinas desviadas, uma linha por rotina (eram quatro listas alinhadas só pela posição).',
        'macro_rules! routines {',
        '    ($( $p:expr => $r:path, $c:path, $n:literal; )*) => {',
        '        const PROLOGOS: [Prologue; N] = [$($p),*];',
        '        fn rust_table() -> [usize; N] {',
        '            [$($r as *const () as usize),*]',
        '        }',
        '        fn compare_table() -> [usize; N] {',
        '            [$($c as *const () as usize),*]',
        '        }',
        '        const NAMES: [&str; N] = [$($n),*];',
        '    };',
        '}',
        '',
        'routines! {',
    ]
    for (coms, pro), r, c, n in zip(entradas, rust, comp, nomes):
        macro += coms
        macro.append('    %s => %s, %s, "%s";' % (pro, r, c, n))
    macro += fim_comentarios
    macro.append('}')
    t = t[:i0] + '\n'.join(macro) + t[i1 + len('\n];'):]
    # the three lists inside instalar
    for cab, novo in (('    let rust: [usize; N] = [\n', '    let rust = rust_table();'),
                      ('    let comp: [usize; N] = [\n', '    let comp = compare_table();'),
                      ('    let nomes = [\n', '    let nomes = NAMES;')):
        a = t.index(cab)
        b = t.index('\n    ];\n', a) + len('\n    ];')
        t = t[:a] + novo + t[b:]
    if '--gravar' in sys.argv:
        open(MOTOR, 'w', encoding='utf-8', newline='').write(t.replace('\n', nl))
        print('gravado')
    else:
        print('ok (sem gravar)')


if __name__ == '__main__':
    if '--converter' in sys.argv:
        converter()
