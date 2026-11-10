r"""Which record each pointer field points to (09/11/2026, typed records, the outside reader's round 10, item 1: "typed
records stop after one hop"). A field whose getter returns a raw address (`person.club_job() -> usize`) is classified by
how the code uses its value: as the base of another record's field (`person.club_job() + club::MANAGER`), wrapped in a
record type (`Club(person.club_job())`), or at an offset that is exactly one field of a record that the same value
is otherwise used as. A field goes in when one record type has at least 2 uses and 80% of them. Writes
levas/reorganizacao/ponteiros.json ({"module::FIELD": {"type": "Club", "uses": n, "examples": [...]}}), which
registros.py reads to make the getter return the type.
Usage: python ponteiros_registros.py
[pt] Para que registro aponta cada campo-ponteiro, pelo uso no código; grava levas/reorganizacao/ponteiros.json.
"""
import collections
import json
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import tipar_registros as tr  # noqa: E402


def registros():
    """{Type: module}, {(Type, method): return type}"""
    t = open(os.path.join(SRC, 'motor', 'records.rs'), encoding='utf-8').read()
    tipo_mod = {m.group(2): m.group(1) for m in re.finditer(r'/// A `(\w+)` record in the game.*\n(?:.*\n){0,3}pub\(crate\) struct (\w+)\(', t)}
    ret, atual, campo = {}, None, {}
    for l in t.split('\n'):
        m = re.match(r'impl (\w+) \{', l)
        if m:
            atual = m.group(1)
        m = re.match(r'\s*/// `(\w+)::(\w+)` \(', l)
        if m:
            ult = (m.group(1), m.group(2))
        m = re.match(r'\s*pub\(crate\) unsafe fn (?:r#)?(\w+)\(self\) -> (\w+)', l)
        if m and atual:
            ret[(atual, m.group(1))] = m.group(2)
            campo[(atual, m.group(1))] = ult
    return tipo_mod, ret, campo


def main():
    tipo_mod, ret, campo = registros()
    mod_tipo = {v: k for k, v in tipo_mod.items()}
    tipos = '|'.join(sorted(tipo_mod, key=len, reverse=True))
    votos = collections.defaultdict(collections.Counter)
    ex = collections.defaultdict(list)
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs') or f in ('records.rs', 'fields.rs', 'exe_calls.rs'):
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8').read()
            for _, a, b in mover_rotinas.blocos(t):
                mk = tr.mascara(t[a:b])
                tip = {}
                for m in re.finditer(r'(?<![\w.])([a-z_][a-z0-9_]*)\s*:\s*(%s)\b' % tipos, mk):
                    tip[m.group(1)] = m.group(2)
                for m in re.finditer(r'\blet\s+([a-z_][a-z0-9_]*)\s*=\s*(%s)\(' % tipos, mk):
                    tip[m.group(1)] = m.group(2)
                if not tip:
                    continue
                nomes = '|'.join(map(re.escape, tip))
                # x.f() + module::G   /   x.f().addr() + module::G
                for m in re.finditer(r'(?<![\w.])(%s)\.(\w+)\(\)(?:\.addr\(\))?\s*\+\s*([a-z_]+)::[A-Z]' % nomes, mk):
                    k = (tip[m.group(1)], m.group(2))
                    if k in ret and m.group(3) in mod_tipo:
                        votos[k][mod_tipo[m.group(3)]] += 1
                        ex[k].append(mk[m.start():m.end() + 20].split('\n')[0])
                # Type(x.f())
                for m in re.finditer(r'\b(%s)\(\s*(%s)\.(\w+)\(\)\s*\)' % (tipos, nomes), mk):
                    k = (tip[m.group(2)], m.group(3))
                    if k in ret:
                        votos[k][m.group(1)] += 1
                        ex[k].append(mk[m.start():m.end()])
    out = {}
    for k, c in votos.items():
        if ret[k] in tipo_mod or ret[k] not in ('usize', 'u32', 'i32'):
            continue  # already typed, or not an address
        tipo, n = c.most_common(1)[0]
        if n >= 2 and n * 5 >= sum(c.values()) * 4:
            mod, fld = campo[k]
            out['%s::%s' % (mod, fld)] = {'type': tipo, 'uses': n, 'examples': ex[k][:3]}
    json.dump(dict(sorted(out.items())), open(os.path.join(R, 'levas', 'reorganizacao', 'ponteiros.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    print('%d campos-ponteiro com tipo' % len(out))
    for k, v in sorted(out.items(), key=lambda kv: -kv[1]['uses'])[:30]:
        print('  %-40s -> %-14s %d' % (k, v['type'], v['uses']))


if __name__ == '__main__':
    main()
