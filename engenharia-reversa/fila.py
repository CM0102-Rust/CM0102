r"""A fila de levas: levas escritas antes da hora, com os índices calculados só ao integrar.

Cada item fica em levas/fila/<nome>/:
  codigo.rs    as rotinas e os comparadores; os índices de `orig::<..>(N)` e `comparar_valor!(.., N, ..)`
               escritos como %%I0%%, %%I1%%, %%I2%% (na ordem de ROTINAS)
  rotinas.py   ROTINAS = [(nome_rust, endereco, [bytes do prologo])]
               DOCS = (titulo pt, titulo en, [(nome, nome_en, endereco, arquivo, convencao, o que faz pt, en)])
  mutacoes.py  MUTACOES = [(nome, trecho, troca)]  (mesmo formato de mutacoes/levaN.py)
A ordem está em levas/fila/ordem.txt (um nome por linha).

Uso: python fila.py proxima      materializa o primeiro da ordem como levas/N (N = a próxima leva) com
                                 integrar.py genérico (âncoras achadas no motor.rs), docs.py e mutacoes/levaN.py
     python fila.py checar       aplica a fila inteira numa cópia do motor.rs e confere as âncoras das mutações
O publicar_leva.py chama `proxima` quando levas/N+1 não existe.
"""
import os
import re
import runpy
import shutil
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FILA = os.path.join(R, 'levas', 'fila')

INTEGRAR = r'''"""Integra a {n}a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = {rotinas!r}
BASE = {base}
m = re.search(r'\];\nconst N: usize = (\d+);', s)
assert m and int(m.group(1)) == BASE, ('o motor.rs mudou desde a materialização', m and m.group(1), BASE)
pro = ''.join('    Prologo {{ endereco: %#x, bytes: &[%s] }},\n' % (e, ', '.join('0x%02x' % b for b in bs)) for _, e, bs in ROTINAS)
s = s[:m.start()] + pro + '];\nconst N: usize = %d;' % (BASE + len(ROTINAS)) + s[m.end():]
fins = [x.start() for x in re.finditer(r' as \*const \(\) as usize,\n    \];', s)]
assert len(fins) == 2, fins
for k, prefixo in ((1, 'comparar_'), (0, '')):
    i = fins[k] + len(' as *const () as usize,\n')
    s = s[:i] + ''.join('        %s%s as *const () as usize,\n' % (prefixo, a) for a, _, _ in ROTINAS) + s[i:]
anc = '\n    ];\n    for i in 0..N {{'
assert s.count(anc) == 1
s = s.replace(anc, ' ' + ' '.join('"motor_%s",' % a for a, _, _ in ROTINAS) + anc)
anc = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
assert s.count(anc) == 1
s = s.replace(anc, anc + '\n' + open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read().strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
'''


def ordem():
    p = os.path.join(FILA, 'ordem.txt')
    return [l.strip() for l in open(p, encoding='utf-8') if l.strip() and not l.startswith('#')]


def materializar(nome, n, base, antes):
    src = os.path.join(FILA, nome)
    rot = runpy.run_path(os.path.join(src, 'rotinas.py'))
    destino = os.path.join(R, 'levas', str(n))
    os.makedirs(destino, exist_ok=False)
    cod = open(os.path.join(src, 'codigo.rs'), encoding='utf-8').read()
    for k in range(len(rot['ROTINAS'])):
        cod = cod.replace('%%%%I%d%%%%' % k, str(base + k))
    assert '%%I' not in cod, 'índice sem valor'
    cod = cod.replace('%%LEVA%%', str(n))
    open(os.path.join(destino, 'codigo.rs'), 'w', encoding='utf-8').write(cod)
    open(os.path.join(destino, 'integrar.py'), 'w', encoding='utf-8').write(INTEGRAR.format(n=n, rotinas=[(a, e, list(b)) for a, e, b in rot['ROTINAS']], base=base))
    shutil.copy2(os.path.join(src, 'mutacoes.py'), os.path.join(R, 'engenharia-reversa', 'mutacoes', f'leva{n}.py'))
    tpt, ten, docs = rot['DOCS']
    molde = open(os.path.join(R, 'engenharia-reversa', 'molde_docs.py'), encoding='utf-8').read()
    open(os.path.join(destino, 'docs.py'), 'w', encoding='utf-8').write(
        molde.replace('%%N%%', str(n)).replace('%%ANTES%%', str(antes)).replace('%%R%%', repr(docs))
        .replace('%%TPT%%', repr(tpt)).replace('%%TEN%%', repr(ten)))
    with open(os.path.join(src, 'materializada.txt'), 'w', encoding='utf-8') as f:
        f.write(f'leva {n}, índices {base}..{base + len(rot["ROTINAS"]) - 1}\n')
    linhas = [l for l in open(os.path.join(FILA, 'ordem.txt'), encoding='utf-8') if l.strip() != nome]
    open(os.path.join(FILA, 'ordem.txt'), 'w', encoding='utf-8').writelines(linhas)
    print(f'{nome} -> levas/{n} (índices a partir de {base})')


def estado():
    s = open(os.path.join(R, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
    base = int(re.search(r'\];\nconst N: usize = (\d+);', s).group(1))
    leia = open(os.path.join(R, 'README.md'), encoding='utf-8').read()
    n, total = map(int, re.search(r'\*\*Estado:\*\* (\d+) levas, (\d+) rotinas', leia).groups())
    return n, total, base


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else ''
    if cmd == 'proxima':
        n, total, base = estado()
        while os.path.isdir(os.path.join(R, 'levas', str(n + 1))):
            n += 1  # já materializada (ou escrita à mão) e ainda não publicada
        ordem_ = ordem()
        assert ordem_, 'fila vazia'
        materializar(ordem_[0], n + 1, base, total)
    elif cmd == 'checar':
        import tempfile
        tmp = tempfile.mkdtemp()
        shutil.copytree(os.path.join(R, 'rust', 'src'), os.path.join(tmp, 'rust', 'src'))
        m = open(os.path.join(tmp, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
        base = int(re.search(r'\];\nconst N: usize = (\d+);', m).group(1))
        for nome in ordem():
            rot = runpy.run_path(os.path.join(FILA, nome, 'rotinas.py'))
            cod = open(os.path.join(FILA, nome, 'codigo.rs'), encoding='utf-8').read()
            for k in range(len(rot['ROTINAS'])):
                cod = cod.replace('%%%%I%d%%%%' % k, str(base + k))
            m += '\n' + cod
            base += len(rot['ROTINAS'])
            for mut, a, _ in runpy.run_path(os.path.join(FILA, nome, 'mutacoes.py'))['MUTACOES']:
                if m.count(a) != 1:
                    print('ancora', nome, mut, m.count(a))
        print('checado', len(ordem()), 'itens')
    else:
        sys.exit(__doc__)


if __name__ == '__main__':
    main()
