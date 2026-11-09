r"""Compiles, one by one, the mutations whose text changed since a commit (09/11/2026): a tool that renames or wraps
code also edits the mutation lists, and a replacement that names something gone (an alias, a closure) only fails
when the mutation test runs, hours later. Each changed mutation is applied to the bench copy's source, `cargo check`
runs, and the source is put back. Sync the bench first (`bancada.py sincronizar K`).
Usage: python compilar_mutacoes.py [--desde COMMIT (default HEAD) | --lista FILE.json] [--bancada K (default 3)]
[pt] Compila uma a uma, numa bancada, as mutações cujo texto mudou desde um commit (aplica, `cargo check`, desfaz).
"""
import os
import runpy
import subprocess
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def arg(nome, padrao):
    return sys.argv[sys.argv.index(nome) + 1] if nome in sys.argv else padrao


def main():
    desde = arg('--desde', 'HEAD')
    banco = os.path.join(R, 'copias', 'bancada' + arg('--bancada', '3'))
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    fs = subprocess.run(['git', 'diff', '--name-only', desde, '--', 'engenharia-reversa/mutacoes'], cwd=R,
                        capture_output=True, text=True).stdout.split()
    antes = {}
    for f in fs:
        s = subprocess.run(['git', 'show', '%s:%s' % (desde, f)], cwd=R, capture_output=True, text=True, encoding='utf-8').stdout
        d = {}
        try:
            exec(compile(s, f, 'exec'), d)
        except Exception:
            pass
        antes.update({(f, m[0]): m for m in d.get('MUTACOES', [])})
    alvo = []
    for f in fs:
        cwd = os.getcwd()
        os.chdir(os.path.join(R, 'engenharia-reversa'))
        try:
            ms = runpy.run_path(os.path.join(R, f)).get('MUTACOES', [])
        finally:
            os.chdir(cwd)
        alvo += [(f, m) for m in ms if antes.get((f, m[0])) != m]
    if '--lista' in sys.argv:
        # an explicit list (JSON [[file, mutation name, ...], ...]): e.g. mutations whose replacement calls a function
        # whose signature changed, though their own text did not change
        import json
        pedidas = {(os.path.join('engenharia-reversa', 'mutacoes', x[0]).replace(os.sep, '/'), x[1]) for x in json.load(open(arg('--lista', ''), encoding='utf-8'))}
        alvo = []
        for f in sorted(os.listdir(pasta)):
            if not f.endswith('.py'):
                continue
            cwd = os.getcwd()
            os.chdir(os.path.join(R, 'engenharia-reversa'))
            try:
                ms = runpy.run_path(os.path.join(pasta, f)).get('MUTACOES', [])
            except Exception:
                ms = []
            finally:
                os.chdir(cwd)
            alvo += [('engenharia-reversa/mutacoes/' + f, m) for m in ms if ('engenharia-reversa/mutacoes/' + f, m[0]) in pedidas]
    print(len(alvo), 'mutações mudadas', flush=True)
    src = [os.path.join(dp, y) for dp, _, x in os.walk(os.path.join(banco, 'rust', 'src')) for y in x if y.endswith('.rs')]
    ruins = 0
    for f, m in alvo:
        hit = [p for p in src if m[1] in open(p, encoding='utf-8').read()]
        if len(hit) != 1:
            print('ÂNCORA', f, m[0], len(hit), flush=True)
            ruins += 1
            continue
        p = hit[0]
        t = open(p, encoding='utf-8').read()
        open(p, 'w', encoding='utf-8').write(t.replace(m[1], m[2], 1))
        r = subprocess.run(['cargo', 'check', '--release', '--message-format=short'], cwd=os.path.join(banco, 'rust'),
                           capture_output=True, text=True)
        open(p, 'w', encoding='utf-8').write(t)
        if r.returncode != 0:
            ruins += 1
            print('ERRO', f, m[0], [l for l in r.stderr.splitlines() if ': error' in l][:2], flush=True)
    print('ruins', ruins)
    return 1 if ruins else 0


if __name__ == '__main__':
    sys.exit(main())
