r"""Field access widths (09/11/2026): every read or write of a named field (`read_i16(club + club::REPUTATION)`,
`write_u8(..)`, `*((p + match_player::SIDE) as *const i8)`, `write_unaligned((x + league::LEGS) as *mut i16, ..)`) with
the width of the access, against the width documented in fields.rs (`// 2 byte(s), UInt16`) and against the other
accesses of the same field. A field read with two widths, or with a width other than the documented one, reads part of
it or the neighbour. Output: levas/reorganizacao/larguras_campos.tsv (width conflicts first; sign-only differences as
info). With --original, each access inside a ported routine is checked against the widths the original routine uses at
that displacement (levas/reorganizacao/larguras_no_original.tsv); rows classified by hand as harmless are in
larguras_conferidas.json (by field and Rust line: a line that moves shows up again, to be checked again).
Usage: python larguras_campos.py [--original]
[pt] As larguras de acesso dos campos com nome: cada leitura/escrita contra a largura documentada no fields.rs e contra
as outras; campo lido com duas larguras lê só parte dele ou o vizinho.
"""
import collections
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import involucros_exe as ie  # noqa: E402

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
LARG = {'u8': 1, 'i8': 1, 'bool': 1, 'u16': 2, 'i16': 2, 'u32': 4, 'i32': 4, 'usize': 4, 'isize': 4, 'f32': 4,
        'f64': 8, 'u64': 8, 'i64': 8}
CAMPO = r'\b([a-z_][a-z0-9_]*)::([A-Z][A-Z0-9_]*)\b'


def campos():
    """{(module, CONST): (offset, documented width or None, documented type)}"""
    out, mod = {}, None
    for l in open(os.path.join(SRC, 'motor', 'fields.rs'), encoding='utf-8'):
        m = re.match(r'\s*pub\(crate\) mod (\w+)', l)
        if m:
            mod = m.group(1)
            continue
        m = re.match(r'\s*pub\(crate\) const (\w+): usize = (0x[0-9a-f]+|\d+);(?:\s*//\s*(\d+) byte\(s\),?\s*(.*))?', l)
        if m and mod:
            out[(mod, m.group(1))] = (int(m.group(2), 0), int(m.group(3)) if m.group(3) else None, (m.group(4) or '').strip())
    return out


FIM = re.compile(r'(?:^|\+)\s*' + CAMPO + r'\s*$')


def no_fim(endereco):
    """The field when the address is `base + module::FIELD` at the outer level (not inside a nested read)."""
    e = endereco.strip()
    while e.startswith('(') and e.endswith(')'):
        args, fim = ie.args_da_chamada(e, 0)
        if fim != len(e) or args is None or len(args) != 1:
            break
        e = args[0].strip()
    m = FIM.search(e)
    if not m:
        return None
    antes = e[:m.start()]
    if antes.count('(') != antes.count(')'):
        return None
    return m.group(1), m.group(2)


def acessos(texto):
    """(module, CONST, type, kind, position) for each access whose address ends in the field."""
    for m in re.finditer(r'\b(read|write)_(u8|i8|u16|i16|u32|i32|usize|f32|f64|u64|i64)\(', texto):
        args, _ = ie.args_da_chamada(texto, m.end() - 1)
        if args:
            c = no_fim(args[0])
            if c:
                yield c[0], c[1], m.group(2), m.group(1), m.start()
    for m in re.finditer(r'\)\s*as\s*\*(const|mut)\s+(\w+)', texto):
        if m.group(2) not in LARG:
            continue
        # walk back to the matching '('
        nivel, j = 0, m.start()
        while j >= 0:
            if texto[j] == ')':
                nivel += 1
            elif texto[j] == '(':
                nivel -= 1
                if nivel == 0:
                    break
            j -= 1
        if j < 0:
            continue
        c = no_fim(texto[j + 1:m.start()])
        if c:
            yield c[0], c[1], m.group(2), 'ptr', j


def original_por_rotina():
    """{rust function name: original address}: the routines! table, plus `[pt] { 0xADDR }` headers of helpers."""
    import tabela_motor
    out = {r[2]: r[0] for r in tabela_motor.rotinas()}
    return out


ACESSOS_ORIG = {}


def larguras_no_original(va):
    """{displacement: {sizes}} of the memory operands [reg + disp] (not esp/ebp-based) in the original routine."""
    if va in ACESSOS_ORIG:
        return ACESSOS_ORIG[va]
    import conferir_assinaturas as ca
    from capstone import x86
    tam = ca.TAM.get(va, 0x400)
    out = collections.defaultdict(set)
    for i in ca.md.disasm(ca.TB[va - ca.T0:va - ca.T0 + tam], va):
        for o in i.operands:
            if o.type == x86.X86_OP_MEM and o.mem.base not in (0, x86.X86_REG_ESP, x86.X86_REG_EBP):
                out[o.mem.disp].add(o.size)
    ACESSOS_ORIG[va] = out
    return out


def conferir_no_original(cs):
    """Each access to a named field inside a ported routine, against the widths the original routine uses at that
    displacement. Returns [(field, offset, width used, widths in the original, where)] for the disagreements."""
    import mover_rotinas
    orig = original_por_rotina()
    # rows already classified by hand (collision / merged / pointer: harmless), by field and Rust line
    arq = os.path.join(R, 'levas', 'reorganizacao', 'larguras_conferidas.json')
    conferidas = {(x['field'], x['where']): x['class'] for x in json.load(open(arq, encoding='utf-8'))} if os.path.exists(arq) else {}
    achados, vistos, iguais = [], 0, 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs') or f == 'fields.rs':
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8').read()
            blocos = [(n, a, b) for n, a, b in mover_rotinas.blocos(t) if n in orig]
            for mod, c, tipo, kind, pos in acessos(t):
                if (mod, c) not in cs:
                    continue
                dono = next((n for n, a, b in blocos if a <= pos < b), None)
                if dono is None:
                    continue
                off = cs[(mod, c)][0]
                ws = larguras_no_original(orig[dono]).get(off)
                if not ws:
                    continue
                vistos += 1
                if LARG[tipo] in ws:
                    iguais += 1
                    continue
                onde = '%s:%d' % (os.path.relpath(p, R).replace(os.sep, '/'), t.count('\n', 0, pos) + 1)
                if conferidas.get(('%s::%s' % (mod, c), onde)) in ('collision', 'merged', 'pointer'):
                    iguais += 1
                    continue
                achados.append(('%s::%s' % (mod, c), hex(off), tipo, sorted(ws), dono, hex(orig[dono]), onde))
    return achados, vistos, iguais


def main():
    cs = campos()
    usos = collections.defaultdict(lambda: collections.defaultdict(list))
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs') or f == 'fields.rs':
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8').read()
            for mod, c, tipo, kind, pos in acessos(t):
                if (mod, c) in cs:
                    usos[(mod, c)][tipo].append('%s:%d' % (os.path.relpath(p, R).replace('\\', '/'), t.count('\n', 0, pos) + 1))
    linhas = []
    for (mod, c), por_tipo in sorted(usos.items()):
        off, doc, doc_t = cs[(mod, c)]
        larguras = {LARG[t] for t in por_tipo}
        onde = '; '.join('%s x%d (%s)' % (t, len(v), ', '.join(v[:2])) for t, v in sorted(por_tipo.items()))
        if len(larguras) > 1:
            linhas.append(('larguras', '%s::%s' % (mod, c), hex(off), str(doc or '?'), doc_t, onde))
        elif doc and doc <= 8 and larguras != {doc}:
            linhas.append(('doc', '%s::%s' % (mod, c), hex(off), str(doc), doc_t, onde))
        elif len({t for t in por_tipo if LARG[t] < 4 or t in ('i32', 'u32')}) > 1 and len({t[0] for t in por_tipo if t[0] in 'iu'}) > 1:
            linhas.append(('sinal', '%s::%s' % (mod, c), hex(off), str(doc or '?'), doc_t, onde))
    ordem = {'larguras': 0, 'doc': 1, 'sinal': 2}
    linhas.sort(key=lambda x: (ordem[x[0]], x[1]))
    with open(os.path.join(R, 'levas', 'reorganizacao', 'larguras_campos.tsv'), 'w', encoding='utf-8') as f:
        f.write('tipo\tcampo\tdeslocamento\tlargura_doc\ttipo_doc\tusos\n')
        for l in linhas:
            f.write('\t'.join(l) + '\n')
    if '--original' in sys.argv:
        achados, vistos, iguais = conferir_no_original(cs)
        with open(os.path.join(R, 'levas', 'reorganizacao', 'larguras_no_original.tsv'), 'w', encoding='utf-8') as f:
            f.write('campo\tdeslocamento\tlargura_rust\tlarguras_no_original\trotina\tendereco\tonde\n')
            for a in achados:
                f.write('\t'.join(map(str, a)) + '\n')
        print('no original: %d acessos com o deslocamento achado na rotina original; %d com a mesma largura; %d diferentes'
              % (vistos, iguais, len(achados)))
    n = collections.Counter(l[0] for l in linhas)
    print('%d campos com acesso; larguras diferentes entre usos: %d; diferente da documentada: %d; só o sinal: %d' % (
        len(usos), n['larguras'], n['doc'], n['sinal']))


if __name__ == '__main__':
    main()
