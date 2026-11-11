r"""Machine code per function, between two builds of the DLL (11/11/2026). A readability batch only renames, so every
function should come out with the same machine code; one that does not is a suspect for a behaviour change the career
proofs did not reach (two were found this way by readers: the match stats screen of round 24, the scout report of batch
43). Each build is linked with `/MAP` (the linker's symbol map: `RUSTFLAGS="-C link-arg=/MAP:x.map"`); the code of each
function is disassembled (capstone) and normalised: the target of a call or jump outside the function becomes the
symbol's name, inside it the offset from the function's start, and any other address inside the DLL image becomes `@`
(data moves between builds). Two functions differ when their normalised code differs.
Usage: python maquina_por_funcao.py A.dll A.map B.dll B.map [--mostrar NAME] [--diffs OUT.txt] [--assinatura]
`--assinatura` keeps only the functions whose displacements, immediates, widths, extensions, signed/unsigned
branches or callees changed (the suspects; register allocation and order are left out).
[pt] O código de máquina de cada função entre duas compilações da DLL: uma leva que só troca nomes deixa tudo igual; a
função que muda é suspeita de mudança de comportamento.
"""
import hashlib
import re
import sys

import capstone
import pefile


def simbolos(mapa):
    base, out = None, []
    for l in open(mapa, encoding='latin-1'):
        m = re.match(r'\s*Preferred load address is ([0-9a-f]+)', l)
        if m:
            base = int(m.group(1), 16)
        m = re.match(r'\s*0001:[0-9a-f]+\s+(\S+)\s+([0-9a-f]{8})\s', l)
        if m:
            # names without the parts that change between builds: the legacy hash, LLVM's local suffix, the crate hash
            n = re.sub(r'17h[0-9a-f]{16}E$', 'E', m.group(1))
            n = re.sub(r'Cs[0-9A-Za-z]{9,13}_', '', re.sub(r'\.llvm\.\d+', '', n))
            out.append((int(m.group(2), 16), n))
    out.sort()
    return base, out


def funcoes(dll, mapa):
    base, sims = simbolos(mapa)
    pe = pefile.PE(dll, fast_load=True)
    ib = pe.OPTIONAL_HEADER.ImageBase
    fim_img = ib + pe.OPTIONAL_HEADER.SizeOfImage
    txt = [s for s in pe.sections if s.Name.rstrip(b'\0') == b'.text'][0]
    dados, va0 = txt.get_data(), ib + txt.VirtualAddress
    nome_em = {a: n for a, n in sims}
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
    res = {}
    for k, (a, n) in enumerate(sims):
        fim = sims[k + 1][0] if k + 1 < len(sims) else va0 + len(dados)
        if not (va0 <= a < va0 + len(dados)) or fim <= a or fim - a > 1 << 20:
            continue
        cod = dados[a - va0:fim - va0]
        linhas = []
        for i in md.disasm(cod, a):
            op = i.op_str
            if i.mnemonic.startswith(('call', 'j')) and re.fullmatch(r'0x[0-9a-f]+', op):
                t = int(op, 16)
                op = '+%x' % (t - a) if a <= t < fim else nome_em.get(t, '@')
            else:
                op = re.sub(r'0x[0-9a-f]+', lambda m: '@' if ib <= int(m.group(0), 16) < fim_img else m.group(0), op)
            linhas.append(i.mnemonic + ' ' + op)
        while linhas and linhas[-1].strip() in ('int3', 'nop'):  # the padding up to the next function
            linhas.pop()
        res[n] = linhas
    return res


def main():
    a, am, b, bm = sys.argv[1:5]
    fa, fb = funcoes(a, am), funcoes(b, bm)
    if '--mostrar' in sys.argv:
        import difflib
        n = sys.argv[sys.argv.index('--mostrar') + 1]
        for k in [k for k in fa if n in k]:
            print('\n'.join(difflib.unified_diff(fa[k], fb.get(k, []), lineterm='', n=2)))
        return
    h = lambda l: hashlib.sha1('\n'.join(l).encode()).hexdigest()
    # jump tables and data inside .text ($LN.., Trailing..Vec) disassemble as noise
    comuns = {k for k in set(fa) & set(fb) if not k.startswith(('$', 'Trailing'))}
    dif = sorted(k for k in comuns if h(fa[k]) != h(fb[k]))
    if '--assinatura' in sys.argv:
        # what a renaming cannot change: the multiset of displacements, immediates, memory widths, sign/zero
        # extensions, signed/unsigned branches and callees; register allocation and order are left out
        import collections

        def marca(lin):
            c = collections.Counter()
            for x in lin:
                mn, _, op = x.partition(' ')
                if mn.startswith('nop'):
                    continue
                # the stack frame (esp/ebp) moves with register allocation and the internal calling convention: left
                # out; what counts is an offset or width on a record (any other base register) and the constants
                pilha = re.search(r'\[e[sb]p\b', op)
                for t in re.findall(r'(?:byte|word|dword|qword) ptr \[[^\]]*\]', op):
                    if not re.search(r'\[e[sb]p\b', t):
                        d = re.search(r'([+-] 0x[0-9a-f]+)\]', t)
                        c[t.split(' [')[0] + (d.group(1) if d else '')] += 1
                if not pilha and '--so-memoria' not in sys.argv:
                    for t in re.findall(r'(?<![+-] )\b0x[0-9a-f]+\b(?!\])', op):
                        c[t] += 1
                if mn.startswith(('movsx', 'movzx') if '--so-memoria' in sys.argv else ('movsx', 'movzx', 'cbw', 'cwde', 'cdq', 'imul', 'idiv', 'div', 'mul', 'sar', 'shr')):
                    c[mn] += 1
                if mn.startswith('call') and '--sem-chamadas' not in sys.argv:
                    c['call ' + op] += 1
                if '--so-memoria' not in sys.argv and mn.startswith(('jl', 'jg', 'ja', 'jb', 'setl', 'setg', 'seta', 'setb', 'cmovl', 'cmovg', 'cmova', 'cmovb')):
                    c[mn.rstrip('e')] += 1
            return c
        dif = [k for k in dif if marca(fa[k]) != marca(fb[k])]
    print('%d funções em A, %d em B, %d em comum, %d diferentes; só em A: %d, só em B: %d' % (
        len(fa), len(fb), len(comuns), len(dif), len(set(fa) - set(fb)), len(set(fb) - set(fa))))
    for k in dif:
        print('  ', k, len(fa[k]), len(fb[k]))
    if '--diffs' in sys.argv:
        # every differing function's diff in one file, for a reader to judge which ones change behaviour
        import difflib
        with open(sys.argv[sys.argv.index('--diffs') + 1], 'w', encoding='utf-8') as f:
            for k in dif:
                f.write('=== %s\n' % k)
                f.write('\n'.join(difflib.unified_diff(fa[k], fb[k], lineterm='', n=3)) + '\n')


if __name__ == '__main__':
    main()
