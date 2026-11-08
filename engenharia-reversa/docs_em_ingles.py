r"""O inglês completo nos docs (`///`) que ainda estão só em português ou com o inglês bem mais curto que o [pt] (etapa 4
da reorganização, 08/11/2026; o cabecalhos_completos.py tratou os cabeçalhos com `Original:`). Cada bloco é identificado
pelo hash do texto. `--extrair N --pasta DIR` grava N lotes JSONL; os tradutores devolvem TSV `<hash>\t<inglês>`
(parágrafos separados por ` \n `). `--aplicar a.tsv [...]` põe o inglês em cima (quebrado em 120 colunas) e o português
num parágrafo `[pt]` embaixo; a linha `Original:` (se houver) fica entre os dois. É só comentário.

Uso: python docs_em_ingles.py --extrair 2 --pasta DIR | --aplicar a.tsv [...] [--gravar]
"""
import glob
import hashlib
import json
import os
import re
import sys
import textwrap

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
CAB = re.compile(r'((?:^[ \t]*///[^\n]*\n)+)', re.M)
PT = re.compile(r'\b(não|é|uma|para|com|quando|então|pela|pelo|dos|das|nas|nos|também|já|até|cópia|rotina|função|devolve|'
                r'grava|lê)\b|ção\b|ções\b')


def blocos():
    for p in sorted(glob.glob(os.path.join(SRC, '**', '*.rs'), recursive=True)):
        t = open(p, encoding='utf-8').read()
        for m in CAB.finditer(t):
            h = m.group(1)
            ind = re.match(r'[ \t]*', h).group(0)
            linhas = [re.sub(r'^[ \t]*///\s?', '', x) for x in h.rstrip('\n').split('\n')]
            orig = [x for x in linhas if x.startswith('Original:')]
            if '[pt]' in h:
                i = next((k for k, x in enumerate(linhas) if x.startswith('[pt]')), None)
                if i is None:  # `[pt]` no meio de uma linha (uma linha só, bilíngue): fica
                    continue
                en = '\n'.join(x for x in linhas[:i] if not x.startswith('Original:')).strip()
                pt = '\n'.join(linhas[i:])[4:].strip()
                if not (len(pt) > 2 * len(en) and len(pt) > 250):
                    continue
                tipo = 'curto'
            else:
                txt = '\n'.join(x for x in linhas if not x.startswith('Original:'))
                if len(PT.findall(txt)) < 2:
                    continue
                en, pt, tipo = '', txt.strip(), 'so_pt'
            chave = hashlib.sha1(h.encode('utf-8')).hexdigest()[:16]
            yield p, m, chave, tipo, en, pt, orig, ind


def main():
    if '--extrair' in sys.argv:
        n = int(sys.argv[sys.argv.index('--extrair') + 1])
        pasta = sys.argv[sys.argv.index('--pasta') + 1]
        lotes, tam = [[] for _ in range(n)], [0] * n
        for p, m, chave, tipo, en, pt, orig, ind in blocos():
            k = tam.index(min(tam))
            lotes[k].append(dict(hash=chave, file=os.path.relpath(p, SRC).replace(os.sep, '/'), english=en, pt=pt))
            tam[k] += len(pt)
        for k, l in enumerate(lotes):
            with open(os.path.join(pasta, 'doc_%d.jsonl' % k), 'w', encoding='utf-8') as f:
                for it in l:
                    f.write(json.dumps(it, ensure_ascii=False) + '\n')
        print('%d blocos em %d lotes: %s' % (sum(map(len, lotes)), n, tam))
        return
    novos = {}
    for a in sys.argv[sys.argv.index('--aplicar') + 1:]:
        if a.startswith('--'):
            break
        for l in open(a, encoding='utf-8'):
            x = l.rstrip('\n').split('\t')
            if len(x) == 2 and x[1].strip():
                novos[x[0].strip()] = x[1].strip()
    por = {}
    for p, m, chave, tipo, en, pt, orig, ind in blocos():
        if chave in novos:
            por.setdefault(p, []).append((m, novos[chave], pt, orig, ind))
    n = 0
    for p, ms in por.items():
        t = open(p, encoding='utf-8').read()
        for m, txt, pt, orig, ind in sorted(ms, key=lambda x: -x[0].start()):
            larg = 116 - len(ind)
            L = []
            for par in [x.strip() for x in txt.replace('\\n', '\n').split('\n')]:
                if par:
                    L += [ind + '/// ' + x for x in textwrap.wrap(par, larg, break_long_words=False, break_on_hyphens=False)]
                    L.append(ind + '///')
            L = L[:-1]
            L += [ind + '/// ' + o for o in orig]
            L.append(ind + '///')
            pl = pt.split('\n')
            L.append(ind + '/// [pt] ' + pl[0])
            L += [ind + '/// ' + x if x else ind + '///' for x in pl[1:]]
            t = t[:m.start(1)] + '\n'.join(L) + '\n' + t[m.end(1):]
            n += 1
        if '--gravar' in sys.argv:
            open(p, 'w', encoding='utf-8').write(t)
    print('%d blocos com o inglês completo' % n)


if __name__ == '__main__':
    main()
