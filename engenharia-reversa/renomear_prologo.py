r"""`Prologo { endereco, bytes }` -> `Prologue { address, bytes }` (etapa 4 da reorganização, 08/11/2026; o leitor de
fora, rodada 6). No Rust: o tipo, o campo na definição, nos literais `Prologue { endereco: ...` e no acesso
`.endereco`. Nas ferramentas e na fila: só as formas que leem ou escrevem esse código (`Prologo \{ endereco:`,
`endereco: (0x`, `'endereco: %#x,'`, `.endereco`); a coluna `endereco` do docs/NOMES.csv e as variáveis Python
chamadas `endereco` ficam.

Uso: python renomear_prologo.py [--gravar]
"""
import glob
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def rust(t):
    t = re.sub(r'\bPrologo\b', 'Prologue', t)
    t = re.sub(r'(Prologue \{\{? ?)endereco:', r'\1address:', t)
    t = re.sub(r'\bpub endereco:', 'pub address:', t)
    t = re.sub(r'\.endereco\b', '.address', t)
    return t


def ferramenta(t):
    t = re.sub(r'\bPrologo\b', 'Prologue', t)
    t = re.sub(r'(Prologue \\?\{\{? ?)endereco:', r'\1address:', t)
    t = re.sub(r"(r?['\"])endereco:", r'\1address:', t)          # r'endereco: (0x...' e 'endereco: %#x,'
    t = re.sub(r'(?<=[(\'"])endereco:\\s\*', r'address:\\s*', t)
    t = re.sub(r'(\]|\b[zpq](?:\.0)?)\.endereco\b', r'\1.address', t)  # Rust code in strings; args.endereco stays
    return t


def main():
    n = 0
    alvos = [(p, rust) for p in glob.glob(os.path.join(R, 'rust', 'src', '**', '*.rs'), recursive=True)]
    alvos += [(p, rust) for p in glob.glob(os.path.join(R, 'levas', 'fila', '**', '*.rs'), recursive=True)]
    alvos += [(p, ferramenta) for p in glob.glob(os.path.join(R, 'levas', 'fila', '**', '*.py'), recursive=True)]
    alvos += [(p, ferramenta) for p in glob.glob(os.path.join(R, 'engenharia-reversa', '*.py'))
              if os.path.basename(p) != 'renomear_prologo.py']
    alvos += [(p, ferramenta) for p in glob.glob(os.path.join(R, 'engenharia-reversa', 'mutacoes', '*.py'))]
    for p, f in alvos:
        t = open(p, encoding='utf-8', newline='').read()
        novo = f(t)
        if novo != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(novo)
    print('%d arquivos' % n)


if __name__ == '__main__':
    main()
