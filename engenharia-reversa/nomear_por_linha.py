r"""Applies the field readers' verdicts given per source line (10/11/2026, decision (e), pilot on manager_manager.rs).

Each entry of the JSON files: `{line, offset, base, module, verdict, name, width, rust_type, description, confidence,
evidence[, new_module]}` for a raw `base + 0xOFF` at that line of FILE:
- `new`: the field goes into rust/src/motor/fields.rs, in `module` (created at the end when `new_module`), ordered by
  offset, with the reliability mark of docs/CONFIANCA_DOS_CAMPOS.md (`[verified: ..]`, nothing for `read`, `[inference]`);
- `existing`: the name already in `module` (`NAME` or `NAME + k`);
- `not_field` / `unknown`: left alone.
The same (module, offset) given two names is a conflict: reported, nothing applied for it. Then, at each line,
every `+ 0xOFF` becomes `+ module::NAME` when all the entries for that line and offset agree. Run registros.py, then
`tipar_registros.py --acessos module` and metodos_em_cadeia.py so typed values read them as methods.
An offset written in decimal (`ln() + 6`) comes with `literal`, the exact text to replace; a base inside the record
comes with `field_offset` (the field is defined, the line is left alone).
Usage: python nomear_por_linha.py FILE.rs VERDICTS.json [...] [--gravar]
[pt] Aplica os vereditos dos leitores de campo dados por linha: campos novos no fields.rs com a marca de confiança e
o hexadecimal cru da linha vira `modulo::NOME`.
"""
import collections
import json
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FIELDS = os.path.join(R, 'rust', 'src', 'motor', 'fields.rs')


def num(x):
    """An offset as the reader wrote it: `0x1c` or, as in the code, `6`."""
    return int(x, 16) if x.lower().startswith('0x') else int(x)


def marca(e):
    ev = re.sub(r'\s+', ' ', (e.get('evidence') or '')).strip().rstrip('.')
    if e.get('confidence') == 'verified':
        return ' [verified: %s]' % ev[:160]
    if e.get('confidence') == 'inferred':
        return ' [inference]'
    return ''


def inserir(s, e, off, nome):
    mod = e['module']
    m = re.search(r'pub\(crate\) mod %s \{\n' % mod, s)
    if not m:
        if not e.get('new_module'):
            return s, 'módulo %s não existe' % mod
        s = s.rstrip('\n') + '\n\n/// %s\n#[allow(dead_code)]\npub(crate) mod %s {\n}\n' % (
            re.sub(r'\s+', ' ', e.get('record') or ('`%s` record, named by the field readers' % mod)), mod)
        m = re.search(r'pub\(crate\) mod %s \{\n' % mod, s)
    fim = s.index('\n}', m.end() - 1)
    corpo = s[m.end():fim]
    achado = re.search(r'pub\(crate\) const (\w+): usize = %s;' % hex(off), corpo)
    if achado:
        return s, achado.group(1)
    if re.search(r'const %s: usize' % nome, corpo):
        return s, 'nome %s já usado com outro deslocamento' % nome
    desc = re.sub(r'\s+', ' ', e.get('description') or '').strip().rstrip('.')
    linha = '    pub(crate) const %s: usize = %s; // %s byte(s): %s%s' % (nome, hex(off), e.get('width') or '?', desc, marca(e))
    linhas = corpo.split('\n') if corpo else []
    k = 0
    while k < len(linhas):
        mm = re.match(r'\s*pub\(crate\) const \w+: usize = (0x[0-9a-f]+|\d+);', linhas[k])
        if mm and int(mm.group(1), 0) > off:
            break
        k += 1
    linhas.insert(k, linha)
    return s[:m.end()] + '\n'.join(linhas) + ('\n' if not corpo else '') + s[fim:], nome


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    alvo, jsons = args[0], args[1:]
    grava = '--gravar' in sys.argv
    ent = []
    for j in jsons:
        ent += [e for e in json.load(open(j, encoding='utf-8')) if e.get('kind') not in ('bit', 'review') and re.fullmatch(r'0x[0-9a-fA-F]+|\d+', str(e.get('offset', '')))]
    usar = [e for e in ent if e.get('verdict') in ('new', 'existing') and e.get('name') and e.get('module')]
    print(collections.Counter(e.get('verdict') for e in ent), collections.Counter(e.get('confidence') for e in usar))
    nomes = collections.defaultdict(set)
    for e in usar:
        nomes[(e['module'], num(e['offset']))].add(e['name'].split('::')[-1].strip())
    conflito = {k for k, v in nomes.items() if len(v) > 1}
    for k in sorted(conflito):
        print('CONFLITO', k[0], hex(k[1]), sorted(nomes[k]))
    s = open(FIELDS, encoding='utf-8').read()
    troca, novos = {}, 0
    # the base is inside the record (`x = row + 0x1c; x + 0x1a`): the reader gives the field's own offset; the field
    # is defined, the line stays as it is
    for e in [e for e in usar if e.get('field_offset')]:
        if e['verdict'] == 'new':
            nome = (e.get('field_name') or e['name']).split('::')[-1].split('+')[0].strip()
            antes = s
            s, r = inserir(s, e, num(e['field_offset']), nome)
            novos += s != antes
    # an existing `NAME + k` (`cup_round::DAY + 0x68`, the next round's record) is a plain replacement all the same
    usar = [e for e in usar if not e.get('field_offset') or (e['verdict'] == 'existing' and '+' in e['name'])]
    for e in usar:
        off = num(e['offset'])
        k = (e['module'], off)
        if k in conflito or k in troca:
            continue
        nome = e['name'].split('::')[-1].strip()
        if e['verdict'] == 'existing':
            if not re.fullmatch(r'[A-Z][A-Z0-9_]*(\s*\+\s*(0x[0-9a-f]+|\d+))?', nome):
                print('nome com termo variável, pulado:', e['line'], nome)
                continue
            base = nome.split('+')[0].strip()
            if not re.search(r'pub\(crate\) mod %s \{[^}]*const %s: usize' % (e['module'], base), s):
                print('existente não achado:', e['module'], nome)
                continue
            troca[k] = '%s::%s' % (e['module'], nome)
            continue
        if not re.fullmatch(r'[A-Z][A-Z0-9_]*', nome):
            # a `new` named `BASE + 0x13` would write `const BASE + 0x13: usize`, which rustc's error recovery chews on
            # for hours (11/11/2026): skipped
            print('nome novo inválido, pulado:', e['line'], nome)
            continue
        antes = s
        s, r = inserir(s, e, off, nome)
        if not re.fullmatch(r'[A-Z][A-Z0-9_]*', r):
            print('pulado:', e['module'], hex(off), r)
            continue
        novos += s != antes
        troca[k] = '%s::%s' % (e['module'], r)
    p = os.path.join(R, 'rust', 'src', alvo) if not os.path.isabs(alvo) else alvo
    linhas = open(p, encoding='utf-8', newline='').read().split('\n')
    # the readers describe the base in words, so the line is matched by the offset alone: every `+ 0xOFF` of that
    # line, when all the entries for (line, offset) agree (a not_field there, or two names, leaves it alone)
    por_linha = collections.defaultdict(set)
    feitas = 0
    for e in ent:
        k = (e.get('module'), num(e['offset']))
        if e.get('literal') and e in usar and k in troca:
            # an offset written in decimal (`ln() + 6`): the reader gives the exact text
            lit = e['literal'].strip('`')
            novo = re.sub(r'\+\s*%s$' % re.escape(e['offset']), '+ ' + troca[k], lit)
            if novo == lit:
                # an offset passed to a closure (`grava(0x104` with `let grava = |o| .. base + o ..`)
                novo = re.sub(r'^(\w+\(\s*)%s(?![0-9a-fA-Fx])' % re.escape(e['offset']), lambda m: m.group(1) + troca[k], lit)
            cod, sep, com = linhas[int(e['line']) - 1].partition('//')
            if novo != lit and lit in cod:
                linhas[int(e['line']) - 1] = cod.replace(lit, novo) + sep + com
                feitas += 1
            else:
                print('linha %s: `%s` não achado' % (e['line'], lit))
            continue
        por_linha[(int(e['line']), e['offset'])].add(troca.get(k) if e in usar else None)
    for (ln, off), alvos in sorted(por_linha.items()):
        if len(alvos) != 1 or None in alvos:
            continue
        nome = alvos.pop()
        cod, sep, com = linhas[ln - 1].partition('//')
        pat = r'(\+\s*)' + re.escape(off) + r'(?![0-9a-fA-Fx])(?!\s*\*)'
        # only outside string literals (a log message that prints "+0x3e" keeps its text)
        partes = re.split(r'("(?:[^"\\]|\\.)*")', cod)
        novo = ''.join(p if p.startswith('"') else re.sub(pat, lambda m: m.group(1) + nome, p) for p in partes)
        if novo == cod:
            print('linha %d: `+ %s` não achado' % (ln, off))
            continue
        linhas[ln - 1] = novo + sep + com
        feitas += 1
    print('%d campos novos; %d deslocamentos com nome; %d linhas trocadas' % (novos, len(troca), feitas))
    if grava:
        open(FIELDS, 'w', encoding='utf-8').write(s)
        open(p, 'w', encoding='utf-8', newline='').write('\n'.join(linhas))


if __name__ == '__main__':
    main()
