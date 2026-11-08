r"""O prefixo de arquivo certo no nome das rotinas (etapa 4 da reorganização, 08/11/2026). O renomear_rotinas.py pôs o
arquivo na frente dos nomes repetidos (`eng_cc_cup_create_round_table`), mas o arquivo era o da seção onde a rotina
estava, não o do .cpp dela; o mover_rotinas.py já a levou ao módulo certo (`comp/cups/eng_fa_cup.rs`). Aqui o nome
troca o prefixo do outro módulo pelo do módulo onde ela mora (`eng_fa_cup_create_round_table`), e o endereço no fim
sai quando o nome sem ele está livre. Comparador, IDX_, fila, listas de mutação e docs/NOMES.csv acompanham.

Uso: python renomear_prefixo.py [--gravar]
"""
import csv
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import renomear_rotinas as rr  # noqa: E402


def main():
    import cabecalhos
    orig = {}
    for l in csv.DictReader(open(rr.NOMES, encoding='utf-8')):
        orig[l['en']] = l['original']
    orig.update({k: v for k, v in cabecalhos.originais().items() if v})
    modulo, cpp_de = {}, {}
    for p in rr.fontes():
        if os.sep + 'motor' + os.sep in p and os.sep + 'compare' + os.sep not in p:
            t = open(p, encoding='utf-8').read()
            for n, a, b in mover_rotinas.blocos(t):
                modulo[n] = os.path.splitext(os.path.basename(p))[0]
                m = mover_rotinas.ORIG.search(t[a:b])
                if m:
                    cpp_de[n] = os.path.splitext(m.group(2).lower())[0]
    stems = sorted(set(modulo.values()), key=len, reverse=True)
    codigo = '\n'.join(open(p, encoding='utf-8').read() for p in rr.fontes())
    codigo = re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', ' ', codigo)
    idents = set(re.findall(r'\b[A-Za-z_]\w*\b', codigo))
    troca = {}
    for n, mod in modulo.items():
        if cpp_de.get(n) != mod or n.startswith(mod + '_'):
            continue
        outro = next((s for s in stems if s != mod and n.startswith(s + '_')), None)
        if not outro:
            continue
        resto = re.sub(r'_[0-9a-f]{6}$', '', n[len(outro) + 1:])
        met = orig.get(n, '').rpartition('::')[2]
        if not met or rr.snake(met) != resto:  # o prefixo é parte do sentido, não o arquivo posto pelo renomear_rotinas
            continue
        novo = mod + '_' + n[len(outro) + 1:]
        curto = re.sub(r'_[0-9a-f]{6}$', '', novo)
        for c in (curto, novo):
            if c not in idents and c not in troca.values():
                troca[n] = c
                break
    print('%d rotinas com o prefixo do arquivo certo' % len(troca))
    for n in list(troca)[:12]:
        print('  %s -> %s' % (n, troca[n]))
    if '--gravar' not in sys.argv:
        return
    mapa = dict(troca)
    mapa.update({'compare_' + a: 'compare_' + b for a, b in troca.items()})
    mapa.update({'IDX_' + a.upper(): 'IDX_' + b.upper() for a, b in troca.items()})
    rx = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(mapa, key=len, reverse=True))))

    def trocar(t):
        return rx.sub(lambda m: mapa[m.group(1)], t)
    for p in rr.fontes():
        t = open(p, encoding='utf-8').read()
        if trocar(t) != t:
            open(p, 'w', encoding='utf-8').write(trocar(t))
    fila = os.path.join(R, 'levas', 'fila')
    for item in os.listdir(fila):
        for f in ('codigo.rs', 'mutacoes.py'):
            p = os.path.join(fila, item, f)
            if os.path.exists(p) and trocar(open(p, encoding='utf-8').read()) != open(p, encoding='utf-8').read():
                open(p, 'w', encoding='utf-8').write(trocar(open(p, encoding='utf-8').read()))
    import trocar_nas_mutacoes
    trocar_nas_mutacoes.aplicar(trocar)
    linhas = list(csv.DictReader(open(rr.NOMES, encoding='utf-8')))
    for l in linhas:
        l['en'] = troca.get(l['en'], l['en'])
    with open(rr.NOMES, 'w', encoding='utf-8', newline='') as f:
        w = csv.DictWriter(f, ['pt', 'en', 'endereco', 'original'])
        w.writeheader()
        w.writerows(linhas)
    print('gravado')


if __name__ == '__main__':
    main()
