r"""Narrows the return type of exe:: wrappers whose routine only defines al/ax (09/11/2026, conferir_assinaturas.py):
records the new signature in levas/reorganizacao/assinaturas_conferidas.json, rewrites the wrapper and wraps every
existing call `exe::name(..)` as `(exe::name(..) as OLD)`, in the sources and in the mutation texts, so each site keeps
its type but reads only the defined bits.
Usage: python estreitar_retorno.py FILE.json [--gravar], FILE.json = {"FN_X": {"width": 1|2, "signed": bool (callers movsx), "evidence": "..."}}
[pt] Estreita o retorno dos invólucros cuja rotina só define al/ax: grava a assinatura conferida, reescreve o invólucro
e envolve cada chamada existente com `as` do tipo antigo, nas fontes e nos textos das mutações.
"""
import json
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import involucros_exe as ie  # noqa: E402

CALLS = os.path.join(R, 'rust', 'src', 'motor', 'exe_calls.rs')
CONF = os.path.join(R, 'levas', 'reorganizacao', 'assinaturas_conferidas.json')
NARROW = {('u', 1): 'u8', ('u', 2): 'u16', ('i', 1): 'i8', ('i', 2): 'i16'}


def envolver(texto, nome, velho):
    out, pos = [], 0
    for m in re.finditer(r'(?<![\w:])exe::%s\(' % re.escape(nome), texto):
        if m.start() < pos:
            continue
        args, fim = ie.args_da_chamada(texto, m.end() - 1)
        if args is None:
            continue
        out += [texto[pos:m.start()], '(%s as %s)' % (texto[m.start():fim], velho)]
        pos = fim
    out.append(texto[pos:])
    return ''.join(out)


def main():
    pedidos = json.load(open(sys.argv[1], encoding='utf-8'))
    grava = '--gravar' in sys.argv
    calls = open(CALLS, encoding='utf-8').read()
    conf = json.load(open(CONF, encoding='utf-8'))
    trocas = []
    for fn, p in sorted(pedidos.items()):
        m = re.search(r'let f: (unsafe extern "\w+" fn\([^)]*\)(?: -> ([\w*: ]+?))?) = core::mem::transmute\(%s\);' % fn, calls)
        if not m or not m.group(2):
            print('sem invólucro com retorno:', fn)
            continue
        velho = m.group(2).strip()
        if ie.classe(velho) != 'int' or ie.largura(velho) != 4:
            print('retorno não é inteiro de 4 bytes:', fn, velho)
            continue
        novo = NARROW[('i' if velho.startswith('i') or p.get('signed') else 'u', p['width'])]
        sig = m.group(1)[:m.group(1).rindex('->')] + '-> ' + novo
        conf[fn] = {'signature': sig, 'confidence': p.get('confidence', 'high'), 'evidence': p['evidence'],
                    'wrong_sites': [m.group(1)], 'source': 'conferir_assinaturas.py'}
        trocas.append((ie.nome_de(fn), velho, novo))
        print('%-55s %s -> %s' % (fn, velho, novo))
    if not grava:
        return
    json.dump(dict(sorted(conf.items())), open(CONF, 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    # the wrappers themselves: involucros_exe.py --so-arquivo regenerates them from the checked signatures
    os.system('python "%s" --so-arquivo' % os.path.join(R, 'engenharia-reversa', 'involucros_exe.py'))
    for raiz, _, fs in os.walk(os.path.join(R, 'rust', 'src')):
        for f in fs:
            p = os.path.join(raiz, f)
            if not f.endswith('.rs') or p == CALLS:
                continue
            t = open(p, encoding='utf-8').read()
            t2 = t
            for nome, velho, _ in trocas:
                t2 = envolver(t2, nome, velho)
            if t2 != t:
                open(p, 'w', encoding='utf-8').write(t2)
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in sorted(os.listdir(pasta)):
        if not f.endswith('.py'):
            continue
        p = os.path.join(pasta, f)
        t = open(p, encoding='utf-8').read()
        t2 = t
        for nome, velho, _ in trocas:
            t2 = envolver(t2, nome, velho)
        if t2 != t:
            open(p, 'w', encoding='utf-8').write(t2)
            print('mutações:', f)


if __name__ == '__main__':
    main()
