r"""Dá nome aos endereços de função do exe passados aos auxiliares de chamada (`chamar(0x6e1090, ..)`,
`call_thiscall1(0x775ba0, ..)`, `call_recording(..)`): o endereço vira a constante FN_ do mapa de maxevans60 (as que
faltam entram no fim do exe_fns.rs). Só nesses auxiliares (um número que por acaso coincidisse com o começo de uma
função em outro contexto fica como está). Etapa 4 da reorganização, 08/11/2026, achado do leitor de fora, rodada 3.

Uso: python nomear_funcoes_chamadas.py [--gravar]
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import nomear_funcoes as nf  # noqa: E402
import trocar_nas_mutacoes  # noqa: E402

CHAMADA = re.compile(r'\b(chamar|call_recording|call_thiscall\d|call_cdecl\d?|call_stdcall\d?)\((0x[4-9][0-9a-f]{5})\b')


def gravar(p, t):
    """Grava tentando de novo (o Windows às vezes trava o arquivo por um instante: Errno 22)."""
    import time
    for k in range(10):
        try:
            open(p, 'w', encoding='utf-8').write(t)
            return
        except OSError:
            time.sleep(0.5 * (k + 1))
    open(p, 'w', encoding='utf-8').write(t)


def main():
    pe = os.path.join(SRC, 'motor', 'exe_fns.rs')
    exe = open(pe, encoding='utf-8').read()
    ja = {int(a, 16): n for n, a in re.findall(r'const (FN_\w+): usize = (0x[0-9a-f]+);', exe)}
    usados_nomes = set(ja.values())
    mapa = nf.mapa()
    nome = {}

    def nome_de(a):
        if a in ja:
            return ja[a]
        if a in nome:
            return nome[a][0]
        if a in nf.CRT:
            n, s = 'FN_CRT_' + nf.CRT[a].upper(), 'C runtime: %s' % nf.CRT[a]
        elif a in mapa:
            n, s = nf.nome_de(mapa[a][0]), mapa[a][0]
        else:
            return None  # sem nome no mapa: fica o número
        if n in usados_nomes:
            n = '%s_%X' % (n, a)
        usados_nomes.add(n)
        nome[a] = (n, s)
        return n

    def trocar_linha(l):
        cod, sep, com = l.partition('//')
        if cod.lstrip().startswith('///'):
            return l
        return CHAMADA.sub(lambda m: '%s(%s' % (m.group(1), nome_de(int(m.group(2), 16)) or m.group(2)), cod) + sep + com

    n = 0
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if f.endswith('.rs') and f != 'exe_fns.rs':
                p = os.path.join(d, f)
                t = open(p, encoding='utf-8').read()
                novo = '\n'.join(trocar_linha(l) for l in t.split('\n'))
                n += sum(a != b for a, b in zip(t.split('\n'), novo.split('\n')))
                if novo != t and '--gravar' in sys.argv:
                    gravar(p, novo)
    print('%d linhas; %d funções novas no exe_fns.rs' % (n, len(nome)))
    if '--gravar' not in sys.argv:
        return
    trocar_nas_mutacoes.aplicar(trocar_linha)
    # as FN_ que o código já usa e que ainda não estão definidas (uma rodada interrompida): o endereço pelo nome
    codigo = '\n'.join(open(os.path.join(d, f), encoding='utf-8').read() for d, _, fs in os.walk(os.path.join(SRC, 'motor'))
                       for f in fs if f.endswith('.rs') and f != 'exe_fns.rs')
    definidas = set(re.findall(r'const (FN_\w+):', exe)) | {v[0] for v in nome.values()}
    faltam = set(re.findall(r'\bFN_\w+\b', codigo)) - definidas
    if faltam:
        reverso = {}
        for a in list(mapa) + list(nf.CRT):
            n0 = ('FN_CRT_' + nf.CRT[a].upper()) if a in nf.CRT else nf.nome_de(mapa[a][0])
            reverso.setdefault(n0, a)
            reverso.setdefault('%s_%X' % (n0, a), a)
        for n_ in sorted(faltam):
            if n_ in reverso:
                a = reverso[n_]
                nome[a] = (n_, ('C runtime: %s' % nf.CRT[a]) if a in nf.CRT else mapa[a][0])
        print('recuperadas:', len([x for x in faltam if x in reverso]), 'sem endereço:', sorted(x for x in faltam if x not in reverso)[:5])
    corpo = ''.join('pub const %s: usize = %#x; // %s\n' % (n_, a, s) for a, (n_, s) in sorted(nome.items()))
    gravar(pe, exe.rstrip('\n') + '\n' + corpo)
    print('gravado')


if __name__ == '__main__':
    main()
