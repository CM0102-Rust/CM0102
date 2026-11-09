r"""Applies the readers' verdicts on unnamed field offsets (09/11/2026, typed records migration): for each entry of the
JSON files (`{type, offset, verdict, name, width, rust_type, description, confidence, evidence}`):

- `new`: the field goes into rust/src/motor/fields.rs, in the record's module, ordered by offset, with its width,
  description and reliability mark (`[verified: evidence]`, nothing for `read`, `[inference]` for `inferred`; see
  docs/CONFIANCA_DOS_CAMPOS.md);
- `existing` (a name in the record's own module, possibly `NAME + k` for an array element): kept as given;
- `subclass` / `wrong_variable`: left alone.

Then, in every function, `x.addr() + 0xOFF` on a variable `x` of that record type becomes `x.addr() + module::NAME`
(run `tipar_registros.py --acessos module` afterwards to turn them into methods).
Usage: python aplicar_nomes_lidos.py VERDICTS.json [...] [--gravar]
[pt] Aplica os vereditos dos leitores sobre os deslocamentos sem nome: os novos entram no fields.rs com a marca de
confiança; os hexadecimais crus sobre variáveis tipadas viram os nomes.
"""
import json
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import tipar_registros as tr  # noqa: E402

FIELDS = os.path.join(R, 'rust', 'src', 'motor', 'fields.rs')


def modulo_do_tipo():
    t = open(os.path.join(R, 'rust', 'src', 'motor', 'records.rs'), encoding='utf-8').read()
    return {m.group(2): m.group(1) for m in re.finditer(r'/// A `(\w+)` record in the game.*\n(?:.*\n){0,3}pub\(crate\) struct (\w+)\(', t)}


def marca(e):
    ev = re.sub(r'\s+', ' ', (e.get('evidence') or '')).strip().rstrip('.')
    if e.get('confidence') == 'verified':
        return ' [verified: %s]' % ev[:160]
    if e.get('confidence') == 'inferred':
        return ' [inference]'
    return ''


def main():
    arqs = [a for a in sys.argv[1:] if not a.startswith('--')]
    grava = '--gravar' in sys.argv
    mods = modulo_do_tipo()
    vered = []
    for a in arqs:
        vered += json.load(open(a, encoding='utf-8'))
    s = open(FIELDS, encoding='utf-8').read()
    troca = {}
    novos = 0
    for e in vered:
        mod = mods.get(e['type'])
        if not mod or e['verdict'] not in ('new', 'existing') or not e.get('name'):
            continue
        off = int(e['offset'], 16)
        nome = e['name'].split('::')[-1].split('(')[0].strip()
        if e['verdict'] == 'existing':
            mm = re.fullmatch(r'(?:(\w+)::)?([A-Z][A-Z0-9_]*)(\s*\+\s*[0-9 *]+)?', e['name'].split(' (')[0].strip())
            if not mm or (mm.group(1) and mm.group(1) != mod):
                continue
            troca[(e['type'], off)] = '%s::%s%s' % (mod, mm.group(2), mm.group(3) or '')
            continue
        m = re.search(r'pub\(crate\) mod %s \{\n' % mod, s)
        fim = s.index('\n}', m.end())
        corpo = s[m.end():fim]
        if re.search(r'const %s: usize' % nome, corpo):
            troca[(e['type'], off)] = '%s::%s' % (mod, nome)
            continue
        desc = re.sub(r'\s+', ' ', e.get('description') or '').strip().rstrip('.')
        linha = '    pub(crate) const %s: usize = %s; // %s byte(s): %s%s' % (nome, hex(off), e.get('width') or '?', desc, marca(e))
        linhas = corpo.split('\n')
        k = 0
        while k < len(linhas):
            mm = re.match(r'\s*pub\(crate\) const \w+: usize = (0x[0-9a-f]+|\d+);', linhas[k])
            if mm and int(mm.group(1), 0) > off:
                break
            k += 1
        linhas.insert(k, linha)
        s = s[:m.end()] + '\n'.join(linhas) + s[fim:]
        troca[(e['type'], off)] = '%s::%s' % (mod, nome)
        novos += 1
    print('%d campos novos; %d deslocamentos com nome' % (novos, len(troca)))
    if grava:
        open(FIELDS, 'w', encoding='utf-8').write(s)
    tipos = '|'.join(sorted({k[0] for k in troca}))
    if not tipos:
        return
    n = 0
    for raiz, _, fs in os.walk(os.path.join(R, 'rust', 'src')):
        for f in fs:
            if not f.endswith('.rs') or f in ('records.rs', 'exe_calls.rs', 'fields.rs'):
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8').read()
            partes, pos = [], 0
            for _, a, b in mover_rotinas.blocos(t):
                bl = t[a:b]
                mk = tr.mascara(bl)
                tip = {}
                for m in re.finditer(r'(?<![\w.])([a-z_][a-z0-9_]*)\s*:\s*(%s)\b' % tipos, mk):
                    tip[m.group(1)] = m.group(2)
                for m in re.finditer(r'\blet\s+([a-z_][a-z0-9_]*)\s*=\s*(%s)\(' % tipos, mk):
                    tip[m.group(1)] = m.group(2)
                if not tip:
                    continue
                ed = []
                for m in re.finditer(r'(?<![\w.])(%s)\.addr\(\)\s*\+\s*(0x[0-9a-f]+|\d+)\b(?!\s*\*)' % '|'.join(map(re.escape, tip)), mk):
                    k = (tip[m.group(1)], int(m.group(2), 0))
                    if k in troca:
                        ed.append((m.start(2), m.end(2), troca[k]))
                if not ed:
                    continue
                novo = bl
                for a2, b2, txt in sorted(ed, reverse=True):
                    novo = novo[:a2] + txt + novo[b2:]
                n += len(ed)
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and ''.join(partes) != t:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d hexadecimais crus viraram nomes' % n)


if __name__ == '__main__':
    main()
