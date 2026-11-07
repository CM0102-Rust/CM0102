r"""Confere as cópias do cm0102-base nos outros repositórios (enquanto o CM0102 for privado, 07/11/2026).

O Tweaks (público) não pode depender do CM0102 (privado): fica com cópias dos arquivos da base. Cada cópia tem um
número combinado de linhas diferentes (os ganchos e o cabeçalho de lá); se o número mudar, a base ou a cópia mudou e
alguém precisa copiar. Quando o CM0102 abrir, as cópias viram dependência git com tag e este script sai.

Uso: python copias_da_base.py      (sai com erro se alguma cópia estiver fora do combinado)
"""
import difflib
import os
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = os.path.join(R, 'cm0102-base', 'src')
TWEAKS = os.path.join(os.path.dirname(R), 'CM0102-Rust-Tweaks', 'src')
# (cópia, arquivo da base, linhas diferentes combinadas)
COPIAS = [
    (os.path.join(TWEAKS, 'save', 'conteiner.rs'), 'conteiner.rs', 14),
    (os.path.join(TWEAKS, 'video', 'win.rs'), 'win.rs', 5),
    (os.path.join(TWEAKS, 'video', 'video.rs'), 'video.rs', 14),
    (os.path.join(TWEAKS, 'video', 'apresentacao.rs'), 'apresentacao.rs', 4),
    (os.path.join(TWEAKS, 'video', 'reducao.rs'), 'reducao.rs', 2),
]


def linhas(p):
    return open(p, encoding='utf-8').read().splitlines()


erro = False
for copia, nome, combinado in COPIAS:
    if not os.path.exists(copia):
        print('falta', copia)
        erro = True
        continue
    n = sum(1 for d in difflib.unified_diff(linhas(copia), linhas(os.path.join(BASE, nome)), n=0, lineterm='')
            if d[:1] in '+-' and d[:3] not in ('+++', '---'))
    ok = n == combinado
    erro |= not ok
    print('%-4s %-60s %3d linhas diferentes%s' % ('ok' if ok else 'MUDOU', os.path.relpath(copia, os.path.dirname(R)), n,
                                                  '' if ok else ' (combinado: %d): copiar ou atualizar o combinado' % combinado))
sys.exit(1 if erro else 0)
