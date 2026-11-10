r"""Compiles, one by one, the mutations whose text changed since a commit (09/11/2026): a tool that renames or wraps
code also edits the mutation lists, and a replacement that names something gone (an alias, a closure) only fails
when the mutation test runs, hours later. Each changed mutation is applied to the bench copy's source, `cargo check`
runs, and the source is put back. Sync the bench first (`bancada.py sincronizar K`).
Usage: python compilar_mutacoes.py [--desde COMMIT (default HEAD) | --lista FILE.json] [--bancada K (default 3)]
[pt] Compila uma a uma, numa bancada, as mutações cujo texto mudou desde um commit (aplica, `cargo check`, desfaz).
"""
import os
import runpy
import re
import subprocess
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def arg(nome, padrao):
    return sys.argv[sys.argv.index(nome) + 1] if nome in sys.argv else padrao


def em_lotes(alvo, src, banco, tamanho=60):
    """Batches (10/11/2026): up to `tamanho` mutations whose anchors do not overlap are applied together and checked
    once. On errors, the mutations whose enclosing function holds an error line (or the whole batch, when an error
    falls elsewhere) go back to the one-by-one check. Returns (those to check one by one, bad anchors)."""
    sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
    import mover_rotinas
    textos = {p: open(p, encoding='utf-8').read() for p in src}
    validas, ruins = [], 0
    for f, m in alvo:
        hit = [p for p, t in textos.items() if m[1] in t]
        if len(hit) != 1 or textos[hit[0]].count(m[1]) != 1:
            print('ÂNCORA', f, m[0], len(hit), flush=True)
            ruins += 1
            continue
        i = textos[hit[0]].index(m[1])
        validas.append((f, m, hit[0], i, i + len(m[1])))
    sobra = []
    pend = validas
    while pend:
        lote, resto, usados = [], [], {}
        for v in pend:
            spans = usados.setdefault(v[2], [])
            if len(lote) < tamanho and not any(a < v[4] + 200 and v[3] - 200 < b for a, b in spans):
                lote.append(v)
                spans.append((v[3], v[4]))
            else:
                resto.append(v)
        pend = resto
        faixas = {}  # file -> [(first line, last line, item)] of each mutation's enclosing function, in the mutated text
        for p in {v[2] for v in lote}:
            t = textos[p]
            vs = sorted([v for v in lote if v[2] == p], key=lambda v: -v[3])
            for v in vs:
                t = t[:v[3]] + v[1][2] + t[v[4]:]
            open(p, 'w', encoding='utf-8').write(t)
            # each mutation's new position: shift by the length changes of the ones before it
            desloc = 0
            pos = {}
            for v in sorted(vs, key=lambda v: v[3]):
                pos[id(v)] = v[3] + desloc
                desloc += len(v[1][2]) - len(v[1][1])
            blocos = mover_rotinas.blocos(t)
            for v in vs:
                a = pos[id(v)]
                bl = next(((x, y) for _, x, y in blocos if x <= a < y), (a, a + len(v[1][2])))
                faixas.setdefault(os.path.normcase(os.path.normpath(p)), []).append((t.count('\n', 0, bl[0]) + 1, t.count('\n', 0, bl[1]) + 1, v))
        r = subprocess.run(['cargo', 'check', '--release', '--message-format=short'], cwd=os.path.join(banco, 'rust'),
                           capture_output=True, text=True)
        for p in {v[2] for v in lote}:
            open(p, 'w', encoding='utf-8').write(textos[p])
        if r.returncode == 0:
            print('lote de %d: ok' % len(lote), flush=True)
            continue
        suspeitas = set()
        for fl, ln in re.findall(r'^(src[^:]+):(\d+):\d+: error', r.stderr, re.M):
            q = os.path.normcase(os.path.normpath(os.path.join(banco, 'rust', fl)))
            achou = [v for a, b, v in faixas.get(q, []) if a <= int(ln) <= b]
            if not achou:
                suspeitas = {id(v) for v in lote}
                break
            suspeitas |= {id(v) for v in achou}
        if not suspeitas:
            suspeitas = {id(v) for v in lote}
        print('lote de %d: %d suspeitas' % (len(lote), len(suspeitas)), flush=True)
        sobra += [(v[0], v[1]) for v in lote if id(v) in suspeitas]
    return sobra, ruins


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
    if len(alvo) > 3 and '--uma-a-uma' not in sys.argv:
        alvo, ruins = em_lotes(alvo, src, banco)
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
