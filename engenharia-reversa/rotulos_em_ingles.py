r"""Os rótulos dos contadores de ramo em inglês (etapa 4 da reorganização, 08/11/2026; o leitor de fora, rodadas 6 a 8:
1.758 `branch("cqf_1 ofertas rivais ligadas")`). Entrada: TSV `<rótulo>\t<inglês>`. O mesmo texto troca em todo lugar
que as ferramentas o leem: `branch("...")` no código e nos itens da fila, o quarto campo (e os trechos) das listas de
mutação, e as crases do docs/PENDENCIAS.md (o registro "ramo <rótulo>: N" é lido pelo mutacoes.py e pelo
pendencias.py). Confere que cada rótulo novo é único e não colide com outro rótulo.

Uso: python rotulos_em_ingles.py a.tsv [...] [--gravar]
"""
import glob
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def main():
    mapa = {}
    for a in [x for x in sys.argv[1:] if not x.startswith('--')]:
        for l in open(a, encoding='utf-8'):
            p = l.rstrip('\n').split('\t')
            if len(p) == 2 and p[0].strip() and p[1].strip() and p[0].strip() != p[1].strip():
                mapa[p[0].strip()] = p[1].strip()
    mapa = {k: v for k, v in mapa.items() if len(k) >= 6 and not re.search(r'["\'\\]', v)}
    existentes = set()
    for p in glob.glob(os.path.join(R, 'rust', 'src', '**', '*.rs'), recursive=True):
        existentes |= set(re.findall(r'\bbranch\("([^"\\]*)"\)', open(p, encoding='utf-8').read()))
    novos = list(mapa.values())
    dup = {v for v in novos if novos.count(v) > 1} | {v for v in novos if v in existentes and v not in mapa}
    for k in [k for k, v in mapa.items() if v in dup]:
        del mapa[k]
    print('%d rótulos (%d fora por colisão)' % (len(mapa), len(dup)))
    rx_q = re.compile(r'(["\'])(%s)\1' % '|'.join(re.escape(k) for k in sorted(mapa, key=len, reverse=True)))
    rx_esc = re.compile(r'(\\")(%s)(\\")' % '|'.join(re.escape(k) for k in sorted(mapa, key=len, reverse=True)))
    rx_b = re.compile(r'(`)(%s)(`)' % '|'.join(re.escape(k) for k in sorted(mapa, key=len, reverse=True)))
    alvos = glob.glob(os.path.join(R, 'rust', 'src', '**', '*.rs'), recursive=True)
    alvos += glob.glob(os.path.join(R, 'engenharia-reversa', 'mutacoes', '*.py'))
    alvos += [p for p in glob.glob(os.path.join(R, 'levas', 'fila', '**', '*'), recursive=True) if p.endswith(('.rs', '.py'))]
    n = 0
    for p in alvos:
        t = open(p, encoding='utf-8', newline='').read()
        s = rx_q.sub(lambda m: m.group(1) + mapa[m.group(2)] + m.group(1), t)
        s = rx_esc.sub(lambda m: m.group(1) + mapa[m.group(2)] + m.group(3), s)
        if s != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(s)
    for p in (os.path.join(R, 'docs', 'PENDENCIAS.md'), os.path.join(R, 'docs', 'PENDENCIAS.en.md')):
        t = open(p, encoding='utf-8', newline='').read()
        s = rx_b.sub(lambda m: m.group(1) + mapa[m.group(2)] + m.group(3), t)
        if s != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(s)
    print('%d arquivos' % n)


if __name__ == '__main__':
    main()
