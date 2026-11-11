"""Documentacao de uma leva: ROTINAS, PROJETO, wiki (pt/en), glossario e armadilha do COMO_CONTINUAR.
Uso: o levas/NNN/docs.py importa e chama fazer() (ver levas/LEIA.md)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))  # raiz do projeto

# As referências da cadeia (o original determinizado): mudam quando muda uma zeragem ou um desvio desligado.
# 22/10/2026: a zeragem da 0x84be20 (212a) foi desfeita e a 0x7c6bf0 desligada (DESLIGADAS no motor.rs);
# o jogo real volta a 963cea289f055ac3 e a ampla a 9ee2b5ac (Rust = original); antes: carreira 034ff3eb3e55dfd3.
# 28/10/2026: o alocador da simulacao passa a juntar os blocos livres (a5a949d): o jogo real vai a 2f65fd6b68d1027b e a
# carreira a be54a1386f51073a (Rust = original na cadeia da 245a); antes: jogo 963cea289f055ac3, carreira b28542870027e432.
REF_JOGO = '8c95873456684fd1'
REF_CARREIRA = '7f4ecded7ef233a0'
CARREIRA_PT = '2.378.003.110 sorteios, impressão be54a1386f51073a, partidas 8832310884e7ef66, banco da73be6ffdbb550b'
CARREIRA_EN = '2,378,003,110 draws, fingerprint be54a1386f51073a, matches 8832310884e7ef66, database da73be6ffdbb550b'


def _extenso_pt(n):
    """O número por extenso no feminino (as levas), de 1 a 999."""
    u = ['', 'uma', 'duas', 'três', 'quatro', 'cinco', 'seis', 'sete', 'oito', 'nove', 'dez', 'onze', 'doze', 'treze',
         'catorze', 'quinze', 'dezesseis', 'dezessete', 'dezoito', 'dezenove']
    d = ['', '', 'vinte', 'trinta', 'quarenta', 'cinquenta', 'sessenta', 'setenta', 'oitenta', 'noventa']
    c = ['', 'cento', 'duzentas', 'trezentas', 'quatrocentas', 'quinhentas', 'seiscentas', 'setecentas', 'oitocentas', 'novecentas']
    if n == 100:
        return 'cem'
    partes = []
    if n >= 100:
        partes.append(c[n // 100])
    r = n % 100
    if r >= 20:
        partes.append(d[r // 10] + (' e ' + u[r % 10] if r % 10 else ''))
    elif r:
        partes.append(u[r])
    return ' e '.join(partes)


def _extenso_en(n):
    """The number in words, 1 to 999."""
    u = ['', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine', 'ten', 'eleven', 'twelve', 'thirteen',
         'fourteen', 'fifteen', 'sixteen', 'seventeen', 'eighteen', 'nineteen']
    d = ['', '', 'twenty', 'thirty', 'forty', 'fifty', 'sixty', 'seventy', 'eighty', 'ninety']
    r = n % 100
    dez = (d[r // 10] + ('-' + u[r % 10] if r % 10 else '')) if r >= 20 else u[r]
    if n < 100:
        return dez
    return u[n // 100] + ' hundred' + (' and ' + dez if r else '')


class _Extenso(dict):
    """EXT[n] = (pt, en); a tabela antiga ia só até 199 (a publicação da 200ª parou nela)."""
    def __missing__(self, n):
        return (_extenso_pt(n), _extenso_en(n))


EXT = _Extenso()
# confere contra a tabela antiga (escrita à mão)
assert EXT[75] == ('setenta e cinco', 'seventy-five') and EXT[81] == ('oitenta e uma', 'eighty-one')
assert EXT[161] == ('cento e sessenta e uma', 'one hundred and sixty-one') and EXT[160] == ('cento e sessenta', 'one hundred and sixty')


def sub(p, a, b):
    s = open(R + '\\' + p, encoding='utf-8').read()
    assert s.count(a) == 1, (p, a)
    open(R + '\\' + p, 'w', encoding='utf-8').write(s.replace(a, b))


def fazer(n, antes, depois, titulo_pt, titulo_en, linhas_pt, linhas_en, mut_pt, mut_en, glossario, ancora, armadilha=None, secao=('Motor de partidas', 'Match engine')):
    import datetime
    hoje = datetime.date.today()
    ord_en = 'th' if n % 100 in (11, 12, 13) else {1: 'st', 2: 'nd', 3: 'rd'}.get(n % 10, 'th')
    cab_pt = '| Rotina | Original | Convenção | Observação |\n|---|---|---|---|\n'
    cab_en = '| Routine | Original | Convention | Notes |\n|---|---|---|---|\n'
    open(R + r'\docs\ROTINAS.md', 'a', encoding='utf-8').write(
        f'\n### {secao[0]}, {n}ª leva — {titulo_pt} (oficial em {hoje:%d/%m/%Y})\n\n' + cab_pt + '\n'.join(linhas_pt) + f'\n\nMutações: {mut_pt}\n')
    open(R + r'\docs\ROTINAS.en.md', 'a', encoding='utf-8').write(
        f'\n### {secao[1]}, {n}{ord_en} batch — {titulo_en} (official on {hoje:%Y-%m-%d})\n\n' + cab_en + '\n'.join(linhas_en) + f'\n\nMutations: {mut_en}\n')
    sub(r'docs\PROJETO.md', f'**{n-1} levas, {antes} rotinas**', f'**{n} levas, {depois} rotinas**')
    sub(r'docs\PROJETO.en.md', f'**{n-1} batches, {antes} routines**', f'**{n} batches, {depois} routines**')
    sub('README.pt.md', f'**Estado:** {n-1} levas, {antes} rotinas', f'**Estado:** {n} levas, {depois} rotinas')
    sub('README.md', f'**State:** {n-1} batches, {antes} routines', f'**State:** {n} batches, {depois} routines')
    sub(r'wiki\index.md', f'{antes} rotinas em Rust** (`rust/src/motor.rs`, {EXT[n-1][0]} levas', f'{depois} rotinas em Rust** (`rust/src/motor.rs`, {EXT[n][0]} levas')
    s = open(R + r'\wiki\index.en.md', encoding='utf-8').read()
    s2 = s.replace(f'{antes} routines in Rust', f'{depois} routines in Rust', 1).replace(f'{EXT[n-1][1]} batches', f'{EXT[n][1]} batches', 1)
    assert s2 != s
    open(R + r'\wiki\index.en.md', 'w', encoding='utf-8').write(s2)
    open(R + r'\wiki\log.md', 'a', encoding='utf-8').write(f'- {n}ª leva: {titulo_pt}. {depois} rotinas.\n')
    open(R + r'\wiki\log.en.md', 'a', encoding='utf-8').write(f'- {n}{ord_en} batch: {titulo_en}. {depois} routines.\n')
    g = open(R + r'\engenharia-reversa\glossario.py', encoding='utf-8').read()
    assert g.count(ancora) == 1
    g = g.replace(ancora, ancora + '\n ' + ' '.join(f"'{a}': '{b}'," for a, b in glossario))
    open(R + r'\engenharia-reversa\glossario.py', 'w', encoding='utf-8').write(g)
    if armadilha:
        for arq, t in (('COMO_CONTINUAR.md', armadilha[0]), ('COMO_CONTINUAR.en.md', armadilha[1])):
            open(R + '\\docs\\' + arq, 'a', encoding='utf-8').write('- ' + t + '\n')
    import fronteiras
    fronteiras.placar(fronteiras.calcular())
    print('docs ok')


def leia(n, titulo_pt, titulo_en, rotinas, mutacoes, acusadas, lacuna_pt='', lacuna_en='', campos=(), interessa=()):
    """LEIA.md e LEIA.en.md da leva (levas/NNN/), para as levas com comparação de verdade e a cadeia
    completa. `rotinas` = [(nome, endereço, arquivo, o que faz pt, o que faz en)]. Desde a 215ª (pedido
    da sessão orquestradora, autorizado pelo Lucas em 22/10/2026): `campos` = [(linha pt, linha en)] no
    formato `registro +desloc: o que é; ponteiro|id|valor; lê|grava (rotina)` e `interessa` = as sessões
    irmãs que podem aproveitar a leva (CMRTE, SGE, CMScout, GPF2)."""
    base = os.path.join(R, 'levas', str(n))
    ord_en = 'th' if n % 100 in (11, 12, 13) else {1: 'st', 2: 'nd', 3: 'rd'}.get(n % 10, 'th')
    rp, re_ = resultado_pt_en(n)
    ref = (rp.split(': ', 1)[1].split('. Carreiras')[0], rp.split('Carreiras até 2002-06-01: ')[1].rstrip('.'),
           re_.split(': ', 1)[1].split('. Careers')[0], re_.split('Careers to 2002-06-01: ')[1].rstrip('.'))
    pt = [f'# {n}ª leva — {titulo_pt}', '', '**Rotinas:**']
    pt += [f'- `{a}()` = {e} ({f}): {d};' for a, e, f, d, _ in rotinas]
    pt += ['', '**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).',
           '', '**Resultado** (`cadeia.txt`):',
           f'- **Mutações ({mutacoes}):** {acusadas} acusadas.' + (' ' + lacuna_pt if lacuna_pt else ''),
           f'- **Jogo real comparando até 2002-01-01:** {ref[0]}.',
           f'- **Carreiras Rust ×2 e original até 2002-06-01:** {ref[1]}.',
           '', f'**Refazer**, sobre o commit da {n - 1}ª:', '1. `python integrar.py`',
           '2. `cargo build --release` e copiar a DLL para `jogo_testes/`', f'3. `bash engenharia-reversa/cadeia.sh {n}`', '']
    en = [f'# Batch {n} — {titulo_en}', '', '**Routines:**']
    en += [f'- `{a}()` = {e} ({f}): {d};' for a, e, f, _, d in rotinas]
    en += ['', '**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).',
           '', '**Result** (`cadeia.txt`):',
           f'- **Mutations ({mutacoes}):** {acusadas} caught.' + (' ' + lacuna_en if lacuna_en else ''),
           f'- **Real game comparing to 2002-01-01:** {ref[2]}.',
           f'- **Rust ×2 and original careers to 2002-06-01:** {ref[3]}.',
           '', f'**Redo**, on top of batch {n - 1}\'s commit:', '1. `python integrar.py`',
           '2. `cargo build --release` and copy the DLL to `jogo_testes/`', f'3. `bash engenharia-reversa/cadeia.sh {n}`', '']
    ps = prova_save(n)
    if ps:  # leva do save: as mutações foram pela prova do save, e ela entra no resultado
        i = pt.index('**Resultado** (`cadeia.txt`):') + 1
        carga = os.path.exists(os.path.join(base, 'CARGA'))  # leva de carregar: a prova também carrega o save de férias
        pt.insert(i, f'- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos' + (', e o save de férias carregado até 2001-08-28 com e sem elas, mesma impressão' if carga else '') + f'; as mutações também foram por ela): {ps}.')
        i = en.index('**Result** (`cadeia.txt`):') + 1
        en.insert(i, f'- **Save proof** (the `.qst` saved in Rust mode with and without the batch\'s routines, outside the bytes that vary on their own' + (', and the holiday save loaded to 2001-08-28 with and without them, same fingerprint' if carga else '') + f'; the mutations also went through it): {ps.replace("IGUAL", "EQUAL").replace("DIFERENTE", "DIFFERENT")}.')
    if campos:
        pt += ['**Campos tocados:**'] + [f'- {c}' for c, _ in campos] + ['']
        en += ['**Fields touched:**'] + [f'- {c}' for _, c in campos] + ['']
    if interessa:
        pt += [f'**Interessa a:** {", ".join(interessa)}', '']
        en += [f'**Of interest to:** {", ".join(interessa)}', '']
    open(os.path.join(base, 'LEIA.md'), 'w', encoding='utf-8').write('\n'.join(pt))
    open(os.path.join(base, 'LEIA.en.md'), 'w', encoding='utf-8').write('\n'.join(en))


def prova_save(n):
    """O resultado da prova do save no cadeia.txt (o que vem depois de "prova da leva N: "), ou None
    se a leva nao for do bloco do save."""
    import re
    t = open(os.path.join(R, 'levas', str(n), 'cadeia.txt'), encoding='utf-8', errors='replace').read()
    m = re.findall(r'^prova da leva %d: (.*)$' % n, t, re.M)
    return m[-1].strip() if m else None


def cadeia(n):
    """O resultado de levas/NNN/cadeia.txt: (total de mutações, acusadas, escapadas, cadeia ok).
    Cadeia ok = jogo real sem divergência e as três carreiras com a impressão de referência."""
    import re
    t = open(os.path.join(R, 'levas', str(n), 'cadeia.txt'), encoding='utf-8', errors='replace').read()
    nomes = {}
    for nome, res in re.findall(r'^mutacao (\S+)\s+\d+s (ACUSADA|IGUAL A BASE|EXCECAO|EXCEPTION|[A-Z ]+?)[:(]', t, re.M):
        res = res.strip()
        if nome not in nomes or res == 'ACUSADA':
            nomes[nome] = res
    acusadas = [k for k, v in nomes.items() if v != 'IGUAL A BASE']
    escapadas = [k for k, v in nomes.items() if v == 'IGUAL A BASE']
    conf = [(m.group(1), re.search('DIVERGEN(?:CIA|CE)', m.group(2)) or re.search(r'\b[1-9]\d* divergen(?:cias|ces)', m.group(2)))
            for m in re.finditer(r'^mutacao (\S+)\s+\d+s comparando[^:]*: (.*)$', t, re.M)]
    for nome, div in conf:
        if div and nome in escapadas:
            escapadas.remove(nome)
            acusadas.append(nome)
    # desde 31/10/2026 (Lucas): sem referência fixa; ok = jogo real comparando sem divergência e as carreiras
    # chegando à data (Rust ×2 iguais entre si); a carreira original só é registrada
    imp = impressoes(n)
    # 05/11/2026 (Lucas): na leva de competições o jogo real fica adiado para a auditoria (comparação acumulada)
    ok = ((imp['jogo'] is not None or '=== JOGO REAL ADIADO' in t) and len(imp['rust']) >= 2 and len(set(imp['rust'])) == 1 and bool(imp['original'])
          and not re.search(r'\b[1-9]\d* divergen(?:cias|ces)|DIVERGEN(?:CIA|CE) ', t.split('=== JOGO REAL')[-1].split('=== CARREIRA')[0]))
    # a linha DIVERGENCIA do ddraw.log também barra: o total de comparações só é impresso em marcos, e uma
    # divergência depois do último marco não entra nele (a 268ª, 02/11/2026)
    return len(nomes), len(acusadas), escapadas, ok


def impressoes(n):
    """As impressões do cadeia.txt: {'jogo': a do jogo real comparando, 'rust': [as carreiras Rust], 'original': a
    carreira original, 'detector': o resultado do detector de lixo ('igual', 'DIFERENTE (...)') ou None} (None / []
    quando a etapa não chegou à data). A carreira do detector (01/11/2026) sai em linhas "detector...", nunca em
    "execucao N:": não entra nas Rust nem na original."""
    import re
    t = open(os.path.join(R, 'levas', str(n), 'cadeia.txt'), encoding='utf-8', errors='replace').read()
    jogo = re.findall(r'^execucao \d+: parou em .*?impressao (\w+)', t.split('=== JOGO REAL')[-1].split('=== CARREIRA')[0], re.M) if '=== JOGO REAL' in t else []
    car = re.findall(r'^execucao (\d+): parou em .*?impressao (\w+)', t.split('=== CARREIRA')[-1], re.M) if '=== CARREIRA' in t else []
    # Rust ×2 (execucao 1 e 2) e depois a original (execucao 1 de novo)
    rust = [i for _, i in car[:2]] if len(car) >= 2 and car[1][0] == '2' else [i for _, i in car[:1]]
    original = car[len(rust)][1] if len(car) > len(rust) else None
    det = re.findall(r'^detector de lixo: (.*)$', t.split('=== CARREIRA')[-1], re.M) if '=== CARREIRA' in t else []
    return {'jogo': jogo[0] if jogo else None, 'rust': rust, 'original': original, 'detector': det[-1].strip() if det else None}


def resultado_pt_en(n):
    """O texto do jogo real e das carreiras para o PROJETO/wiki (pt, en), pelas impressões da leva."""
    i = impressoes(n)
    r = (i['rust'] or ['?'])[0]
    igual = i['original'] == r
    o_pt = 'igual' if igual else '%s (registrada; não barra a leva desde 31/10/2026)' % (i['original'] or '?')[:8]
    o_en = 'equal' if igual else '%s (recorded; does not block the batch since 31/10/2026)' % (i['original'] or '?')[:8]
    j = (i['jogo'] or '?')[:8]
    # o detector de lixo é só registro (não decide a leva); entra no texto quando a cadeia o rodou
    d_pt = ' Detector de lixo: %s.' % i['detector'] if i['detector'] else ''
    d_en = ' Garbage detector: %s.' % i['detector'].replace('igual', 'equal').replace('DIFERENTE', 'DIFFERENT') if i['detector'] else ''
    if '=== JOGO REAL ADIADO' in open(os.path.join(R, 'levas', str(n), 'cadeia.txt'), encoding='utf-8', errors='replace').read():
        return (f"Jogo real comparando 8 ligas até 2002-06-01: adiado para a comparação acumulada da auditoria (decisão de 05/11/2026). Carreiras até 2002-06-01: Rust ×2 {r[:8]}, original {o_pt}.{d_pt}",
                f"Real game comparing 8 leagues to 2002-06-01: deferred to the audit's accumulated comparison (decision of 5 Nov 2026). Careers to 2002-06-01: Rust ×2 {r[:8]}, original {o_en}.{d_en}")
    return (f"Jogo real comparando até 2002-01-01: 0 divergências ({j}). Carreiras até 2002-06-01: Rust ×2 {r[:8]}, original {o_pt}.{d_pt}",
            f"Real game comparing to 2002-01-01: 0 divergences ({j}). Careers to 2002-06-01: Rust ×2 {r[:8]}, original {o_en}.{d_en}")
