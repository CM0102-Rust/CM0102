r"""Source patterns of the two behaviour changes the readability rounds let through (11/11/2026), searched over the
whole engine: (1) a read whose value is thrown away (`read_i8(0x127 + 0);` as a statement: a local closure taken for
the memory helper, round 24), (2) a field added twice (`let p = r + scout_report::SCOUT_NAME_REF + ..;` and later
`p + scout_report::SCOUT_NAME_REF + 0xc`, batch 43): a variable built with `+ module::NAME` and then offset again by the
same `module::NAME` in the same function. Prints the candidates for a reader to check.
Usage: python padroes_suspeitos.py
[pt] Os dois padrões de defeito que as rodadas de legibilidade deixaram passar, procurados no motor inteiro.
"""
import os
import re

R = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'rust', 'src')


def main():
    n = 0
    for raiz, _, fs in os.walk(R):
        for f in fs:
            if not f.endswith('.rs'):
                continue
            p = os.path.join(raiz, f)
            linhas = open(p, encoding='utf-8').read().split('\n')
            rel = os.path.relpath(p, R)
            defs = {}
            for i, l in enumerate(linhas, 1):
                cod = l.split('//')[0]
                if re.match(r'\s*(pub(\(crate\))? )?(unsafe )?(extern "\w+" )?fn ', cod):
                    defs = {}
                if re.fullmatch(r'\s*read_(?:i|u|f)(?:8|16|32|64|size)\([^;]*\);\s*', cod):
                    print('%s:%d: leitura descartada: %s' % (rel, i, cod.strip()))
                    n += 1
                m = re.match(r'\s*let (?:mut )?(\w+)\s*=\s*([^;]*);', cod)
                if m:
                    nomes = set(re.findall(r'\+\s*(\w+::[A-Z][A-Z0-9_]*)', m.group(2)))
                    defs[m.group(1)] = nomes
                    continue
                for v, nomes in defs.items():
                    for nome in nomes:
                        if re.search(r'\b%s(?:\.addr\(\))?\s*\+\s*%s\b' % (re.escape(v), re.escape(nome)), cod):
                            print('%s:%d: %s já tem %s e soma de novo: %s' % (rel, i, v, nome, cod.strip()[:140]))
                            n += 1
    print(n, 'candidatos')


if __name__ == '__main__':
    main()
