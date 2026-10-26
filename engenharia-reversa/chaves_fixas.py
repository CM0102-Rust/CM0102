"""Gera rust/src/chaves_fixas.rs: as listas da set_key_ids (0x601a40, index.cpp) lidas do cm0102.exe com o capstone.

Uso: python chaves_fixas.py

A rotina é quase toda lista: o -1 em 1.282 globais, a cadeia das 34 cores, as 6 dos continentes (duas vezes, iguais) e
o relato de 1.264 chaves sem valor. O gerador anda pela rotina inteira conferindo cada padrão e falha no imprevisto
(assert); os textos dos nomes não são copiados: vai o endereço deles no exe, e a cópia Rust passa o endereço às rotinas
do jogo. Confere também a entrada do cm0102-1080.exe (0x603834: o mesmo -1 com rep stosd nas faixas inteiras, a
chamada de 0x52d8fa desviada para ela e o resto da rotina igual ao original a partir de 0x603864).
O arquivo gerado entra no motor.rs pelo `include!("chaves_fixas.rs")` do código da leva."""
import os
from capstone import Cs, CS_ARCH_X86, CS_MODE_32

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INICIO, EMENDA, ENTRADA_1080 = 0x601a40, 0x603864, 0x603834
md = Cs(CS_ARCH_X86, CS_MODE_32)


def ler_exe(nome):
    return open(os.path.join(R, 'jogo', nome), 'rb').read()


ORIG, R1080 = ler_exe('cm0102.exe'), ler_exe('cm0102-1080.exe')


def instrucoes(d, a, fim):
    r = list(md.disasm(d[a - 0x400000:fim - 0x400000], a))
    assert r and r[-1].address + r[-1].size == fim, ('desmontagem cortada', hex(a))
    return [(i.address, i.mnemonic, i.op_str) for i in r]


def texto(op):
    """O endereço de um `push 0x...` de texto: dentro do .data/.rdata e terminado em NUL (o texto não é copiado)."""
    a = int(op, 16)
    assert 0x967000 <= a < 0xde5000, ('texto fora do .rdata/.data', op)
    assert b'\0' in ORIG[a - 0x400000:a - 0x400000 + 256], ('texto sem fim', op)
    return a


def mem(op):
    """`dword ptr [0x...]` → o endereço."""
    assert op.startswith('dword ptr [0x') and op.endswith(']'), op
    return int(op[len('dword ptr ['):-1], 16)


def cadeia(I, k, reg):
    """Uma cadeia de _stricmp em linha a partir de I[k]: devolve ([(texto, global)], k depois do fim)."""
    pares, fim = [], None
    while True:
        a, mn, op = I[k]
        if fim is not None and a == fim:
            return pares, k
        assert mn == 'push' and op.startswith('0x'), ('cadeia: esperava push do texto', hex(a), mn, op)
        t = texto(op)
        k += 1
        if I[k][1] == 'lea':  # o primeiro item calcula o nome
            assert I[k][2].startswith('edi, ['), I[k]
            k += 1
        assert I[k][1:] == ('push', 'edi') and I[k + 1][1:] == ('call', '0x951c50') and I[k + 2][1:] == ('add', 'esp, 8') \
            and I[k + 3][1:] == ('test', 'eax, eax') and I[k + 4][1] == 'jne', ('cadeia', hex(I[k][0]))
        prox = int(I[k + 4][2], 16)
        a5, mn5, op5 = I[k + 5]
        assert mn5 == 'mov' and op5.endswith(', ' + reg), ('cadeia: esperava mov [g], ' + reg, hex(a5), op5)
        pares.append((t, mem(op5.split(', ')[0])))
        k += 6
        if I[k][1] == 'jmp':
            j = int(I[k][2], 16)
            assert fim in (None, j), ('cadeia: saídas diferentes', hex(I[k][0]))
            fim = j
            k += 1
        assert I[k][0] == prox, ('cadeia: o jne não vai ao próximo item', hex(I[k][0]), hex(prox))
        if fim is None or I[k][0] == fim:
            assert fim is not None and pares, 'cadeia sem saída'
            return pares, k


def gerar():
    # o fim: o epílogo (pop edi ... ret) depois do relato
    fim = ORIG.index(bytes.fromhex('5f5e5db8010000005b81c420020000c3'), INICIO - 0x400000) + 0x400000 + 16
    I = instrucoes(ORIG, INICIO, fim)
    assert [x[1:] for x in I[:6]] == [('sub', 'esp, 0x220'), ('push', 'ebx'), ('push', 'ebp'), ('push', 'esi'),
                                      ('push', 'edi'), ('or', 'edi, 0xffffffff')], 'prólogo'
    # 1) o -1: mov [g], edi até a emenda, com o quarteto do começo da cadeia das cores no meio
    menos_um, quarteto, k = [], [], 6
    while I[k][0] < EMENDA:
        a, mn, op = I[k]
        if mn == 'mov' and op.endswith('], edi'):
            menos_um.append(mem(op[:-len(', edi')]))
        else:
            quarteto.append((mn, op))
        k += 1
    assert quarteto == [('mov', 'eax, dword ptr [0xae2394]'), ('xor', 'ebx, ebx'), ('xor', 'esi, esi'), ('cmp', 'eax, ebx')], quarteto
    assert len(menos_um) == len(set(menos_um)) == 1282, len(menos_um)
    # 2) as cores
    assert I[k][1:] == ('jle', '0x603c52') and I[k + 1][1:] == ('xor', 'ebp, ebp') \
        and I[k + 2][1:] == ('mov', 'eax, dword ptr [0xae23ec]'), 'cores: entrada'
    cores, k = cadeia(I, k + 3, 'esi')
    assert [x[1:] for x in I[k:k + 7]] == [('mov', 'eax, dword ptr [0xae2394]'), ('inc', 'esi'), ('add', 'ebp, 0x3a'),
                                           ('cmp', 'esi, eax'), ('jl', '0x60386c'), ('or', 'edi, 0xffffffff'),
                                           ('cmp', 'dword ptr [0x9b5984], ebx')], 'cores: saída'
    assert len(cores) == 34 and [g for _, g in cores] == list(range(0x9cf178, 0x9cf178 + 4 * 34, 4)), 'cores'
    # 3) o meio (eng.lng e idioma 0): as chamadas na ordem, os passos e as globais lidas; as duas cadeias dos continentes
    k0 = k
    while I[k][1:] != ('cmp', 'dword ptr [0x9cf178], edi'):
        k += 1
    meio = I[k0:k]
    conts = []
    for j, (a, mn, op) in enumerate(meio):
        if (mn, op) == ('push', '0x9d9458'):  # "Africa": começo das cadeias dos continentes
            conts.append(cadeia(meio, j, 'ebp' if a < 0x60420d else 'esi')[0])
    assert len(conts) == 2 and conts[0] == conts[1] and len(conts[0]) == 6, 'continentes'
    assert [g for _, g in conts[0]] == list(range(0x9cfa08, 0x9cfa20, 4)), 'continentes: globais'
    chamadas = [int(op, 16) for _, mn, op in meio if mn == 'call' and op != '0x951c50']
    esperadas = [0x90bde0, 0x945b00, 0x90c140, 0x945bfe, 0x944e46, 0x945bfe, 0x6146b0, 0x6146b0, 0x9452ca, 0x946a22,
                 0x944e46, 0x945bfe, 0x9452ca, 0x946a22, 0x944e46, 0x945bfe, 0x60efd0, 0x9452ca, 0x946a22,
                 0x944e46, 0x945bfe, 0x60d810, 0x9452ca, 0x946a22, 0x944e46, 0x946a22, 0x946a22, 0x945bfe, 0x6115e0,
                 0x9452ca, 0x946a22, 0x946a22, 0x946a22, 0x944e46, 0x945bfe, 0x60efd0, 0x9452ca, 0x945a09, 0x946a22,
                 0x945a09, 0x60d810, 0x60efd0, 0x6115e0, 0x6146b0]
    assert chamadas == esperadas, ('meio: chamadas', [hex(x) for x in chamadas])
    lidas = sorted({mem(o) for _, mn, op in meio for o in op.split(', ') if o.startswith('dword ptr [0x')})
    assert lidas == [0x9b5984, 0x9cfa08, 0x9cfa0c, 0x9cfa10, 0x9cfa14, 0x9cfa18, 0x9cfa1c, 0xae234c, 0xae2350,
                     0xae235c, 0xae2378, 0xae2388, 0xae2394, 0xae23a4, 0xae23a8, 0xae23b4, 0xae23d0, 0xae23e0, 0xae23fc], [hex(x) for x in lidas]
    imediatos = [op for _, mn, op in meio if mn in ('push', 'add', 'cmp') and op.split(', ')[-1] in
                 ('0x4f', '0xba', '0x53', '0x6d', '0x20', '0x122', '0x6b', '0xc6', '0x65', '0x245', '-0x7f', '0xf1fffff1')]
    assert imediatos == ['eax, 0xf1fffff1', '0x20', '0x4f', 'ebx, 0x4f', 'ebx, 0x4f', '0xba', 'edi, 0xba', '0x53',
                         'ebp, 0x53', '0x6d', 'ebx, 0x6d', '0x4f', 'ebp, 0x4f', '0x53', 'ebp, 0x53', 'edi, 0x122',
                         'eax, -0x7f', 'eax, -0x7f', 'edi, 0x6b', 'ebp, 0xc6', 'edi, 0x65', 'edi, 0x245'], imediatos
    # 4) o relato: cmp [g], edi / jne próximo / corpo
    relato = []
    while I[k][1] == 'cmp':
        a, mn, op = I[k]
        assert op.endswith('], edi') and I[k + 1][1] == 'jne', ('relato', hex(a))
        g, prox = mem(op[:-len(', edi')]), int(I[k + 1][2], 16)
        j = k + 2
        while I[j][0] < prox:
            j += 1
        assert I[j][0] == prox, ('relato: o jne cai no meio', hex(a))
        corpo = I[k + 2:j]
        chs = [int(o, 16) for _, m_, o in corpo if m_ == 'call']
        if len(corpo) == 3:  # 0: a 0x60d780 (nome)
            assert corpo[0][1] == 'push' and chs == [0x60d780] and corpo[2][1:] == ('add', 'esp, 4'), ('relato P', hex(a))
            relato.append((g, texto(corpo[0][2]), 0, 0, 0))
        elif corpo[1][1:] == ('push', '0x9d93f4'):  # 1: o corpo da 0x60d780 em linha (as cores)
            assert corpo[0][1] == 'push' and chs == [0x944c9f, 0x944cff, 0x90d130, 0x944c9f, 0x5e8290] \
                and ('push', '0x16b1') in [x[1:] for x in corpo] and ('push', '0x9d93cc') in [x[1:] for x in corpo] \
                and corpo[-1][1:] == ('mov', 'dword ptr [0xb67a34], ebx'), ('relato A', hex(a))
            relato.append((g, texto(corpo[0][2]), 1, 0, 0))
        else:  # 2: fatal se o byte +0x11c da nação da chave ≠ 0, senão a 0x60d780
            nacao = mem(corpo[0][2].split(', ')[1])
            assert [x[1:] for x in corpo[:8]] == [
                ('mov', 'eax, dword ptr [%#x]' % nacao), ('lea', 'edx, [eax + eax*8]'), ('shl', 'edx, 4'), ('add', 'edx, eax'),
                ('mov', 'eax, dword ptr [0xae23a8]'), ('mov', 'cl, byte ptr [eax + edx*2 + 0x11c]'), ('test', 'cl, cl'),
                ('je', hex(corpo[-3][0]))], ('relato C', hex(a))
            linhas = [int(x[2], 16) for x in corpo if x[1] == 'push' and len(x[2]) == 5]
            assert chs == [0x944cff, 0x90d130, 0x944c9f, 0x5e8290, 0x5cc7d0, 0x5a8f60, 0x945934, 0x60d780] and len(linhas) == 1 \
                and ('push', '0x9d93cc') in [x[1:] for x in corpo] and ('push', 'edi') in [x[1:] for x in corpo] \
                and corpo[-3][1] == 'push' and corpo[-1][1:] == ('add', 'esp, 4'), ('relato C', hex(a))
            relato.append((g, texto(corpo[-3][2]), 2, nacao, linhas[0]))
        k = j
    assert [x[1:] for x in I[k:]] == [('pop', 'edi'), ('pop', 'esi'), ('pop', 'ebp'), ('mov', 'eax, 1'), ('pop', 'ebx'),
                                      ('add', 'esp, 0x220'), ('ret', '')], 'epílogo'
    tipos = [sum(1 for r in relato if r[2] == t) for t in range(3)]
    assert len(relato) == 1264 and tipos == [1228, 29, 7], (len(relato), tipos)
    # 5) a entrada do 1080
    assert ORIG[0x52d8fa - 0x400000:0x52d8ff - 0x400000] == b'\xe8' + (INICIO - 0x52d8ff).to_bytes(4, 'little', signed=True)
    assert R1080[0x52d8fa - 0x400000:0x52d8ff - 0x400000] == b'\xe8' + (ENTRADA_1080 - 0x52d8ff).to_bytes(4, 'little', signed=True)
    J = instrucoes(R1080, ENTRADA_1080, EMENDA)
    assert [x[1:] for x in J] == [
        ('sub', 'esp, 0x220'), ('push', 'ebx'), ('push', 'ebp'), ('push', 'esi'), ('push', 'edi'), ('push', 'ecx'),
        ('mov', 'ecx, 0x4ec'), ('or', 'eax, 0xffffffff'), ('mov', 'edi, 0x9cf178'), ('rep stosd', 'dword ptr es:[edi], eax'),
        ('push', '0x1a'), ('pop', 'ecx'), ('mov', 'edi, 0xb63c9c'), ('rep stosd', 'dword ptr es:[edi], eax'), ('pop', 'ecx'),
        ('mov', 'eax, dword ptr [0xae2394]'), ('xor', 'ebx, ebx'), ('xor', 'esi, esi'), ('cmp', 'eax, ebx')], ('1080', J)
    faixas = [(int(J[8][2][5:], 16), int(J[6][2][5:], 16)), (int(J[12][2][5:], 16), int(J[10][2], 16))]
    assert R1080[EMENDA - 0x400000:fim - 0x400000] == ORIG[EMENDA - 0x400000:fim - 0x400000], '1080: o resto da rotina mudou'
    cheias = {a + 4 * i for a, n in faixas for i in range(n)}
    assert set(menos_um) <= cheias
    a_mais = sorted(cheias - set(menos_um))
    marca = faixas[-1][0] + 4 * faixas[-1][1]  # o edi depois do último rep stosd
    return menos_um, faixas, a_mais, marca, cores, conts[0], relato


def gravar():
    menos_um, faixas, a_mais, marca, cores, conts, relato = gerar()
    h = lambda x: '%#x' % x
    L = ['// GERADO por `python engenharia-reversa/chaves_fixas.py` (não editar): as listas da set_key_ids (0x601a40, index.cpp)',
         '// lidas do cm0102.exe e conferidas padrão por padrão; regenerar se o gerador mudar. Os nomes são endereços de',
         '// texto no exe (não copiados). Entra no motor.rs pelo `include!("chaves_fixas.rs")` do código da leva.',
         '',
         '/// As globais que a original põe em -1 (`mov [g], edi`), na ordem do código.',
         'const MENOS_UM_DAS_CHAVES: [u32; %d] = [' % len(menos_um)]
    L += ['    ' + ', '.join(h(x) for x in menos_um[i:i + 10]) + ',' for i in range(0, len(menos_um), 10)]
    L += ['];', '',
          '/// O -1 do cm0102-1080.exe (0x603834): rep stosd em (início, dwords); põe também %s.' % ', '.join(h(x) for x in a_mais),
          'const MENOS_UM_DAS_CHAVES_1080: [(u32, u32); %d] = [%s];' % (len(faixas), ', '.join('(%s, %s)' % (h(a), h(n)) for a, n in faixas)),
          '/// O edi do 1080 depois do rep stosd (a marca do relato se nenhum laço rodar; na original é -1).',
          'const MARCA_DAS_CHAVES_1080: u32 = %s;' % h(marca), '',
          '/// A cadeia das cores: (texto, global que recebe a linha), na ordem do código.',
          'const CORES_DAS_CHAVES: [(u32, u32); %d] = [' % len(cores)]
    L += ['    ' + ' '.join('(%s, %s),' % (h(t), h(g)) for t, g in cores[i:i + 6]) for i in range(0, len(cores), 6)]
    L += ['];', '',
          '/// A cadeia dos continentes: (texto, global que recebe a linha), na ordem do código (a mesma nos dois caminhos).',
          'const CONTINENTES_DAS_CHAVES: [(u32, u32); %d] = [%s];' % (len(conts), ' '.join('(%s, %s),' % (h(t), h(g)) for t, g in conts)), '',
          '/// O relato das chaves sem valor: (global, texto do nome, tipo, chave de nação, linha). Tipo 0 = a 0x60d780(nome);',
          '/// 1 = o corpo da 0x60d780 em linha (as cores); 2 = fatal se o byte +0x11c da nação [chave de nação] ≠ 0 (caixa com',
          '/// a linha do index.cpp e exit), senão a 0x60d780(nome).',
          'const RELATO_DAS_CHAVES: [(u32, u32, u8, u32, u16); %d] = [' % len(relato)]
    L += ['    ' + ' '.join('(%s, %s, %d, %s, %s),' % (h(g), h(t), tp, h(n), h(ln)) for g, t, tp, n, ln in relato[i:i + 4])
          for i in range(0, len(relato), 4)]
    L += ['];', '']
    open(os.path.join(R, 'rust', 'src', 'chaves_fixas.rs'), 'w', encoding='utf-8', newline='\n').write('\n'.join(L))
    print('rust/src/chaves_fixas.rs: %d de -1 (1080: %s, +%d), %d cores, %d continentes, relato %d' % (
        len(menos_um), faixas, len(a_mais), len(cores), len(conts), len(relato)))


if __name__ == '__main__':
    gravar()
