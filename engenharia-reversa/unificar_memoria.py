r"""Troca as cópias de acesso à memória dos itens (cqw12_d, cpx3_pb...) por funções únicas em rust/src/motor/mem.rs
(etapa 4 da reorganização, 07/11/2026).

Só junta cópias de corpo equivalente: a mesma leitura ou escrita, do mesmo tipo, com a mesma assinatura. Cada cópia
vira um nome em inglês (read_u32, write_u16_from_i32...), as chamadas são renomeadas e as definições saem. O que o
script não reconhece fica como está.

Uso: python unificar_memoria.py          (mostra o plano)
     python unificar_memoria.py --gravar
"""
import collections
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
DEF = re.compile(r'((?:^///[^\n]*\n)*)^unsafe fn (\w+_(?:d|w|b|f|pd|pw|pb|pf|por_w|por_d|por_b))\(([^)]*)\)(?: -> (\w+))? \{\n'
                 r'\s*([^\n]*?);?\n\}\n', re.M)
TIPOS = 'u8 i8 u16 i16 u32 i32 usize isize f32 f64'.split()


def arquivos():
    yield os.path.join(SRC, 'motor.rs')
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if f.endswith('.rs'):
                yield os.path.join(d, f)


def canonico(params, ret, corpo):
    """(nome, definição) da função única equivalente, ou None."""
    p = [x.strip() for x in params.split(',') if x.strip()]
    m = re.fullmatch(r'core::ptr::read_unaligned\(a as \*const (\w+)\)(?: as (\w+))?', corpo) or \
        re.fullmatch(r'\*\(a as \*const (u8|i8)\)(?: as (\w+))?', corpo)
    if m and p == ['a: usize'] and ret:
        t, como = m.group(1), m.group(2)
        if (como or t) != ret or t not in TIPOS:
            return None
        nome = 'read_%s' % t if not como else 'read_%s_as_%s' % (t, como)
        return nome, 'pub unsafe fn %s(a: usize) -> %s {\n    core::ptr::read_unaligned(a as *const %s)%s\n}\n' % (
            nome, ret, t, ' as %s' % como if como else '')
    m = re.fullmatch(r'core::ptr::write_unaligned\(a as \*mut (\w+), v(?: as (\w+))?\)', corpo) or \
        re.fullmatch(r'\*\(a as \*mut (u8|i8)\) = v(?: as (\w+))?', corpo)
    if m and len(p) == 2 and p[0] == 'a: usize' and p[1].startswith('v: ') and not ret:
        t, como, v = m.group(1), m.group(2), p[1][3:]
        if (como or v) != t and not (como == t):
            return None
        if como and como != t:
            return None
        if t not in TIPOS:
            return None
        nome = 'write_%s' % t if v == t else 'write_%s_from_%s' % (t, v)
        return nome, 'pub unsafe fn %s(a: usize, v: %s) {\n    core::ptr::write_unaligned(a as *mut %s, v%s);\n}\n' % (
            nome, v, t, ' as %s' % t if v != t else '')
    m = re.fullmatch(r'core::ptr::write_unaligned\(\(o \+ d\) as \*mut (\w+), v\)', corpo) or \
        re.fullmatch(r'\*\(\(o \+ d\) as \*mut (u8|i8)\) = v', corpo)
    if m and p == ['o: usize', 'd: usize', 'v: %s' % m.group(1)] and not ret:
        t = m.group(1)
        nome = 'write_%s_at' % t
        return nome, 'pub unsafe fn %s(o: usize, d: usize, v: %s) {\n    core::ptr::write_unaligned((o + d) as *mut %s, v);\n}\n' % (nome, t, t)
    return None


def main():
    textos = {f: open(f, encoding='utf-8').read() for f in arquivos()}
    tudo = '\n'.join(textos.values())
    troca, defs, fora = {}, {}, []
    for f, t in textos.items():
        for m in DEF.finditer(t):
            c = canonico(m.group(3), m.group(4), m.group(5).strip())
            if c is None:
                fora.append((m.group(2), m.group(5).strip()[:60]))
                continue
            troca[m.group(2)] = c[0]
            defs[c[0]] = c[1]
    pmem = os.path.join(SRC, 'motor', 'mem.rs')
    mem = open(pmem, encoding='utf-8').read() if os.path.exists(pmem) else ''
    ja = [n for n in defs if re.search(r'\b%s\b' % n, tudo.replace(mem, '')) and n not in mem]
    assert not ja, ('nomes que já existem', ja)
    print('%d cópias -> %d funções; ficam %d' % (len(troca), len(defs), len(fora)))
    for n, c in collections.Counter(troca.values()).most_common():
        print('  %-28s %d' % (n, c))
    for x in fora[:20]:
        print('  fica:', x)
    if '--gravar' not in sys.argv:
        return
    rx = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(troca, key=len, reverse=True))))
    for f, t in textos.items():
        novo = DEF.sub(lambda m: '' if m.group(2) in troca else m.group(0), t)
        novo = rx.sub(lambda m: troca[m.group(1)], novo)
        if novo != t:
            open(f, 'w', encoding='utf-8').write(novo)
    renomear_mutacoes(troca)
    cab = ('// Unaligned memory access for the game\'s records, one function per type (stage 4 of the reorganisation:\n'
           '// they replace the copies each batch item used to carry).\n'
           '// [pt] Acesso à memória dos registros do jogo, sem alinhamento, uma função por tipo (etapa 4 da reorganização:\n'
           '// substituem as cópias que cada item de leva carregava).\n\n')
    novas = [defs[n] for n in sorted(defs) if 'fn %s(' % n not in mem]
    open(pmem, 'w', encoding='utf-8').write((mem or cab) + ''.join('\n' + d for d in novas))
    print('gravado')


def renomear_mutacoes(troca):
    """As listas de mutação citam trechos do código: a mesma troca de nomes vale nelas."""
    rx = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(troca, key=len, reverse=True))))
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in os.listdir(pasta):
        if f.endswith('.py'):
            p = os.path.join(pasta, f)
            t = open(p, encoding='utf-8').read()
            novo = rx.sub(lambda m: troca[m.group(1)], t)
            if novo != t:
                open(p, 'w', encoding='utf-8').write(novo)


def troca_de(src):
    """O mapa de troca calculado num outro fonte (ex.: o motor de antes da unificação)."""
    troca = {}
    for d, _, fs in os.walk(src):
        for f in fs:
            if f.endswith('.rs'):
                for m in DEF.finditer(open(os.path.join(d, f), encoding='utf-8').read()):
                    c = canonico(m.group(3), m.group(4), m.group(5).strip())
                    if c:
                        troca[m.group(2)] = c[0]
    return troca


if __name__ == '__main__':
    main()
