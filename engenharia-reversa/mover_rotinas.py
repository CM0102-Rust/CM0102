r"""Põe cada rotina portada no módulo do seu arquivo de origem (etapa 4 da reorganização, 07/11/2026).

O cabeçalho de cada rotina diz o arquivo de origem (`Original: ... (0x5d6280, game.cpp, cdecl)`). Se a rotina está em
outro módulo (as seções vieram por leva, e uma leva juntava rotinas de arquivos diferentes), ela vai para o fim do
módulo certo, com o cabeçalho e o comparador (`comparar_<nome>`). Os auxiliares ficam onde estão: todos os módulos se
enxergam (motor.rs re-exporta tudo), então mover não muda o código gerado.

Uso: python mover_rotinas.py [--gravar]
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
BASE = os.path.join(SRC, 'motor')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import partir_motor  # noqa: E402

FN = re.compile(r'^(?:pub\(crate\) )?(?:unsafe )?(?:extern "\w+" )?fn (\w+)\b', re.M)
ORIG = re.compile(r'^/// Original: [^(\n]*\((0x[0-9a-f]+), ([\w.]+\.cpp)', re.M)


def blocos(t):
    """[(nome, início do cabeçalho, fim do corpo)] das funções de topo."""
    r = []
    for m in FN.finditer(t):
        ini = m.start()
        while True:
            k = t.rfind('\n', 0, ini - 1) + 1 if ini > 0 else 0
            linha = t[k:ini]
            if ini > 0 and linha.lstrip().startswith(('///', '#[')):
                ini = k
            else:
                break
        fim = t.find('\n}\n', m.end())
        if fim < 0:
            continue
        r.append((m.group(1), ini, fim + 3))
    return r


def main():
    mapa = partir_motor.mapa()
    pasta = {}
    for c in mapa.values():
        pasta.setdefault(os.path.basename(c).lower(), c)
    arquivos = {}
    for d, _, fs in os.walk(BASE):
        for f in fs:
            if f.endswith('.rs') and f != 'macros.rs':
                rel = os.path.relpath(os.path.join(d, f), BASE).replace('\\', '/')
                arquivos[rel] = open(os.path.join(d, f), encoding='utf-8').read()
    mover = []  # (de, para, nome)
    for rel, t in arquivos.items():
        bl = {n: (a, b) for n, a, b in blocos(t)}
        for n, (a, b) in bl.items():
            m = ORIG.search(t[a:b])
            if not m:
                continue
            cpp = pasta.get(m.group(2).lower())
            if not cpp:
                continue
            alvo = partir_motor.destino(cpp)
            if alvo != rel and alvo in arquivos:
                mover.append((rel, alvo, n))
    print('%d rotinas fora do lugar' % len(mover))
    if '--gravar' not in sys.argv:
        for x in mover[:15]:
            print('  %s -> %s: %s' % x)
        return
    acrescimos = {}
    for rel, alvo, n in mover:
        for nome in (n, 'comparar_' + n):
            t = arquivos[rel]
            achado = [(a, b) for x, a, b in blocos(t) if x == nome]
            if not achado:
                continue
            a, b = achado[0]
            acrescimos.setdefault(alvo, []).append(t[a:b])
            arquivos[rel] = t[:a] + t[b:]
    for alvo, partes in acrescimos.items():
        arquivos[alvo] = arquivos[alvo].rstrip('\n') + '\n\n' + '\n'.join(partes)
    for rel, t in arquivos.items():
        p = os.path.join(BASE, rel)
        if open(p, encoding='utf-8').read() != t:
            open(p, 'w', encoding='utf-8').write(t)
    print('gravado')


if __name__ == '__main__':
    main()
