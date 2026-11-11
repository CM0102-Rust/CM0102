r"""The `[pt]` paragraphs leave the Rust code (11/11/2026, Lucas: decision (c)). The public code is in English; the
Portuguese original of each routine stays in docs/ROTINAS.md and in the history. A `/// [pt]`, `//! [pt]` or `// [pt]`
paragraph runs to the end of its comment block (the next line that is not the same kind of comment, or a blank comment
line followed by another tag such as `Original:`), and the blank comment line before it goes with it.
Usage: python sem_pt.py [DIR] [--gravar]
[pt] Os parágrafos [pt] saem do código Rust; o português fica no ROTINAS.md e no histórico.
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def limpar(t):
    linhas = t.split('\n')
    out, i, n = [], 0, 0
    while i < len(linhas):
        m = re.match(r'(\s*)(///|//!|//) ?\[pt\]', linhas[i])
        if not m:
            out.append(linhas[i])
            i += 1
            continue
        ind, tag = m.group(1), m.group(2)
        i += 1
        # the paragraph's own continuation lines: same comment kind, same indent, not empty, no new tag
        while i < len(linhas):
            l = linhas[i]
            mm = re.match(r'(\s*)(///|//!|//)(?!/)( ?)(.*)$', l)
            if not mm or mm.group(1) != ind or mm.group(2) != tag:
                break
            if not mm.group(4).strip():
                # a blank comment line: a doc block's [pt] runs to the block's end (several Portuguese paragraphs);
                # a plain `//` paragraph ends here
                if tag == '//':
                    break
                i += 1
                continue
            if re.match(r'(Original|Usage|Safety|# )', mm.group(4)):
                break
            i += 1
        # trailing blank comment lines go back (they separate what follows); the one before the [pt] goes with it
        while re.fullmatch(r'\s*%s\s*' % re.escape(tag), linhas[i - 1]):
            i -= 1
        if out and re.fullmatch(r'\s*%s\s*' % re.escape(tag), out[-1]):
            out.pop()
        n += 1
    return '\n'.join(out), n


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    base = args[0] if args else os.path.join(R, 'rust', 'src')
    total = 0
    for raiz, _, fs in os.walk(base):
        for f in fs:
            if not f.endswith('.rs'):
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8', newline='').read()
            novo, n = limpar(t)
            total += n
            if n and '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(novo)
    print('%d parágrafos [pt] retirados' % total)


if __name__ == '__main__':
    main()
