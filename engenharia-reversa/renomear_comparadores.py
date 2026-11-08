r"""O prefixo dos comparadores em inglês: `comparar_<x>` -> `compare_<x>`, e os macros (`comparar_valor!` ->
`compare_value!`, `leitor!` -> `reader!`...) (etapa 4 da reorganização, 08/11/2026; o leitor de fora, rodada 6). Vale no
código Rust (código e comentários; os rótulos de texto "motor_..." não têm fronteira antes de `comparar_` e ficam), nos
itens da fila e nas ferramentas que geram ou leem comparadores. O glossario.py guarda os nomes antigos e fica.

Uso: python renomear_comparadores.py [--gravar]
"""
import glob
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MACROS = {'comparar_valor': 'compare_value', 'comparar_leitor': 'compare_reader', 'comparar_gravador': 'compare_recorder',
          'comparar_repartir': 'compare_split', 'comparar_ordem': 'compare_order', 'leitor': 'reader', 'gravador': 'recorder'}
FERRAMENTAS = ('aplicar_campos_motor', 'aplicar_tipos_variaveis', 'arrumar_rodada4', 'esvaziar_motor', 'fila', 'mover_rotinas',
               'nomear_indices', 'partir_motor', 'renomear_prefixo', 'renomear_rotinas', 'separar_comparadores',
               'unificar_auxiliares', 'renomear_global')


def trocar(t):
    for a, b in MACROS.items():
        t = re.sub(r'\b%s!' % a, b + '!', t)
        t = re.sub(r'(macro_rules! )%s\b' % a, r'\g<1>' + b, t)
    t = re.sub(r'\b(comparar_(?:leitor|gravador|repartir|valor|ordem))\b', lambda m: MACROS[m.group(1)], t)  # regex texts
    return re.sub(r'\bcomparar_', 'compare_', t)


def main():
    alvos = glob.glob(os.path.join(R, 'rust', 'src', '**', '*.rs'), recursive=True)
    alvos += [p for p in glob.glob(os.path.join(R, 'levas', 'fila', '**', '*'), recursive=True) if p.endswith(('.rs', '.py'))]
    alvos += [os.path.join(R, 'engenharia-reversa', n + '.py') for n in FERRAMENTAS]
    n = 0
    for p in alvos:
        if not os.path.isfile(p):
            continue
        t = open(p, encoding='utf-8', newline='').read()
        novo = trocar(t)
        if novo != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(novo)
    print('%d arquivos' % n)


if __name__ == '__main__':
    main()
