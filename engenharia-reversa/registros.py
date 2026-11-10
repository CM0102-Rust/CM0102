r"""Typed record pointers (readability, 09/11/2026): for each object of rust/src/motor/fields.rs, a
`#[repr(transparent)]` pointer type (`Club(usize)`, the same ABI as the raw address) with one getter and one setter
per field, generated in rust/src/motor/records.rs. `club.reputation()` reads `read_i16(club + club::REPUTATION)`.

The type of each field: the one the code already uses most at that field (larguras_campos.py, whose widths were checked
against the original routines), else the width in fields.rs (1 = u8, 2 = i16, 4 = i32, 8 = f64 if documented double /
i64 if documented i64). Fields wider than 8 bytes (names, arrays, dates) get only `x_addr()`. A field that holds a
pointer to another record returns that record's type when POINTERS says so.
Usage: python registros.py [--gravar]
[pt] Os ponteiros tipados dos registros: um tipo por objeto do fields.rs, com um método de leitura e um de escrita por
campo (`club.reputation()`), gerados em rust/src/motor/records.rs.
"""
import collections
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import larguras_campos as lc  # noqa: E402

LARG_EL = {'u8': 1, 'i8': 1, 'u16': 2, 'i16': 2, 'u32': 4, 'i32': 4, 'usize': 4}
SAIDA = os.path.join(R, 'rust', 'src', 'motor', 'records.rs')
KEYWORDS = {'as', 'break', 'const', 'continue', 'crate', 'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in',
            'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self', 'static', 'struct', 'super',
            'trait', 'true', 'type', 'unsafe', 'use', 'where', 'while', 'abstract', 'become', 'box', 'do', 'final',
            'macro', 'override', 'priv', 'typeof', 'unsized', 'virtual', 'yield', 'try', 'async', 'await', 'dyn'}
# fields that hold a pointer to another record: (module, FIELD) -> record type
POINTERS = {
    ('club', 'STADIUM'): 'Stadium',
    ('club', 'MANAGER'): 'Staff',
    ('club', 'CHAIRMAN'): 'Staff',
    ('club', 'NATION'): 'Nation',
    ('club', 'DIVISION'): 'Competition',
    ('club', 'LAST_DIVISION'): 'Competition',
    ('club', 'RESERVE_DIVISION'): 'Competition',
    ('league', 'COMPETITION'): 'Competition',
}


# and the ones found by ponteiros_registros.py from how the code uses the value (levas/reorganizacao/ponteiros.json)
_PJ = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'levas', 'reorganizacao', 'ponteiros.json')
if os.path.exists(_PJ):
    import json as _json
    for _k, _v in _json.load(open(_PJ, encoding='utf-8')).items():
        POINTERS.setdefault(tuple(_k.split('::')), _v['type'])

# array fields: (module, FIELD) -> (element type, element count); a method `x.squad(i)` / `x.set_squad(i, v)`
ARRAYS = {
    ('club', 'SQUAD'): ('usize', 50),
    ('club', 'COACHES'): ('usize', 5),
    ('club', 'SCOUTS'): ('usize', 7),
    ('club', 'PHYSIOS'): ('usize', 3),
    ('match_player', 'MOVE_SKILL'): ('i16', 63),
}


def camel(nome):
    return ''.join(p[:1].upper() + p[1:] for p in nome.split('_'))


def tipo_doc(largura, doc_t):
    d = doc_t.lower()
    if largura == 1:
        return 'i8' if 'sbyte' in d or d.startswith('char') else 'u8'
    if largura == 2:
        return 'u16' if 'uint16' in d else 'i16'
    if largura == 4:
        return 'f32' if 'float' in d else 'i32'
    if largura == 8:
        if 'double' in d:
            return 'f64'
        if 'i64' in d:
            return 'i64'
    return None


def usos_por_campo():
    out = collections.defaultdict(collections.Counter)
    for raiz, _, fs in os.walk(lc.SRC):
        for f in fs:
            if f.endswith('.rs') and f not in ('fields.rs', 'records.rs'):
                for mod, c, tipo, kind, pos in lc.acessos(open(os.path.join(raiz, f), encoding='utf-8').read()):
                    out[(mod, c)][tipo] += 1
    return out


def tipos_ja_gerados():
    """{(module, FIELD): type} of the records.rs already generated: a field keeps its type once generated (the usage
    count changes as the code moves to the methods, and a type that changed would change what the code reads)."""
    out = {}
    if not os.path.exists(SAIDA):
        return out
    t = open(SAIDA, encoding='utf-8').read()
    for m in re.finditer(r'/// `(\w+)::(\w+)` \(\+0x[0-9a-f]+\)\.[^\n]*\n\s*#\[inline\(always\)\]\n\s*pub\(crate\) (?:unsafe )?fn (?:r#)?(\w+)\(self\) -> (\w+)', t):
        if not (m.group(3).endswith('_addr') and m.group(4) == 'usize'):
            out[(m.group(1), m.group(2))] = m.group(4)
    return out


def main():
    cs = lc.campos()
    usos = usos_por_campo()
    fixos = tipos_ja_gerados()
    import confianca_campos
    niveis = confianca_campos.niveis_por_campo()  # how far to trust each name (docs/CONFIANCA_DOS_CAMPOS.md)
    mods = collections.OrderedDict()
    for (mod, c), (off, larg, doc_t) in cs.items():
        mods.setdefault(mod, []).append((c, off, larg, doc_t))
    L = ['use super::*;', '',
         '// Typed record pointers, generated by engenharia-reversa/registros.py from fields.rs: `Club(addr)` has the same',
         '// ABI as the raw address; `club.reputation()` reads `club + club::REPUTATION` with the field\'s type (the one the',
         '// code uses, checked against the original routines). Fields wider than 8 bytes give only their address.',
         '// [pt] Os ponteiros tipados dos registros, gerados do fields.rs: `club.reputation()` lê o campo com o tipo dele.',
         '',
         '/// Anything that is an address in the game\'s memory: a raw `usize` or a typed record pointer. The exe:: wrappers take',
         '/// `impl Addr` where the original passes a pointer, so `exe::x(club)` needs no `.addr()`. `i32` is here only for',
         '/// untyped integer literals (`exe::x(this, 0)`): Rust gives a bare `0` the type `i32`.',
         '/// [pt] Um endereço na memória do jogo: `usize` cru ou um ponteiro de registro tipado (os invólucros exe:: aceitam os dois).',
         'pub(crate) trait Addr: Copy {',
         '    fn addr(self) -> usize;',
         '}',
         '',
         'impl Addr for usize {',
         '    #[inline(always)]',
         '    fn addr(self) -> usize {',
         '        self',
         '    }',
         '}',
         '',
         'impl Addr for i32 {',
         '    #[inline(always)]',
         '    fn addr(self) -> usize {',
         '        self as usize',
         '    }',
         '}',
         '']
    n_campos = 0
    for mod, campos in mods.items():
        tipo = camel(mod)
        L += ['/// A `%s` record in the game\'s memory (fields in `fields::%s`). [pt] Um registro `%s` na memória do jogo.' % (mod, mod, mod),
              '#[repr(transparent)]', '#[derive(Clone, Copy, PartialEq, Eq, Debug)]', '#[allow(dead_code)]',
              'pub(crate) struct %s(pub(crate) usize);' % tipo, '', '#[allow(dead_code)]', 'impl %s {' % tipo,
              '    #[inline(always)]', '    pub(crate) fn addr(self) -> usize {', '        self.0', '    }',
              '    #[inline(always)]', '    pub(crate) fn is_null(self) -> bool {', '        self.0 == 0', '    }']
        trait_impl = ['impl Addr for %s {' % tipo, '    #[inline(always)]', '    fn addr(self) -> usize {', '        self.0', '    }', '}', '']
        trait_impl += ['impl core::fmt::LowerHex for %s {' % tipo, '    fn fmt(&self, f: &mut core::fmt::Formatter<\'_>) -> core::fmt::Result {', '        core::fmt::LowerHex::fmt(&self.0, f)', '    }', '}', '']
        for c, off, larg, doc_t in campos:
            nome = c.lower()
            fn = 'r#' + nome if nome in KEYWORDS else nome
            alvo = POINTERS.get((mod, c))
            u = usos.get((mod, c))
            t = None
            if alvo:
                t = alvo
            elif (mod, c) in fixos:
                t = fixos[(mod, c)]
            elif u:
                t = u.most_common(1)[0][0]
                if larg and larg <= 8 and lc.LARG[t] != larg:
                    t = tipo_doc(larg, doc_t) or t
            elif larg:
                t = tipo_doc(larg, doc_t)
            if (mod, c) in ARRAYS:
                et, n = ARRAYS[(mod, c)]
                tam = LARG_EL[et]
                L += ['    /// `%s::%s`[i] (+%s, %d x %s). Name: %s.' % (mod, c, hex(off), n, et, niveis.get((mod, c), 'read')),
                      '    #[inline(always)]', '    pub(crate) unsafe fn %s(self, i: usize) -> %s {' % (nome, et),
                      '        read_%s(self.0 + %s::%s + i * %d)' % (et, mod, c, tam), '    }',
                      '    #[inline(always)]', '    pub(crate) unsafe fn set_%s(self, i: usize, v: %s) {' % (nome, et),
                      '        write_%s(self.0 + %s::%s + i * %d, v)' % (et, mod, c, tam), '    }']
                continue
            L.append('    /// `%s::%s` (+%s). Name: %s.' % (mod, c, hex(off), niveis.get((mod, c), 'read')))
            if t is None:
                L += ['    #[inline(always)]', '    pub(crate) fn %s_addr(self) -> usize {' % nome, '        self.0 + %s::%s' % (mod, c), '    }']
                continue
            n_campos += 1
            if alvo:
                L += ['    #[inline(always)]', '    pub(crate) unsafe fn %s(self) -> %s {' % (fn, alvo),
                      '        %s(read_usize(self.0 + %s::%s))' % (alvo, mod, c), '    }',
                      '    #[inline(always)]', '    pub(crate) unsafe fn set_%s(self, v: %s) {' % (nome, alvo),
                      '        write_usize(self.0 + %s::%s, v.0)' % (mod, c), '    }']
            else:
                L += ['    #[inline(always)]', '    pub(crate) unsafe fn %s(self) -> %s {' % (fn, t),
                      '        read_%s(self.0 + %s::%s)' % (t, mod, c), '    }',
                      '    #[inline(always)]', '    pub(crate) unsafe fn set_%s(self, v: %s) {' % (nome, t),
                      '        write_%s(self.0 + %s::%s, v)' % (t, mod, c), '    }']
        L += ['}', ''] + trait_impl
    texto = '\n'.join(L) + '\n'
    print('%d registros, %d campos com tipo' % (len(mods), n_campos))
    if '--gravar' in sys.argv:
        open(SAIDA, 'w', encoding='utf-8').write(texto)


if __name__ == '__main__':
    main()
