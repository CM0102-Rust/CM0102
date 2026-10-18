r"""Pacote de trabalho de uma rotina: tudo o que se le antes de porta-la, numa pasta so.

Uso: python pacote.py 0x6b98c0 [--sem-ghidra]

Grava em pacotes/<end>/ (fora do git: tem desmontagem e pseudocodigo do jogo):
  LEIA.md          modulo, tamanho, se ja e Rust; quem chama e o que ela chama (com o nome Rust de
                   cada rotina ja portada ou "original"); constantes de ponto flutuante lidas do
                   .rdata; tabelas de salto; onde a rotina ja aparece nas notas (levas/, docs/)
  desmontagem.txt  o assembly inteiro (desmonta.py), a fonte das contas exatas
  pseudocodigo.c   o pseudocodigo do Ghidra (descompilar.py), a estrutura (sem --sem-ghidra)
  desmontagem_olly.txt  o mesmo trecho no texto do OllyDbg trazido do forum (referencias/
                   cm0102_3.9.68_text.zip): lacos desenhados, Arg1.., switch e strings comentados
E no LEIA.md, com o texto do OllyDbg: os campos (deslocamentos >= 0x400) que a rotina usa e as
rotinas do exe inteiro que os leem ou escrevem, cada uma com o nome Rust ou "original".
Ideia emprestada do decomp.wtf (MIT, github.com/i12bp8/decomp.wtf): cada rotina vira um pacote
com o assembly, as referencias, quem chama, um rascunho e as notas das tentativas anteriores.
"""
import glob, os, re, shutil, struct, subprocess, sys

import fronteiras

AQUI = os.path.dirname(os.path.abspath(__file__))
R = fronteiras.R
EXE = os.path.join(R, 'jogo', 'cm0102.exe')


def nomes_rust():
    """endereco -> (nome da fn, arquivo:linha) pelas linhas `/// { 0xADDR }` do codigo Rust."""
    r = {}
    for f in glob.glob(os.path.join(R, 'rust', 'src', '*.rs')):
        linhas = open(f, encoding='utf-8').read().split('\n')
        for i, l in enumerate(linhas):
            m = re.match(r'/// \{ (0x[0-9a-f]+) \}', l)
            if not m:
                continue
            for k in range(i + 1, min(i + 40, len(linhas))):
                n = re.search(r'\bfn ([a-z_0-9]+)\s*[(<]', linhas[k])
                if n:
                    r.setdefault(int(m.group(1), 16), (n.group(1), f'{os.path.relpath(f, R)}:{k + 1}'))
                    break
    return r


def constantes(d, texto):
    """Os float/double lidos de enderecos fixos (dword/qword ptr [0x9.....]) com o valor."""
    vistos = {}
    for tam, a in re.findall(r'(dword|qword) ptr \[(0x9[0-9a-f]{5})\]', texto):
        a = int(a, 16)
        o = a - 0x400000
        if tam == 'dword':
            v = struct.unpack_from('<f', d, o)[0]
        else:
            v = struct.unpack_from('<d', d, o)[0]
        vistos[(a, tam)] = v
    return sorted(vistos.items())


def tabelas(d, texto):
    """Tabelas de salto (jmp dword ptr [reg*4 + 0x...]) com os destinos."""
    r = []
    for t in sorted(set(re.findall(r'jmp\s+dword ptr \[\w+\*4 \+ (0x[0-9a-f]+)\]', texto))):
        a = int(t, 16)
        dest = []
        for k in range(64):
            v = struct.unpack_from('<I', d, a - 0x400000 + 4 * k)[0]
            if not (fronteiras.TXT0 <= v < fronteiras.TXT1):
                break
            dest.append(v)
        r.append((a, dest))
    return r


OLLY = os.path.join(R, 'referencias', 'cm0102_3.9.68_text.zip')  # o exe inteiro em texto (OllyDbg), do forum
ESCREVE = ('MOV', 'ADD', 'SUB', 'INC', 'DEC', 'AND', 'OR', 'XOR', 'NEG', 'NOT', 'SHL', 'SHR', 'SAR', 'FSTP', 'FST', 'FISTP', 'SETE', 'SETNE')


def olly_linhas():
    """As linhas do texto do OllyDbg (ou [] sem o arquivo)."""
    if not os.path.exists(OLLY):
        return []
    import zipfile
    z = zipfile.ZipFile(OLLY)
    return z.open(z.namelist()[0]).read().decode('latin-1').split('\n')


def campos(c, linhas, a0, a1, minimo=0x400):
    """Deslocamentos >= `minimo` usados em [a0, a1) e, para cada um, as rotinas do exe que o leem ou
    escrevem (o mesmo deslocamento em outro objeto tambem conta: e uma pista, nao prova)."""
    import bisect
    ref = re.compile(r'\+([0-9A-F]{3,5})\]')
    usados = set()
    for l in linhas:
        if len(l) > 8 and l[:8].isalnum():
            a = int(l[:8], 16) if all(ch in '0123456789ABCDEF' for ch in l[:8]) else -1
            if a0 <= a < a1:
                usados |= {int(x, 16) for x in ref.findall(l) if int(x, 16) >= minimo}
    uso = {k: {} for k in usados}
    ini = c['inicios']
    for l in linhas:
        if '+' not in l or ']' not in l:
            continue
        for x in ref.findall(l):
            k = int(x, 16)
            if k in uso:
                try:
                    a = int(l[:8], 16)
                except ValueError:
                    continue
                f = ini[bisect.bisect_right(ini, a) - 1] if a >= ini[0] else a
                # a instrucao comeca na coluna 28, depois das barras que o OllyDbg desenha nos lacos
                instr = l[28:].split(';')[0].lstrip('|\\/>^ ').strip() if len(l) > 28 else ''
                mn = instr.split(' ')[0] if instr else ''
                op1 = instr[len(mn):].split(',')[0]
                w = mn in ESCREVE and f'+{x}]' in op1
                r_, e_ = uso[k].get(f, (0, 0))
                uso[k][f] = (r_ + (not w), e_ + w)
    return uso


def olly_trecho(linhas, a0, a1):
    """O trecho [a0, a1) do texto do OllyDbg (laços desenhados, Arg1.., switch e strings comentados)."""
    r = []
    for l in linhas:
        try:
            a = int(l[:8], 16)
        except ValueError:
            continue
        if a0 <= a < a1:
            r.append(l.rstrip('\r'))
        elif a >= a1:
            break
    return '\n'.join(r) + '\n'


def notas(addr):
    """Onde o endereco ja aparece nas notas do projeto."""
    padrao = f'{addr:#x}'
    r = []
    for f in sorted(glob.glob(os.path.join(R, 'levas', '*', 'LEIA.md')) + glob.glob(os.path.join(R, 'docs', '*.md'))):
        if f.endswith('.en.md'):
            continue
        for i, l in enumerate(open(f, encoding='utf-8').read().split('\n')):
            if padrao in l:
                r.append(f'{os.path.relpath(f, R)}:{i + 1}: {l.strip()[:200]}')
    return r



def mapa_da_comunidade():
    """O mapa de funções e globais de maxevans60 (CC BY 4.0, github.com/maxevans60/cm0102-function-map),
    clonado em referencias/cm0102-function-map. Devolve (funções por endereço, globais por endereço)."""
    import csv
    base = os.path.join(R, 'referencias', 'cm0102-function-map')
    if not os.path.isdir(base):
        return {}, {}
    f = {int(r['address'], 16): r for r in csv.DictReader(open(os.path.join(base, 'map', 'functions.tsv'), encoding='utf-8'), delimiter='\t')}
    g = {int(r['address'], 16): r for r in csv.DictReader(open(os.path.join(base, 'data-map', 'globals.tsv'), encoding='utf-8'), delimiter='\t')}
    return f, g


def secao_mapa(addr, desm):
    """Nome, convenção e propósito da rotina no mapa da comunidade, e os globais que ela usa."""
    import re
    f, g = mapa_da_comunidade()
    if not f:
        return ['', '## No mapa da comunidade', '', '(clonar github.com/maxevans60/cm0102-function-map em referencias/)']
    r = f.get(addr)
    L = ['', '## No mapa da comunidade (maxevans60, CC BY 4.0)', '']
    if r:
        L += [f'- **{r["name"]}** ({r["doc"]}; confiança {r["confidence"]}): {r["purpose"]}', f'- Convenção: `{r["convention"]}`']
    else:
        L += ['- (não é início de função no mapa)']
    usados = sorted({int(x, 16) for x in re.findall(r'(0x[0-9a-f]{6})\b', desm) if int(x, 16) in g})
    for a in usados:
        L.append(f'- global {a:#x} **{g[a]["name"]}** ({g[a]["type"]}, {g[a]["size"]} bytes, {g[a]["confidence"]})')
    return L


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    addr = int(sys.argv[1], 16)
    c = fronteiras.calcular()
    d = c['d']
    if addr not in c['ext']:
        sys.exit(f'{addr:#x} nao e inicio de funcao conhecido (ver docs/mapa_funcoes.csv e fronteiras.py)')
    n = c['ext'][addr]
    rust = nomes_rust()
    saida = os.path.join(R, 'pacotes', f'{addr:#x}')
    os.makedirs(saida, exist_ok=True)
    desm = subprocess.run([sys.executable, os.path.join(AQUI, 'desmonta.py'), EXE, f'{addr:x}', f'{addr + n:x}'],
                          capture_output=True, text=True, encoding='utf-8').stdout
    open(os.path.join(saida, 'desmontagem.txt'), 'w', encoding='utf-8').write(desm)
    pseudo = 'pseudocodigo.c (Ghidra)'
    if '--sem-ghidra' in sys.argv:
        pseudo = '(sem pseudocodigo: --sem-ghidra)'
    else:
        subprocess.run([sys.executable, os.path.join(AQUI, 'descompilar.py'), f'{addr:#x}'], capture_output=True)
        c_ghidra = os.path.join(R, 'referencias', 'ferramentas', 'saida', f'{addr:#x}.c')
        if os.path.exists(c_ghidra):
            shutil.copy(c_ghidra, os.path.join(saida, 'pseudocodigo.c'))
        else:
            pseudo = '(o Ghidra nao gerou pseudocodigo; ver descompilar.py)'

    def estado(a):
        if a in rust:
            return f'Rust `{rust[a][0]}` ({rust[a][1]})' + ('' if a in c['port'] else ' (so consulta, sem desvio)')
        return 'Rust (desviada)' if a in c['port'] else 'original'

    linhas = olly_linhas()
    secao_campos = ['', '## Campos (deslocamentos >= 0x400) e quem mais os usa', '']
    if linhas:
        open(os.path.join(saida, 'desmontagem_olly.txt'), 'w', encoding='utf-8').write(olly_trecho(linhas, addr, addr + n))
        uso = campos(c, linhas, addr, addr + n)
        secao_campos.append('(o mesmo deslocamento pode ser de outro objeto: pista, nao prova; l = leituras, e = escritas)')
        for k in sorted(uso):
            fs = sorted(uso[k].items(), key=lambda kv: -(kv[1][0] + kv[1][1]))
            secao_campos.append(f'- **+{k:#x}** ({len(fs)} rotinas): ' + '; '.join(
                f'{f:#x}{" (esta)" if f == addr else ""} {estado(f)} l{r_}/e{e_}' for f, (r_, e_) in fs[:12]) + (' ...' if len(fs) > 12 else ''))
    else:
        secao_campos.append('(sem o texto do OllyDbg em referencias/cm0102_3.9.68_text.zip)')
    chama = sorted(fronteiras.chamadas_em(d, addr, n, c['alvos']))
    chamadores = [a for a in c['inicios'] if addr in fronteiras.chamadas_em(d, a, c['ext'][a], c['alvos'])]
    faltam = [a for a in chama if a not in c['port'] and a < 0x940000]
    L = [f'# Pacote de {addr:#x}', '',
         f'- Modulo: `{c["mod"][addr]}`; tamanho ate a proxima fronteira: {n} bytes; instrucoes: {desm.count(chr(10))}',
         f'- Estado: {estado(addr)}',
         f'- Chamadas ainda originais (fora a CRT): {len(faltam)}' + (' — ' + ', '.join(hex(a) for a in faltam) if faltam else ''),
         f'- Pseudocodigo: {pseudo}', '',
         '## O que ela chama', ''] + [f'- {a:#x} — {estado(a)}' for a in chama] + \
        ['', '## Quem a chama', ''] + [f'- {a:#x} (`{c["mod"][a]}`) — {estado(a)}' for a in chamadores] + \
        ['', '## Constantes de ponto flutuante', '', '| Endereco | Tipo | Valor |', '|---|---|---|'] + \
        [f'| {a:#x} | {"f32" if t == "dword" else "f64"} | {v!r} |' for (a, t), v in constantes(d, desm)] + \
        ['', '## Tabelas de salto', '', '(lidas ate o primeiro valor fora do .text; a ultima pode emendar na tabela vizinha)', ''] + [f'- {a:#x}: ' + ', '.join(hex(x) for x in dest) for a, dest in tabelas(d, desm)] + \
        secao_mapa(addr, desm) + \
        secao_campos + \
        ['', '## Nas notas do projeto', ''] + (notas(addr) or ['(nenhuma)']) + \
        ['', 'Ciclo de uma leva e armadilhas: `levas/LEIA.md` e `docs/COMO_CONTINUAR.md`.', '']
    open(os.path.join(saida, 'LEIA.md'), 'w', encoding='utf-8').write('\n'.join(L))
    print(f'pacote em {os.path.relpath(saida, R)}: {len(chama)} chamadas, {len(chamadores)} chamadores, {len(faltam)} ainda originais')


if __name__ == '__main__':
    main()
