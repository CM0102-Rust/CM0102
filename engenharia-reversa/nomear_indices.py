r"""Troca os índices numéricos das chamadas à original (`orig::<F>(1598)`, `comparar_valor!(x, y, 357, ...)`) pela
constante com nome que o build.rs gera da tabela (`IDX_POR_OFERTA_DE_TRANSFERENCIA`), no motor e nas listas de
mutação (etapa 4 da reorganização, rodada 2, 07/11/2026). O valor é o mesmo.

Uso: python nomear_indices.py [--gravar]
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
ORIG = re.compile(r'(orig::<.*?>\()(\d+)(\))')
MACRO = re.compile(r'(comparar_(?:leitor|gravador|repartir|valor|ordem)!\(\w+, \w+, )(\d+)\b')


def main():
    s = open(os.path.join(SRC, 'motor.rs'), encoding='utf-8').read()
    i = s.index('let rust: [usize; N] = [')
    nomes = re.findall(r'^\s+(\w+) as \*const \(\) as usize,$', s[i:s.index('];', i)], re.M)
    idx = {k: 'IDX_' + n.upper() for k, n in enumerate(nomes)}

    def trocar_linha(l):
        cod, sep, com = l.partition('//')
        cod = ORIG.sub(lambda m: m.group(1) + idx[int(m.group(2))] + m.group(3), cod)
        cod = MACRO.sub(lambda m: m.group(1) + idx[int(m.group(2))], cod)
        return cod + sep + com

    fs = [os.path.join(SRC, 'motor.rs')] + [os.path.join(d, f) for d, _, n in os.walk(os.path.join(SRC, 'motor')) for f in n if f.endswith('.rs')]
    n = 0
    for p in fs:
        t = open(p, encoding='utf-8').read()
        novo = '\n'.join(trocar_linha(l) for l in t.split('\n'))
        n += sum(a != b for a, b in zip(t.split('\n'), novo.split('\n')))
        if novo != t and '--gravar' in sys.argv:
            open(p, 'w', encoding='utf-8').write(novo)
    print('%d linhas com índice por nome' % n)
    if '--gravar' in sys.argv:
        import trocar_nas_mutacoes
        trocar_nas_mutacoes.aplicar(trocar_linha)
        m = os.path.join(SRC, 'motor.rs')
        t = open(m, encoding='utf-8').read()
        inc = 'include!(concat!(env!("OUT_DIR"), "/indices.rs")); // IDX_<NAME>: the position of each routine (build.rs)\n'
        if inc not in t:
            t = re.sub(r'(\nconst N: usize = \d+;\n)', lambda x: x.group(1) + inc, t, count=1)
            open(m, 'w', encoding='utf-8').write(t)


if __name__ == '__main__':
    main()
