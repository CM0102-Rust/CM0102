r"""The comparers' messages in English (10/11/2026, Lucas: everything that can move to English moves). A comparer
prints a divergence as `"<routine>(<args>) original=.. rust=.."`, with the routine's old Portuguese name; that name
becomes the Rust function's (docs/NOMES_ROTULOS.csv: `motor_<old>` -> new), and the other Portuguese words of the
message go through the word dictionary of traduzir_locais.py (only whole words found there; numbers, `{..}`
placeholders and hex stay). Only string literals of `format!`/`push`/`Some` inside the comparers (rust/src/motor/compare/
and the `compare_*` functions) are touched: no tool reads those texts (the log keywords the tools parse change in
their own batch). The mutation snippets follow.
Usage: python textos_em_ingles.py [--todos] [--gravar]   (--todos: also the debug messages outside the comparers)
[pt] As mensagens dos comparadores em inglês: o nome antigo da rotina vira o nome Rust e as palavras passam pelo
dicionário do traduzir_locais.py.
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import tipar_registros as tr  # noqa: E402
import traduzir_locais as tl  # noqa: E402
import legibilidade as lg  # noqa: E402

EXTRA = {'original': 'original', 'esperado': 'expected', 'obtido': 'got', 'casa': 'cell', 'entrada': 'entry',
         'contagem': 'count', 'lado': 'side', 'pessoa': 'person', 'clube': 'club', 'nacao': 'nation', 'jogador': 'player',
         'vaga': 'slot', 'semana': 'week', 'data': 'date', 'dia': 'day', 'ano': 'year', 'mes': 'month', 'liga': 'league',
         'copa': 'cup', 'tabela': 'table', 'linha': 'row', 'antes': 'before', 'depois': 'after', 'igual': 'equal',
         'diferente': 'different', 'sem': 'without', 'com': 'with', 'nada': 'nothing', 'ficha': 'sheet',
         'contrato': 'contract', 'oferta': 'offer', 'partida': 'match', 'motor': 'engine', 'quantos': 'count',
         'primeiro': 'first', 'ultimo': 'last', 'retorno': 'return', 'devolve': 'returns', 'retornou': 'returned',
         'devolveu': 'returned', 'memoria': 'memory', 'regiao': 'region', 'pilha': 'stack', 'erro': 'error',
         'lista': 'list', 'item': 'item', 'itens': 'items', 'campo': 'field', 'campos': 'fields', 'bloco': 'block',
         'e': 'and', 'ou': 'or', 'de': 'of', 'do': 'of the', 'da': 'of the', 'em': 'in', 'no': 'in the', 'na': 'in the',
         'diferencas': 'differences', 'diferenca': 'difference', 'campeoes': 'champions', 'suspensao': 'suspension',
         'cabecalho': 'header', 'fronteira': 'boundary', 'atualizada': 'updated', 'confianca': 'confidence', 'comeco': 'start',
         'proxima': 'next', 'permissoes': 'permissions', 'so': 'only', 'estaticas': 'static', 'prestacoes': 'instalments',
         'conferencias': 'checks', 'validas': 'valid', 'origem': 'source',
         'para': 'for', 'pelo': 'by the', 'pela': 'by the', 'os': 'the', 'as': 'the', 'o': 'the', 'a': 'the'}
CURTAS = {'e', 'ou', 'de', 'do', 'da', 'em', 'no', 'na', 'os', 'as', 'o', 'a'}


def mapa_rotinas():
    out = {}
    with open(os.path.join(R, 'docs', 'NOMES_ROTULOS.csv'), encoding='utf-8') as f:
        for l in list(f)[1:]:
            velho, novo = l.strip().split(',')
            out[velho[len('motor_'):]] = novo
    return out


def traduzir_texto(s, rotinas, propria=None):
    # the routine name before `(`: in compare_X, a Portuguese name at the start is X; elsewhere the CSV map
    if propria:
        m0 = re.match(r'([a-z_][a-z0-9_]*)(?=\()', s)
        if m0 and m0.group(1) != propria and (m0.group(1) in rotinas or lg.portugues(m0.group(1))):
            s = propria + s[m0.end():]
    s = re.sub(r'(?<![\w{])([a-z_][a-z0-9_]*)(?=\()', lambda m: rotinas.get(m.group(1), m.group(1)), s)

    def palavra(m):
        w = m.group(0)
        if '_' in w:
            partes = w.split('_')
            if all(p in tl.D or p in EXTRA for p in partes if p):
                return '_'.join(EXTRA.get(p, tl.D.get(p, p)).replace(' ', '_') for p in partes)
            return w
        lw = sem_acento(w.lower())
        if lw in CURTAS:
            return w  # a short word only goes with its sentence (below)
        t = EXTRA.get(lw) or tl.D.get(lw)
        if not t:
            return w
        return t.replace('_', ' ') if w.islower() else t.replace('_', ' ').capitalize()
    partes = re.split(r'(\{[^}]*\})', s)
    for k in range(0, len(partes), 2):
        txt = re.sub(r'(?<![\w{%])[A-Za-zÀ-ú][a-zà-ú_]*(?![\w}])', palavra, partes[k])
        # the short Portuguese words, only between words that were translated or are English
        txt = re.sub(r'(?<=[a-z] )(e|ou|de|do|da|em|no|na|os|as)(?= [a-z{])', lambda m: EXTRA[m.group(1)], txt)
        partes[k] = txt
    return ''.join(partes)


def sem_acento(w):
    import unicodedata
    return ''.join(c for c in unicodedata.normalize('NFD', w) if unicodedata.category(c) != 'Mn')


def mensagem(s):
    """A debug message (`--todos`): has a `{}` placeholder and is not a path or a file name."""
    return '{' in s and ' ' in s and not re.search(r'[/\\]|\.\w{2,4}\b(?!\))', s)


def converter(bloco, rotinas, todo=False, todos=False):
    mk = tr.mascara(bloco)
    if not todo and not todos and not re.search(r'\bfn compare_\w+', mk):
        return bloco, 0
    out, pos, n = [], 0, 0
    fm = re.search(r'fn compare_(\w+)', mk)
    propria = fm.group(1) if fm else None
    for m in re.finditer(r'(?:format!|push|Some|record|linha)\s*\(\s*(?:\(\s*)?"((?:[^"\\]|\\.)*)"', bloco):
        a, b = m.start(1), m.end(1)
        if mk[a - 1] != ' ' and mk[a:b].strip():
            continue  # inside a comment or not a literal
        if todos and not todo and not propria and not mensagem(bloco[a:b]):
            continue
        novo = traduzir_texto(bloco[a:b], rotinas, propria)
        if novo != bloco[a:b]:
            out += [bloco[pos:a], novo]
            pos = b
            n += 1
    out.append(bloco[pos:])
    return ''.join(out), n


def main():
    grava = '--gravar' in sys.argv
    rotinas = mapa_rotinas()
    mutacoes = tr.carregar_mutacoes()
    trocas_mut, n = {}, 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs'):
                continue
            p = os.path.join(raiz, f)
            todo = os.sep + 'compare' + os.sep in p
            t = open(p, encoding='utf-8').read()
            tl.ANTES[p] = t
            partes, pos = [], 0
            for _, a, b in mover_rotinas.blocos(t):
                bloco = t[a:b]
                novo, k = converter(bloco, rotinas, todo, '--todos' in sys.argv)
                if not k:
                    continue
                n += k
                for lista, mu in mutacoes:
                    if bloco.count(mu[1]) == 1:
                        trocas_mut[(lista, mu[0])] = (mu, (lambda s, todo=todo: converter(s, rotinas, True)[0]))
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and ''.join(partes) != t:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d mensagens de comparador traduzidas' % n)
    if grava:
        tl.voltar_funcoes_com_erro(trocas_mut)
        tr.gravar_mutacoes(trocas_mut)


if __name__ == '__main__':
    main()
