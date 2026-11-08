r"""Os campos em português das structs do crate em inglês (etapa 4 da reorganização, 08/11/2026; o leitor de fora,
rodadas 7 e 8: `EnglishDivision { vtabela, estadio, fases }`, `{ dia, mes, semana, premio }`...). A tradução vem do
dicionário do traduzir_locais.py. Troca a declaração do campo dentro de `struct X { .. }`, os acessos `.campo` (não os
métodos `.campo(`) e as entradas `campo:` / `campo,` no primeiro nível dos literais `Tipo { .. }`. Só no código. As
listas de mutação e a fila recebem a mesma troca.

Uso: python renomear_campos_struct.py [--gravar]
"""
import glob
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import traduzir_locais as tl  # noqa: E402


def campos_pt():
    m_ = {}
    for p in glob.glob(os.path.join(SRC, '**', '*.rs'), recursive=True):
        s = open(p, encoding='utf-8').read()
        for m in re.finditer(r'struct (\w+)\s*\{(.*?)\n\}', s, re.S):
            for fm in re.finditer(r'^\s*(?:pub(?:\(crate\))? )?(\w+):', m.group(2), re.M):
                tr = tl.traduzir(fm.group(1))
                if tr:
                    m_[fm.group(1)] = tr
    return m_


def trocar(s, mapa):
    nomes = '|'.join(map(re.escape, sorted(mapa, key=len, reverse=True)))
    # 1) declarations inside struct bodies
    def corpo(m):
        return m.group(1) + re.sub(r'^(\s*(?:pub(?:\(crate\))? )?)(%s):' % nomes, lambda k: k.group(1) + mapa[k.group(2)] + ':', m.group(2), flags=re.M) + m.group(3)
    s = re.sub(r'(struct \w+\s*\{)(.*?)(\n\})', corpo, s, flags=re.S)
    # 2) field accesses (not method calls)
    s = re.sub(r'(?<!\.)\.(%s)\b(?!\s*\()' % nomes, lambda k: '.' + mapa[k.group(1)], s)
    # 3) struct literals: `Type { field: x, field2, .. }` at the first level (nested literals too)
    return literais(s, mapa, nomes)


def literais(s, mapa, nomes):
    out, i = [], 0
    for m in re.finditer(r'\b[A-Z]\w*\s*\{', s):
        if m.start() < i:
            continue
        j, nivel = m.end(), 1
        while j < len(s) and nivel:
            if s[j] in '{([':
                nivel += 1
            elif s[j] in '})]':
                nivel -= 1
            j += 1
        bloco = literais(s[m.end():j - 1], mapa, nomes)  # the nested literals first
        # a struct literal has `ident:` / `ident,` entries; an impl/fn body does not (skip those blocks' top level)
        antes = re.findall(r'(->|\w+)\s*$', s[max(0, m.start() - 40):m.start()])
        palavra = antes[-1] if antes else ''
        eh_literal = re.match(r'\s*(\w+\s*:(?!:)|\w+\s*,|\w+\s*$|\.\.)', bloco) is not None and \
            palavra not in ('impl', 'struct', 'enum', 'trait', 'for', 'mod', 'union', 'where', 'dyn', '->', 'match', 'if', 'while')
        res, d, k = [], 0, 0
        while eh_literal and k < len(bloco):
            c = bloco[k]
            if c in '{([':
                d += 1
            elif c in '})]':
                d -= 1
            antes_k = bloco[:k].rstrip()
            if d == 0 and (antes_k == '' or antes_k.endswith(',')) and (k == 0 or not (bloco[k - 1].isalnum() or bloco[k - 1] == '_')):
                mm = re.match(r'(%s)(?=\s*:(?!:))' % nomes, bloco[k:])
                if mm:
                    res.append(mapa[mm.group(1)])
                    k += len(mm.group(1))
                    continue
                mm = re.match(r'(%s)(?=\s*,|\s*$)' % nomes, bloco[k:])
                if mm:  # shorthand `field` (a local of the same name): `new: field`
                    res.append('%s: %s' % (mapa[mm.group(1)], mm.group(1)))
                    k += len(mm.group(1))
                    continue
            res.append(c)
            k += 1
        out.append(s[i:m.end()])
        out.append(''.join(res) if eh_literal else bloco)
        out.append('}')
        i = j
    out.append(s[i:])
    return ''.join(out)


def main():
    mapa = campos_pt()
    print(len(mapa), 'campos')
    n = 0
    alvos = glob.glob(os.path.join(SRC, '**', '*.rs'), recursive=True)
    for p in alvos:
        t = open(p, encoding='utf-8', newline='').read()
        # only code: comments and strings are kept by masking them first
        mascaras = []

        def guarda(m):
            mascaras.append(m.group(0))
            return '\x00%d\x00' % (len(mascaras) - 1)
        mascarado = re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', guarda, t)
        novo = trocar(mascarado, mapa)
        novo = re.sub(r'\x00(\d+)\x00', lambda m: mascaras[int(m.group(1))], novo)
        if novo != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(novo)
    print('%d arquivos' % n)
    if '--gravar' in sys.argv:
        import json
        import runpy
        pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
        for f in sorted(os.listdir(pasta)):
            if not f.endswith('.py'):
                continue
            p = os.path.join(pasta, f)
            try:
                ms = runpy.run_path(p).get('MUTACOES', [])
            except Exception:
                continue
            s = open(p, encoding='utf-8').read()
            for m in ms:
                ini = max(s.find("('%s'," % m[0]), s.find('("%s",' % m[0]))
                for k in (1, 2):
                    nv = trocar(m[k], mapa)
                    if nv == m[k]:
                        continue
                    for a, b in ((repr(m[k]), repr(nv)), (json.dumps(m[k], ensure_ascii=False), json.dumps(nv, ensure_ascii=False))):
                        j = s.find(a, ini)
                        if j >= 0:
                            s = s[:j] + b + s[j + len(a):]
                            break
            open(p, 'w', encoding='utf-8').write(s)
        print('gravado')


if __name__ == '__main__':
    main()
