r"""A named field offset (`usize`) or a named bit (bits.rs) put where the code computed in another integer type (`stadium * 0x6e + 0x40` in
i32): the compiler's "expected `T`, found `usize`" at `module::NAME` becomes `(module::NAME as T)`, the same value the
literal had. Repeats until the build has no such error. Used after nomear_por_linha.py (10/11/2026).
Usage: python ajustar_constantes.py
[pt] O deslocamento com nome (usize) numa conta em outro tipo inteiro vira `(modulo::NOME as T)`, pelo erro do
compilador.
"""
import os
import re
import subprocess

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INTS = {'i8', 'u8', 'i16', 'u16', 'i32', 'u32', 'isize', 'i64', 'u64'}


def main():
    total = 0
    for _ in range(20):
        r = subprocess.run(['cargo', 'build', '--release', '--color=never'], cwd=os.path.join(R, 'rust'),
                           capture_output=True, text=True, encoding='utf-8', errors='replace')
        ed = {}
        for m in re.finditer(r'error\[E0308\]: mismatched types\n\s*--> ([^:\n]+):(\d+):(\d+)\n(?:.*\n){1,4}?.*expected `(\w+)`, found `(\w+)`', r.stderr):
            if m.group(4) in INTS | {'usize'} and m.group(5) in INTS | {'usize'}:
                ed.setdefault(os.path.join(R, 'rust', m.group(1)), []).append((int(m.group(2)), int(m.group(3)), m.group(4)))
        if not ed:
            break
        n = 0
        for p, lista in ed.items():
            linhas = open(p, encoding='utf-8', newline='').read().split('\n')
            for ln, col, t in sorted(set(lista), reverse=True):
                l = linhas[ln - 1]
                m = re.match(r'!?[a-z_][a-z0-9_]*::[A-Z][A-Z0-9_]*', l[col - 1:])
                if not m:
                    continue
                linhas[ln - 1] = l[:col - 1] + '(%s as %s)' % (m.group(0), t) + l[col - 1 + m.end():]
                n += 1
            open(p, 'w', encoding='utf-8', newline='').write('\n'.join(linhas))
        total += n
        if not n:
            break
    print('%d constantes ajustadas' % total)
    print('\n'.join(re.findall(r'^error.*$', r.stderr, re.M)[:10]) or 'compila')


if __name__ == '__main__':
    main()
