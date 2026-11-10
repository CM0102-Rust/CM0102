r"""The bits of flag fields by name (10/11/2026, field readers, decision (e)): the readers' `kind: bit` entries
`{line, value, field: "module::FIELD", name, confidence, evidence}` become constants in rust/src/motor/bits.rs, one
module per field (`expectation_flags::TITLE`; a generic field name like FLAGS gets the record's prefix,
`board_record_flags::...`), typed as the field's getter in records.rs (u8 for a byte access `FIELD + k`), and at that
line the literal of a bit operation (`& 0x40`, `| 0x40`, `^ 0x40`, `!0x40`) becomes the name. Two names for one
(field, value) is a conflict: reported, not applied. A function that no longer compiles goes back whole.
Usage: python nomear_bits.py FILE.rs VERDICTS.json [...] [--gravar]
[pt] Os bits dos campos de flags com nome: constantes no bits.rs, um módulo por campo, e o literal da operação de
bit na linha vira o nome.
"""
import collections
import json
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import traduzir_locais as tl  # noqa: E402

BITS = os.path.join(R, 'rust', 'src', 'motor', 'bits.rs')
CAB = ('// The bits of the records\' flag fields by name, from the field readers (engenharia-reversa/nomear_bits.py;\n'
       '// verdicts in levas/reorganizacao/leitores_de_campo/). Each constant says what the bit means and how far to trust\n'
       '// the name: verified (concrete evidence), read (one clear reading), inferred (a guess).\n'
       '// [pt] Os bits dos campos de flags dos registros, com nome, significado e confiança.\n')
GENERICOS = {'FLAGS', 'STATUS_FLAGS', 'STATUS', 'BITS', 'STATE'}


def tipo_do_campo(records, mod, campo, byte):
    if byte:
        return 'u8'
    # the getter of that record's own impl (another record may have a field of the same name and width)
    t = re.search(r'/// A `%s` record in the game.*\n(?:.*\n){0,3}pub\(crate\) struct (\w+)\(' % mod, records)
    if t:
        i = records.find('impl %s {' % t.group(1))
        records = records[i:records.find('\n}\n', i)] if i >= 0 else records
    m = re.search(r'pub\(crate\) unsafe fn (?:r#)?%s\(self\) -> (\w+)' % campo.lower(), records)
    return m.group(1) if m and m.group(1) in ('u8', 'i8', 'u16', 'i16', 'u32', 'i32', 'usize') else 'u32'


def ler_bits():
    if not os.path.exists(BITS):
        return {}
    s = open(BITS, encoding='utf-8').read()
    out = {}
    for m in re.finditer(r'/// Bits of `(\w+)::(\w+)`( \(byte \d+\))?\.\n#\[allow\(dead_code\)\]\npub\(crate\) mod (\w+) \{\n(.*?)\n\}', s, re.S):
        consts = re.findall(r'    /// (.*)\n    pub\(crate\) const (\w+): (\w+) = (0x[0-9a-f]+);', m.group(5))
        out[m.group(4)] = {'field': '%s::%s' % (m.group(1), m.group(2)), 'byte': m.group(3), 'consts': {int(v, 16): (n, t, d) for d, n, t, v in consts}}
    return out


def escrever_bits(mods):
    partes = [CAB]
    for nome in sorted(mods):
        m = mods[nome]
        partes.append('\n/// Bits of `%s`%s.\n#[allow(dead_code)]\npub(crate) mod %s {\n' % (m['field'], m['byte'] or '', nome))
        for v in sorted(m['consts']):
            n, t, d = m['consts'][v]
            partes.append('    /// %s\n    pub(crate) const %s: %s = %s;\n' % (d, n, t, hex(v)))
        partes.append('}\n')
    open(BITS, 'w', encoding='utf-8').write(''.join(partes))


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    alvo, jsons = args[0], args[1:]
    grava = '--gravar' in sys.argv
    ent = [e for j in jsons for e in json.load(open(j, encoding='utf-8')) if e.get('kind') == 'bit'
           and re.fullmatch(r'[A-Z][A-Z0-9_]*', str(e.get('name') or ''))
           and re.fullmatch(r'0x[0-9a-fA-F]+|\d+', str(e.get('value'))) and str(e.get('line', '')).isdigit()]
    p = os.path.normpath(os.path.join(R, 'rust', 'src', alvo))
    texto = open(p, encoding='utf-8', newline='').read()
    linhas = texto.split('\n')
    for e in ent:
        # a byte access the reader wrote without its `+ k` (`EXPECTATION_FLAGS + 1` with `& 4`): the line says it
        f = e['field'].strip()
        b = re.search(r'%s\s*\+\s*(\d+)\b' % re.escape(f.split('::')[-1]), linhas[int(e['line']) - 1])
        if '+' not in f and b:
            e['field'] = '%s + %s' % (f, b.group(1))
    records = open(os.path.join(R, 'rust', 'src', 'motor', 'records.rs'), encoding='utf-8').read()
    fmods = set(re.findall(r'pub\(crate\) mod (\w+) \{', open(os.path.join(R, 'rust', 'src', 'motor', 'fields.rs'), encoding='utf-8').read()))
    mods = ler_bits()
    por_campo = {(m['field'], m['byte'] or ''): k for k, m in mods.items()}
    nomes = collections.defaultdict(set)
    for e in ent:
        nomes[(e['field'].strip(), int(e['value'], 0))].add(e['name'].strip())
    for k, v in nomes.items():
        if len(v) > 1:
            print('CONFLITO', k[0], hex(k[1]), sorted(v))
    troca = {}
    for e in ent:
        fm = re.fullmatch(r'(\w+)::([A-Z][A-Z0-9_]*)(?:\s*\+\s*(\d+))?', e['field'].strip())
        val = int(e['value'], 0)
        if not fm or len(nomes[(e['field'].strip(), val)]) > 1 or fm.group(1) not in fmods:
            print('pulado:', e.get('line'), e['field'], e['value'])
            continue
        mod, campo, byte = fm.group(1), fm.group(2), fm.group(3)
        chave = ('%s::%s' % (mod, campo), ' (byte %s)' % byte if byte else '')
        nm = por_campo.get(chave)
        if not nm:
            nm = campo.lower() + ('_byte%s' % byte if byte else '')
            if campo in GENERICOS or nm in mods or nm in fmods:
                nm = '%s_%s' % (mod, nm)
            mods[nm] = {'field': chave[0], 'byte': chave[1], 'consts': {}}
            por_campo[chave] = nm
        cs = mods[nm]['consts']
        if val not in cs:
            if any(n == e['name'] for n, _, _ in cs.values()):
                print('nome repetido com outro valor:', nm, e['name'])
                continue
            desc = re.sub(r'\s+', ' ', e.get('description') or e.get('evidence') or '').strip().rstrip('.')
            cs[val] = (e['name'], tipo_do_campo(records, mod, campo, byte), '%s. Name: %s.' % (desc, e.get('confidence', 'read')))
        troca[(int(e['line']), val)] = '%s::%s' % (nm, cs[val][0])
    feitas = 0
    for (ln, val), nome in sorted(troca.items()):
        cod, sep, com = linhas[ln - 1].partition('//')

        def sub(m):
            return m.group(1) + nome if int(m.group(2), 0) == val else m.group(0)
        novo = re.sub(r'([&|^!]\s*|[&|^]=\s*)(0x[0-9a-fA-F_]+|\d+)(?:u8|u16|u32|i16|i32|usize)?\b', sub, cod)
        if novo == cod:
            # no bit operator: the value passed on (`ou(8)`, `or_byte(.., 2)`) or its complement (`& 0xfe`) when it
            # is the only literal of that value in the line
            nums = list(re.finditer(r'(?<![\w.])(0x[0-9a-fA-F_]+|\d+)(?:u8|u16|u32|i16|i32|usize)?\b', cod))
            for alvo_v, txt in ((val, nome), (~val & 0xff, '!' + nome), (~val & 0xffff, '!' + nome)):
                ms = [m for m in nums if int(m.group(1).replace('_', ''), 0) == alvo_v]
                if len(ms) == 1 and alvo_v not in (0, 0xff, 0xffff):
                    novo = cod[:ms[0].start()] + txt + cod[ms[0].end():]
                    break
        if novo == cod:
            print('linha %d: bit %s não achado' % (ln, hex(val)))
            continue
        linhas[ln - 1] = novo + sep + com
        feitas += 1
    print('%d módulos de bits, %d linhas trocadas' % (len(mods), feitas))
    if grava:
        escrever_bits(mods)
        tl.ANTES[p] = texto
        open(p, 'w', encoding='utf-8', newline='').write('\n'.join(linhas))
        tl.voltar_funcoes_com_erro({})


if __name__ == '__main__':
    main()
