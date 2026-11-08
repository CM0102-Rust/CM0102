r"""Tira do motor.rs as rotinas e os comparadores que ainda moram nele (etapa 4 da reorganização, rodada 3,
07/11/2026). Cada rotina portada vai para o módulo do seu arquivo de origem (pelo endereço na tabela e o mapa de
funções); cada comparador `comparar_<x>` vai para o módulo onde `<x>` está. Ficam no motor.rs o cabeçalho, as tabelas,
a instalação, a linha-âncora do integrar.py, as macros e o que não é rotina nem comparador (diagnósticos, auxiliares das
tabelas). Os módulos se enxergam todos: mover não muda o código gerado.

Uso: python esvaziar_motor.py [--gravar]
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import partir_motor  # noqa: E402


def main():
    m = os.path.join(SRC, 'motor.rs')
    s = open(m, encoding='utf-8').read()
    i = s.index('let rust: [usize; N] = [')
    nomes = re.findall(r'^\s+(\w+) as \*const \(\) as usize,$', s[i:s.index('];', i)], re.M)
    pro = re.findall(r'Prologue \{ address: (0x[0-9a-f]+)', s[:s.index('const N')])
    end = {n: int(a, 16) for n, a in zip(nomes, pro)}
    mapa = partir_motor.mapa()
    onde = {}
    base = os.path.join(SRC, 'motor')
    for d, _, fs in os.walk(base):
        for f in fs:
            if f.endswith('.rs'):
                p = os.path.join(d, f)
                for n, _, _ in mover_rotinas.blocos(open(p, encoding='utf-8').read()):
                    onde.setdefault(n, os.path.relpath(p, base).replace(os.sep, '/'))
    plano = {}
    blocos = mover_rotinas.blocos(s)
    for n, a, b in blocos:
        if n in end and end[n] in mapa:
            plano[n] = partir_motor.destino(mapa[end[n]])
    for n, a, b in blocos:
        if n.startswith('comparar_'):
            x = n[len('comparar_'):]
            if x in plano:
                plano[n] = plano[x]
            elif x in onde:
                plano[n] = onde[x]
    print('%d funções saem do motor.rs (de %d)' % (len(plano), len(blocos)))
    if '--gravar' not in sys.argv:
        for n in list(plano)[:12]:
            print('  %s -> %s' % (n, plano[n]))
        return
    acrescimos, partes, pos = {}, [], 0
    for n, a, b in blocos:
        if n in plano:
            acrescimos.setdefault(plano[n], []).append(s[a:b])
            partes.append(s[pos:a])
            pos = b
    partes.append(s[pos:])
    open(m, 'w', encoding='utf-8').write(''.join(partes))
    for alvo, ls in acrescimos.items():
        p = os.path.join(base, alvo)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        antes = open(p, encoding='utf-8').read() if os.path.exists(p) else 'use super::*;\n'
        open(p, 'w', encoding='utf-8').write(antes.rstrip('\n') + '\n\n' + '\n'.join(ls))
    import modulos_motor
    sys.argv.append('--gravar')
    modulos_motor.main()
    print('gravado')


if __name__ == '__main__':
    main()
