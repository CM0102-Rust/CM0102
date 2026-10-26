"""Gera rust/src/adjetivos_nacao.rs: as tabelas da nation_get_nationality_adjective (0x6181d0, index.cpp) lidas do
cm0102.exe com o capstone.

Uso: python adjetivos_nacao.py

A rotina (0x6181d0..0x62a67a, 74.922 bytes; o mapa de funções corta em 26.014) é quase toda tabela: nos idiomas 9, 4,
5, 10 e 6 ([0x9eb2dc]) uma série de cadeias `if (id == chave) switch (forma) strcpy(texto, "...")` com 937 nações, a
forma vinda da 0x545a60(gênero, idioma). O gerador executa a rotina simbolicamente, instrução por instrução, por
todos os caminhos (registradores com valor simbólico: a nação, o id, a forma, o texto, o idioma; a pilha pela
profundidade), e falha no imprevisto (instrução fora do conjunto conhecido, comparação de registro sem a forma, cópia
fora do padrão `repne scasb`/`rep movsd`/`rep movsb`, laço, instrução nunca alcançada). O grafo resultante é reduzido
(nós iguais fundidos) e lido como cadeias; uma conferência exaustiva compara o grafo com as tabelas geradas. Os
textos não são copiados: vai o endereço deles no exe. Confere também que o cm0102-1080.exe tem a rotina igual.
O arquivo gerado entra no motor.rs pelo `include!("adjetivos_nacao.rs")` do código da leva."""
import os
import sys
import threading
from capstone import Cs, CS_ARCH_X86, CS_MODE_32

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INI, FIM = 0x6181d0, 0x62a67a
ORIG = open(os.path.join(R, 'jogo', 'cm0102.exe'), 'rb').read()
R1080 = open(os.path.join(R, 'jogo', 'cm0102-1080.exe'), 'rb').read()
assert ORIG[INI - 0x400000:FIM - 0x400000] == R1080[INI - 0x400000:FIM - 0x400000], 'a rotina mudou no 1080'
md = Cs(CS_ARCH_X86, CS_MODE_32)
I = {}
for i in md.disasm(ORIG[INI - 0x400000:FIM - 0x400000], INI):
    mn, op = i.mnemonic, i.op_str
    if mn.startswith('rep'):  # o capstone dá "rep movsd" como mnemônico
        mn, _, resto = (mn + ' ' + op).partition(' ')
        op = resto
    I[i.address] = (mn, op, i.address + i.size)
assert max(I) + 1 == FIM and I[max(I)][:2] == ('ret', ''), 'desmontagem cortada'
REGS = ('eax', 'ebx', 'ecx', 'edx', 'esi', 'edi', 'ebp')
ARGS = {4: 'nac', 8: 'buf', 0xc: 'len', 0x10: 'gen'}  # deslocamento a partir de esp na entrada
nos, grafo, vistos = {}, [], set()


def no(*t):
    grafo.append(t)
    return len(grafo) - 1


def andar(a, st):
    chave = (a, tuple(sorted(st.items())))
    if chave in nos:
        assert nos[chave] is not None, ('laço', hex(a))
        return nos[chave]
    nos[chave] = None
    r = passo(a, dict(st))
    nos[chave] = r
    return r


def mem(op):
    assert op.startswith('dword ptr [0x') and op.endswith(']'), op
    return int(op[11:-1], 16)


def arg(op, dep):
    assert op.startswith('dword ptr [esp + '), op
    return ARGS[int(op[len('dword ptr [esp + '):-1], 16) - dep]


def passo(a, st):
    """Executa a partir de `a` até um desvio condicional, uma cópia completa ou o ret; devolve o nó."""
    while True:
        vistos.add(a)
        mn, op, nx = I[a]
        dep = st['dep']
        if (mn, op) == ('mov', 'eax, dword ptr [esp + 4]') and dep == 0:
            st['eax'] = 'nac'
        elif mn == 'push' and op in ('ebp', 'ebx', 'esi', 'edi', 'eax'):
            st['dep'] = dep + 4
            st['pilha'] = st['pilha'] + (st[op] if op in ('eax', 'ebx') else op,)
        elif mn == 'mov' and op[:5] in ('ebp, ', 'ebx, ', 'ecx, ', 'edx, ', 'eax, ') and op[5:].startswith('dword ptr [esp + '):
            st[op[:3]] = arg(op[5:], dep)
            assert op[:3] != 'ebp' or st['ebp'] == 'buf'
        elif (mn, op) == ('mov', 'byte ptr [ebp], 0'):
            assert st['ebp'] == 'buf' and dep == 4, hex(a)  # o zero no texto, antes de testar a nação
            st['zero'] = 1
        elif mn == 'test' and op in ('eax, eax', 'ebp, ebp'):
            st['f'] = ('nulo', st[op[:3]])
        elif (mn, op) == ('cmp', 'word ptr [esp + 0x10], 0x32'):
            assert arg('dword ptr [esp + 0x10]', dep) == 'len'
            st['f'] = ('len50',)
        elif mn == 'test' and op in ('bl, bl', 'dl, dl'):
            st['f'] = ('b', st['e' + op[0] + 'x'], 0)
        elif mn == 'cmp' and op[:4] in ('dl, ', 'bl, '):
            st['f'] = ('b', st['e' + op[0] + 'x'], int(op[4:], 16))
        elif mn == 'cmp' and op.startswith('eax, dword ptr [0x'):
            assert st['eax'] == 'id', (hex(a), st)
            st['f'] = ('id', mem(op[5:]))
        elif (mn, op) == ('cmp', 'eax, ecx'):
            assert st['eax'] == 'id' and st['ecx'][0] == 'G', (hex(a), st)
            st['f'] = ('id', st['ecx'][1])
        elif mn == 'cmp' and op.startswith('eax, ') and st['eax'] == 'lang':
            st['f'] = ('lang', int(op[5:], 16))
        elif (mn, op) == ('cmp', 'byte ptr [ebp], 0'):
            assert 'pendente' not in st and st['ebp'] == 'buf', hex(a)
            st['f'] = ('vazio',)
        elif mn == 'mov' and op.startswith('ecx, dword ptr [0x'):
            st['ecx'] = ('G', mem(op[5:]))
        elif mn == 'mov' and op[5:] in ('dword ptr [ecx]', 'dword ptr [edx]', 'dword ptr [eax]'):
            assert st[op[16:19]] == 'nac', (hex(a), st)
            st[op[:3]] = 'id'
        elif mn == 'mov' and op.startswith('edi, 0x'):
            t = int(op[5:], 16)
            assert 0x967000 <= t < 0xde5000 and b'\0' in ORIG[t - 0x400000:t - 0x400000 + 256], ('texto', hex(a))
            st['edi'] = ('T', t)
        elif mn == 'lea' and op.startswith('edi, [') and op.endswith(' + 0x57]'):
            assert st[op[6:9]] == 'nac', (hex(a), st)
            st['edi'] = 'nome'
        elif (mn, op) == ('call', '0x66f4d0'):  # o idioma: mov eax, [0x9eb2dc]
            st['eax'] = 'lang'
        elif (mn, op) == ('call', '0x545a60'):  # a forma: (gênero, idioma), cdecl
            assert st['pilha'][-2:] == ('lang', 'gen'), (hex(a), st['pilha'])
            st['eax'] = 'g0'
            st['ecx'] = st['edx'] = 'lixo'
        elif (mn, op) == ('add', 'esp, 8'):
            st['dep'] = dep - 8
            st['pilha'] = st['pilha'][:-2]
        elif mn == 'mov' and op in ('dl, al', 'bl, al'):
            assert st['eax'] == 'g0'
            st['e' + op[0] + 'x'] = 'g'
        # a cópia (strcpy edi -> ebp), uma instrução de cada vez: o padrão tem de fechar
        elif (mn, op) == ('or', 'ecx, 0xffffffff'):
            st['ecx'] = 'm1'
        elif (mn, op) == ('repne', 'scasb al, byte ptr es:[edi]'):
            src = st['edi']
            assert st['eax'] == 'k0' and st['ecx'] == 'm1' and (src == 'nome' or src[0] == 'T'), (hex(a), st)
            st['edi'], st['ecx'] = ('fim', src), ('nlen', src)
        elif (mn, op) == ('not', 'ecx'):
            assert st['ecx'][0] == 'nlen', hex(a)
            st['ecx'] = ('len', st['ecx'][1])
        elif (mn, op) == ('sub', 'edi, ecx'):
            assert st['edi'][0] == 'fim' and st['ecx'] == ('len', st['edi'][1]), hex(a)
            st['edi'] = st['edi'][1]
        elif mn == 'mov' and len(op) == 8 and op[3:5] == ', ' and op[:3] in REGS and op[5:] in REGS:
            assert op[:3] != 'ebp', hex(a)
            st[op[:3]] = st[op[5:]]
        elif (mn, op) == ('shr', 'ecx, 2'):
            assert st['ecx'][0] == 'len', hex(a)
            st['ecx'] = ('len4', st['ecx'][1])
        elif (mn, op) == ('rep', 'movsd dword ptr es:[edi], dword ptr [esi]'):
            src = st['esi']
            assert st['edi'] == 'buf' and st['ecx'] == ('len4', src), (hex(a), st)
            st['edi'], st['esi'], st['ecx'] = ('meio', src), ('meio', src), 'lixo'
            st['pendente'] = src
        elif (mn, op) == ('and', 'ecx, 3'):
            assert st['ecx'][0] == 'len', hex(a)
            st['ecx'] = ('len3', st['ecx'][1])
        elif (mn, op) == ('rep', 'movsb byte ptr es:[edi], byte ptr [esi]'):
            src = st.pop('pendente')
            assert st['edi'] == st['esi'] == ('meio', src) and st['ecx'] == ('len3', src), (hex(a), st)
            st['edi'] = st['esi'] = st['ecx'] = 'lixo'
            return no('copia', src, andar(nx, st))
        elif mn == 'jmp':
            a = int(op, 16)
            continue
        elif mn in ('jne', 'je', 'jl'):
            f = st.pop('f')
            alvo = int(op, 16)
            # o primeiro filho é o da condição: igual (jne/je), nulo (je de test), tamanho >= 50 (jl)
            sim, nao = (alvo, nx) if mn == 'je' else (nx, alvo)
            return no('se', f, andar(sim, st), andar(nao, st))
        elif (mn, op) == ('xor', 'eax, eax'):
            st['eax'] = 'k0'
        elif (mn, op) == ('mov', 'eax, 1'):
            st['eax'] = 'k1'
        elif mn == 'pop' and op in REGS:
            st['dep'] = dep - 4
            st['pilha'] = st['pilha'][:-1]
        elif (mn, op) == ('ret', ''):
            assert dep == 0 and 'pendente' not in st and st['eax'] in ('k0', 'k1'), hex(a)
            return no('fim', int(st['eax'][1]))
        else:
            raise AssertionError(('imprevisto', hex(a), mn, op))
        a = nx


def reduzir(raiz):
    """Funde os nós iguais (o mesmo teste e os mesmos filhos); teste com os dois filhos iguais some."""
    unicos, mapa, N = {}, {}, []

    def canon(i):
        if i in mapa:
            return mapa[i]
        t = grafo[i]
        if t[0] == 'se':
            k = ('se', t[1], canon(t[2]), canon(t[3]))
            if k[2] == k[3]:
                mapa[i] = k[2]
                return k[2]
        elif t[0] == 'copia':
            k = ('copia', t[1], canon(t[2]))
        else:
            k = t
        if k not in unicos:
            unicos[k] = len(N)
            N.append(k)
        mapa[i] = unicos[k]
        return mapa[i]
    return canon(raiz), N


def cadeias(raiz, N):
    """O grafo lido como: começo fixo, idiomas e, em cada um, cadeias de (chave, fonte por forma 0..255)."""
    def se(n, cond):
        assert N[n][0] == 'se' and N[n][1] == cond, (N[n], cond)
        return N[n][2], N[n][3]
    fim0, fim1 = N.index(('fim', 0)), N.index(('fim', 1))
    n, s = se(raiz, ('nulo', 'nac'))
    assert n == fim0
    n, s = se(s, ('nulo', 'buf'))
    assert n == fim0
    s, n = se(s, ('len50',))
    assert n == fim0
    nome_fim, s = se(s, ('b', 'gen', 0))
    assert N[nome_fim] == ('copia', 'nome', fim1)
    idiomas = []
    while N[s][0] == 'se' and N[s][1][0] == 'lang':
        idiomas.append((N[s][1][1], N[s][2]))
        s = N[s][3]
    assert s == nome_fim, 'idioma sem tabela: o nome'

    def forma(n):
        r = []
        for g in range(256):
            m = n
            while N[m][0] == 'se' and N[m][1][0] == 'b':
                assert N[m][1][1] == 'g'
                m = N[m][2] if g == N[m][1][2] else N[m][3]
            r.append((N[m][1], N[m][2]) if N[m][0] == 'copia' else (None, m))
        return r
    tabela = []
    for L, n in idiomas:
        cs = []
        while N[n][0] == 'se' and N[n][1][0] == 'id':
            cad, fim = [], None
            while True:
                assert N[n][0] == 'se' and N[n][1][0] == 'id', N[n]
                sw = forma(N[n][2])
                conts = {c for _, c in sw}
                assert len(conts) == 1 and fim in (None, *conts), ('cadeia com duas saídas', L)
                fim = conts.pop()
                assert all(sw[g] == sw[5] for g in range(5, 256)), ('forma >= 5 diferente', L)
                cad.append((N[n][1][1], [x for x, _ in sw[:6]]))
                n = N[n][3]
                if n == fim:
                    break
            cs.append(cad)
        if n == fim1:
            vazio = False  # idiomas 4 e 5: sem o nome no fim
        else:
            assert N[n] == ('se', ('vazio',), nome_fim, fim1), N[n]
            vazio = True
        tabela.append((L, vazio, cs))
    return tabela


def conferir(raiz, N, tabela):
    """Conferência exaustiva: grafo × tabelas, para cada idioma, cada chave igual ao id (ou nenhuma) e cada forma."""
    def pelo_grafo(L, igual, g):
        out, n = [], raiz
        while N[n][0] != 'fim':
            t = N[n]
            if t[0] == 'copia':
                out.append(t[1])
                n = t[2]
                continue
            c = t[1]
            v = {'nulo': False, 'len50': True, 'vazio': not out or out[-1] == ''}.get(c[0])
            if c[0] == 'b':
                v = (1 if c[1] == 'gen' else g) == c[2]
            elif c[0] == 'lang':
                v = L == c[1]
            elif c[0] == 'id':
                v = c[1] == igual
            n = t[2] if v else t[3]
        return out
    for L, vazio, cs in tabela:
        chaves = {G for cad in cs for G, _ in cad}
        for igual in sorted(chaves) + [None]:
            for g in list(range(8)) + [255]:
                out = []
                for cad in cs:
                    for G, sw in cad:
                        if G == igual:
                            if sw[min(g, 5)] is not None:
                                out.append(sw[min(g, 5)])
                            break
                if vazio and not out:
                    out.append('nome')
                assert pelo_grafo(L, igual, g) == out, (L, igual, g)
    assert pelo_grafo(0, None, 0) == ['nome']  # idioma sem tabela (o inglês)


def fonte(x):
    return '0' if x is None else '1' if x == 'nome' else '%#x' % x[1]


def main():
    sys.setrecursionlimit(200000)
    threading.stack_size(256 * 1024 * 1024 - 4096)
    res = {}
    t = threading.Thread(target=lambda: res.update(r=andar(INI, {'dep': 0, 'eax': '?', 'ecx': '?', 'edx': '?', 'ebx': '?',
                                                                    'edi': '?', 'esi': '?', 'ebp': '?', 'pilha': ()})))
    t.start()
    t.join()
    assert 'r' in res, 'o passeio falhou'
    sobra = [hex(a) for a in I if a not in vistos and I[a][0] != 'nop']
    assert not sobra, ('instruções nunca alcançadas', sobra[:10])
    raiz, N = reduzir(res['r'])
    tabela = cadeias(raiz, N)
    conferir(raiz, N, tabela)
    total = sum(len(c) for _, _, cs in tabela for c in cs)
    L = ['// GERADO por `python engenharia-reversa/adjetivos_nacao.py` (não editar): as tabelas da',
         '// nation_get_nationality_adjective (0x6181d0, index.cpp) lidas do cm0102.exe por todos os caminhos e conferidas',
         '// (grafo × tabelas); regenerar se o gerador mudar. Os textos são endereços no exe (não copiados). Entra no',
         '// motor.rs pelo `include!("adjetivos_nacao.rs")` do código da leva.',
         '',
         '/// Uma nação de uma cadeia: a global da chave e a fonte para a forma 0, 1, 2, 3, 4 e as outras (0: nada,',
         '/// 1: o nome da nação em +0x57, senão o endereço do texto).',
         'type EntradaDbx = (u32, [u32; 6]);',
         '']
    nomes = []
    for Lg, vazio, cs in tabela:
        for k, cad in enumerate(cs):
            nome = 'ADJETIVOS_%d_%d' % (Lg, k)
            nomes.append(nome)
            L.append('const %s: [EntradaDbx; %d] = [' % (nome, len(cad)))
            L += ['    (%#x, [%s]),' % (G, ', '.join(fonte(x) for x in sw)) for G, sw in cad]
            L.append('];')
    L += ['',
          '/// Os idiomas (0x66f4d0) na ordem da original: (idioma, o nome da nação se o texto ficar vazio, as cadeias).',
          '/// Em cada cadeia vale a primeira chave igual ao id; depois segue para a cadeia seguinte.',
          'const ADJETIVOS_DA_NACAO: [(i32, bool, &[&[EntradaDbx]]); %d] = [' % len(tabela)]
    for Lg, vazio, cs in tabela:
        L.append('    (%d, %s, &[%s]),' % (Lg, 'true' if vazio else 'false', ', '.join('&ADJETIVOS_%d_%d' % (Lg, k) for k in range(len(cs)))))
    L.append('];')
    open(os.path.join(R, 'rust', 'src', 'adjetivos_nacao.rs'), 'w', encoding='utf-8', newline='\n').write('\n'.join(L) + '\n')
    print('ok: %d idiomas, %d nações nas cadeias, %d nós (%d reduzidos)' % (len(tabela), total, len(grafo), len(N)),
          [(Lg, vazio, [len(c) for c in cs]) for Lg, vazio, cs in tabela])


if __name__ == '__main__':
    main()
