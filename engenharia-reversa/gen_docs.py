"""Documentacao de uma leva: ROTINAS, PROJETO, wiki (pt/en), glossario e armadilha do COMO_CONTINUAR.
Uso: o levas/NNN/docs.py importa e chama fazer() (ver levas/LEIA.md)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))  # raiz do projeto

# As referências da cadeia (o original determinizado): mudam quando muda uma zeragem ou um desvio desligado.
# 22/10/2026: a zeragem da 0x84be20 (212a) foi desfeita e a 0x7c6bf0 desligada (DESLIGADAS no motor.rs);
# o jogo real volta a 963cea289f055ac3 e a ampla a 9ee2b5ac (Rust = original); antes: carreira 034ff3eb3e55dfd3.
REF_JOGO = '963cea289f055ac3'
REF_CARREIRA = 'b28542870027e432'
CARREIRA_PT = '2.368.478.169 sorteios, impressão b28542870027e432, partidas 959a02ada4a23639, banco ce037e0141221551'
CARREIRA_EN = '2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551'


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
    sub('README.md', f'**Estado:** {n-1} levas, {antes} rotinas', f'**Estado:** {n} levas, {depois} rotinas')
    sub('README.en.md', f'**State:** {n-1} batches, {antes} routines', f'**State:** {n} batches, {depois} routines')
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
    ref = (f'0 divergências ({REF_JOGO})', f'as três iguais ({REF_CARREIRA})',
           f'0 divergences ({REF_JOGO})', f'all three equal ({REF_CARREIRA})')
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
    if campos:
        pt += ['**Campos tocados:**'] + [f'- {c}' for c, _ in campos] + ['']
        en += ['**Fields touched:**'] + [f'- {c}' for _, c in campos] + ['']
    if interessa:
        pt += [f'**Interessa a:** {", ".join(interessa)}', '']
        en += [f'**Of interest to:** {", ".join(interessa)}', '']
    open(os.path.join(base, 'LEIA.md'), 'w', encoding='utf-8').write('\n'.join(pt))
    open(os.path.join(base, 'LEIA.en.md'), 'w', encoding='utf-8').write('\n'.join(en))


def cadeia(n):
    """O resultado de levas/NNN/cadeia.txt: (total de mutações, acusadas, escapadas, cadeia ok).
    Cadeia ok = jogo real sem divergência e as três carreiras com a impressão de referência."""
    import re
    t = open(os.path.join(R, 'levas', str(n), 'cadeia.txt'), encoding='utf-8', errors='replace').read()
    nomes = {}
    for nome, res in re.findall(r'^mutacao (\S+)\s+\d+s (ACUSADA|IGUAL A BASE|EXCECAO|[A-Z ]+?)[:(]', t, re.M):
        res = res.strip()
        if nome not in nomes or res == 'ACUSADA':
            nomes[nome] = res
    acusadas = [k for k, v in nomes.items() if v != 'IGUAL A BASE']
    escapadas = [k for k, v in nomes.items() if v == 'IGUAL A BASE']
    conf = [(m.group(1), 'DIVERGENCIA' in m.group(2) or re.search(r'\b[1-9]\d* divergencias', m.group(2)))
            for m in re.finditer(r'^mutacao (\S+)\s+\d+s comparando: (.*)$', t, re.M)]
    for nome, div in conf:
        if div and nome in escapadas:
            escapadas.remove(nome)
            acusadas.append(nome)
    ok = (f'impressao {REF_JOGO}' in t and t.count(f'impressao {REF_CARREIRA}') >= 3
          and not re.search(r'\b[1-9]\d* divergencias', t.split('=== JOGO REAL')[-1]))
    return len(nomes), len(acusadas), escapadas, ok
