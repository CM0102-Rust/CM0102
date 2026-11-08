r"""As closures locais que só leem memória viram leituras do mem.rs (etapa 4 da reorganização, 08/11/2026; o leitor de
fora, rodadas 7 e 8: ~600 `let u = |o: usize| *((this + o) as *const u8);` definidas função a função).

Reconhece, em cada função, `let NOME = |o: usize| *((BASE + o) as *const T)` (e com `core::ptr::read_unaligned`, e com
um `as X` no fim) e a forma de dois argumentos `|p: usize, o: usize| *((p + o) as *const T)`, com T em u8, i8, u16,
i16, u32, i32, usize, f32, f64. Se o nome só aparece em chamadas, a definição sai e cada chamada vira `read_T(BASE + o)`
(ou `(read_T(..) as X)`). A função que não compilar volta ao texto de antes; as listas de mutação acompanham, trecho a
trecho. O valor lido é o mesmo (leitura desalinhada = alinhada no x86).

Uso: python fechar_closures.py [--gravar]
"""
import collections
import glob
import json
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import traduzir_locais as tl  # noqa: E402
from involucros_exe import args_da_chamada  # noqa: E402

T = r'(u8|i8|u16|i16|u32|i32|usize|f32|f64)'
CORPO = r'(?:\*\(|core::ptr::read_unaligned\()\(([\w.]+(?:\(\))?) \+ (\w+)\) as \*const %s\)( as \w+)?' % T
DEF1 = re.compile(r'^[ \t]*let (\w+) = \|(\w+): usize\|\s*%s;[ \t]*\r?\n' % CORPO, re.M)
DEF2 = re.compile(r'^[ \t]*let (\w+) = \|(\w+): usize, (\w+): usize\|\s*%s;[ \t]*\r?\n' % CORPO, re.M)


def simples(a):
    return re.fullmatch(r'[\w:.]+', a) is not None


def main():
    mutacoes = []
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in sorted(os.listdir(pasta)):
        if f.endswith('.py'):
            try:
                mutacoes += [(f, m) for m in runpy.run_path(os.path.join(pasta, f)).get('MUTACOES', [])]
            except Exception:
                pass
    trocas_mut, total = {}, 0
    for p in glob.glob(os.path.join(SRC, '**', '*.rs'), recursive=True):
        t = open(p, encoding='utf-8').read()
        tl.ANTES[p] = t
        partes, pos = [], 0
        for _, a, b in mover_rotinas.blocos(t):
            bloco = t[a:b]
            cod = re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', ' ', bloco)
            alvos = {}
            for m in DEF1.finditer(bloco):
                nome, arg, base, var, ty, cast = m.groups()
                if var == arg:
                    alvos[nome] = (1, base, ty, cast, m.group(0))
            for m in DEF2.finditer(bloco):
                nome, a1, a2, base, var, ty, cast = m.groups()
                if base == a1 and var == a2:
                    alvos[nome] = (2, None, ty, cast, m.group(0))
            bons = {}
            for nome, v in alvos.items():
                usos = re.findall(r'\b%s\b(\s*\()?' % re.escape(nome), cod)
                if sum(1 for u in usos if not u) != 1 or len(re.findall(r'\blet (?:mut )?%s\b' % re.escape(nome), cod)) != 1:
                    continue
                if v[1] and v[1].endswith('()') is False and re.search(r'\b%s\s*=[^=]' % re.escape(v[1].split('.')[0]), cod):
                    continue  # the base is reassigned in the function
                bons[nome] = v
            if not bons:
                continue

            def f_(s, bons=bons):
                for nome, (n, base, ty, cast, defn) in bons.items():
                    s = s.replace(defn, '', 1) if defn in s else re.sub(r'^[ \t]*let %s = \|[^\n]*\n' % re.escape(nome), '', s, count=1, flags=re.M)
                    out, pos2 = [], 0
                    for m in re.finditer(r'\b%s\s*\(' % re.escape(nome), s):
                        if m.start() < pos2:
                            continue
                        args, fim = args_da_chamada(s, m.end() - 1)
                        if args is None or len(args) != n:
                            continue
                        if n == 1:
                            end = '%s + %s' % (base, args[0] if simples(args[0]) else '(%s)' % args[0])
                        else:
                            end = '%s + %s' % (args[0] if simples(args[0]) else '(%s)' % args[0], args[1] if simples(args[1]) else '(%s)' % args[1])
                        expr = 'read_%s(%s)' % (ty, end)
                        if cast:
                            expr = '(%s%s)' % (expr, cast)
                        out.append(s[pos2:m.start()])
                        out.append(expr)
                        pos2 = fim
                    out.append(s[pos2:])
                    s = ''.join(out)
                return s
            novo = tl.so_codigo(bloco, lambda x: x) if False else f_(bloco)
            if novo == bloco:
                continue
            total += len(bons)
            for lista, mu in mutacoes:
                if bloco.count(mu[1]) == 1:
                    trocas_mut[(lista, mu[0])] = (mu, f_)
            partes.append(t[pos:a])
            partes.append(novo)
            pos = b
        partes.append(t[pos:])
        if ''.join(partes) != t and '--gravar' in sys.argv:
            open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d closures saem; %d trechos de mutação' % (total, len(trocas_mut)))
    if '--gravar' not in sys.argv:
        return
    tl.voltar_funcoes_com_erro(trocas_mut)
    por = collections.defaultdict(list)
    for (lista, _), v in trocas_mut.items():
        por[lista].append(v)
    for lista, ms in por.items():
        pl = os.path.join(pasta, lista)
        s = open(pl, encoding='utf-8').read()
        for mu, g in ms:
            ini = max(s.find("('%s'," % mu[0]), s.find('("%s",' % mu[0]))
            if ini < 0:
                continue
            pos = ini
            for k in (1, 2):
                nv = g(mu[k])
                if nv == mu[k]:
                    continue
                for v, n2 in ((repr(mu[k]), repr(nv)), (json.dumps(mu[k], ensure_ascii=False), json.dumps(nv, ensure_ascii=False))):
                    j = s.find(v, pos)
                    if j >= 0:
                        s = s[:j] + n2 + s[j + len(v):]
                        pos = j + len(n2)
                        break
        open(pl, 'w', encoding='utf-8').write(s)
    print('gravado')


if __name__ == '__main__':
    main()
