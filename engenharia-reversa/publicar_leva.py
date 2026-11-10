r"""Publica a leva N (depois da cadeia ok) e prepara a N+1. Uso: python publicar_leva.py N

1. levas/N/docs.py (documentação gerada; falha se a cadeia não bateu ou se há escapadas sem explicação)
   e o glossário;
2. commit "Simulacao: Na leva (X rotinas): título" com docs, wiki, README, levas/N, mutações e motor.rs;
   push;
3. levas/N+1/integrar.py, o estilo de legibilidade (padronizar.sh), cargo build, a DLL em jogo_testes/, e confere as âncoras das mutações.
Depois disso, a cadeia da N+1: bash engenharia-reversa/cadeia.sh N+1 (em segundo plano).
"""
import os
import re
import runpy
import shutil
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gen_docs import REF_CARREIRA  # noqa: E402

RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def rodar(*cmd, **kw):
    print('>', ' '.join(cmd), flush=True)
    subprocess.run(cmd, cwd=kw.get('cwd', RAIZ), check=True)


def main():
    n = int(sys.argv[1])
    st = subprocess.run(['git', 'status', '--short', 'rust/src/motor.rs'], cwd=RAIZ, capture_output=True, text=True).stdout
    if not st.strip():  # a leva pode ter entrado num commit anterior (a seguinte integrada antes da hora)
        print('aviso: motor.rs sem mudanças; seguindo com a documentação e o commit', flush=True)
    if not os.environ.get('DOCS_JA_FEITOS'):  # o docs.py já rodou à mão (ele não roda duas vezes)
        rodar(sys.executable, os.path.join('levas', str(n), 'docs.py'))
    rodar(sys.executable, os.path.join('engenharia-reversa', 'glossario.py'))
    rodar(sys.executable, os.path.join('engenharia-reversa', 'placar_arquivos.py'))
    rodar(sys.executable, os.path.join('engenharia-reversa', 'competicoes.py'))  # o mapa das competições acompanha cada leva
    titulo = open(os.path.join(RAIZ, 'levas', str(n), 'LEIA.md'), encoding='utf-8').readline().split('—', 1)[1].strip()
    total = re.search(r'\*\*Estado:\*\* \d+ levas, (\d+) rotinas', open(os.path.join(RAIZ, 'README.md'), encoding='utf-8').read()).group(1)
    rodar('git', 'add', '-A', 'docs', 'README.md', 'README.en.md', os.path.join('levas', str(n)),
          os.path.join('engenharia-reversa', 'glossario.py'), os.path.join('engenharia-reversa', 'mutacoes', f'leva{n}.py'),
          'CLAUDE.md', 'CLAUDE.en.md', os.path.join('engenharia-reversa', 'gen_docs.py'), os.path.join('engenharia-reversa', 'publicar_leva.py'),
          os.path.join('engenharia-reversa', 'fila.py'), os.path.join('engenharia-reversa', 'molde_docs.py'), os.path.join('levas', 'fila'),
          os.path.join('rust', 'src', 'motor.rs'), os.path.join('rust', 'src', 'signatures.rs'), os.path.join('levas', 'carreira_ampla', 'pendentes.txt'), os.path.join('engenharia-reversa', 'fila_continua.sh'))
    prefixo = 'Motor' if 'Motor de partidas' in open(os.path.join(RAIZ, 'levas', str(n), 'docs.py'), encoding='utf-8').read() else 'Simulacao'
    adiado = '=== JOGO REAL ADIADO' in open(os.path.join(RAIZ, 'levas', str(n), 'cadeia.txt'), encoding='utf-8', errors='replace').read()
    real = 'jogo real adiado para a comparacao acumulada da auditoria (05/11/2026)' if adiado else 'jogo real 0 divergencias'
    msg = (f'{prefixo}: {n}a leva ({total} rotinas): {titulo}\n\nComparacao de verdade em cada rotina; cadeia completa '
           f'(levas/{n}/cadeia.txt): {real}, carreiras Rust x2 iguais (criterio de 31/10/2026: a original so registrada).\n\n'
           'Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n')
    rodar('git', 'commit', '-q', '-m', msg)
    rodar('git', 'push', '-q')
    prox = n + 1
    if not os.path.isdir(os.path.join(RAIZ, 'levas', str(prox))):
        rodar(sys.executable, os.path.join('engenharia-reversa', 'fila.py'), 'proxima')
    from fila import texto_do_motor
    ja = texto_do_motor(RAIZ)
    primeira = re.search(r"ROTINAS = \[\('(\w+)'", open(os.path.join(RAIZ, 'levas', str(prox), 'integrar.py'), encoding='utf-8').read())
    if primeira and f'fn {primeira.group(1)}(' in ja:
        print(f'a {prox}a já está integrada; pulando o integrar.py', flush=True)
    else:
        rodar(sys.executable, os.path.join('levas', str(prox), 'integrar.py'))
        # the readability style for the code that came in (10/11/2026): typed records, read_T/write_T, English names
        # and messages, named constants; the batch's chain then tests it in that form
        rodar('bash', os.path.join('engenharia-reversa', 'padronizar.sh'))
    # a tabela das assinaturas acompanha a PROLOGOS (a DLL não desvia rotina que não seja a do exe original/1080)
    rodar(sys.executable, os.path.join('engenharia-reversa', 'assinaturas.py'))
    # every exe function the copies call, checked against the original machine code (stack, ecx, return width)
    rodar(sys.executable, os.path.join('engenharia-reversa', 'conferir_assinaturas.py'))
    rodar('cargo', 'build', '--release', cwd=os.path.join(RAIZ, 'rust'))
    shutil.copy2(os.path.join(RAIZ, 'rust', 'target', 'i686-pc-windows-msvc', 'release', 'ddraw.dll'), os.path.join(RAIZ, 'jogo_testes', 'ddraw.dll'))
    from fila import texto_do_motor
    m = texto_do_motor(RAIZ)  # o motor.rs e os arquivos que ele inclui
    for nome, a, *_ in runpy.run_path(os.path.join(RAIZ, 'engenharia-reversa', 'mutacoes', f'leva{prox}.py'))['MUTACOES']:
        assert m.count(a) == 1, ('ancora da mutacao', nome, m.count(a))
    print(f'pronto: publicada a {n}a; a {prox}a integrada e compilada. Agora: bash engenharia-reversa/cadeia.sh {prox}')


if __name__ == '__main__':
    main()
