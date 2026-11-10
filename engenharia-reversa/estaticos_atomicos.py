r"""The engine's scalar `static mut` become atomics (readability batch 32, 10/11/2026; decision (a), safety):

    pub(crate) static mut X_DRY_RUN: bool = false;   ->  pub(crate) static X_DRY_RUN: AtomicBool = AtomicBool::new(false);
    X_DRY_RUN = true;                                 ->  X_DRY_RUN.store(true, Relaxed);
    if X_DRY_RUN { .. }                               ->  if X_DRY_RUN.load(Relaxed) { .. }

Only file-level scalars (bool, u8, u32, i32, usize) of motor.rs and motor/, never one that is borrowed
(`addr_of!`, `&X`) or read by asm: those stay `static mut` (the infrastructure in detours.rs, stack_probe.rs...).
Everything runs on the game's thread, so `Relaxed` is enough: the atomic is there so the access needs no `unsafe`.
The compiler checks the result. Usage: python estaticos_atomicos.py [--gravar]
[pt] Os `static mut` escalares do motor viram atômicos (só os de arquivo, sem empréstimo nem asm).
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
ATOM = {'bool': 'AtomicBool', 'u8': 'AtomicU8', 'u32': 'AtomicU32', 'i32': 'AtomicI32', 'usize': 'AtomicUsize'}
DECL = re.compile(r'^((?:pub\(crate\) )?)static mut ([A-Z][A-Z0-9_]*): (bool|u8|u32|i32|usize) = ([^;]+);', re.M)


def arquivos():
    yield os.path.join(SRC, 'motor.rs')
    for raiz, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if f.endswith('.rs'):
                yield os.path.join(raiz, f)


def fim_da_expr(s, i):
    """End of the expression starting at i (the `;` at depth 0)."""
    d = 0
    while i < len(s):
        c = s[i]
        if c in '([{':
            d += 1
        elif c in ')]}':
            d -= 1
        elif c == ';' and d == 0:
            return i
        i += 1
    return None


def trocar_usos(s, nomes):
    pat = re.compile(r'(?<![\w.])(?:(?:crate|super|self)::(?:\w+::)*)?(%s)\b' % '|'.join(nomes))
    out, pos = [], 0
    for m in pat.finditer(s):
        if m.start() < pos:
            continue
        ini_linha = s.rfind('\n', 0, m.start()) + 1
        linha = s[ini_linha:s.find('\n', m.start())]
        if re.match(r'\s*(pub(\(crate\))? )?(use|static) ', linha) or linha.lstrip().startswith('//'):
            continue
        dep = s[m.end():]
        a = re.match(r'\s*(\+=|-=|=)(?!=)\s*', dep)
        if a:
            ini = m.end() + a.end()
            fim = fim_da_expr(s, ini)
            expr = s[ini:fim].strip()
            metodo = {'=': 'store', '+=': 'fetch_add', '-=': 'fetch_sub'}[a.group(1)]
            out += [s[pos:m.end()], '.%s(%s, Relaxed)' % (metodo, expr)]
            pos = fim
        else:
            out += [s[pos:m.end()], '.load(Relaxed)']
            pos = m.end()
    out.append(s[pos:])
    return ''.join(out)


def main():
    grava = '--gravar' in sys.argv
    textos = {p: open(p, encoding='utf-8').read() for p in arquivos()}
    alvos = {}
    for p, s in textos.items():
        for m in DECL.finditer(s):
            alvos[m.group(2)] = (p, m)
    tudo = '\n'.join(textos.values())
    for n in list(alvos):
        if re.search(r'addr_of(_mut)?!\(\s*%s\b|&(mut )?%s\b|\bsym %s\b' % (n, n, n), tudo):
            del alvos[n]
    print('%d estáticos escalares viram atômicos' % len(alvos))
    nomes = sorted(alvos, key=len, reverse=True)
    for p, s in textos.items():
        s2 = trocar_usos(s, nomes)
        s2 = DECL.sub(lambda m: ('%sstatic %s: %s = %s::new(%s);' % (m.group(1), m.group(2), ATOM[m.group(3)], ATOM[m.group(3)], m.group(4)))
                      if m.group(2) in alvos else m.group(0), s2)
        if s2 != s and grava:
            open(p, 'w', encoding='utf-8').write(s2)
    for n in nomes:
        print('  ', n, os.path.relpath(alvos[n][0], SRC))


if __name__ == '__main__':
    main()
