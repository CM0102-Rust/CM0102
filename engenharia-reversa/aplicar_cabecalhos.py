r"""Aplica os resumos em inglês propostos para as rotinas cujo cabeçalho só tinha português (etapa 4 da reorganização,
07/11/2026). Entrada: TSV `<função>\t<resumo>`. Em cima da função entra:

    /// <resumo>
    /// Original: `Classe::metodo` (0x6cd330, match_eng.cpp).
    ///
    /// [pt] <o comentário que já existia>

O nome original e o endereço vêm de docs/NOMES.csv; o arquivo, do mapa de funções. Idempotente (cabeçalho com [pt]
fica como está). É só comentário: o código gerado não muda.

Uso: python aplicar_cabecalhos.py arquivo1.tsv [...] [--gravar]
"""
import csv
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import cabecalhos  # noqa: E402
import mover_rotinas  # noqa: E402
import partir_motor  # noqa: E402


def main():
    resumos = {}
    for t in [a for a in sys.argv[1:] if not a.startswith('--')]:
        for l in open(t, encoding='utf-8'):
            p = l.rstrip('\n').split('\t')
            if len(p) >= 2 and p[0] and p[1]:
                resumos[p[0].strip()] = p[1].strip()
    info = {}
    nomes = os.path.join(R, 'docs', 'NOMES.csv')
    for r in csv.DictReader(open(nomes, encoding='utf-8')):
        info[r['en']] = (r['endereco'], r['original'])
    mapa = partir_motor.mapa()
    feitos = 0
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if not f.endswith('.rs'):
                continue
            p = os.path.join(d, f)
            t = open(p, encoding='utf-8').read()
            partes, pos = [], 0
            for n, a, b in mover_rotinas.blocos(t):
                if n not in resumos:
                    continue
                bloco = t[a:b]
                ini = re.search(r'^(?:pub\(crate\) )?(?:unsafe )?(?:extern "\w+" )?fn ', bloco, re.M).start()
                topo = bloco[:ini].split('\n')[:-1]
                docs = [l for l in topo if l.lstrip().startswith('///')]
                attrs = [l for l in topo if l.lstrip().startswith('#[')]
                if any('[pt]' in l for l in docs):
                    continue
                cab = ['/// ' + l for l in cabecalhos.quebrar(resumos[n])]
                end, orig = info.get(n, ('', ''))
                arq = os.path.basename(mapa.get(int(end, 16), '')) if end else ''
                if end:
                    cab.append('/// Original: %s(%s%s).' % ('`%s` ' % orig if orig and 'sem nome' not in orig else '', end,
                                                             ', ' + arq if arq else ''))
                if docs:
                    docs[0] = docs[0].replace('/// ', '/// [pt] ', 1) if docs[0].startswith('/// ') else docs[0]
                    cab += ['///'] + docs
                partes.append(t[pos:a])
                partes.append('\n'.join(cab + attrs) + '\n' + bloco[ini:])
                pos = b
                feitos += 1
            partes.append(t[pos:])
            novo = ''.join(partes)
            if novo != t and '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8').write(novo)
    print('%d cabeçalhos em inglês (de %d resumos)' % (feitos, len(resumos)))


if __name__ == '__main__':
    main()
