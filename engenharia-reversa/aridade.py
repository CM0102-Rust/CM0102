r"""Confere a aridade das rotinas do jogo chamadas por endereco no Rust (transmute de 0x...usize).

Uso: python aridade.py arquivo.rs [arquivo.rs ...]

Para cada `let f: unsafe extern "thiscall"|"stdcall" fn(...) = core::mem::transmute(0xADDRusize)`, conta
os argumentos declarados (fora o `this` no thiscall) e compara com o `ret N` da rotina no exe (N/4
argumentos que ela retira da pilha). Diferenca = pilha desalinhada (a original retira mais ou menos do
que a chamada pos). cdecl nao entra (quem chama limpa). Nasceu do erro human_manager.cpp 1668 (17/10/2026):
0x5fe290 (ret 8) chamada com um argumento so.
"""
import os, re, sys

import capstone

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EXE = open(os.path.join(R, 'jogo', 'cm0102.exe'), 'rb').read()
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)


def retira(va, limite=0x4000):
    """Bytes que a rotina retira no primeiro `ret` (None se nao achou)."""
    off = va - 0x400000
    for ins in md.disasm(EXE[off:off + limite], va):
        if ins.mnemonic == 'ret':
            return int(ins.op_str, 16) if ins.op_str else 0
    return None


def argumentos(assinatura):
    dentro = assinatura[assinatura.index('(') + 1:assinatura.rindex(')')]
    nivel, partes, atual = 0, [], ''
    for c in dentro:
        if c in '(<[':
            nivel += 1
        elif c in ')>]':
            nivel -= 1
        if c == ',' and nivel == 0:
            partes.append(atual)
            atual = ''
        else:
            atual += c
    if atual.strip():
        partes.append(atual)
    return [p for p in partes if p.strip()]


PADRAO = re.compile(r'unsafe extern "(thiscall|stdcall)" fn(\([^=\n]*?\))(?:\s*->\s*[^=\n]+)?\s*=\s*core::mem::transmute\((0x[0-9a-f]+)usize\)')
ruins = 0
for arq in sys.argv[1:]:
    texto = open(arq, encoding='utf-8').read()
    for m in PADRAO.finditer(texto):
        conv, assin, end = m.group(1), m.group(2), int(m.group(3), 16)
        args = argumentos(assin)
        # u64/i64/f64 ocupam dois dwords na pilha
        n = sum(2 if a.strip() in ('u64', 'i64', 'f64') else 1 for a in args) - (1 if conv == 'thiscall' else 0)
        r = retira(end)
        if r is None:
            continue
        if r != 4 * n:
            linha = texto.count('\n', 0, m.start()) + 1
            print(f'{arq}:{linha}: {end:#x} {conv} declarada com {n} argumento(s), a rotina retira {r // 4} (ret {r:#x})')
            ruins += 1
print(f'{ruins} diferenca(s)')
