r"""Dá nome aos campos dos objetos do motor de partidas: `m + 0x8eae` -> `m + engine::POSSESSION_SIDE`,
`j + 0x102` -> `j + match_player::X_POS` (etapa 4 da reorganização, 08/11/2026). Entrada: as tabelas propostas por
leitura (`campos_engine.tsv`, `campos_match_player.tsv`: `<desloc>\t<NOME>\t<bytes>\t<descrição>` e linhas
`EXCLUDE\t<função>` onde a variável não é aquele objeto). Vale só nos arquivos do motor (match_*.rs e os comparadores
deles), só no código; o valor é o mesmo (o código gerado não muda). As listas de mutação acompanham, função a função.

Uso: python aplicar_campos_motor.py campos_engine.tsv campos_match_player.tsv [--gravar]
"""
import collections
import json
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = os.path.join(R, 'rust', 'src', 'motor')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import traduzir_locais  # noqa: E402

VARS = {'engine': ('m', 'engine'), 'match_player': ('j',)}
ARQS = ('match_eng.rs', 'match_pl.rs', 'match_day.rs', 'match_man.rs', 'match_events.rs')


def main():
    tabelas, excluir = {}, collections.defaultdict(set)
    for t in [a for a in sys.argv[1:] if not a.startswith('--')]:
        obj = re.sub(r'^campos_|\.tsv$', '', os.path.basename(t))
        tabelas[obj] = {}
        for l in open(t, encoding='utf-8'):
            p = [x.strip() for x in l.rstrip('\n').split('\t')]
            if len(p) >= 2 and p[0] == 'EXCLUDE':
                excluir[obj].add(p[1])
            elif len(p) >= 2 and re.fullmatch(r'0x[0-9a-f]+', p[0]) and re.fullmatch(r'[A-Z][A-Z0-9_]*', p[1]):
                tabelas[obj][int(p[0], 16)] = (p[1], p[2] if len(p) > 2 else '?', p[3] if len(p) > 3 else '')
    usados = collections.defaultdict(dict)
    mutacoes = []
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in sorted(os.listdir(pasta)):
        if f.endswith('.py'):
            try:
                mutacoes += [(f, m) for m in runpy.run_path(os.path.join(pasta, f)).get('MUTACOES', [])]
            except Exception:
                pass
    trocas_mut, total = {}, 0
    for p in [os.path.join(BASE, a) for a in ARQS] + [os.path.join(BASE, 'compare', a) for a in ARQS]:
        if not os.path.exists(p):
            continue
        t = open(p, encoding='utf-8').read()
        partes, pos = [], 0
        for nome, a, b in mover_rotinas.blocos(t):
            ativos = {obj: tab for obj, tab in tabelas.items() if nome not in excluir[obj]
                      and nome.replace('comparar_', '') not in excluir[obj]}
            if not ativos:
                continue

            def f_(s, ativos=ativos):
                for obj, tab in ativos.items():
                    vs = '|'.join(VARS.get(obj, ()))
                    if not vs:
                        continue

                    def r(mm, obj=obj, tab=tab):
                        off = int(mm.group(2), 16)
                        if off not in tab:
                            return mm.group(0)
                        n_, by, de = tab[off]
                        usados[obj][n_] = (off, by, de)
                        return '%s + %s::%s' % (mm.group(1), obj, n_)
                    s = re.sub(r'\b(%s) \+ (0x[0-9a-f]+)\b' % vs, r, s)
                return s
            bloco = t[a:b]
            novo = traduzir_locais.so_codigo(bloco, f_)
            if novo == bloco:
                continue
            total += sum(1 for x, y in zip(bloco.split('\n'), novo.split('\n')) if x != y)
            for lista, mu in mutacoes:
                if bloco.count(mu[1]) == 1:
                    trocas_mut[(lista, mu[0])] = (mu, f_)
            partes.append(t[pos:a])
            partes.append(novo)
            pos = b
        partes.append(t[pos:])
        if ''.join(partes) != t and '--gravar' in sys.argv:
            open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d linhas; %s' % (total, ', '.join('%s(%d)' % (k, len(v)) for k, v in usados.items())))
    if '--gravar' not in sys.argv:
        return
    por = collections.defaultdict(list)
    for (lista, _), v in trocas_mut.items():
        por[lista].append(v)
    for lista, ms in por.items():
        p = os.path.join(pasta, lista)
        s = open(p, encoding='utf-8').read()
        for mu, f_ in ms:
            ini = max(s.find("('%s'," % mu[0]), s.find('("%s",' % mu[0]))
            if ini < 0:
                continue
            pos = ini
            for k in (1, 2):
                nv = traduzir_locais.so_codigo(mu[k], f_)
                if nv == mu[k]:
                    continue
                for v, n2 in ((repr(mu[k]), repr(nv)), (json.dumps(mu[k], ensure_ascii=False), json.dumps(nv, ensure_ascii=False))):
                    j = s.find(v, pos)
                    if j >= 0:
                        s = s[:j] + n2 + s[j + len(v):]
                        pos = j + len(n2)
                        break
        open(p, 'w', encoding='utf-8').write(s)
    pf = os.path.join(BASE, 'fields.rs')
    texto = open(pf, encoding='utf-8').read()
    for obj, campos in usados.items():
        linhas = ['    pub(crate) const %s: usize = %#x; // %s byte(s): %s' % (n_, off, by, de)
                  for n_, (off, by, de) in sorted(campos.items(), key=lambda x: x[1][0])]
        texto = texto.rstrip('\n') + '\n\n' + '\n'.join(['/// Fields of the match `%s` object, named from how the engine uses them. [pt] Campos do objeto `%s` do motor.' % (obj, obj),
                                                         '#[allow(dead_code)]', 'pub(crate) mod %s {' % obj] + linhas + ['}', ''])
    open(pf, 'w', encoding='utf-8').write(texto)
    print('gravado')


if __name__ == '__main__':
    main()
