r"""O inglês completo nos cabeçalhos das rotinas (etapa 4 da reorganização, 08/11/2026). Muitos cabeçalhos têm só um
resumo curto em inglês e o detalhe no parágrafo [pt]. `--extrair N` grava em N lotes JSONL (pasta dada em --pasta) os
cabeçalhos em que o português é mais que o dobro do inglês; os tradutores devolvem TSV `<arquivo>\t<função>\t<inglês>`
(uma linha, parágrafos separados por ` \n `). `--aplicar a.tsv [...]` troca o texto em inglês acima de `Original:` pelo
inglês completo (quebrado em 120 colunas); o [pt] fica. É só comentário: o código gerado não muda.

`--extrair-hex N`: os cabeçalhos que narram deslocamentos e endereços em hexadecimal (decisão (b), 11/11/2026), para
os leitores reescreverem com os nomes do corpo.

Uso: python cabecalhos_completos.py --extrair N | --extrair-hex N --pasta DIR | --aplicar a.tsv [...] [--gravar]
"""
import glob
import json
import os
import re
import sys
import textwrap

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = os.path.join(R, 'rust', 'src', 'motor')
CAB = re.compile(r'((?:^///[^\n]*\n)+)((?:#\[[^\n]*\n)*(?:pub\(crate\) )?(?:unsafe )?(?:extern "\w+" )?fn (\w+))', re.M)


def cabecalhos():
    for p in sorted(glob.glob(os.path.join(BASE, '**', '*.rs'), recursive=True)):
        rel = os.path.relpath(p, BASE).replace(os.sep, '/')
        if rel.startswith('compare/'):
            continue
        t = open(p, encoding='utf-8').read()
        for m in CAB.finditer(t):
            h = m.group(1)
            if '/// Original:' not in h:
                continue
            i_or = h.index('/// Original:')
            en = re.sub(r'^///\s?', '', h[:i_or], flags=re.M).strip()
            pt = re.sub(r'^///\s?', '', h[h.index('[pt]') + 4:], flags=re.M).strip() if '[pt]' in h else ''
            yield rel, m.group(3), en, pt, p, m


def main():
    if '--extrair' in sys.argv or '--extrair-hex' in sys.argv:
        hexa = '--extrair-hex' in sys.argv
        n = int(sys.argv[sys.argv.index('--extrair-hex' if hexa else '--extrair') + 1])
        pasta = sys.argv[sys.argv.index('--pasta') + 1]
        # --extrair-hex (11/11/2026, decision (b)): the headers whose English narrates offsets and addresses in hex,
        # for the readers to rewrite with the names the body uses
        itens = [dict(file=rel, fn=fn, english=en, pt=pt) for rel, fn, en, pt, _, _ in cabecalhos()
                 if (re.search(r'0x[0-9a-fA-F]{2,}|`\+\d', en) if hexa else len(pt) > 2 * len(en) and len(pt) > 250)]
        for it in itens:
            del it['pt']
        lotes = [[] for _ in range(n)]
        tam = [0] * n
        for it in itens:  # por arquivo, para cada tradutor ler um arquivo inteiro
            k = tam.index(min(tam)) if not lotes or not any(l and l[-1]['file'] == it['file'] for l in lotes) else \
                next(i for i, l in enumerate(lotes) if l and l[-1]['file'] == it['file'])
            lotes[k].append(it)
            tam[k] += len(it.get('pt') or it['english'])
        for k, l in enumerate(lotes):
            with open(os.path.join(pasta, 'cab_%d.jsonl' % k), 'w', encoding='utf-8') as f:
                for it in l:
                    f.write(json.dumps(it, ensure_ascii=False) + '\n')
        print('%d cabeçalhos em %d lotes: %s' % (len(itens), n, tam))
        return
    novos = {}
    for a in sys.argv[sys.argv.index('--aplicar') + 1:]:
        if a.startswith('--'):
            break
        for l in open(a, encoding='utf-8'):
            p = l.rstrip('\n').split('\t')
            if len(p) == 3 and p[2].strip():
                novos[(p[0].strip(), p[1].strip())] = p[2].strip()
    por_arq = {}
    for rel, fn, en, pt, p, m in cabecalhos():
        if (rel, fn) in novos:
            por_arq.setdefault(p, []).append((m, novos[(rel, fn)]))
    n = 0
    for p, ms in por_arq.items():
        t = open(p, encoding='utf-8').read()
        for m, txt in sorted(ms, key=lambda x: -x[0].start()):
            h = m.group(1)
            i_or = h.index('/// Original:')
            linhas = []
            for par in [x.strip() for x in txt.replace('\\n', '\n').split('\n')]:
                if par:
                    linhas += ['/// ' + x for x in textwrap.wrap(par, 116, break_long_words=False, break_on_hyphens=False)]
                    linhas.append('///')
            novo_h = '\n'.join(linhas[:-1]) + '\n' + h[i_or:]
            t = t[:m.start(1)] + novo_h + t[m.end(1):]
            n += 1
        if '--gravar' in sys.argv:
            open(p, 'w', encoding='utf-8').write(t)
    print('%d cabeçalhos com o inglês completo' % n)


if __name__ == '__main__':
    main()
