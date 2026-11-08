r"""Dá nome aos campos dos objetos de classe (`this + 0x3e` -> `this + league::CLUB_COUNT`), pelas tabelas propostas
por leitura do código (etapa 4 da reorganização, 08/11/2026). Entrada: TSV `<deslocamento>\t<escopo>\t<NOME>\t<bytes>\t
<descrição>`, um por família (League, Cup, TRANSFER_MANAGER, Friendly...).

Numa função com `Original: <Classe>::...`, `this` é um objeto daquela classe: valem os campos `base` da família da
classe e os do escopo igual à classe. A família de uma classe: League (as ligas de comp/leagues/, League, League_Stage,
SubLeague), Cup (as copas de comp/cups/, eurocomp/, intercomp/, worldcup/, Cup, Cup_Stage) ou a própria classe. Só troca
`this + 0x..` no código (não em comentário nem em texto); as constantes vão para fields.rs, um módulo por família
(base) ou por subclasse. O valor é o mesmo: o código gerado não muda. As listas de mutação acompanham, trecho a trecho.

Uso: python aplicar_campos_classes.py campos_League.tsv [...] [--gravar]
"""
import collections
import json
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
BASE = os.path.join(SRC, 'motor')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import traduzir_locais  # noqa: E402


def snake(x):
    return re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', x).replace('-', '_').lower()


# "copas" com o leiaute de liga (+0xb1 ponteiro da tabela, +0xbe..+0xc8...), achado na leitura da família Cup: ficam com
# os campos da família League
LEIAUTE_DE_LIGA = {'GoldCup', 'ConmebolLiber', 'ConMercCup', 'ConmebolMerc', 'OceaniaClubChamp', 'AfricanNations',
                   'WorldCup', 'ItaCup', 'EuroChampQual', 'WcEuropeLeague', 'WcAsiaLeague', 'WcSouthAmericanLeague',
                   'WcOceaniaLeague', 'WcAfricanCup', 'WcConcacafCup'}


def familia_de(cls, rel):
    if cls in LEIAUTE_DE_LIGA:
        return 'League'
    if cls in ('League', 'League_Stage', 'SubLeague') or rel.startswith('comp/leagues/'):
        return 'League'
    if cls in ('Cup', 'Cup_Stage') or rel.startswith(('comp/cups/', 'comp/eurocomp/', 'comp/intercomp/', 'comp/worldcup/')):
        return 'Cup'
    return cls


def main():
    tabelas = collections.defaultdict(dict)  # familia -> {(escopo, off): (nome, bytes, desc)}
    for t in [a for a in sys.argv[1:] if not a.startswith('--')]:
        fam = re.sub(r'^campos_|\.tsv$', '', os.path.basename(t))
        for l in open(t, encoding='utf-8'):
            p = [x.strip() for x in l.rstrip('\n').split('\t')]
            if len(p) >= 3 and re.fullmatch(r'0x[0-9a-f]+', p[0]) and re.fullmatch(r'[A-Z][A-Z0-9_]*', p[2]):
                v = (p[2], p[3] if len(p) > 3 else '?', p[4] if len(p) > 4 else '')
                tabelas[fam][(p[1], int(p[0], 16))] = v
                # um escopo de classe (ex. PLAYER_REGEN no arquivo do Friendly) vale também como a base dessa classe
                if p[1] not in ('base', 'offer'):
                    tabelas[p[1]][('base', int(p[0], 16))] = v
    usados = collections.defaultdict(dict)  # modulo -> {NOME: (off, bytes, desc)}
    # nomes de módulo do motor (o módulo de campos não pode repetir um: transfer_manager -> transfer_manager_fields)
    motor_rs = open(os.path.join(SRC, 'motor.rs'), encoding='utf-8').read()
    ocupados = set(re.findall(r'pub\(crate\) mod (\w+);', motor_rs))

    def nome_mod(x):
        m_ = snake(x)
        return m_ + '_fields' if m_ in ocupados else m_
    mutacoes = []
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in sorted(os.listdir(pasta)):
        if f.endswith('.py'):
            try:
                mutacoes += [(f, m) for m in runpy.run_path(os.path.join(pasta, f)).get('MUTACOES', [])]
            except Exception:
                pass
    trocas_mut = {}
    total = 0
    for d, _, fs in os.walk(BASE):
        for f in fs:
            if not f.endswith('.rs') or f == 'fields.rs':
                continue
            p = os.path.join(d, f)
            rel = os.path.relpath(p, BASE).replace(os.sep, '/')
            rel_logico = rel[len('compare/'):] if rel.startswith('compare/') else rel
            t = open(p, encoding='utf-8').read()
            partes, pos = [], 0
            for nome, a, b in mover_rotinas.blocos(t):
                bloco = t[a:b]
                m = re.search(r'Original: `([A-Za-z_0-9]+)::', bloco)
                if not m:
                    continue
                cls = m.group(1)
                fam = familia_de(cls, rel_logico)
                tab = tabelas.get(fam)
                if not tab:
                    continue
                mapa = {}
                for (esc, off), (n_, by, de) in tab.items():
                    if esc == 'base' or esc == cls or (esc.lower() == 'offer' and False):
                        mod = nome_mod(fam) if esc == 'base' else nome_mod(esc)
                        mapa[off] = (mod, n_, by, de)
                if not mapa:
                    continue

                def f_(s, mapa=mapa):
                    def r(mm):
                        off = int(mm.group(1), 16)
                        if off not in mapa:
                            return mm.group(0)
                        mod, n_, by, de = mapa[off]
                        usados[mod][n_] = (off, by, de)
                        return 'this + %s::%s' % (mod, n_)
                    return re.sub(r'\bthis \+ (0x[0-9a-f]+)\b', r, s)
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
            novo_t = ''.join(partes)
            if novo_t != t and '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8').write(novo_t)
    print('%d linhas com campos de classe; módulos: %s' % (total, ', '.join('%s(%d)' % (k, len(v)) for k, v in sorted(usados.items()))))
    if '--gravar' not in sys.argv:
        return
    # as listas de mutação, tupla a tupla
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
    # fields.rs: os módulos novos
    pf = os.path.join(BASE, 'fields.rs')
    texto = open(pf, encoding='utf-8').read()
    novos = []
    for mod, campos in sorted(usados.items()):
        if 'pub(crate) mod %s {' % mod in texto:
            continue
        novos.append('/// Fields of the `%s` objects (`this` in its routines), named from how the routines use them. [pt] Campos dos objetos `%s`.' % (mod, mod))
        novos.append('#[allow(dead_code)]')
        novos.append('pub(crate) mod %s {' % mod)
        for n_, (off, by, de) in sorted(campos.items(), key=lambda x: x[1][0]):
            novos.append('    pub(crate) const %s: usize = %#x; // %s byte(s): %s' % (n_, off, by, de))
        novos.append('}')
        novos.append('')
    open(pf, 'w', encoding='utf-8').write(texto.rstrip('\n') + '\n\n' + '\n'.join(novos))
    print('gravado')


if __name__ == '__main__':
    main()
