r"""Checks every exe function the Rust code calls against the original machine code (09/11/2026): the declared
signature (`unsafe extern "conv" fn(args) -> ret`, in the exe:: wrappers and in the remaining local transmutes) against
what the routine and its callers in cm0102.exe actually do:

- stack bytes: the routine's `ret N` (stdcall/thiscall) or the callers' `add esp, N` after the call (cdecl);
- ecx: whether the routine reads ecx before writing it (thiscall `this`); a non-thiscall declaration of a routine that
  reads ecx passes garbage;
- return: the width the routine writes last before each `ret` (al/ax/eax/edx:eax), and what the original callers read
  right after the call (al, ax, eax, edx, st0). A declared return wider than what the routine writes reads garbage.

Signatures already checked by hand (levas/reorganizacao/assinaturas_conferidas.json: `signature` imposed on the
wrapper, or `true_signature` with `accepted_sites` judged harmless) are downgraded to info. --validar checks the tool
against those answers. Output: one
line per finding in levas/reorganizacao/conferencia_assinaturas.tsv, sorted by severity (bug, then info).
Usage: python conferir_assinaturas.py [--fn FN_X | --validar]; exit code 1 when a bug is found (publicar_leva.py
stops on it)
[pt] Confere cada função do exe chamada pelo Rust contra o código de máquina original: a pilha (`ret N` ou o
`add esp` dos chamadores), o uso do ecx e a largura do retorno (o que a rotina escreve antes do `ret` e o que os
chamadores leem). Saída em levas/reorganizacao/conferencia_assinaturas.tsv, erros primeiro.
"""
import bisect
import collections
import json
import os
import re
import sys

import capstone
from capstone import x86
import pefile

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EXE = os.path.join(R, 'jogo', 'cm0102.exe')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import involucros_exe as ie  # noqa: E402

pe = pefile.PE(EXE, fast_load=True)
BASE = pe.OPTIONAL_HEADER.ImageBase
TEXT = next(s for s in pe.sections if s.Name.rstrip(b'\0') == b'.text')
T0 = BASE + TEXT.VirtualAddress
TB = TEXT.get_data()
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
md.detail = True

# function extents from the community map
INICIOS, TAM = [], {}
for l in open(os.path.join(R, 'docs', 'mapa_funcoes.csv'), encoding='utf-8').read().splitlines()[1:]:
    a, t = l.split(',')[:2]
    INICIOS.append(int(a, 16))
    TAM[int(a, 16)] = int(t)
INICIOS.sort()
INICIO_SET = set(INICIOS)

ECX = {x86.X86_REG_ECX, x86.X86_REG_CX, x86.X86_REG_CL, x86.X86_REG_CH}
EDX = {x86.X86_REG_EDX, x86.X86_REG_DX, x86.X86_REG_DL, x86.X86_REG_DH}
EAX_W = {x86.X86_REG_AL: 1, x86.X86_REG_AH: 1, x86.X86_REG_AX: 2, x86.X86_REG_EAX: 4}


def ins_em(va):
    off = va - T0
    if off < 0 or off >= len(TB):
        return None
    for i in md.disasm(TB[off:off + 16], va, 1):
        return i
    return None


def le_escreve(i):
    try:
        r, w = i.regs_access()
    except capstone.CsError:
        return set(), set()
    return set(r), set(w)


def idioma_zera(i):
    """A write whose result does not depend on the register's old value: xor/sub/sbb reg,reg; or reg,-1; and reg,0."""
    if len(i.operands) != 2 or i.operands[0].type != x86.X86_OP_REG:
        return False
    o1 = i.operands[1]
    if i.mnemonic in ('xor', 'sub', 'sbb') and o1.type == x86.X86_OP_REG and i.operands[0].reg == o1.reg:
        return True
    return o1.type == x86.X86_OP_IMM and ((i.mnemonic == 'or' and o1.imm in (-1, 0xffffffff, 0xffff, 0xff)) or (i.mnemonic == 'and' and o1.imm == 0))


FAM = {}
for _n, _f in (('al ah ax eax', 'a'), ('cl ch cx ecx', 'c'), ('dl dh dx edx', 'd')):
    for _x in _n.split():
        FAM[getattr(x86, 'X86_REG_' + _x.upper())] = _f


def familias(regs):
    return {FAM[r] for r in regs if r in FAM}


def zera_cima(i):
    """An eax write that leaves the upper 24 bits zero."""
    if idioma_zera(i) or i.mnemonic.startswith('movzx'):
        return True
    if i.mnemonic in ('mov', 'and') and len(i.operands) == 2 and i.operands[1].type == x86.X86_OP_IMM:
        return 0 <= i.operands[1].imm < 0x100
    return False


def alvo(i):
    if i.operands and i.operands[0].type == x86.X86_OP_IMM:
        return i.operands[0].imm
    return None


def analisa(va):
    """Walks the routine's paths: the `ret N` values, whether ecx/edx are read before written, and the eax width
    written last before each ret ('call' = the value of an inner call, 'tail' = a jump into another function)."""
    rets, eax_w = set(), set()
    le_ecx = le_edx = False
    vistos = set()
    pilha = [(va, False, False, None)]  # address, ecx written, edx written, last eax write width
    passos = 0
    while pilha and passos < 60000:
        a, ecx_w, edx_w, w = pilha.pop()
        while True:
            passos += 1
            if (a, ecx_w, edx_w, w) in vistos:
                break
            vistos.add((a, ecx_w, edx_w, w))
            i = ins_em(a)
            if i is None:
                break
            lidos, escritos = le_escreve(i)
            zera = idioma_zera(i)
            if not zera and not (i.mnemonic == 'push' and i.op_str in ('ecx', 'edx')):
                if not ecx_w and lidos & ECX:
                    le_ecx = True
                    RASTRO.setdefault('ecx', '%x %s %s' % (i.address, i.mnemonic, i.op_str))
                if not edx_w and lidos & EDX:
                    le_edx = True
            if escritos & ECX:
                ecx_w = True
            if escritos & EDX:
                edx_w = True
            # the widest of al/ax/eax this instruction writes (movzx eax, al writes 4; mov al, 1 writes 1); a narrow
            # write over an eax whose upper bits are known zero (xor eax, eax / movzx / small constant) is still 4
            ws_eax = [larg for reg, larg in EAX_W.items() if reg in escritos]
            if ws_eax:
                RASTRO.setdefault('eax', []).append('%x %s %s' % (i.address, i.mnemonic, i.op_str))
                if max(ws_eax) == 4:
                    w = 4 if not zera_cima(i) else 'z'
                elif w != 'z':
                    w = max(ws_eax)
            if i.mnemonic == 'call':
                ecx_w = edx_w = True
                w = 'call'
                a = i.address + i.size
                continue
            if i.mnemonic in ('ret', 'retn'):
                rets.add(i.operands[0].imm if i.operands else 0)
                eax_w.add(w)
                break
            if i.mnemonic == 'jmp':
                t = alvo(i)
                if t is None:
                    break  # jump table / indirect: stop this path
                # a jump to another real function (one the exe calls) is a tail call; the community map also splits
                # routines into chunks, and a jump into a chunk stays inside the routine
                if t != va and t in chamadores_de_todos():
                    eax_w.add('tail')
                    rets.add(('tail', t))
                    break
                a = t
                continue
            if i.mnemonic.startswith('j') or i.mnemonic.startswith('loop'):
                t = alvo(i)
                if t is not None and t not in chamadores_de_todos():
                    pilha.append((t, ecx_w, edx_w, w))
            a = i.address + i.size
    return rets, le_ecx, le_edx, eax_w


CHAMADORES = None
RASTRO = {}


def chamadores_de_todos():
    chamadores(0)
    return CHAMADORES


def chamadores(va):
    global CHAMADORES
    if CHAMADORES is None:
        CHAMADORES = collections.defaultdict(list)
        b = TB
        n = len(b)
        k = b.find(b'\xe8')
        while 0 <= k < n - 5:
            d = int.from_bytes(b[k + 1:k + 5], 'little', signed=True)
            t = T0 + k + 5 + d
            if t in INICIO_SET:
                CHAMADORES[t].append(T0 + k)
            k = b.find(b'\xe8', k + 1)
    return CHAMADORES.get(va, [])


def leitura_depois(call_va):
    """What the caller reads of the result right after the call: 'al', 'ax', 'eax', 'edx', 'st0' or None; and the
    `add esp, N` right after it (cdecl cleanup), or None. Also whether ecx was loaded right before the call."""
    i = ins_em(call_va)
    a = call_va + i.size
    usa, limpa, ws = set(), None, set()
    for k in range(10):
        j = ins_em(a)
        if j is None:
            break
        if k == 0 and j.mnemonic == 'add' and j.op_str.startswith('esp, '):
            try:
                limpa = int(j.op_str[5:], 0)
            except ValueError:
                pass
        if j.mnemonic.startswith('f') and ('st(0)' in j.op_str or j.mnemonic in ('fstp', 'fst', 'fistp', 'fcomp', 'fcom', 'fcompp', 'fxch', 'fmul', 'fadd', 'fdiv', 'fsub', 'fmulp', 'faddp', 'fdivp', 'fsubp', 'fsubrp', 'fdivrp', 'fcomip', 'fucomip', 'fucomp')):
            if not j.mnemonic.startswith('fld') and not j.mnemonic.startswith('fild'):
                usa.add('st0')
            break
        lidos, escritos = le_escreve(j)
        if idioma_zera(j):
            lidos = set()
        if 'a' not in ws:
            for reg, nome in ((x86.X86_REG_AL, 'al'), (x86.X86_REG_AX, 'ax'), (x86.X86_REG_EAX, 'eax')):
                if reg in lidos:
                    usa.add(nome)
        if 'd' in familias(lidos) and 'd' not in ws:
            usa.add('edx')
        ws |= familias(escritos)
        if j.mnemonic in ('call', 'ret', 'jmp') or j.mnemonic.startswith('j'):
            break
        a = j.address + j.size
    return usa, limpa


def ecx_antes(call_va):
    """Whether one of the 6 instructions before the call writes ecx (the original passes `this`)."""
    a = call_va - 24
    ws = False
    for j in md.disasm(TB[a - T0:call_va - T0], a):
        if j.address + j.size > call_va:
            break
        _, w = le_escreve(j)
        if w & ECX:
            ws = True
        if j.mnemonic == 'call':
            ws = False
    return ws


def bytes_pilha(conv, params):
    larg = [8 if ie.norm(p) in ('u64', 'i64', 'f64') else 4 for p in params]
    if conv == 'thiscall':
        return sum(larg[1:])
    if conv == 'fastcall':
        return sum(larg[2:])
    return sum(larg)


def parametros(cabeca):
    """Parameter names of a `fn name(a: T, b: U) -> R` head, in order."""
    i = cabeca.find('(')
    args, _ = ie.args_da_chamada(cabeca, i)
    return [a.split(':')[0].strip().replace('mut ', '') for a in (args or [])]


def por_parametro(fontes, out):
    """Routines received as a parameter: `fn helper(.., routine: usize, ..) { let f: SIG = transmute(routine); ..}`;
    the fixed addresses come from the callers (`helper(.., 0x8ca8a0, ..)` or `FN_X`). Also the address dispatch
    `0x4cf5f0 | 0x7c3a40 => core::mem::transmute::<usize, T2>(routine)(..)`."""
    import mover_rotinas
    consts = {n: v for n, v in re.findall(r'const (FN_\w+): usize = (0x[0-9a-f]+);', open(os.path.join(R, 'rust', 'src', 'motor', 'exe_fns.rs'), encoding='utf-8').read())}
    ajudantes = {}  # name -> [(param index, signature)]
    for p, t in fontes.items():
        tipos = dict((n, re.sub(r'\s+', ' ', sg)) for n, sg in re.findall(r'type (\w+) = (unsafe extern "\w+" fn\([^)]*\)(?: -> [\w*: ]+?)?);', t))
        for m in re.finditer(r'((?:0x[0-9a-f]+\s*\|\s*)*0x[0-9a-f]+)\s*=>\s*core::mem::transmute::<usize, (\w+)>\(\w+\)', t):
            if m.group(2) in tipos:
                for a in re.findall(r'0x[0-9a-f]+', m.group(1)):
                    out['ADDR_' + a][tipos[m.group(2)]].append('%s:%d' % (os.path.relpath(p, R).replace('\\', '/'), t.count('\n', 0, m.start()) + 1))
        for nome, a, b in mover_rotinas.blocos(t):
            corpo = t[a:b]
            cab = corpo[:corpo.find('{')] if '{' in corpo else ''
            ps = parametros(cab) if '(' in cab else []
            for m in re.finditer(r'let \w+: (unsafe extern "\w+" fn\([^)]*\)(?: -> [\w*: ]+?)?) = core::mem::transmute\((\w+)\);', corpo):
                if m.group(2) in ps:
                    ajudantes.setdefault(nome, []).append((ps.index(m.group(2)), re.sub(r'\s+', ' ', m.group(1))))
    for p, t in fontes.items():
        for nome, usos in ajudantes.items():
            for c in re.finditer(r'(?<![\w.])%s\(' % re.escape(nome), t):
                args, _ = ie.args_da_chamada(t, c.end() - 1)
                if not args:
                    continue
                for k, sig in usos:
                    if k >= len(args):
                        continue
                    x = re.sub(r'\s+as\s+usize$', '', args[k].strip())
                    end = x if re.fullmatch(r'0x[0-9a-f]+', x) else consts.get(x)
                    if end:
                        out['ADDR_' + end][sig].append('%s:%d (%s)' % (os.path.relpath(p, R).replace('\\', '/'), t.count('\n', 0, c.start()) + 1, nome))


def declaracoes():
    """{FN_X: {signature: [where, ...]}} from the wrappers and from the local transmutes."""
    out = collections.defaultdict(lambda: collections.defaultdict(list))
    fontes = {}
    for raiz, _, fs in os.walk(os.path.join(R, 'rust', 'src')):
        for f in fs:
            if not f.endswith('.rs'):
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8').read()
            fontes[p] = t
            for m in re.finditer(r'let \w+: (unsafe extern "\w+" fn\([^)]*\)(?: -> [\w*: ]+?)?) = core::mem::transmute\((FN_\w+)\)', t):
                lin = t.count('\n', 0, m.start()) + 1
                out[m.group(2)][re.sub(r'\s+', ' ', m.group(1))].append('%s:%d' % (os.path.relpath(p, R).replace('\\', '/'), lin))
            # calls by a fixed address through a typed closure: `type F1 = unsafe extern ..; let t = |a: usize|
            # core::mem::transmute::<usize, F1>(a);` and then `t(0x70df20)(p)`
            tipos = dict((n, re.sub(r'\s+', ' ', sg)) for n, sg in re.findall(r'type (\w+) = (unsafe extern "\w+" fn\([^)]*\)(?: -> [\w*: ]+?)?);', t))
            for m in re.finditer(r'let (\w+) = \|\w+: usize\| (?:core::mem::)?transmute::<usize, (\w+)>\(\w+\);', t):
                if m.group(2) not in tipos:
                    continue
                for c in re.finditer(r'(?<![\w.])%s\((0x[0-9a-f]+)\)' % re.escape(m.group(1)), t):
                    lin = t.count('\n', 0, c.start()) + 1
                    out['ADDR_' + c.group(1)][tipos[m.group(2)]].append('%s:%d' % (os.path.relpath(p, R).replace('\\', '/'), lin))
    por_parametro(fontes, out)
    return out


def julgar(sig, ns, le_ecx, le_edx, eax_w, leituras, limpezas):
    """[(severity, text)] for one declared signature against the facts of the routine."""
    conv, params, ret = ie.partes(sig)
    prob = []
    bp = bytes_pilha(conv, params)
    if conv in ('stdcall', 'thiscall', 'fastcall'):
        if ns and bp not in ns:
            prob.append(('bug', 'pilha: declara %d bytes, a rotina faz ret %s' % (bp, sorted(ns))))
    elif conv == 'C':
        if ns and ns != {0}:
            prob.append(('bug', 'cdecl declarado, mas a rotina faz ret %s' % sorted(ns)))
        elif limpezas and bp not in limpezas and None not in limpezas and limpezas.most_common(1)[0][0] is not None:
            prob.append(('info', 'cdecl: declara %d bytes, os chamadores limpam %s' % (bp, dict(limpezas))))
    if le_ecx and conv not in ('thiscall', 'fastcall'):
        prob.append(('bug', 'a rotina lê ecx (this) e a declaração é %s' % conv))
    if not le_ecx and conv == 'thiscall' and params:
        prob.append(('info', 'thiscall declarado, mas a rotina não lê ecx (pilha igual se ret confere)'))
    if le_edx and conv != 'fastcall' and not le_ecx:
        prob.append(('info', 'a rotina lê edx antes de escrever'))
    if ret:
        cl = ie.classe(ret)
        if cl in ('f32', 'f64'):
            if leituras and 'st0' not in leituras and leituras.get('eax'):
                prob.append(('bug', 'retorno float declarado, os chamadores leem eax'))
        else:
            if leituras.get('st0', 0) >= 2 and not (leituras.keys() - {'st0'}):
                prob.append(('bug', 'retorno inteiro declarado, os chamadores leem st0'))
            lw = ie.largura(ret)
            ws = {4 if x == 'z' else x for x in eax_w if isinstance(x, int) or x == 'z'}
            if ws and 4 >= lw > min(ws):
                prob.append(('bug', 'retorno de %d bytes declarado; a rotina escreve só %s em algum caminho' % (lw, sorted(ws))))
            if lw == 8 and 'edx' not in leituras and leituras:
                prob.append(('info', 'retorno de 64 bits declarado; os chamadores não leem edx'))
            if lw < 8 and leituras.get('edx', 0) * 5 >= sum(leituras.values()) and leituras.get('edx', 0) >= 2:
                prob.append(('bug', 'os chamadores do original leem edx depois da chamada (retorno de 64 bits) %s' % dict(leituras)))
            so_al = leituras.keys() <= {'al'} and leituras.get('al', 0) >= 2
            so_ax = leituras.keys() <= {'al', 'ax'} and leituras.get('ax', 0) >= 2
            if lw == 4 and (so_al or so_ax):
                prob.append(('bug', 'os chamadores do original leem só %s; o retorno declarado tem 4 bytes' % ('al' if so_al else 'ax')))
    return prob


def fatos_de(va):
    rets, le_ecx, le_edx, eax_w = analisa(va)
    ns = {r for r in rets if not isinstance(r, tuple)}
    cs = chamadores(va)
    leituras, limpezas, com_ecx = collections.Counter(), collections.Counter(), 0
    for c in cs:
        u, lp = leitura_depois(c)
        for x in u:
            leituras[x] += 1
        limpezas[lp] += 1
        com_ecx += ecx_antes(c)
    texto = 'ret=%s ecx=%s edx=%s eax_w=%s chamadores=%d lidos=%s add_esp=%s ecx_antes=%d' % (
        sorted(ns) or '-', int(le_ecx), int(le_edx), sorted(map(str, eax_w)), len(cs), dict(leituras),
        dict(limpezas), com_ecx)
    return (ns, le_ecx, le_edx, eax_w, leituras, limpezas), texto


def validar(ends, conf):
    """Against the hand-checked answers: the true signature should give no bug, each wrong site should."""
    falso_bug, pegos, perdidos = [], 0, []
    for fn, v in sorted(conf.items()):
        if 'signature' not in v:
            continue
        f, texto = fatos_de(ends[fn])
        b = [t for sev, t in julgar(v['signature'], *f) if sev == 'bug']
        if b:
            falso_bug.append((fn, v['signature'], b, texto))
        for w in v.get('wrong_sites', []):
            if not ie.SIG.match(w.strip()):
                continue
            if any(sev == 'bug' for sev, _ in julgar(w, *f)):
                pegos += 1
            else:
                perdidos.append((fn, w))
    print('verdadeiras acusadas: %d de %d' % (len(falso_bug), len(conf)))
    for x in falso_bug:
        print('  ', *x)
    print('erradas pegas: %d; não pegas: %d (muitas são inofensivas pela evidência)' % (pegos, len(perdidos)))
    for x in perdidos:
        print('   ', x[0], '|', x[1], '|', conf[x[0]]['evidence'][:150])


def main():
    ends = {n: int(v, 16) for n, v in re.findall(r'const (FN_\w+): usize = (0x[0-9a-f]+);', open(os.path.join(R, 'rust', 'src', 'motor', 'exe_fns.rs'), encoding='utf-8').read())}
    conf = json.load(open(os.path.join(R, 'levas', 'reorganizacao', 'assinaturas_conferidas.json'), encoding='utf-8'))
    if '--validar' in sys.argv:
        return validar(ends, conf)
    decl = declaracoes()
    so = sys.argv[sys.argv.index('--fn') + 1] if '--fn' in sys.argv else None
    achados = []
    for fn in sorted(decl):
        if so and fn != so:
            continue
        va = int(fn[5:], 16) if fn.startswith('ADDR_') else ends.get(fn)
        if va is None or va not in INICIO_SET:
            achados.append(('info', fn, '', 'sem início no mapa', ''))
            continue
        f, fatos = fatos_de(va)
        for sig, onde in decl[fn].items():
            if '...' in sig:
                continue
            prob = julgar(sig, *f)
            aceitas = {re.sub(r'\s+', ' ', x) for x in conf.get(fn, {}).get('accepted_sites', [])}
            if sig in aceitas or (conf.get(fn, {}).get('signature') and re.sub(r'\s+', ' ', conf[fn]['signature']) == sig):
                # checked by hand: the true signature, or a site the evidence says is harmless
                prob = [('info', 'conferida à mão: ' + t) for _, t in prob]
            for sev, txt in prob:
                achados.append((sev, fn, sig, txt, fatos + ' | ' + ', '.join(onde[:3]) + (' (+%d)' % (len(onde) - 3) if len(onde) > 3 else '') + (' | CONFERIDA' if fn in conf else '')))
    achados.sort(key=lambda x: (x[0] != 'bug', x[1]))
    saida = os.path.join(R, 'levas', 'reorganizacao', 'conferencia_assinaturas.tsv')
    with open(saida, 'w', encoding='utf-8') as f:
        f.write('severidade\tfunção\tassinatura\tachado\tfatos\n')
        for a in achados:
            f.write('\t'.join(a) + '\n')
    c = collections.Counter((a[0], a[3].split(':')[0].split(',')[0][:40]) for a in achados)
    print('%d funções; %d achados' % (len(decl), len(achados)))
    for k, v in c.most_common():
        print('  %s %-42s %d' % (k[0], k[1], v))
    erros = [a for a in achados if a[0] == 'bug']
    for a in erros:
        print('BUG', a[1], '|', a[2], '|', a[3])
    return 1 if erros else 0


if __name__ == '__main__':
    sys.exit(main())
