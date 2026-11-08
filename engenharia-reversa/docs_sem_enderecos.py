r"""Os endereços de rotina nos docs em inglês viram nomes (etapa 4 da reorganização, 08/11/2026; o leitor de fora,
rodadas 6 a 8: ~24.600 hex nas linhas `///` em inglês). Numa linha `///` em inglês (não no parágrafo `[pt]` nem na
linha `Original:`), um `0xADDR` que é o endereço de uma rotina portada vira `` `nome_em_rust` ``; o de uma função do exe
com invólucro vira `` `exe::nome` ``. O endereço continua no `Original:` de cada rotina e no [pt]. É só comentário.

Uso: python docs_sem_enderecos.py [--gravar]
"""
import glob
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')


def mapas():
    rot = {}
    for p in glob.glob(os.path.join(SRC, 'motor', '**', '*.rs'), recursive=True):
        if os.sep + 'compare' + os.sep in p:
            continue
        t = open(p, encoding='utf-8').read()
        for m in re.finditer(r'^/// Original: [^(\n]*\((0x[0-9a-f]+),[^\n]*\n(?:///[^\n]*\n|#\[[^\n]*\n)*(?:pub\(crate\) )?(?:unsafe )?(?:extern "\w+" )?fn (\w+)', t, re.M):
            rot.setdefault(int(m.group(1), 16), m.group(2))
    exe = {}
    calls = os.path.join(SRC, 'motor', 'exe_calls.rs')
    fns = {}
    for n, v in re.findall(r'const (FN_\w+): usize = (0x[0-9a-f]+);', open(os.path.join(SRC, 'motor', 'exe_fns.rs'), encoding='utf-8').read()):
        fns[n] = int(v, 16)
    if os.path.exists(calls):
        for nome, fn in re.findall(r'pub\(crate\) unsafe fn (\w+)\([^\n]*\n\s*let f: [^\n]*transmute\((FN_\w+)\)', open(calls, encoding='utf-8').read()):
            if fn in fns:
                exe.setdefault(fns[fn], 'exe::' + nome)
    return rot, exe


def campos_do_motor():
    """No motor de partidas, um `+0xNN` acima de 0x1be (o tamanho do jogador da partida) só pode ser do motor."""
    sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
    import aplicar_tipos_variaveis as a
    return {v: n for v, n in a.modulos_de_campos()['engine'].items() if v > 0x1be}


def main():
    rot, exe = mapas()
    motor = campos_do_motor()
    print('%d rotinas portadas, %d funções do exe com invólucro' % (len(rot), len(exe)))
    total, n_arq = 0, 0
    for p in glob.glob(os.path.join(SRC, '**', '*.rs'), recursive=True):
        t = open(p, encoding='utf-8', newline='').read()
        nl = '\r\n' if '\r\n' in t else '\n'
        L = t.split(nl)
        em_pt = False
        mudou = False
        for i, l in enumerate(L):
            s = l.lstrip()
            if not s.startswith('///'):
                em_pt = False
                continue
            corpo = s[3:].lstrip()
            if corpo.startswith('[pt]'):
                em_pt = True
            if em_pt or corpo.startswith('Original:'):
                continue

            def r(m):
                v = int(m.group(1), 16)
                nome = rot.get(v) or exe.get(v)
                if not nome:
                    return m.group(0)
                return '`%s`' % nome
            # not inside backticks already; a bare 0x6-hex address
            partes = re.split(r'(`[^`]*`)', l)
            nova = ''.join(x if x.startswith('`') else re.sub(r'(?<![\w+])0x([0-9a-f]{6})\b(?!\s*=)', r, x) for x in partes)
            if re.search(r'[\\/]match_\w+\.rs$', p):
                partes = re.split(r'(`[^`]*`)', nova)
                nova = ''.join(x if x.startswith('`') else re.sub(
                    r'\+0x([0-9a-f]+)\b', lambda m: '`engine::%s`' % motor[int(m.group(1), 16)] if int(m.group(1), 16) in motor else m.group(0), x)
                    for x in partes)
            if nova != l:
                total += len(re.findall(r'0x[0-9a-f]{6}', l)) - len(re.findall(r'0x[0-9a-f]{6}', nova))
                L[i] = nova
                mudou = True
        if mudou:
            n_arq += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(nl.join(L))
    print('%d endereços trocados em %d arquivos' % (total, n_arq))


if __name__ == '__main__':
    main()
