r"""Junta ao rust/src/motor/fields.rs os campos propostos por leitura (etapa 4 da reorganização, 08/11/2026). Entrada:
TSV `<módulo>\t<deslocamento>\t<NOME>\t<bytes>\t<descrição>`. Num módulo que existe, entra o campo cujo deslocamento
ainda não tem nome e cujo nome não está em uso; um módulo novo é criado no fim (com `_fields` no nome se ele colidir com
um módulo do motor; os TSV de tipos dados com --tipos são reescritos com o nome final). Só constantes: o código não muda.

Uso: python juntar_campos.py campos5_A.tsv [...] [--tipos tipos5_A.tsv ...] [--gravar]
"""
import collections
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
F = os.path.join(R, 'rust', 'src', 'motor', 'fields.rs')


def main():
    args = sys.argv[1:]
    tipos = args[args.index('--tipos') + 1:] if '--tipos' in args else []
    tipos = [t for t in tipos if not t.startswith('--')]
    campos = [a for a in args[:args.index('--tipos')] if not a.startswith('--')] if '--tipos' in args else \
        [a for a in args if not a.startswith('--')]
    t = open(F, encoding='utf-8').read()
    existentes = {m: c for m, c in re.findall(r'^pub\(crate\) mod (\w+) \{\n(.*?)^\}', t, re.M | re.S)}
    motor_rs = open(os.path.join(R, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
    ocupados = set(re.findall(r'pub\(crate\) mod (\w+);', motor_rs))
    novos = collections.defaultdict(dict)  # módulo -> {off: (NOME, bytes, desc)}
    for c in campos:
        for l in open(c, encoding='utf-8'):
            p = [x.strip() for x in l.rstrip('\n').split('\t')]
            if len(p) < 5 or not re.fullmatch(r'[a-z][a-z0-9_]*', p[0]) or not re.fullmatch(r'0x[0-9a-f]+', p[1]) \
                    or not re.fullmatch(r'[A-Z][A-Z0-9_]*', p[2]):
                continue
            novos[p[0]].setdefault(int(p[1], 16), (p[2], p[3], p[4]))
    renome, n_add, n_mod = {}, 0, 0
    for mod, cs in sorted(novos.items()):
        if mod in existentes:
            corpo = existentes[mod]
            offs = {int(v, 0) for v in re.findall(r'usize = (0x[0-9a-f]+|\d+);', corpo)}
            nomes = set(re.findall(r'const (\w+):', corpo))
            add = []
            for off, (n_, by, de) in sorted(cs.items()):
                if off in offs or n_ in nomes:
                    continue
                nomes.add(n_)
                add.append('    pub(crate) const %s: usize = %#x; // %s byte(s): %s' % (n_, off, by, de))
            if add:
                abre = 'pub(crate) mod %s {\n' % mod
                i = t.index(abre) + len(abre)
                fim = t.index('\n}', i)
                t = t[:fim] + ''.join('\n' + a for a in add) + t[fim:]
                n_add += len(add)
        else:
            final = mod + '_fields' if mod in ocupados else mod
            renome[mod] = final
            vistos, L = set(), []
            for off, (n_, by, de) in sorted(cs.items()):
                if n_ in vistos:
                    continue
                vistos.add(n_)
                L.append('    pub(crate) const %s: usize = %#x; // %s byte(s): %s' % (n_, off, by, de))
            t = t.rstrip('\n') + '\n\n' + '\n'.join(['/// Fields of the `%s` object, named from how the routines use them. [pt] Campos do objeto `%s`.' % (mod, mod),
                                                      '#[allow(dead_code)]', 'pub(crate) mod %s {' % final] + L + ['}']) + '\n'
            n_mod += 1
            n_add += len(L)
    print('%d campos, %d módulos novos (%s)' % (n_add, n_mod, ', '.join('%s->%s' % kv for kv in renome.items() if kv[0] != kv[1]) or 'sem colisão'))
    if '--gravar' not in sys.argv:
        return
    open(F, 'w', encoding='utf-8').write(t)
    for tp in tipos:  # o nome final dos módulos novos nos TSV de tipos
        s = open(tp, encoding='utf-8').read()
        for a, b in renome.items():
            if a != b:
                s = re.sub(r'\t%s$' % re.escape(a), '\t' + b, s, flags=re.M)
        open(tp, 'w', encoding='utf-8').write(s)
    print('gravado')


if __name__ == '__main__':
    main()
