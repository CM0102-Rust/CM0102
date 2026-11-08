r"""Nomes em inglês para os módulos da infraestrutura do crate rust/ (etapa 4 da reorganização, 08/11/2026):
`desvios.rs` -> `detours.rs`, `data.rs` -> `date.rs`... O arquivo muda de nome (git mv), a declaração `mod` no lib.rs e
os caminhos `crate::desvios::` / `desvios::` no código (só no código, não em comentário nem em texto entre aspas), e o
nome do arquivo nas ferramentas e nos docs que o citam.

Uso: python renomear_modulos.py [--gravar]
"""
import glob
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import traduzir_locais  # noqa: E402

MODULOS = {'desvios': 'detours', 'simulacao': 'simulation', 'alocador': 'allocator', 'data': 'date', 'jogo': 'crt',
           'espiao': 'spy', 'extensao': 'extension', 'sonda_pilha': 'stack_probe', 'ganchos': 'iat_hooks',
           'resolucao': 'resolution',
           # os gerados (08/11/2026, rodada 9)
           'assinaturas': 'signatures', 'chaves_fixas': 'fixed_keys', 'adjetivos_nacao': 'nation_adjectives'}
SRC = os.path.join(R, 'rust', 'src')


def no_codigo(t):
    def f(s):
        for a, b in MODULOS.items():
            s = re.sub(r'\bmod %s;' % a, 'mod %s;' % b, s)
            s = re.sub(r'(?<!cm0102_base::)\b%s::' % a, '%s::' % b, s)  # cm0102_base::ganchos is another module
            # `use crate::{rng, data, win};`
            s = re.sub(r'(use crate::\{[^}]*)\b%s\b' % a, r'\g<1>%s' % b, s)
        return s
    return traduzir_locais.so_codigo(t, f)


def nos_textos(t):
    for a, b in MODULOS.items():
        if a == 'ganchos':  # cm0102-base/src/ganchos.rs keeps its name (the rust/src one was renamed once, 08/11/2026)
            continue
        t = re.sub(r'(?<![\w/])(rust/src/)?%s\.rs\b' % a, lambda m: (m.group(1) or '') + b + '.rs', t)
    return t


def main():
    n = 0
    for p in glob.glob(os.path.join(SRC, '**', '*.rs'), recursive=True):
        t = open(p, encoding='utf-8', newline='').read()
        novo = no_codigo(t)
        for a_, b_ in MODULOS.items():  # include!("../chaves_fixas.rs"): o caminho fica num texto entre aspas
            novo = re.sub(r'include!\("((?:\.\./)*)%s\.rs"\)' % a_, lambda m, b_=b_: 'include!("%s%s.rs")' % (m.group(1), b_), novo)
        if novo != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(novo)
    outros = (glob.glob(os.path.join(R, 'engenharia-reversa', '*.py')) + glob.glob(os.path.join(R, 'engenharia-reversa', '*.sh'))
              + glob.glob(os.path.join(R, 'levas', 'carreira_ampla', '*.ps1')) + glob.glob(os.path.join(R, 'docs', '*.md'))
              + [os.path.join(R, x) for x in ('CLAUDE.md', 'CLAUDE.en.md', 'README.md', 'README.en.md')])
    for p in outros:
        if os.path.basename(p) == 'renomear_modulos.py':
            continue
        t = open(p, encoding='utf-8', newline='').read()
        novo = nos_textos(t)
        if novo != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(novo)
    print('%d arquivos' % n)
    if '--gravar' in sys.argv:
        for a, b in MODULOS.items():
            if os.path.exists(os.path.join(SRC, a + '.rs')):
                os.system('git -C "%s" mv rust/src/%s.rs rust/src/%s.rs' % (R, a, b))
        print('gravado')


if __name__ == '__main__':
    main()
