r"""Arruma o rust/src/motor/fields.rs (etapa 4 da reorganização, 08/11/2026; o leitor de fora, rodada 7):
- funde um módulo em outro que descreve o mesmo registro (`staff_contract` em `contract`): o nome do módulo que some
  substitui um `UNKNOWN*` no mesmo deslocamento; os outros campos entram; os usos `staff_contract::X` passam a
  `contract::<nome final>`;
- renomeia constantes (`staff::LOYALITY` -> `LOYALTY`) e tira a repetição do nome do módulo (`city::CITY_NATION` ->
  `city::NATION`), quando o nome curto está livre no módulo e não começa com algarismo;
- ordena as constantes de cada módulo pelo deslocamento.
As trocas valem no código, nas listas de mutação e na fila. Só nomes: o código gerado não muda.

Uso: python arrumar_campos.py [--gravar]
"""
import glob
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
F = os.path.join(R, 'rust', 'src', 'motor', 'fields.rs')
FUNDIR = [('staff_contract', 'contract')]
RENOMEAR = {('staff', 'LOYALITY'): 'LOYALTY'}
PREFIXOS = {'city': 'CITY_', 'stadium': 'STADIUM_', 'competition': 'CLUB_COMP_', 'nation': 'NATION_', 'club': 'CLUB_'}
MOD = re.compile(r'^pub\(crate\) mod (\w+) \{\n(.*?)^\}', re.M | re.S)
CONST = re.compile(r'^(\s*pub\(crate\) const )(\w+)(: usize = )(0x[0-9a-f]+|\d+)(;.*)$')


def main():
    t = open(F, encoding='utf-8').read()
    trocas = {}  # 'mod::OLD' -> 'mod::NEW'
    for de, para in FUNDIR:
        md, mp = MOD.search(t, 0), None
        corpos = {m.group(1): m for m in MOD.finditer(t)}
        if de not in corpos or para not in corpos:
            continue
        cd, cp = corpos[de].group(2), corpos[para].group(2)
        lp = cp.rstrip('\n').split('\n')
        por_off = {}
        for i, l in enumerate(lp):
            m = CONST.match(l)
            if m:
                por_off.setdefault(int(m.group(4), 0), (i, m.group(2)))
        nomes_p = {CONST.match(l).group(2) for l in lp if CONST.match(l)}
        for l in cd.rstrip('\n').split('\n'):
            m = CONST.match(l)
            if not m:
                continue
            nome, off = m.group(2), int(m.group(4), 0)
            if off in por_off:
                i, atual = por_off[off]
                if atual.startswith('UNKNOWN') and nome not in nomes_p:
                    lp[i] = CONST.sub(lambda k: k.group(1) + nome + k.group(3) + k.group(4) + m.group(5), lp[i])
                    trocas['%s::%s' % (para, atual)] = '%s::%s' % (para, nome)
                    nomes_p.add(nome)
                    trocas['%s::%s' % (de, nome)] = '%s::%s' % (para, nome)
                else:
                    trocas['%s::%s' % (de, nome)] = '%s::%s' % (para, atual)
            else:
                final = nome if nome not in nomes_p else nome + '_2'
                nomes_p.add(final)
                lp.append(m.group(1) + final + m.group(3) + m.group(4) + m.group(5))
                trocas['%s::%s' % (de, nome)] = '%s::%s' % (para, final)
        mp = corpos[para]
        t = t[:mp.start(2)] + '\n'.join(lp) + '\n' + t[mp.end(2):]
        md = MOD.search(t, t.index('pub(crate) mod %s {' % de))
        ini = md.start()
        while True:  # doc lines and attributes right above the module
            k = t.rfind('\n', 0, ini - 1) + 1
            if t[k:ini].startswith(('///', '#[')):
                ini = k
            else:
                break
        t = t[:ini] + t[md.end() + 1:]
    # renames and the module-name stutter
    saida = []
    pos = 0
    for m in MOD.finditer(t):
        mod = m.group(1)
        linhas = m.group(2).rstrip('\n').split('\n')
        nomes = {CONST.match(l).group(2) for l in linhas if CONST.match(l)}
        novas = []
        for l in linhas:
            k = CONST.match(l)
            if not k:
                novas.append(l)
                continue
            nome = k.group(2)
            novo = RENOMEAR.get((mod, nome), nome)
            pre = PREFIXOS.get(mod)
            if pre and novo.startswith(pre):
                curto = novo[len(pre):]
                if curto and not curto[0].isdigit() and curto not in nomes:
                    novo = curto
            if novo != nome:
                nomes.discard(nome)
                nomes.add(novo)
                trocas['%s::%s' % (mod, nome)] = '%s::%s' % (mod, novo)
            novas.append(k.group(1) + novo + k.group(3) + k.group(4) + k.group(5))
        consts = sorted([l for l in novas if CONST.match(l)], key=lambda l: int(CONST.match(l).group(4), 0))
        outras = [l for l in novas if not CONST.match(l)]
        saida.append(t[pos:m.start(2)])
        saida.append('\n'.join(outras + consts) + '\n')
        pos = m.end(2)
    saida.append(t[pos:])
    t = ''.join(saida)
    print('%d trocas de nome' % len(trocas))
    if '--gravar' not in sys.argv:
        for a, b in list(trocas.items())[:20]:
            print('  ', a, '->', b)
        return
    open(F, 'w', encoding='utf-8').write(t)
    if not trocas:
        print('gravado (só a ordem)')
        return
    rx = re.compile(r'\b(%s)\b' % '|'.join(re.escape(x) for x in sorted(trocas, key=len, reverse=True)))
    fs = [p for p in glob.glob(os.path.join(R, 'rust', 'src', '**', '*.rs'), recursive=True) if not p.endswith('fields.rs')]
    fs += glob.glob(os.path.join(R, 'engenharia-reversa', 'mutacoes', '*.py'))
    fs += [p for p in glob.glob(os.path.join(R, 'levas', 'fila', '**', '*'), recursive=True) if p.endswith(('.rs', '.py'))]
    n = 0
    for p in fs:
        s = open(p, encoding='utf-8', newline='').read()
        s2 = rx.sub(lambda k: trocas[k.group(1)], s)
        if s2 != s:
            open(p, 'w', encoding='utf-8', newline='').write(s2)
            n += 1
    print('gravado; %d arquivos' % n)


if __name__ == '__main__':
    main()
