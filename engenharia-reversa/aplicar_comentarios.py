r"""Troca os comentários em português do corpo das funções pelas traduções em inglês propostas (etapa 4 da
reorganização, 08/11/2026; pela regra de língua, comentário curto no corpo é só em inglês). Entrada: TSV
`<texto original>\t<inglês>`. Só troca o comentário `//` (não `///` nem `//!`) cujo texto é exatamente o original; as
marcas de mutação não estão na lista. As listas de mutação recebem a mesma troca. É só comentário: o código gerado não
muda.

Uso: python aplicar_comentarios.py arquivo1.tsv [...] [--gravar]
"""
import os
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import trocar_nas_mutacoes  # noqa: E402


def main():
    trad = {}
    for t in [a for a in sys.argv[1:] if not a.startswith('--')]:
        for l in open(t, encoding='utf-8'):
            p = l.rstrip('\n').split('\t')
            # "[pt] ..." é a metade portuguesa de um cabeçalho bilíngue: fica
            if len(p) >= 2 and p[0].strip() and p[1].strip() and '\t' not in p[1] and not p[0].strip().startswith('[pt]'):
                trad[p[0].strip()] = p[1].strip()

    def linha(l):
        cod, sep, com = l.partition('//')
        if not sep or com.startswith('/') or com.startswith('!'):
            return l
        c = com.strip()
        if c in trad:
            esp = com[:len(com) - len(com.lstrip())]
            return cod + sep + esp + trad[c]
        return l

    n = 0
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if f.endswith('.rs'):
                p = os.path.join(d, f)
                t = open(p, encoding='utf-8').read()
                novo = '\n'.join(linha(l) for l in t.split('\n'))
                n += sum(a != b for a, b in zip(t.split('\n'), novo.split('\n')))
                if novo != t and '--gravar' in sys.argv:
                    open(p, 'w', encoding='utf-8').write(novo)
    print('%d linhas de comentário traduzidas (%d traduções)' % (n, len(trad)))
    if '--gravar' in sys.argv:
        trocar_nas_mutacoes.aplicar(linha)


if __name__ == '__main__':
    main()
