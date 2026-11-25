r"""Puts in RELATIVAS (rust/src/detours.rs) the (address, offset) pairs the items of batch N ask for in their
INTEGRAR_ANTES.txt. Usage: python relativas_pedidas.py N

A prologue with a relative call or jump copied to the trampoline unrelocated jumps to garbage: batch 333 crashed on
the first comparison day because sx09_4 asked for (0x54d130, 0) and nothing applied it (25/11/2026). Only pairs whose
address is one of the batch's routines are taken, so a note about another item's routine is not applied by mistake.
"""
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DETOURS = os.path.join(R, 'rust', 'src', 'detours.rs')


def itens_da_leva(n):
    fila = os.path.join(R, 'levas', 'fila')
    return [it for it in os.listdir(fila)
            if os.path.exists(os.path.join(fila, it, 'materializada.txt'))
            and open(os.path.join(fila, it, 'materializada.txt'), encoding='utf-8').read().startswith('leva %d,' % n)]


def pedidos(n):
    fila = os.path.join(R, 'levas', 'fila')
    pares = set()
    for it in itens_da_leva(n):
        enderecos = {a for _, a, _ in runpy.run_path(os.path.join(fila, it, 'rotinas.py'))['ROTINAS']}
        p = os.path.join(fila, it, 'INTEGRAR_ANTES.txt')
        if not os.path.exists(p):
            continue
        for a, k in re.findall(r'\(\s*(0x[0-9a-fA-F]+)\s*,\s*(\d+)\s*\)', open(p, encoding='utf-8').read()):
            if int(a, 16) in enderecos:
                pares.add((int(a, 16), int(k)))
    return sorted(pares)


def main():
    n = int(sys.argv[1])
    s = open(DETOURS, encoding='utf-8').read()
    m = re.search(r'pub const RELATIVAS: &\[\(usize, usize\)\] = &\[(.*?)\];', s, re.S)
    ja = {int(a, 16) for a, _ in re.findall(r'\((0x[0-9a-f]+),\s*(\d+)\)', m.group(1))}
    novos = [(a, k) for a, k in pedidos(n) if a not in ja]
    if not novos:
        print('relativas: nada a acrescentar')
        return
    corpo = m.group(1).rstrip() + ''.join(', (%#x, %d)' % p for p in novos)
    s = s[:m.start(1)] + corpo + s[m.end(1):]
    open(DETOURS, 'w', encoding='utf-8').write(s)
    print('relativas: ' + ', '.join('(%#x, %d)' % p for p in novos))


if __name__ == '__main__':
    main()
