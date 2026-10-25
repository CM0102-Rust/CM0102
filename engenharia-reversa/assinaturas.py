"""Assinaturas das rotinas desviadas: a DLL só desvia uma rotina cuja faixa inteira em memória seja a do
exe original ou a do cm0102-1080.exe (a versão de trabalho). Numa rotina alterada por outro patch
(NickPatcher, CLZ...) o código do exe fica rodando ("preferência ao patch").

Uso:
  python assinaturas.py                 regrava rust/src/assinaturas.rs (rodar a cada leva, depois do motor.rs)
  python assinaturas.py --conferir EXE  confere a tabela atual contra outro exe, sem rodar o jogo

Faixa = do início da rotina (tabela PROLOGOS do motor.rs) até a próxima fronteira real (fronteiras.py).
Hash = FNV-1a 64 dos bytes da faixa. Confere também que nenhum outro desvio da DLL (as portas da
comparação, a simulação, data.rs...) cai dentro de uma faixa: ele mudaria os bytes antes da conferência."""
import glob, os, re, sys
import pefile
from fronteiras import calcular, R

FNV0, FNVP = 0xcbf29ce484222325, 0x100000001b3


def fnv(b):
    h = FNV0
    for x in b:
        h = ((h ^ x) * FNVP) & 0xffffffffffffffff
    return h


def leitor(caminho):
    """ler(va, n) pelas seções do PE (vale para exes com seções a mais, como o .nick do NickPatcher)."""
    pe = pefile.PE(caminho, fast_load=True)
    dados = open(caminho, 'rb').read()
    base = pe.OPTIONAL_HEADER.ImageBase
    secs = [(base + s.VirtualAddress, max(s.Misc_VirtualSize, s.SizeOfRawData), s.PointerToRawData) for s in pe.sections]

    def ler(va, n):
        for v, tam, raw in secs:
            if v <= va and va + n <= v + tam:
                return dados[raw + va - v:raw + va - v + n]
        raise ValueError('%s: faixa %#x+%d fora das secoes' % (caminho, va, n))
    return ler


def prologos():
    m = open(os.path.join(R, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
    i = m.index('const PROLOGOS:')
    return sorted({int(x, 16) for x in re.findall(r'endereco:\s*0x([0-9a-f]+)', m[i:m.index('\n];', i)])})


def tabela():
    c = calcular()
    ends = prologos()
    # fronteiras reais do fronteiras.py mais as rotinas portadas que o mapa_funcoes.csv não lista (31 na 233ª)
    inicios = sorted(set(c['inicios']) | set(ends))
    fim = {a: (inicios[k + 1] if k + 1 < len(inicios) else a + c['ext'].get(a, 0x100)) for k, a in enumerate(inicios)}
    faixas = [(a, fim[a] - a) for a in ends]
    # outros desvios da DLL dentro de uma faixa mudariam os bytes antes da conferencia
    outros = set()
    for f in glob.glob(os.path.join(R, 'rust', 'src', '*.rs')):
        outros |= {int(x, 16) for x in re.findall(r'endereco:\s*0x([0-9a-f]+)', open(f, encoding='utf-8').read())}
    for a, n in faixas:
        dentro = [hex(o) for o in outros if a < o < a + n]
        assert not dentro, 'desvio %s dentro da faixa de %#x (+%d)' % (dentro, a, n)
    orig = leitor(os.path.join(R, 'jogo', 'cm0102.exe'))
    r1080 = leitor(os.path.join(R, 'jogo', 'cm0102-1080.exe'))
    return [(a, n, fnv(orig(a, n)), fnv(r1080(a, n))) for a, n in faixas]


def gravar():
    t = tabela()
    linhas = ['//! GERADO por `python engenharia-reversa/assinaturas.py` (não editar): rodar a cada leva, depois de',
              '//! mudar a tabela PROLOGOS do motor.rs. (início, tamanho até a próxima fronteira real, FNV-1a 64 no',
              '//! cm0102.exe original, FNV-1a 64 no cm0102-1080.exe). Ordenada pelo início (busca binária).',
              '',
              'pub const ASSINATURAS: [(usize, usize, u64, u64); %d] = [' % len(t)]
    linhas += ['    (%#x, %d, %#018x, %#018x),' % x for x in t]
    linhas += ['];', '']
    open(os.path.join(R, 'rust', 'src', 'assinaturas.rs'), 'w', encoding='utf-8', newline='\n').write('\n'.join(linhas))
    print('rust/src/assinaturas.rs: %d rotinas, %d bytes, %d diferentes no 1080' % (
        len(t), sum(x[1] for x in t), sum(x[2] != x[3] for x in t)))


def conferir(exe):
    m = open(os.path.join(R, 'rust', 'src', 'assinaturas.rs'), encoding='utf-8').read()
    t = [(int(a, 16), int(n), int(h0, 16), int(h1, 16)) for a, n, h0, h1 in
         re.findall(r'\((0x[0-9a-f]+), (\d+), (0x[0-9a-f]+), (0x[0-9a-f]+)\)', m)]
    ler = leitor(exe)
    ficam = [(a, n) for a, n, h0, h1 in t if fnv(ler(a, n)) not in (h0, h1)]
    print('%s: %d conferidas, %d ficam com o exe' % (exe, len(t), len(ficam)))
    for a, n in ficam:
        print('  %#x (%d bytes)' % (a, n))
    return ficam


if __name__ == '__main__':
    assert fnv(b'') == FNV0 and fnv(b'a') == 0xaf63dc4c8601ec8c  # vetor conhecido do FNV-1a 64
    if '--conferir' in sys.argv:
        conferir(sys.argv[sys.argv.index('--conferir') + 1])
    else:
        gravar()
