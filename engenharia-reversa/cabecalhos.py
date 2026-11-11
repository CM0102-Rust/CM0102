r"""Cabeçalho bilíngue em cada rotina portada do motor (etapa 4 da reorganização, 07/11/2026).

Fonte única: docs/ROTINAS.en.md e docs/ROTINAS.md (gerados do DOCS de cada leva) e o glossário (o nome original no
jogo). Em cima de cada `fn` portada entra:

    /// <o que faz, em inglês>
    /// Original: `Classe::metodo` (0x5d6280, game.cpp, cdecl).

Desde 11/11/2026 (decisão (c) do Lucas) sem o parágrafo [pt]: o português fica no ROTINAS.md (sem_pt.py tirou os
antigos). Idempotente: rotina cujo cabeçalho já tem "Original:" (ou "[pt]") fica como está.

Uso: python cabecalhos.py [--gravar]
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))


def rotinas(arq):
    """{nome: (endereço, convenção, arquivo, texto)} da tabela do ROTINAS."""
    d = {}
    for l in open(os.path.join(R, 'docs', arq), encoding='utf-8'):
        m = re.match(r'\| `(\w+)\(\)` \| (0x[0-9a-f]+) \| ([^|]*) \| (.*) \|\s*$', l)
        if m and m.group(1) not in d:
            arquivo, _, texto = m.group(4).partition(': ')
            if not texto:
                arquivo, texto = '', m.group(4)
            d[m.group(1)] = (m.group(2), m.group(3).strip(), arquivo.strip(), texto.strip())
    return d


def originais():
    s = open(os.path.join(R, 'engenharia-reversa', 'glossario.py'), encoding='utf-8').read()
    i = s.index('EN = {')
    ns = {}
    exec(s[i:s.index('\n}\n', i) + 3], ns)
    return ns['EN']


def arquivos():
    yield os.path.join(SRC, 'motor.rs')
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if f.endswith('.rs'):
                yield os.path.join(d, f)


def quebrar(texto, largura=116):
    linhas, atual = [], ''
    for p in texto.split():
        if atual and len(atual) + 1 + len(p) > largura:
            linhas.append(atual)
            atual = p
        else:
            atual = (atual + ' ' + p).strip()
    return linhas + ([atual] if atual else [])


def main():
    en, pt, orig = rotinas('ROTINAS.en.md'), rotinas('ROTINAS.md'), originais()
    FN = re.compile(r'^((?:pub )?(?:unsafe )?(?:extern "\w+" )?fn (\w+)\b)', re.M)
    feitos = sem_doc = 0
    for f in arquivos():
        t = open(f, encoding='utf-8').read()
        partes, pos = [], 0
        for m in FN.finditer(t):
            nome = m.group(2)
            if nome not in en:
                continue
            # o bloco de cima: comentários /// e atributos #[...]
            ini = m.start()
            linhas_antes = t[:ini].split('\n')[:-1]
            k = len(linhas_antes)
            while k > 0 and linhas_antes[k - 1].lstrip().startswith(('///', '#[')):
                k -= 1
            bloco = linhas_antes[k:]
            if any('[pt]' in l or 'Original:' in l for l in bloco):
                continue
            docs = [l for l in bloco if l.lstrip().startswith('///')]
            attrs = [l for l in bloco if l.lstrip().startswith('#[')]
            a, conv, arq, texto = en[nome]
            cab = ['/// ' + l for l in quebrar(texto)]
            o = orig.get(nome)
            cab.append('/// Original: %s(%s%s%s).' % ('`%s` ' % o if o else '', a, ', ' + arq if arq else '', ', ' + conv if conv else ''))
            # since 11/11/2026 (decision (c)) no [pt] paragraph: the Portuguese stays in docs/ROTINAS.md
            sem_doc += not docs
            inicio_bloco = len('\n'.join(linhas_antes[:k])) + (1 if k else 0)
            partes.append(t[pos:inicio_bloco])
            partes.append('\n'.join(cab + attrs) + '\n')
            pos = ini
            feitos += 1
        partes.append(t[pos:])
        novo = ''.join(partes)
        if novo != t and '--gravar' in sys.argv:
            open(f, 'w', encoding='utf-8').write(novo)
    print('%d cabeçalhos (%d sem comentário antes); rotinas no ROTINAS: %d' % (feitos, sem_doc, len(en)))


if __name__ == '__main__':
    main()
