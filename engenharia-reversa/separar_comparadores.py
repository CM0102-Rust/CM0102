r"""Separa os comparadores da lógica do jogo (etapa 4 da reorganização, 08/11/2026, achado do leitor de fora, rodada 3).

Cada `comparar_<x>` (o gêmeo da rotina que roda no modo comparação) sai de rust/src/motor/<caminho>.rs para
rust/src/motor/compare/<caminho>.rs, com o seu comentário. Quem lê para entender o jogo fica só com a rotina; quem quer
ver o teste abre o arquivo paralelo. Os comparadores gerados por macro (`comparar_valor!` e afins) ficam onde estão.
Todos os módulos se enxergam: mover não muda o código gerado. Rodar de novo move só os comparadores novos.

Uso: python separar_comparadores.py [--gravar]
"""
import os
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
BASE = os.path.join(SRC, 'motor')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402

CAB = ('use super::*;\n\n'
       '// The comparers of the routines in ../{rel} (comparison mode: see the header of motor.rs).\n'
       '// [pt] Os comparadores das rotinas de ../{rel} (modo comparação: ver o cabeçalho do motor.rs).\n')


def main():
    n = 0
    for d, _, fs in os.walk(BASE):
        if os.path.join(BASE, 'compare') in d:
            continue
        for f in fs:
            # index.rs: os comparadores usam as tabelas geradas que entram por include! e são privadas ao módulo
            if not f.endswith('.rs') or f in ('macros.rs', 'index.rs'):
                continue
            p = os.path.join(d, f)
            rel = os.path.relpath(p, BASE).replace(os.sep, '/')
            t = open(p, encoding='utf-8').read()
            blocos = [(nome, a, b) for nome, a, b in mover_rotinas.blocos(t) if nome.startswith('comparar_')]
            if not blocos:
                continue
            n += len(blocos)
            if '--gravar' not in sys.argv:
                continue
            partes, pos, saem = [], 0, []
            for nome, a, b in blocos:
                partes.append(t[pos:a])
                saem.append(t[a:b])
                pos = b
            partes.append(t[pos:])
            open(p, 'w', encoding='utf-8').write(''.join(partes))
            alvo = os.path.join(BASE, 'compare', rel)
            os.makedirs(os.path.dirname(alvo), exist_ok=True)
            antes = open(alvo, encoding='utf-8').read() if os.path.exists(alvo) else CAB.format(rel=rel)
            open(alvo, 'w', encoding='utf-8').write(antes.rstrip('\n') + '\n\n' + '\n'.join(saem))
    print('%d comparadores' % n)
    if '--gravar' in sys.argv:
        import modulos_motor
        modulos_motor.main()
        print('gravado')


if __name__ == '__main__':
    main()
