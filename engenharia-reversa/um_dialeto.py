r"""Um só dialeto de acesso à memória (etapa 4 da reorganização, 08/11/2026; o leitor de fora, rodada 9): os leitores
duplicados `read_f32_at`/`read_f64_at` (match_pl.rs; leitura alinhada) viram `read_f32`/`read_f64` do mem.rs (leitura
desalinhada, o mesmo valor no x86), e `byte(p, off)` (motor.rs) vira `read_i8(p + off)`. As definições duplicadas
saem. Vale no código, nas listas de mutação e na fila.

Uso: python um_dialeto.py [--gravar]
"""
import glob
import json
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
from involucros_exe import args_da_chamada  # noqa: E402


def simples(a):
    return re.fullmatch(r'[\w:.]+', a) is not None


def trocar(s):
    s = re.sub(r'\bread_f32_at\(', 'read_f32(', s)
    s = re.sub(r'\bread_f64_at\(', 'read_f64(', s)
    out, pos = [], 0
    for m in re.finditer(r'(?<![\w.:])byte\s*\(', s):
        if m.start() < pos:
            continue
        args, fim = args_da_chamada(s, m.end() - 1)
        if args is None or len(args) != 2:
            continue
        b = args[1] if simples(args[1]) else '(%s)' % args[1]
        out.append(s[pos:m.start()])
        out.append('read_i8(%s + %s)' % (args[0], b))
        pos = fim
    out.append(s[pos:])
    return ''.join(out)


def tirar_fn(s, cab):
    """Remove the function whose first line is exactly `cab` (with its `///` lines above and its 3-line body)."""
    nl = '\r\n' if '\r\n' in s else '\n'
    L = s.split(nl)
    for i, l in enumerate(L):
        if l == cab and i + 2 < len(L) and L[i + 2] == '}':
            ini = i
            while ini > 0 and L[ini - 1].lstrip().startswith('///'):
                ini -= 1
            fim = i + 3
            if fim < len(L) and L[fim] == '' and (ini == 0 or L[ini - 1] == ''):
                fim += 1
            return nl.join(L[:ini] + L[fim:])
    return s


def por_funcao(t):
    """trocar() função por função, pulando as que definem um `byte` local (closure ou variável)."""
    import mover_rotinas
    # the whole file, except the top-level functions that define a local `byte` (left as they are)
    partes, pos = [], 0
    for _, a, b in mover_rotinas.blocos(t):
        bloco = t[a:b]
        if re.search(r'\blet (mut )?byte\b', bloco):
            partes.append(trocar(t[pos:a]))
            partes.append(bloco)
            pos = b
    partes.append(trocar(t[pos:]))
    return ''.join(partes)


def main():
    n = 0
    alvos = glob.glob(os.path.join(R, 'rust', 'src', '**', '*.rs'), recursive=True)
    alvos += [p for p in glob.glob(os.path.join(R, 'levas', 'fila', '**', '*'), recursive=True) if p.endswith('.rs')]
    for p in alvos:
        t = open(p, encoding='utf-8', newline='').read()
        s = t
        for cab in ('pub(crate) unsafe fn read_f32_at(a: usize) -> f32 {', 'pub(crate) unsafe fn read_f64_at(a: usize) -> f64 {',
                    'unsafe fn byte(p: usize, off: usize) -> i8 {'):
            s = tirar_fn(s, cab)
        s = por_funcao(s)
        s = re.sub(r'\bread_f32_at\(', 'read_f32(', s)
        s = re.sub(r'\bread_f64_at\(', 'read_f64(', s)
        if s != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(s)
    print('%d arquivos' % n)
    if '--gravar' not in sys.argv:
        return
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in sorted(os.listdir(pasta)):
        if not f.endswith('.py'):
            continue
        p = os.path.join(pasta, f)
        try:
            ms = runpy.run_path(p).get('MUTACOES', [])
        except Exception:
            continue
        s = open(p, encoding='utf-8').read()
        for m in ms:
            ini = max(s.find("('%s'," % m[0]), s.find('("%s",' % m[0]))
            pos = ini
            for k in (1, 2):
                nv = trocar(m[k])
                if nv == m[k]:
                    continue
                for a, b in ((repr(m[k]), repr(nv)), (json.dumps(m[k], ensure_ascii=False), json.dumps(nv, ensure_ascii=False))):
                    j = s.find(a, pos)
                    if j >= 0:
                        s = s[:j] + b + s[j + len(a):]
                        pos = j + len(b)
                        break
        open(p, 'w', encoding='utf-8').write(s)
    print('gravado')


if __name__ == '__main__':
    main()
