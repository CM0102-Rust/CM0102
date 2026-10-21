r"""Publica a leva N (depois da cadeia ok) e prepara a N+1. Uso: python publicar_leva.py N

1. levas/N/docs.py (documentação gerada; falha se a cadeia não bateu ou se há escapadas sem explicação)
   e o glossário;
2. commit "Simulacao: Na leva (X rotinas): título" com docs, wiki, README, levas/N, mutações e motor.rs;
   push;
3. levas/N+1/integrar.py, cargo build, a DLL em jogo_testes/, e confere as âncoras das mutações.
Depois disso, a cadeia da N+1: bash engenharia-reversa/cadeia.sh N+1 (em segundo plano).
"""
import os
import re
import runpy
import shutil
import subprocess
import sys

RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def rodar(*cmd, **kw):
    print('>', ' '.join(cmd), flush=True)
    subprocess.run(cmd, cwd=kw.get('cwd', RAIZ), check=True)


def main():
    n = int(sys.argv[1])
    st = subprocess.run(['git', 'status', '--short', 'rust/src/motor.rs'], cwd=RAIZ, capture_output=True, text=True).stdout
    assert st.strip(), 'motor.rs sem mudanças: a leva já foi publicada?'
    rodar(sys.executable, os.path.join('levas', str(n), 'docs.py'))
    rodar(sys.executable, os.path.join('engenharia-reversa', 'glossario.py'))
    rodar(sys.executable, os.path.join('engenharia-reversa', 'placar_arquivos.py'))
    titulo = open(os.path.join(RAIZ, 'levas', str(n), 'LEIA.md'), encoding='utf-8').readline().split('—', 1)[1].strip()
    total = re.search(r'\*\*Estado:\*\* \d+ levas, (\d+) rotinas', open(os.path.join(RAIZ, 'README.md'), encoding='utf-8').read()).group(1)
    rodar('git', 'add', '-A', 'docs', 'README.md', 'README.en.md', 'wiki', os.path.join('levas', str(n)),
          os.path.join('engenharia-reversa', 'glossario.py'), os.path.join('engenharia-reversa', 'mutacoes', f'leva{n}.py'),
          'CLAUDE.md', 'CLAUDE.en.md', os.path.join('engenharia-reversa', 'gen_docs.py'), os.path.join('engenharia-reversa', 'publicar_leva.py'),
          os.path.join('engenharia-reversa', 'fila.py'), os.path.join('engenharia-reversa', 'molde_docs.py'), os.path.join('levas', 'fila'),
          os.path.join('rust', 'src', 'motor.rs'))
    prefixo = 'Motor' if 'Motor de partidas' in open(os.path.join(RAIZ, 'levas', str(n), 'docs.py'), encoding='utf-8').read() else 'Simulacao'
    msg = (f'{prefixo}: {n}a leva ({total} rotinas): {titulo}\n\nComparacao de verdade em cada rotina; cadeia completa '
           f'(levas/{n}/cadeia.txt): jogo real 0 divergencias, carreiras iguais a referencia 034ff3eb.\n\n'
           'Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n')
    rodar('git', 'commit', '-q', '-m', msg)
    rodar('git', 'push', '-q')
    prox = n + 1
    if not os.path.isdir(os.path.join(RAIZ, 'levas', str(prox))):
        rodar(sys.executable, os.path.join('engenharia-reversa', 'fila.py'), 'proxima')
    rodar(sys.executable, os.path.join('levas', str(prox), 'integrar.py'))
    rodar('cargo', 'build', '--release', cwd=os.path.join(RAIZ, 'rust'))
    shutil.copy2(os.path.join(RAIZ, 'rust', 'target', 'i686-pc-windows-msvc', 'release', 'ddraw.dll'), os.path.join(RAIZ, 'jogo_testes', 'ddraw.dll'))
    m = open(os.path.join(RAIZ, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
    for nome, a, _ in runpy.run_path(os.path.join(RAIZ, 'engenharia-reversa', 'mutacoes', f'leva{prox}.py'))['MUTACOES']:
        assert m.count(a) == 1, ('ancora da mutacao', nome, m.count(a))
    print(f'pronto: publicada a {n}a; a {prox}a integrada e compilada. Agora: bash engenharia-reversa/cadeia.sh {prox}')


if __name__ == '__main__':
    main()
