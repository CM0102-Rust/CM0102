r"""A fila de levas: levas escritas antes da hora, com os índices calculados só ao integrar.

Cada item fica em levas/fila/<nome>/:
  codigo.rs    as rotinas e os comparadores; os índices de `orig::<..>(N)` e `comparar_valor!(.., N, ..)`
               escritos como %%I0%%, %%I1%%, %%I2%% (na ordem de ROTINAS)
  rotinas.py   ROTINAS = [(nome_rust, endereco, [bytes do prologo])]
               SAVE = True (opcional): item do bloco do save; a leva ganha levas/N/SAVE e a cadeia prova
               pelo save (prova_save.py --leva N, mutacoes.py --prova-save N) em vez da carreira
               CARGA = True (opcional, com SAVE): item de carregar; a leva ganha também levas/N/CARGA e a
               prova do save carrega o FeriasTesteSave com e sem as rotinas (a impressão tem de bater)
               DOCS = (titulo pt, titulo en, [(nome, nome_en, endereco, arquivo, convencao, o que faz pt, en)])
  mutacoes.py  MUTACOES = [(nome, trecho, troca)]  (mesmo formato de mutacoes/levaN.py)
A ordem está em levas/fila/ordem.txt (um nome por linha).

Uso: python fila.py proxima      materializa os próximos da ordem (até ITENS_POR_LEVA do mesmo tipo) como levas/N (N = a próxima leva) com
                                 integrar.py genérico (âncoras achadas no motor.rs), docs.py e mutacoes/levaN.py
     python fila.py checar       aplica a fila inteira numa cópia do motor.rs e confere as âncoras das mutações
O publicar_leva.py chama `proxima` quando levas/N+1 não existe.
"""
import os
import re
import runpy
import shutil
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FILA = os.path.join(R, 'levas', 'fila')

INTEGRAR = r'''"""Integra a {n}a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = {rotinas!r}
BASE = {base}
m = re.search(r'\];\nconst N: usize = (\d+);', s)
assert m and int(m.group(1)) == BASE, ('o motor.rs mudou desde a materialização', m and m.group(1), BASE)
pro = ''.join('    Prologo {{ endereco: %#x, bytes: &[%s] }},\n' % (e, ', '.join('0x%02x' % b for b in bs)) for _, e, bs in ROTINAS)
s = s[:m.start()] + pro + '];\nconst N: usize = %d;' % (BASE + len(ROTINAS)) + s[m.end():]
fins = [x.start() for x in re.finditer(r' as \*const \(\) as usize,\n    \];', s)]
assert len(fins) == 2, fins
for k, prefixo in ((1, 'comparar_'), (0, '')):
    i = fins[k] + len(' as *const () as usize,\n')
    s = s[:i] + ''.join('        %s%s as *const () as usize,\n' % (prefixo, a) for a, _, _ in ROTINAS) + s[i:]
anc = '\n    ];\n    for i in 0..N {{'
assert s.count(anc) == 1
s = s.replace(anc, ' ' + ' '.join('"motor_%s",' % a for a, _, _ in ROTINAS) + anc)
anc = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
assert s.count(anc) == 1
s = s.replace(anc, anc + '\n' + open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read().strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
'''


def ordem():
    p = os.path.join(FILA, 'ordem.txt')
    return [l.strip() for l in open(p, encoding='utf-8') if l.strip() and not l.startswith('#')]


# 21/10/2026: uma leva junta até ITENS_POR_LEVA itens da fila do mesmo tipo (telas ou simulação), 3 rotinas
# cada (9 por leva); os itens seguem escritos de 3 em 3, então um item que dê problema sai da leva sozinho.
# Para voltar às levas de 3 rotinas: ITENS_POR_LEVA = 1.
ITENS_POR_LEVA = 3


def do_lote(ordem_):
    """Os itens da próxima leva: o primeiro da ordem e os seguintes do mesmo tipo (MOTOR, SAVE), até ITENS_POR_LEVA."""
    def tipo(nome):
        rot = runpy.run_path(os.path.join(FILA, nome, 'rotinas.py'))
        return bool(rot.get('MOTOR')), bool(rot.get('SAVE'))
    lote = [ordem_[0]]
    for nome in ordem_[1:]:
        if len(lote) == ITENS_POR_LEVA:
            break
        if tipo(nome) == tipo(lote[0]):
            lote.append(nome)
    return lote


def materializar(nomes, n, base, antes):
    destino = os.path.join(R, 'levas', str(n))
    os.makedirs(destino, exist_ok=False)
    codigos, rotinas, mutacoes, docs, tpts, tens = [], [], [], [], [], []
    campos, interessa = [], []  # LEIA: campos tocados e sessões irmãs interessadas (desde a 215a)
    b = base
    for nome in nomes:
        src = os.path.join(FILA, nome)
        rot = runpy.run_path(os.path.join(src, 'rotinas.py'))
        cod = open(os.path.join(src, 'codigo.rs'), encoding='utf-8').read()
        for k in range(len(rot['ROTINAS'])):
            cod = cod.replace('%%%%I%d%%%%' % k, str(b + k))
        assert '%%I' not in cod, 'índice sem valor'
        codigos.append(cod.replace('%%LEVA%%', str(n)))
        rotinas += [(a, e, list(bs)) for a, e, bs in rot['ROTINAS']]
        mutacoes += runpy.run_path(os.path.join(src, 'mutacoes.py'))['MUTACOES']
        tpt, ten, d = rot['DOCS']
        tpts.append(tpt)
        tens.append(ten)
        docs += d
        campos += rot.get('CAMPOS', [])
        interessa += [t for t in rot.get('INTERESSA', []) if t not in interessa]
        with open(os.path.join(src, 'materializada.txt'), 'w', encoding='utf-8') as f:
            f.write(f'leva {n}, índices {b}..{b + len(rot["ROTINAS"]) - 1}\n')
        b += len(rot['ROTINAS'])
    assert len({m[0] for m in mutacoes}) == len(mutacoes), 'mutações com o mesmo nome em itens diferentes'
    open(os.path.join(destino, 'codigo.rs'), 'w', encoding='utf-8').write('\n'.join(codigos))
    save = any(runpy.run_path(os.path.join(FILA, x, 'rotinas.py')).get('SAVE') for x in nomes)
    if save:  # a marca da cadeia: prova do save em vez da carreira (ver cadeia.sh)
        open(os.path.join(destino, 'SAVE'), 'w', encoding='utf-8').write('leva do bloco do save\n')
    if any(runpy.run_path(os.path.join(FILA, x, 'rotinas.py')).get('CARGA') for x in nomes):  # a prova também pela carga
        open(os.path.join(destino, 'CARGA'), 'w', encoding='utf-8').write('leva de carregar: a prova do save carrega o save de ferias\n')
    open(os.path.join(destino, 'integrar.py'), 'w', encoding='utf-8').write(INTEGRAR.format(n=n, rotinas=rotinas, base=base))
    with open(os.path.join(R, 'engenharia-reversa', 'mutacoes', f'leva{n}.py'), 'w', encoding='utf-8') as f:
        uso = f'--prova-save {n}' if save else '--carreira 2002-06-01'
        f.write(f'# Mutações da {n}a leva (itens {", ".join(nomes)}). Uso: python mutacoes.py mutacoes\\leva{n}.py {uso}\n')
        f.write('MUTACOES = [\n' + ''.join(f'    {m!r},\n' for m in mutacoes) + ']\n')
    runpy.run_path(os.path.join(R, 'engenharia-reversa', 'mutacoes', f'leva{n}.py'))
    secao = "secao=('Motor de partidas', 'Match engine')" if runpy.run_path(os.path.join(FILA, nomes[0], 'rotinas.py')).get('MOTOR') else "secao=('Simulação', 'Simulation')"
    molde = open(os.path.join(R, 'engenharia-reversa', 'molde_docs.py'), encoding='utf-8').read()
    open(os.path.join(destino, 'docs.py'), 'w', encoding='utf-8').write(
        molde.replace('%%N%%', str(n)).replace('%%ANTES%%', str(antes)).replace('%%R%%', repr(docs))
        .replace('%%CAMPOS%%', repr(campos)).replace('%%INTERESSA%%', repr(interessa))
        .replace('%%TPT%%', repr('; '.join(tpts))).replace('%%TEN%%', repr('; '.join(tens))).replace("secao=('Simulação', 'Simulation')", secao))
    linhas = [l for l in open(os.path.join(FILA, 'ordem.txt'), encoding='utf-8') if l.strip() not in nomes]
    open(os.path.join(FILA, 'ordem.txt'), 'w', encoding='utf-8').writelines(linhas)
    print(f'{" + ".join(nomes)} -> levas/{n} ({len(rotinas)} rotinas, índices a partir de {base})')


def estado():
    s = open(os.path.join(R, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
    base = int(re.search(r'\];\nconst N: usize = (\d+);', s).group(1))
    leia = open(os.path.join(R, 'README.md'), encoding='utf-8').read()
    n, total = map(int, re.search(r'\*\*Estado:\*\* (\d+) levas, (\d+) rotinas', leia).groups())
    return n, total, base


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else ''
    if cmd == 'proxima':
        n, total, base = estado()
        while os.path.isdir(os.path.join(R, 'levas', str(n + 1))):
            n += 1  # já materializada (ou escrita à mão) e ainda não publicada
        ordem_ = ordem()
        assert ordem_, 'fila vazia'
        materializar(do_lote(ordem_), n + 1, base, total)
    elif cmd == 'checar':
        import tempfile
        tmp = tempfile.mkdtemp()
        shutil.copytree(os.path.join(R, 'rust', 'src'), os.path.join(tmp, 'rust', 'src'))
        m = open(os.path.join(tmp, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
        base = int(re.search(r'\];\nconst N: usize = (\d+);', m).group(1))
        # o exe de trabalho (1920x1080) é o cm0102.exe com patches (resolucao.py): rotina alterada nele não
        # se porta igual à original (187a: a 0x5e5cd0, altura da fonte, salta para um trecho novo)
        # a rotina vai do começo até o próximo início de função do docs/mapa_funcoes.csv (o mapa às vezes junta
        # funções: a faixa fica maior, nunca menor); 20/10: o y2 0x257 → 0x437 em 0x719346, no meio da 0x718e90
        exes = [open(os.path.join(R, 'jogo', e), 'rb').read() for e in ('cm0102.exe', 'cm0102-1080.exe')]
        import bisect, csv
        inicios = sorted(int(r[0], 16) for r in csv.reader(open(os.path.join(R, 'docs', 'mapa_funcoes.csv'), encoding='utf-8')) if r and r[0].startswith('0x'))
        alteradas = [i + 0x400000 for i in range(0x1000, 0x567000) if exes[0][i] != exes[1][i]]
        # a mesma rotina não pode ser desviada duas vezes (já no motor.rs ou em dois itens da fila)
        # as ZERADAS tambem tem Prologo, mas zerar e portar a mesma rotina e previsto (o desvio da copia zera)
        sem_zeradas = re.sub(r'const ZERADAS:.*?\n\];', '', m, flags=re.S)
        desviadas = {int(x, 16): 'motor.rs' for x in re.findall(r'Prologo \{ endereco: (0x[0-9a-f]+)', sem_zeradas)}
        for nome in ordem():
            rot = runpy.run_path(os.path.join(FILA, nome, 'rotinas.py'))
            for a, e, bs in rot['ROTINAS']:
                if e in desviadas:
                    print('repetida', nome, a, hex(e), 'já em', desviadas[e])
                desviadas[e] = nome
                o = e - 0x400000
                if list(exes[0][o:o + len(bs)]) != list(bs):
                    print('prologo diferente do exe', nome, a, hex(e))
                k = bisect.bisect_right(inicios, e)
                fim = inicios[k] if k < len(inicios) else e + 0x1000
                trocados = [hex(x) for x in alteradas if e <= x < fim]
                if any(e <= x < e + len(bs) for x in alteradas):
                    # o desvio vai no começo da rotina: no exe de 1080 ele não entra (ex.: 0x55fba0, que lá já é desvio)
                    print('PROLOGO alterado no cm0102-1080.exe', nome, a, hex(e))
                if trocados:
                    # a cópia que lê o valor do próprio exe naquele endereço (`0x... as *const`) é aceita; citar
                    # o endereço num comentário não basta
                    cod = open(os.path.join(FILA, nome, 'codigo.rs'), encoding='utf-8').read()
                    lido = any(re.search(r'\b%s as \*const' % t, cod) for t in trocados)
                    print('alterada no cm0102-1080.exe', nome, a, hex(e), 'em', trocados[:6], '(lida do exe no código)' if lido else '')
            cod = open(os.path.join(FILA, nome, 'codigo.rs'), encoding='utf-8').read()
            # comparador que restaura o gerador e roda a cópia sem `sem_contar`: os sorteios dela entram na
            # contagem e a impressão do jogo real comparando sai da referência (204a, 210a)
            for chamada in re.findall(r'lado_a_lado\((?:[^;]|\n)*?;', cod):
                if 'REGIOES_DO_ESTADO' in chamada and 'sem_contar' not in chamada:
                    print('sem sem_contar', nome, chamada.split('\n')[0][:100])
            for k in range(len(rot['ROTINAS'])):
                cod = cod.replace('%%%%I%d%%%%' % k, str(base + k))
            m += '\n' + cod
            base += len(rot['ROTINAS'])
            for mut, a, _ in runpy.run_path(os.path.join(FILA, nome, 'mutacoes.py'))['MUTACOES']:
                if m.count(a) != 1:
                    print('ancora', nome, mut, m.count(a))
        print('checado', len(ordem()), 'itens')
    else:
        sys.exit(__doc__)


if __name__ == '__main__':
    main()
