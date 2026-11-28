r"""A fila de levas: levas escritas antes da hora, com os índices calculados só ao integrar.

Cada item fica em levas/fila/<nome>/:
  codigo.rs    as rotinas e os comparadores; os índices de `orig::<..>(N)` e `compare_value!(.., N, ..)`
               escritos como %%I0%%, %%I1%%, %%I2%% (na ordem de ROTINAS); viram IDX_<NOME> (build.rs)
  rotinas.py   ROTINAS = [(nome_rust, endereco, [bytes do prologo])]
               SAVE = True (opcional): item do bloco do save; a leva ganha levas/N/SAVE e a cadeia prova
               pelo save (prova_save.py --leva N, mutacoes.py --prova-save N) em vez da carreira
               CARGA = True (opcional, com SAVE): item de carregar; a leva ganha também levas/N/CARGA e a
               prova do save carrega o FeriasTesteSave com e sem as rotinas (a impressão tem de bater)
               EXE = {endereco: '1080' | 'original'} (opcional): entrada de um exe só (checar: a do 1080 confere o
               prólogo contra o cm0102-1080.exe; nenhuma das duas é acusada de "alterada no 1080")
               LIGAS = 'ENG ESP ...' (opcional): item de competições de outras nações; só se junta a itens com as mesmas
               ligas, a leva ganha levas/N/LIGAS e a cadeia compara e muta com elas até 2002-06-01 (03/11/2026)
               ATE = 'AAAA-MM-DD' (opcional, 12/11/2026): a data final da cadeia (mutações, jogo real e carreiras);
               só se junta a itens com a mesma; com ela o jogo real roda na cadeia (não fica para a auditoria)
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


def texto_do_motor(raiz):
    """O motor.rs e os arquivos que ele inclui (rust/src/motor/, etapa 3 da reorganização, 07/11/2026)."""
    src = os.path.join(raiz, 'rust', 'src')
    fs = sorted(os.path.join(d, f) for d, _, n in os.walk(os.path.join(src, 'motor')) for f in n if f.endswith('.rs'))
    return '\n'.join(open(f, encoding='utf-8').read() for f in [os.path.join(src, 'motor.rs')] + fs)

INTEGRAR = r'''"""Integra a {n}a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = {rotinas!r}
BASE = {base}
m = re.search(r'\n\}}\nconst N: usize = (\d+);', s)
assert m and int(m.group(1)) == BASE, ('o motor.rs mudou desde a materialização', m and m.group(1), BASE)
# one line per routine in `routines! {{ .. }}` (rust/src/motor.rs; read by engenharia-reversa/tabela_motor.py)
linhas = ''.join('\n    Prologue {{ address: %#x, bytes: &[%s] }} => %s, compare_%s;' % (e, ', '.join('0x%02x' % b for b in bs), a, a)
                 for a, e, bs in ROTINAS)
s = s[:m.start()] + linhas + '\n}}\nconst N: usize = %d;' % (BASE + len(ROTINAS)) + s[m.end():]
anc = 'compare_value!(compare_staff_criterion, staff_criterion, IDX_STAFF_CRITERION, () -> u32);\n'
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
# 24/10/2026: o Lucas dobrou a aposta: a 232ª junta 7 itens (20 rotinas). Para voltar às de 9: ITENS_POR_LEVA = 3.
# 29/10/2026: o Lucas aprovou levas de 13 itens (39 rotinas) a partir da 251ª. Para voltar às de 20: ITENS_POR_LEVA = 7.
# 26/11/2026: o Lucas pediu o ritmo de 22/11; levas de 26 itens a partir da 341a. Para voltar às de 13: ITENS_POR_LEVA = 13.
ITENS_POR_LEVA = int(os.environ.get('FILA_ITENS', 26))  # 26/11/2026: FILA_ITENS=N for one larger batch (the night round)


def do_lote(ordem_):
    """Os itens da próxima leva: o primeiro da ordem e os seguintes do mesmo tipo (MOTOR, SAVE), até ITENS_POR_LEVA."""
    def tipo(nome):
        rot = runpy.run_path(os.path.join(FILA, nome, 'rotinas.py'))
        if rot.get('TELAS'):  # the screens batch is one type whatever MOTOR/SAVE the items carry (21/11/2026)
            return False, False, '', '', True, ''
        return bool(rot.get('MOTOR')), bool(rot.get('SAVE')), rot.get('LIGAS', ''), rot.get('ATE', ''), bool(rot.get('TELAS')), rot.get('BASE', '')
    # 24/11/2026: an item never jumps ahead of a skipped one (another type) that defines a name it uses, or that its
    # INTEGRAR_ANTES.txt names: batch 332 took sx05b_1..4 without sx05b_5 and cx16_1 without cx06_4 (45 build errors)
    definidos_no_motor = set(DEF.findall(sem_comentarios(texto_do_motor(R))))
    pulados, nomes_pulados = set(), set()

    def depende_de_pulado(nome):
        codigo = sem_comentarios(open(os.path.join(FILA, nome, 'codigo.rs'), encoding='utf-8').read())
        antes = os.path.join(FILA, nome, 'INTEGRAR_ANTES.txt')
        pedidos = set(re.findall(r'\b([a-z]+\d*[a-zA-Z]?(?:_\d+)+)\b', open(antes, encoding='utf-8').read())) if os.path.exists(antes) else set()
        return bool(set(IDENT.findall(codigo)) & pulados) or bool(pedidos & nomes_pulados)

    def pular(nome):
        nomes_pulados.add(nome)
        pulados.update(set(DEF.findall(sem_comentarios(open(os.path.join(FILA, nome, 'codigo.rs'), encoding='utf-8').read()))) - definidos_no_motor)

    lote = [ordem_[0]]
    for nome in ordem_[1:]:
        # 21/11/2026 (Lucas): as telas entram todas numa leva só, provada por ele jogando comparando
        if len(lote) == ITENS_POR_LEVA and not tipo(lote[0])[4]:
            break
        if tipo(nome) == tipo(lote[0]) and not depende_de_pulado(nome):
            lote.append(nome)
        else:
            pular(nome)
    # 26/11/2026: nor ahead of an item further down the queue: the night round moved to the top went in without
    # cx14_6 and rx09_3 (batch 339's first try, 10 build errors). Moving items is by hand, so this only stops.
    # Only the crate-visible definitions of the items further down count (a local `fn group` is not a dependency).
    def codigo_de(nome):
        return sem_comentarios(open(os.path.join(FILA, nome, 'codigo.rs'), encoding='utf-8').read())
    no_lote = set().union(*(set(DEF.findall(codigo_de(nome))) for nome in lote))
    adiante = [nome for nome in ordem_[1:] if nome not in lote]
    # 28/11/2026: and the held items (SEGURADOS.txt) too: batch 345 took cx07_2, which calls a helper only cx05_3 defines
    segurados = os.path.join(FILA, 'SEGURADOS.txt')
    if os.path.exists(segurados):
        adiante += [l.split('\t')[0].strip() for l in open(segurados, encoding='utf-8') if l.strip() and not l.startswith('#')
                    and os.path.isdir(os.path.join(FILA, l.split('\t')[0].strip()))]
    pulados.clear()
    pulados.update(set().union(*(set(DEF_PUB.findall(codigo_de(nome))) for nome in adiante)) - definidos_no_motor - no_lote)
    nomes_pulados.clear()
    nomes_pulados.update(adiante)
    faltam = [nome for nome in lote if depende_de_pulado(nome)]
    if faltam:
        sys.exit('itens que dependem de outros mais adiante na fila (ver INTEGRAR_ANTES.txt e os nomes): %s' % faltam)
    return lote


DEF = re.compile(r'\b(?:fn|struct|enum|const|static|type|mod|trait)\s+([A-Za-z_]\w*)')
IDENT = re.compile(r'\b[A-Za-z_]\w*\b')
DEF_PUB = re.compile(r'^pub\(crate\)\s+(?:unsafe\s+)?(?:extern\s+"[^"]+"\s+)?(?:fn|struct|enum|const|static|type|trait)\s+([A-Za-z_]\w*)', re.M)


def sem_comentarios(codigo):
    """The code without its // comments (a word in a comment is not a name the item defines or uses)."""
    return re.sub(r'//[^\n]*', '', codigo)


def materializar(nomes, n, base, antes):
    destino = os.path.join(R, 'levas', str(n))
    os.makedirs(destino, exist_ok=False)
    codigos, rotinas, mutacoes, docs, tpts, tens = [], [], [], [], [], []
    campos, interessa = [], []  # LEIA: campos tocados e sessões irmãs interessadas (desde a 215a)
    marcas = []  # the items are marked only after the checks pass (25/11/2026: a failed 334 left 13 items marked)
    b = base
    for nome in nomes:
        src = os.path.join(FILA, nome)
        rot = runpy.run_path(os.path.join(src, 'rotinas.py'))
        cod = open(os.path.join(src, 'codigo.rs'), encoding='utf-8').read()
        for k in range(len(rot['ROTINAS'])):
            cod = cod.replace('%%%%I%d%%%%' % k, 'IDX_' + rot['ROTINAS'][k][0].upper())  # a constante do build.rs (07/11/2026)
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
        marcas.append((os.path.join(src, 'materializada.txt'), f'leva {n}, índices {b}..{b + len(rot["ROTINAS"]) - 1}\n'))
        b += len(rot['ROTINAS'])
    assert len({m[0] for m in mutacoes}) == len(mutacoes), 'mutações com o mesmo nome em itens diferentes'
    # o trecho de cada mutação tem de aparecer uma vez só no motor.rs com a leva (24/10: a dq10_contar casava 8
    # vezes e a cadeia da 225a parou sem rodar as mutações)
    junto = texto_do_motor(R) + '\n'.join(codigos)
    repetidas = [(m[0], junto.count(m[1])) for m in mutacoes if junto.count(m[1]) != 1]
    if repetidas:
        os.rmdir(destino)  # nothing written yet: the batch number stays free
        sys.exit('trecho de mutação que não aparece uma vez só: %s' % repetidas)
    for caminho, texto in marcas:
        open(caminho, 'w', encoding='utf-8').write(texto)
    open(os.path.join(destino, 'codigo.rs'), 'w', encoding='utf-8').write('\n'.join(codigos))
    # EXE = {address: 'original'}: the routine is hooked only in the original exe; assinaturas.py reads the list
    so_orig = ['%#x  # %s, %s' % (a, x, n) for x in nomes
               for a, v in runpy.run_path(os.path.join(FILA, x, 'rotinas.py')).get('EXE', {}).items() if v == 'original'
               for n in [next((r[0] for r in runpy.run_path(os.path.join(FILA, x, 'rotinas.py'))['ROTINAS'] if r[1] == a), '?')]]
    if so_orig:
        with open(os.path.join(R, 'levas', 'so_original.txt'), 'a', encoding='utf-8') as f:
            f.write('\n'.join(so_orig) + '\n')
    save = any(runpy.run_path(os.path.join(FILA, x, 'rotinas.py')).get('SAVE') for x in nomes)
    if save:  # a marca da cadeia: prova do save em vez da carreira (ver cadeia.sh)
        open(os.path.join(destino, 'SAVE'), 'w', encoding='utf-8').write('leva do bloco do save\n')
    if any(runpy.run_path(os.path.join(FILA, x, 'rotinas.py')).get('CARGA') for x in nomes):  # a prova também pela carga
        open(os.path.join(destino, 'CARGA'), 'w', encoding='utf-8').write('leva de carregar: a prova do save carrega o save de ferias\n')
    if runpy.run_path(os.path.join(FILA, nomes[0], 'rotinas.py')).get('TELAS'):
        open(os.path.join(destino, 'TELAS'), 'w', encoding='utf-8').write('leva das telas: sem mutacoes na cadeia; a prova e o Lucas jogando comparando\n')
    ligas = runpy.run_path(os.path.join(FILA, nomes[0], 'rotinas.py')).get('LIGAS')
    if ligas:  # leva de competições: a cadeia compara e muta com essas ligas até 2002-06-01 (ver cadeia.sh)
        open(os.path.join(destino, 'LIGAS'), 'w', encoding='utf-8').write(ligas + '\n')
    ate = runpy.run_path(os.path.join(FILA, nomes[0], 'rotinas.py')).get('ATE')
    if ate:  # 12/11/2026: a data final da cadeia (ex.: 2002-08-01, depois da virada); com ela o jogo real roda na cadeia
        open(os.path.join(destino, 'ATE'), 'w', encoding='utf-8').write(ate + '\n')
    base_dados = runpy.run_path(os.path.join(FILA, nomes[0], 'rotinas.py')).get('BASE')
    if base_dados:  # 24/11/2026 (Lucas): divisions that only exist with a bigger database; the chain creates the game with it
        open(os.path.join(destino, 'BASE'), 'w', encoding='utf-8').write(base_dados + '\n')
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
    base = int(re.search(r'\n\}\nconst N: usize = (\d+);', s).group(1))
    leia = open(os.path.join(R, 'README.pt.md'), encoding='utf-8').read()
    n, total = map(int, re.search(r'\*\*Estado:\*\* (\d+) levas, (\d+) rotinas', leia).groups())
    return n, total, base


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else ''
    if cmd == 'proxima':
        n, total, base = estado()
        s = open(os.path.join(R, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
        while os.path.isdir(os.path.join(R, 'levas', str(n + 1))):
            vazia = os.path.join(R, 'levas', str(n + 1))
            if not os.listdir(vazia):
                # 28/11/2026: a failed materialisation left the folder empty; it is not a pending batch (the 344th had
                # come out as 346 over two empty ones)
                os.rmdir(vazia)
                break
            n += 1  # já materializada (ou escrita à mão) e ainda não publicada
            # bancadas paralelas (31/10/2026): a leva pendente conta no total e, se ainda não estiver integrada na
            # principal, também nos índices (a seguinte roda numa bancada com as duas integradas)
            p = os.path.join(R, 'levas', str(n), 'integrar.py')
            if os.path.exists(p):
                rot = re.findall(r"\('\w+', (\d+), \[", open(p, encoding='utf-8').read())
                total += len(rot)
                if rot and ('address: %#x,' % int(rot[0])) not in s:
                    base += len(rot)
        ordem_ = ordem()
        assert ordem_, 'fila vazia'
        materializar(do_lote(ordem_), n + 1, base, total)
    elif cmd == 'checar':
        import tempfile
        tmp = tempfile.mkdtemp()
        shutil.copytree(os.path.join(R, 'rust', 'src'), os.path.join(tmp, 'rust', 'src'))
        m = texto_do_motor(tmp)
        base = int(re.search(r'\n\}\nconst N: usize = (\d+);', m).group(1))
        # o exe de trabalho (1920x1080) é o cm0102.exe com patches (resolucao.py): rotina alterada nele não
        # se porta igual à original (187a: a 0x5e5cd0, altura da fonte, salta para um trecho novo)
        # a rotina vai do começo até o próximo início de função do docs/mapa_funcoes.csv (o mapa às vezes junta
        # funções: a faixa fica maior, nunca menor); 20/10: o y2 0x257 → 0x437 em 0x719346, no meio da 0x718e90
        exes = [open(os.path.join(R, 'jogo', e), 'rb').read() for e in ('cm0102.exe', 'cm0102-1080.exe')]
        import bisect, csv
        inicios = sorted(int(r[0], 16) for r in csv.reader(open(os.path.join(R, 'docs', 'mapa_funcoes.csv'), encoding='utf-8')) if r and r[0].startswith('0x'))
        alteradas = [i + 0x400000 for i in range(0x1000, 0x567000) if exes[0][i] != exes[1][i]]
        # a mesma rotina não pode ser desviada duas vezes (já no motor.rs ou em dois itens da fila)
        # as ZERADAS tambem tem Prologue, mas zerar e portar a mesma rotina e previsto (o desvio da copia zera)
        sem_zeradas = re.sub(r'const ZERADAS:.*?\n\];', '', m, flags=re.S)
        desviadas = {int(x, 16): 'motor.rs' for x in re.findall(r'Prologue \{ address: (0x[0-9a-f]+)', sem_zeradas)}
        for nome in ordem():
            rot = runpy.run_path(os.path.join(FILA, nome, 'rotinas.py'))
            for a, e, bs in rot['ROTINAS']:
                if e in desviadas:
                    print('repetida', nome, a, hex(e), 'já em', desviadas[e])
                desviadas[e] = nome
                o = e - 0x400000
                # EXE = {endereço: '1080' | 'original'} (opcional): a entrada que só existe no cm0102-1080.exe (o
                # prólogo é dele; 0x603834) confere contra o 1080; a original que o 1080 trocou por ela (0x601a40) é
                # alterada lá de propósito (assinaturas.py: cada uma vale só no seu exe)
                exe = rot.get('EXE', {}).get(e)
                if list(exes[1 if exe == '1080' else 0][o:o + len(bs)]) != list(bs):
                    print('prologo diferente do exe', nome, a, hex(e), exe or '')
                if exe:
                    continue
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
            # comparador que restaura o gerador e roda a cópia sem `without_counting`: os sorteios dela entram na
            # contagem e a impressão do jogo real comparando sai da referência (204a, 210a)
            for chamada in re.findall(r'side_by_side(?:_leve)?\((?:[^;]|\n)*?;', cod):
                if 'STATE_REGIONS' in chamada and 'without_counting' not in chamada:
                    print('sem without_counting', nome, chamada.split('\n')[0][:100])
            for k in range(len(rot['ROTINAS'])):
                cod = cod.replace('%%%%I%d%%%%' % k, 'IDX_' + rot['ROTINAS'][k][0].upper())
            m += '\n' + cod
            base += len(rot['ROTINAS'])
            for mut, a, *_ in runpy.run_path(os.path.join(FILA, nome, 'mutacoes.py'))['MUTACOES']:
                if m.count(a) != 1:
                    print('ancora', nome, mut, m.count(a))
        print('checado', len(ordem()), 'itens')
    else:
        sys.exit(__doc__)


if __name__ == '__main__':
    main()
