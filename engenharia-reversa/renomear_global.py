r"""Renomeia uma função de topo do motor em toda chamada `velho(`, função por função (etapa 4 da reorganização,
08/11/2026): pula as funções que têm um local, parâmetro ou closure com o mesmo nome (`let lado = |...|`), e a função
que não compilar volta inteira ao texto de antes (a mesma volta do traduzir_locais.py). A definição muda junto; as
listas de mutação acompanham, trecho a trecho; os itens da fila também (só as chamadas, sem a checagem por função).

Uso: python renomear_global.py velho novo [--gravar]
"""
import glob
import json
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import traduzir_locais as tl  # noqa: E402


def main():
    velho, novo = sys.argv[1], sys.argv[2]
    local = re.compile(r'\blet (?:mut )?%s\b|[(,|]\s*(?:mut )?%s\s*:|\bfn %s\b' % ((re.escape(velho),) * 3))
    chamada = re.compile(r'\b%s\(' % re.escape(velho))
    codigo_todo = '\n'.join(open(p, encoding='utf-8').read() for p in glob.glob(os.path.join(SRC, '**', '*.rs'), recursive=True))
    if re.search(r'\b%s\b' % re.escape(novo), re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', ' ', codigo_todo)):
        sys.exit('%s já existe no código' % novo)
    mutacoes = []
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in sorted(os.listdir(pasta)):
        if f.endswith('.py'):
            try:
                mutacoes += [(f, m) for m in runpy.run_path(os.path.join(pasta, f)).get('MUTACOES', [])]
            except Exception:
                pass
    trocas_mut, total, pulados = {}, 0, 0
    f_ = lambda s: chamada.sub(novo + '(', s)  # noqa: E731
    for p in glob.glob(os.path.join(SRC, '**', '*.rs'), recursive=True):
        t = open(p, encoding='utf-8').read()
        tl.ANTES[p] = t
        partes, pos = [], 0
        for nome, a, b in mover_rotinas.blocos(t):
            bloco = t[a:b]
            if nome == velho:  # a definição
                nb = re.sub(r'\bfn %s\b' % re.escape(velho), 'fn ' + novo, bloco, count=1)
            else:
                cod = re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', ' ', bloco)
                if not chamada.search(cod):
                    continue
                if local.search(cod):
                    pulados += 1
                    continue
                nb = tl.so_codigo(bloco, f_)
            if nb == bloco:
                continue
            total += 1
            for lista, m in mutacoes:
                if bloco.count(m[1]) == 1:
                    trocas_mut[(lista, m[0])] = (m, f_)
            partes.append(t[pos:a])
            partes.append(nb)
            pos = b
        partes.append(t[pos:])
        if ''.join(partes) != t and '--gravar' in sys.argv:
            open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d funções trocadas, %d puladas (local com o mesmo nome); %d trechos de mutação' % (total, pulados, len(trocas_mut)))
    if '--gravar' not in sys.argv:
        return
    tl.voltar_funcoes_com_erro(trocas_mut)
    por_lista = {}
    for (lista, _), (m, g) in trocas_mut.items():
        por_lista.setdefault(lista, []).append((m, g))
    for lista, ms in por_lista.items():
        p = os.path.join(pasta, lista)
        s = open(p, encoding='utf-8').read()
        for m, g in ms:
            ini = max(s.find("('%s'," % m[0]), s.find('("%s",' % m[0]))
            if ini < 0:
                continue
            pos = ini
            for k in (1, 2):
                nv = tl.so_codigo(m[k], g)
                if nv == m[k]:
                    continue
                for v, n2 in ((repr(m[k]), repr(nv)), (json.dumps(m[k], ensure_ascii=False), json.dumps(nv, ensure_ascii=False))):
                    j = s.find(v, pos)
                    if j >= 0:
                        s = s[:j] + n2 + s[j + len(v):]
                        pos = j + len(n2)
                        break
        open(p, 'w', encoding='utf-8').write(s)
    print('gravado')


if __name__ == '__main__':
    main()
