"""Documentacao de uma leva: ROTINAS, PROJETO, wiki (pt/en), glossario e armadilha do COMO_CONTINUAR.
Uso: o levas/NNN/docs.py importa e chama fazer() (ver levas/LEIA.md)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))  # raiz do projeto
EXT = {75: ('setenta e cinco', 'seventy-five'), 76: ('setenta e seis', 'seventy-six'), 77: ('setenta e sete', 'seventy-seven'),
       78: ('setenta e oito', 'seventy-eight'), 79: ('setenta e nove', 'seventy-nine'), 80: ('oitenta', 'eighty'), 81: ('oitenta e uma', 'eighty-one'), 82: ('oitenta e duas', 'eighty-two'), 83: ('oitenta e três', 'eighty-three'), 84: ('oitenta e quatro', 'eighty-four'), 85: ('oitenta e cinco', 'eighty-five'), 86: ('oitenta e seis', 'eighty-six'), 87: ('oitenta e sete', 'eighty-seven'), 88: ('oitenta e oito', 'eighty-eight'), 89: ('oitenta e nove', 'eighty-nine'), 90: ('noventa', 'ninety'), 91: ('noventa e uma', 'ninety-one'), 92: ('noventa e duas', 'ninety-two'), 93: ('noventa e três', 'ninety-three'), 94: ('noventa e quatro', 'ninety-four'), 95: ('noventa e cinco', 'ninety-five'), 96: ('noventa e seis', 'ninety-six'), 97: ('noventa e sete', 'ninety-seven'), 98: ('noventa e oito', 'ninety-eight'), 99: ('noventa e nove', 'ninety-nine'), 100: ('cem', 'one hundred'), 101: ('cento e uma', 'one hundred and one'), 102: ('cento e duas', 'one hundred and two'), 103: ('cento e três', 'one hundred and three'), 104: ('cento e quatro', 'one hundred and four'), 105: ('cento e cinco', 'one hundred and five'), 106: ('cento e seis', 'one hundred and six'), 107: ('cento e sete', 'one hundred and seven'), 108: ('cento e oito', 'one hundred and eight'), 109: ('cento e nove', 'one hundred and nine'), 110: ('cento e dez', 'one hundred and ten'), 111: ('cento e onze', 'one hundred and eleven'), 112: ('cento e doze', 'one hundred and twelve'), 113: ('cento e treze', 'one hundred and thirteen'), 114: ('cento e catorze', 'one hundred and fourteen'), 115: ('cento e quinze', 'one hundred and fifteen'), 116: ('cento e dezesseis', 'one hundred and sixteen'), 117: ('cento e dezessete', 'one hundred and seventeen'), 118: ('cento e dezoito', 'one hundred and eighteen'), 119: ('cento e dezenove', 'one hundred and nineteen'), 120: ('cento e vinte', 'one hundred and twenty'), 121: ('cento e vinte e uma', 'one hundred and twenty-one'), 122: ('cento e vinte e duas', 'one hundred and twenty-two'), 123: ('cento e vinte e três', 'one hundred and twenty-three'), 124: ('cento e vinte e quatro', 'one hundred and twenty-four'), 125: ('cento e vinte e cinco', 'one hundred and twenty-five'), 126: ('cento e vinte e seis', 'one hundred and twenty-six'), 127: ('cento e vinte e sete', 'one hundred and twenty-seven'), 128: ('cento e vinte e oito', 'one hundred and twenty-eight'), 129: ('cento e vinte e nove', 'one hundred and twenty-nine'), 130: ('cento e trinta', 'one hundred and thirty'), 131: ('cento e trinta e uma', 'one hundred and thirty-one'), 132: ('cento e trinta e duas', 'one hundred and thirty-two'), 133: ('cento e trinta e três', 'one hundred and thirty-three'), 134: ('cento e trinta e quatro', 'one hundred and thirty-four'), 135: ('cento e trinta e cinco', 'one hundred and thirty-five'), 136: ('cento e trinta e seis', 'one hundred and thirty-six'), 137: ('cento e trinta e sete', 'one hundred and thirty-seven'), 138: ('cento e trinta e oito', 'one hundred and thirty-eight'), 139: ('cento e trinta e nove', 'one hundred and thirty-nine'), 140: ('cento e quarenta', 'one hundred and forty'), 141: ('cento e quarenta e uma', 'one hundred and forty-one'), 142: ('cento e quarenta e duas', 'one hundred and forty-two'), 143: ('cento e quarenta e três', 'one hundred and forty-three'), 144: ('cento e quarenta e quatro', 'one hundred and forty-four'), 145: ('cento e quarenta e cinco', 'one hundred and forty-five'), 146: ('cento e quarenta e seis', 'one hundred and forty-six'), 147: ('cento e quarenta e sete', 'one hundred and forty-seven'), 148: ('cento e quarenta e oito', 'one hundred and forty-eight'), 149: ('cento e quarenta e nove', 'one hundred and forty-nine'), 150: ('cento e cinquenta', 'one hundred and fifty'), 151: ('cento e cinquenta e uma', 'one hundred and fifty-one'), 152: ('cento e cinquenta e duas', 'one hundred and fifty-two'), 153: ('cento e cinquenta e três', 'one hundred and fifty-three'), 154: ('cento e cinquenta e quatro', 'one hundred and fifty-four'), 155: ('cento e cinquenta e cinco', 'one hundred and fifty-five'), 156: ('cento e cinquenta e seis', 'one hundred and fifty-six'), 157: ('cento e cinquenta e sete', 'one hundred and fifty-seven'), 158: ('cento e cinquenta e oito', 'one hundred and fifty-eight'), 159: ('cento e cinquenta e nove', 'one hundred and fifty-nine'),
       74: ('setenta e quatro', 'seventy-four')}


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
