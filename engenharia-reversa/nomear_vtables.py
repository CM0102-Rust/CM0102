r"""Virtual method slots by name (11/11/2026): a reader agent read the vtables of the competition classes (Comp, League,
Cup), the national transfer rules and the ruling bodies in the exe's .rdata and named each slot the Rust code calls
(`levas/reorganizacao/vtables.json`; community names from cm0102-function-map). The slots become constants in
rust/src/motor/vtables.rs (`comp_vtable::CREATE_SCHEDULE`), and at each call site of `vtable_sites.tsv` the slot
literal (`read_usize(read_usize(x) + 0x3c)`, `call_comp_virtual(x, 0x3c, ..)`) becomes the name.
Usage: python nomear_vtables.py VTABLES.json SITES.tsv [--gravar]
[pt] Os slots dos métodos virtuais com nome (vtables.rs), e o literal do slot em cada chamada vira o nome.
"""
import json
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
SAIDA = os.path.join(SRC, 'motor', 'vtables.rs')
AJUDANTES = r'vtable_method|call_cup_virtual|cup_call_virtual_no_args|bra_nat_first_call_virtual_no_args|call_comp_virtual|virtual_'


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    grava = '--gravar' in sys.argv
    vt = json.load(open(args[0], encoding='utf-8'))
    L = ['use super::*;', '',
         '// The virtual method slots the Rust code calls, by name: the offset of each method in the class\'s vtable, read',
         '// from the exe\'s .rdata (engenharia-reversa/nomear_vtables.py; levas/reorganizacao/vtables.json; method names from',
         '// the community function map). `read_usize(read_usize(comp) + comp_vtable::CREATE_SCHEDULE)` is the address of',
         '// the competition\'s create_schedule override.',
         '// [pt] Os slots dos métodos virtuais, com nome: o deslocamento de cada método na vtable da classe.', '']
    nomes = {}
    for chave, cl in vt.items():
        mod = '%s_vtable' % chave
        L += ['/// Virtual method slots of %s.' % cl.get('class', chave), '#[allow(dead_code)]', 'pub(crate) mod %s {' % mod]
        for off, s in sorted(cl.get('slots', {}).items(), key=lambda kv: int(kv[0], 16)):
            nome = re.sub(r'[^A-Z0-9_]', '_', s['name'].upper())
            alvos = ', '.join(re.sub(r' \(\d+ vtables?\)', '', a if isinstance(a, str) else a.get('name', '')) for a in s.get('targets', [])[:3])
            L += ['    /// %s. Name: %s.' % (('Slot %s: %s' % (off, alvos)).rstrip(': '), s.get('confidence', 'read')),
                  '    pub(crate) const %s: usize = %s;' % (nome, off)]
            nomes[(chave, int(off, 16))] = '%s::%s' % (mod, nome)
        L += ['}', '']
    feitas, falhas = 0, 0
    por_arquivo = {}
    for l in open(args[1], encoding='utf-8'):
        c = l.rstrip('\n').split('\t')
        if len(c) < 3 or ':' not in c[0]:
            continue
        f, ln = c[0].rsplit(':', 1)
        try:
            off, chave = int(c[1], 16), c[2].strip()
        except ValueError:
            continue
        if off == 0 or (chave, off) not in nomes or not ln.isdigit():
            continue
        por_arquivo.setdefault(f, []).append((int(ln), off, nomes[(chave, off)]))
    for f, itens in por_arquivo.items():
        p = os.path.join(SRC, 'motor', f) if not f.startswith('rust') else os.path.join(R, f)
        if not os.path.exists(p):
            p = os.path.join(R, f)
        if not os.path.exists(p):
            print('arquivo?', f)
            continue
        linhas = open(p, encoding='utf-8', newline='').read().split('\n')
        for ln, off, nome in itens:
            cod, sep, com = linhas[ln - 1].partition('//')
            lit = r'(?:0x0*%x|%d)\b' % (off, off)
            novo = re.sub(r'(read_usize\((?:[^;]*?)\)\s*\+\s*)' + lit, lambda m: m.group(1) + nome, cod, count=1)
            if novo == cod:
                novo = re.sub(r'((?:%s)\(\s*[^,()]*(?:\([^()]*\))*[^,()]*,\s*)' % AJUDANTES + lit, lambda m: m.group(1) + nome, cod, count=1)
            if novo == cod:
                falhas += 1
                continue
            linhas[ln - 1] = novo + sep + com
            feitas += 1
        if grava:
            open(p, 'w', encoding='utf-8', newline='').write('\n'.join(linhas))
    print('%d slots com nome; %d chamadas trocadas; %d não achadas' % (len(nomes), feitas, falhas))
    if grava:
        open(SAIDA, 'w', encoding='utf-8').write('\n'.join(L) + '\n')


if __name__ == '__main__':
    main()
