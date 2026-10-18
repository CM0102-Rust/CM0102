"""Documentacao de uma leva: ROTINAS, PROJETO, wiki (pt/en), glossario e armadilha do COMO_CONTINUAR.
Uso: o levas/NNN/docs.py importa e chama fazer() (ver levas/LEIA.md)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))  # raiz do projeto
EXT = {75: ('setenta e cinco', 'seventy-five'), 76: ('setenta e seis', 'seventy-six'), 77: ('setenta e sete', 'seventy-seven'),
       78: ('setenta e oito', 'seventy-eight'), 79: ('setenta e nove', 'seventy-nine'), 80: ('oitenta', 'eighty'), 81: ('oitenta e uma', 'eighty-one'), 82: ('oitenta e duas', 'eighty-two'), 83: ('oitenta e três', 'eighty-three'), 84: ('oitenta e quatro', 'eighty-four'), 85: ('oitenta e cinco', 'eighty-five'), 86: ('oitenta e seis', 'eighty-six'), 87: ('oitenta e sete', 'eighty-seven'), 88: ('oitenta e oito', 'eighty-eight'), 89: ('oitenta e nove', 'eighty-nine'), 90: ('noventa', 'ninety'), 91: ('noventa e uma', 'ninety-one'), 92: ('noventa e duas', 'ninety-two'), 93: ('noventa e três', 'ninety-three'), 94: ('noventa e quatro', 'ninety-four'), 95: ('noventa e cinco', 'ninety-five'), 96: ('noventa e seis', 'ninety-six'), 97: ('noventa e sete', 'ninety-seven'), 98: ('noventa e oito', 'ninety-eight'), 99: ('noventa e nove', 'ninety-nine'), 100: ('cem', 'one hundred'), 101: ('cento e uma', 'one hundred and one'), 102: ('cento e duas', 'one hundred and two'), 103: ('cento e três', 'one hundred and three'), 104: ('cento e quatro', 'one hundred and four'), 105: ('cento e cinco', 'one hundred and five'), 106: ('cento e seis', 'one hundred and six'), 107: ('cento e sete', 'one hundred and seven'), 108: ('cento e oito', 'one hundred and eight'), 109: ('cento e nove', 'one hundred and nine'), 110: ('cento e dez', 'one hundred and ten'), 111: ('cento e onze', 'one hundred and eleven'), 112: ('cento e doze', 'one hundred and twelve'), 113: ('cento e treze', 'one hundred and thirteen'), 114: ('cento e catorze', 'one hundred and fourteen'), 115: ('cento e quinze', 'one hundred and fifteen'), 116: ('cento e dezesseis', 'one hundred and sixteen'), 117: ('cento e dezessete', 'one hundred and seventeen'), 118: ('cento e dezoito', 'one hundred and eighteen'), 119: ('cento e dezenove', 'one hundred and nineteen'), 120: ('cento e vinte', 'one hundred and twenty'), 121: ('cento e vinte e uma', 'one hundred and twenty-one'), 122: ('cento e vinte e duas', 'one hundred and twenty-two'), 123: ('cento e vinte e três', 'one hundred and twenty-three'), 124: ('cento e vinte e quatro', 'one hundred and twenty-four'), 125: ('cento e vinte e cinco', 'one hundred and twenty-five'), 126: ('cento e vinte e seis', 'one hundred and twenty-six'), 127: ('cento e vinte e sete', 'one hundred and twenty-seven'), 128: ('cento e vinte e oito', 'one hundred and twenty-eight'), 129: ('cento e vinte e nove', 'one hundred and twenty-nine'), 130: ('cento e trinta', 'one hundred and thirty'), 131: ('cento e trinta e uma', 'one hundred and thirty-one'), 132: ('cento e trinta e duas', 'one hundred and thirty-two'), 133: ('cento e trinta e três', 'one hundred and thirty-three'), 134: ('cento e trinta e quatro', 'one hundred and thirty-four'), 135: ('cento e trinta e cinco', 'one hundred and thirty-five'), 136: ('cento e trinta e seis', 'one hundred and thirty-six'), 137: ('cento e trinta e sete', 'one hundred and thirty-seven'), 138: ('cento e trinta e oito', 'one hundred and thirty-eight'), 139: ('cento e trinta e nove', 'one hundred and thirty-nine'), 140: ('cento e quarenta', 'one hundred and forty'), 141: ('cento e quarenta e uma', 'one hundred and forty-one'), 142: ('cento e quarenta e duas', 'one hundred and forty-two'), 143: ('cento e quarenta e três', 'one hundred and forty-three'), 144: ('cento e quarenta e quatro', 'one hundred and forty-four'), 145: ('cento e quarenta e cinco', 'one hundred and forty-five'), 146: ('cento e quarenta e seis', 'one hundred and forty-six'), 147: ('cento e quarenta e sete', 'one hundred and forty-seven'), 148: ('cento e quarenta e oito', 'one hundred and forty-eight'), 149: ('cento e quarenta e nove', 'one hundred and forty-nine'), 150: ('cento e cinquenta', 'one hundred and fifty'), 151: ('cento e cinquenta e uma', 'one hundred and fifty-one'), 152: ('cento e cinquenta e duas', 'one hundred and fifty-two'), 153: ('cento e cinquenta e três', 'one hundred and fifty-three'), 154: ('cento e cinquenta e quatro', 'one hundred and fifty-four'), 155: ('cento e cinquenta e cinco', 'one hundred and fifty-five'), 156: ('cento e cinquenta e seis', 'one hundred and fifty-six'), 157: ('cento e cinquenta e sete', 'one hundred and fifty-seven'), 158: ('cento e cinquenta e oito', 'one hundred and fifty-eight'), 159: ('cento e cinquenta e nove', 'one hundred and fifty-nine'),
       74: ('setenta e quatro', 'seventy-four'),
       160: ('cento e sessenta', 'one hundred and sixty'), 161: ('cento e sessenta e uma', 'one hundred and sixty-one'), 162: ('cento e sessenta e duas', 'one hundred and sixty-two'), 163: ('cento e sessenta e três', 'one hundred and sixty-three'), 164: ('cento e sessenta e quatro', 'one hundred and sixty-four'), 165: ('cento e sessenta e cinco', 'one hundred and sixty-five'), 166: ('cento e sessenta e seis', 'one hundred and sixty-six'), 167: ('cento e sessenta e sete', 'one hundred and sixty-seven'), 168: ('cento e sessenta e oito', 'one hundred and sixty-eight'), 169: ('cento e sessenta e nove', 'one hundred and sixty-nine'), 170: ('cento e setenta', 'one hundred and seventy'), 171: ('cento e setenta e uma', 'one hundred and seventy-one'), 172: ('cento e setenta e duas', 'one hundred and seventy-two'), 173: ('cento e setenta e três', 'one hundred and seventy-three'), 174: ('cento e setenta e quatro', 'one hundred and seventy-four'), 175: ('cento e setenta e cinco', 'one hundred and seventy-five'), 176: ('cento e setenta e seis', 'one hundred and seventy-six'), 177: ('cento e setenta e sete', 'one hundred and seventy-seven'), 178: ('cento e setenta e oito', 'one hundred and seventy-eight'), 179: ('cento e setenta e nove', 'one hundred and seventy-nine'), 180: ('cento e oitenta', 'one hundred and eighty'), 181: ('cento e oitenta e uma', 'one hundred and eighty-one'), 182: ('cento e oitenta e duas', 'one hundred and eighty-two'), 183: ('cento e oitenta e três', 'one hundred and eighty-three'), 184: ('cento e oitenta e quatro', 'one hundred and eighty-four'), 185: ('cento e oitenta e cinco', 'one hundred and eighty-five'), 186: ('cento e oitenta e seis', 'one hundred and eighty-six'), 187: ('cento e oitenta e sete', 'one hundred and eighty-seven'), 188: ('cento e oitenta e oito', 'one hundred and eighty-eight'), 189: ('cento e oitenta e nove', 'one hundred and eighty-nine'), 190: ('cento e noventa', 'one hundred and ninety'), 191: ('cento e noventa e uma', 'one hundred and ninety-one'), 192: ('cento e noventa e duas', 'one hundred and ninety-two'), 193: ('cento e noventa e três', 'one hundred and ninety-three'), 194: ('cento e noventa e quatro', 'one hundred and ninety-four'), 195: ('cento e noventa e cinco', 'one hundred and ninety-five'), 196: ('cento e noventa e seis', 'one hundred and ninety-six'), 197: ('cento e noventa e sete', 'one hundred and ninety-seven'), 198: ('cento e noventa e oito', 'one hundred and ninety-eight'), 199: ('cento e noventa e nove', 'one hundred and ninety-nine')}


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


def leia(n, titulo_pt, titulo_en, rotinas, mutacoes, acusadas, lacuna_pt='', lacuna_en=''):
    """LEIA.md e LEIA.en.md da leva (levas/NNN/), para as levas com comparação de verdade e a cadeia
    completa. `rotinas` = [(nome, endereço, arquivo, o que faz pt, o que faz en)]."""
    base = os.path.join(R, 'levas', str(n))
    ord_en = 'th' if n % 100 in (11, 12, 13) else {1: 'st', 2: 'nd', 3: 'rd'}.get(n % 10, 'th')
    ref = ('0 divergências (963cea289f055ac3)', 'as três iguais (034ff3eb3e55dfd3)',
           '0 divergences (963cea289f055ac3)', 'all three equal (034ff3eb3e55dfd3)')
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
    ok = ('impressao 963cea289f055ac3' in t and t.count('impressao 034ff3eb3e55dfd3') >= 3
          and not re.search(r'\b[1-9]\d* divergencias', t.split('=== JOGO REAL')[-1]))
    return len(nomes), len(acusadas), escapadas, ok
