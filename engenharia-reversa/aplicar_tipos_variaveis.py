r"""Dá nome aos campos pelas variáveis cujo registro foi dito por leitura (etapa 4 da reorganização, 08/11/2026):
`p + 0x1e` -> `p + staff::SECOND_NATION` na função onde `p` é uma pessoa. Entrada: TSV
`<arquivo relativo a rust/src/motor>\t<função>\t<variável>\t<registro>` (registro do docs/FORMATOS.md: TStaff,
TPlayer...). Só o começo exato de um campo do leiaute, só no código (não em comentário nem em texto); o valor é o mesmo:
o código gerado não muda. As constantes entram em fields.rs (o mesmo módulo do nomear_campos.py); as listas de mutação
acompanham, trecho a trecho.

Uso: python aplicar_tipos_variaveis.py tipos_A.tsv [...] [--gravar]
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
import nomear_campos  # noqa: E402
import traduzir_locais  # noqa: E402

MODULO = {'TStaff': 'staff', 'TPlayer': 'player_sheet', 'TNonPlayer': 'non_player', 'TClub': 'club',
          'TNation': 'nation', 'TContract': 'contract', 'TComp': 'competition', 'TStadiums': 'stadium',
          'TCities': 'city', 'TPreferences': 'preferences', 'TContinent': 'continent'}


def modulos_de_campos():
    """{módulo do fields.rs: {deslocamento: CONSTANTE}} (a primeira constante de cada deslocamento)."""
    t = open(os.path.join(BASE, 'fields.rs'), encoding='utf-8').read()
    r = {}
    for mod, corpo in re.findall(r'^pub\(crate\) mod (\w+) \{\n(.*?)^\}', t, re.M | re.S):
        d = r.setdefault(mod, {})
        for n_, v in re.findall(r'const (\w+): usize = (0x[0-9a-f]+|\d+);', corpo):
            d.setdefault(int(v, 0), n_)
    return r


def main():
    lei = nomear_campos.leiautes()
    mods = modulos_de_campos()
    tipos = collections.defaultdict(dict)  # arquivo -> {função: {var: registro}}
    for t in [a for a in sys.argv[1:] if not a.startswith('--')]:
        for l in open(t, encoding='utf-8'):
            p = [x.strip() for x in l.rstrip('\n').split('\t')]
            if len(p) >= 4 and (p[3] in MODULO or p[3] in mods) and re.fullmatch(r'[a-z_][a-z0-9_]*', p[2]):
                tipos[p[0].replace('\\', '/')].setdefault(p[1], {})[p[2]] = p[3]
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
    for rel, funcs in sorted(tipos.items()):
        p = os.path.join(BASE, rel)
        if not os.path.exists(p):
            print('falta', rel)
            continue
        t = open(p, encoding='utf-8').read()
        partes, pos = [], 0
        for nome, a, b in mover_rotinas.blocos(t):
            vs = funcs.get(nome)
            if not vs:
                continue

            def f_(s, vs=vs):
                def r(mm):
                    reg = vs[mm.group(1)]
                    off = int(mm.group(2), 0)
                    if reg in mods and reg not in MODULO:  # um módulo do fields.rs: só as constantes que já existem
                        n_ = mods[reg].get(off)
                        return '%s + %s::%s' % (mm.group(1), reg, n_) if n_ else mm.group(0)
                    campo = lei.get(reg, {}).get(off)
                    if not campo:
                        return mm.group(0)
                    usados[MODULO[reg]][campo[0]] = (off, campo[1], campo[2], reg)
                    return '%s + %s::%s' % (mm.group(1), MODULO[reg], campo[0])
                return re.sub(r'\b(%s) \+ (0x[0-9a-f]+|[1-9][0-9]*)\b(?![.\w])' % '|'.join(map(re.escape, vs)), r, s)
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
    print('%d linhas; %s' % (total, ', '.join('%s(%d)' % (k, len(v)) for k, v in sorted(usados.items()))))
    if '--gravar' not in sys.argv:
        return
    por = collections.defaultdict(list)
    for (lista, _), v in trocas_mut.items():
        por[lista].append(v)
    for lista, ms in por.items():
        pl = os.path.join(pasta, lista)
        s = open(pl, encoding='utf-8').read()
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
        open(pl, 'w', encoding='utf-8').write(s)
    pf = os.path.join(BASE, 'fields.rs')
    texto = open(pf, encoding='utf-8').read()
    for mod in sorted(usados):
        consts = ['    pub(crate) const %s: usize = %#x; // %d byte(s), %s' % (n_, d, b_, ty)
                  for n_, (d, b_, ty, _) in sorted(usados[mod].items(), key=lambda x: x[1][0])]
        abre = 'pub(crate) mod %s {\n' % mod
        if abre in texto:
            i = texto.index(abre) + len(abre)
            fim = texto.index('\n}', i)
            dentro = texto[i:fim]
            novas = [c for c in consts if 'const %s:' % c.split('const ')[1].split(':')[0] not in dentro]
            texto = texto[:fim] + ''.join('\n' + c for c in novas) + texto[fim:]
        else:
            reg = next(iter(usados[mod].values()))[3]
            texto = texto.rstrip('\n') + '\n\n' + '\n'.join(['/// `%s` record. [pt] Registro `%s`.' % (reg, reg), '#[allow(dead_code)]', abre.rstrip('\n')] + consts + ['}', ''])
    open(pf, 'w', encoding='utf-8').write(texto)
    print('gravado')


if __name__ == '__main__':
    main()
