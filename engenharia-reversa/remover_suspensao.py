r"""Remove a suspensao de uma pessoa numa COPIA de save (.sav), como o recurso aceito do jogo (apply_appeal_result
0x557000): o modelo de bytes do editor para "remover suspensao" (docs/MAPA_DISCIPLINA.md §2, conferido no jogo em
docs/CONFERENCIA_SUSPENSAO.md).

Uso:
  python remover_suspensao.py ENTRADA                        (lista as suspensoes valendo e por quem)
  python remover_suspensao.py ENTRADA SAIDA --pessoa 51013 [--pessoa 29220 ...]

No discipline.dat (leiaute em arquivo, MAPA §1): registros por pessoa (N x 29), K regras (i32 n, n x 14 entradas,
i16 p, p x 0x70 pendentes; K = o unico que fecha o item), i16 b + b x 20 suspensoes, i16 m + m x 8 indice por
nacao (FIFA), i16 h + h x 10 guardadas. Para cada vaga k da pessoa com regra rs: a entrada idx da regra (conferida:
[e] == pessoa); tira os pendentes da regra com +0x1c == pessoa (0x7e91d0); se e+6 != -1, release_ban 0x556d00:
para cada vaga j com ban+0xc+j != -1, a entrada da pessoa na regra ban+0xc+j (pela vaga j do registro) fica com
+6 = -1; j = 3 (FIFA): sai o par (pessoa, nacao) do indice (fica ordenado; m - 1); a suspensao fica livre
(+0 = 0xff, +2 = 0xffff; b nao muda). Registro da pessoa, amarelos e historico ficam. Pessoa com vagas guardadas
(emprestimo, reg+0x1c > 0) nao e tratada: o script recusa.
A SAIDA e regravada com renomear_no_save.regravar e conferida (todos os outros itens iguais); a ENTRADA nunca muda.
"""
import argparse
import os
import struct
import sys

import prova_save as ps
from renomear_no_save import conferir, regravar


def ler(D, N):
    """O discipline.dat em pecas mutaveis."""
    o = N * 29
    for K in range(3, 44):
        try:
            q, regras = o, []
            for _ in range(K):
                n = struct.unpack_from('<i', D, q)[0]
                if not 0 <= n <= N:
                    raise ValueError
                ents = [bytearray(D[q + 4 + 14 * i:q + 18 + 14 * i]) for i in range(n)]
                if any(not -1 <= struct.unpack_from('<i', e)[0] < N for e in ents):
                    raise ValueError
                q += 4 + 14 * n
                p = struct.unpack_from('<h', D, q)[0]
                if p < 0:
                    raise ValueError
                pend = [D[q + 2 + 0x70 * i:q + 2 + 0x70 * (i + 1)] for i in range(p)]
                q += 2 + 0x70 * p
                regras.append([ents, pend])
            b = struct.unpack_from('<h', D, q)[0]
            bans = [bytearray(D[q + 2 + 20 * i:q + 22 + 20 * i]) for i in range(b)]
            q += 2 + 20 * b
            m = struct.unpack_from('<h', D, q)[0]
            idx = [struct.unpack_from('<ii', D, q + 2 + 8 * i) for i in range(m)]
            q += 2 + 8 * m
            h = struct.unpack_from('<h', D, q)[0]
            guard = D[q + 2:q + 2 + 10 * h]
            if min(b, m, h) >= 0 and q + 2 + 10 * h == len(D):
                return {'reg': bytearray(D[:o]), 'regras': regras, 'bans': bans, 'idx': idx, 'guard': (h, guard)}
        except (ValueError, struct.error):
            pass
    sys.exit('discipline.dat: nenhum K fecha o item')


def gravar(d):
    s = bytearray(d['reg'])
    for ents, pend in d['regras']:
        s += struct.pack('<i', len(ents)) + b''.join(ents) + struct.pack('<h', len(pend)) + b''.join(pend)
    s += struct.pack('<h', len(d['bans'])) + b''.join(d['bans'])
    s += struct.pack('<h', len(d['idx'])) + b''.join(struct.pack('<ii', *x) for x in d['idx'])
    return bytes(s + struct.pack('<h', d['guard'][0]) + d['guard'][1])


def vaga(d, s, k):
    """(regra, indice da entrada) da vaga k da pessoa s."""
    return struct.unpack_from('<bi', d['reg'], s * 29 + 5 * k)


def mapa_regras(d, N):
    """regra (0..42) -> posicao no arquivo, pelas vagas que apontam de volta (regras vazias ficam de fora)."""
    m = {}
    for pos, (ents, _) in enumerate(d['regras']):
        for i, e in enumerate(ents):
            s = struct.unpack_from('<i', e)[0]
            for k in range(4) if s >= 0 else ():
                rs, v = vaga(d, s, k)
                if rs != -1 and v == i:
                    assert m.setdefault(rs, pos) == pos, 'regra %d em duas posicoes' % rs
    assert list(m) == sorted(m) and list(m.values()) == sorted(m.values()), 'ordem das regras'
    return m


def entrada(d, mapa, s, rs, k):
    """A entrada da pessoa s na regra rs pela vaga k (como 0x7e82e0), ou None."""
    r, i = vaga(d, s, k)
    if r != rs or rs not in mapa:
        return None
    ents = d['regras'][mapa[rs]][0]
    return ents[i] if 0 <= i < len(ents) and struct.unpack_from('<i', ents[i])[0] == s else None


def release_ban(d, mapa, s, b):
    """0x556d00."""
    ban = d['bans'][b]
    for j in range(4):
        rs = struct.unpack_from('<b', ban, 0xc + j)[0]
        if rs == -1:
            continue
        e = entrada(d, mapa, s, rs, j)
        if e is not None:
            struct.pack_into('<h', e, 6, -1)
        if j == 3:
            par = [x for x in d['idx'] if x[0] == s]
            assert len(par) == 1, 'pessoa %d: %d pares no indice por nacao' % (s, len(par))
            d['idx'].remove(par[0])
            assert d['idx'] == sorted(d['idx'], key=lambda x: (x[1], x[0])), 'indice fora de ordem'
    ban[0] = 0xff
    struct.pack_into('<h', ban, 2, -1)


def remover(d, mapa, s):
    """apply_appeal_result: o que foi tirado (para o relatorio)."""
    assert d['reg'][s * 29 + 0x1c] == 0, 'pessoa %d tem vagas guardadas (emprestimo): nao tratado' % s
    feito = []
    for k in range(4):
        rs, i = vaga(d, s, k)
        if rs == -1 or rs not in mapa:
            continue
        e = entrada(d, mapa, s, rs, k)
        if e is None:
            continue
        regra = d['regras'][mapa[rs]]
        antes = len(regra[1])
        regra[1] = [p for p in regra[1] if struct.unpack_from('<i', p, 0x1c)[0] != s]
        if len(regra[1]) != antes:
            feito.append('regra %d: %d pendente(s)' % (rs, antes - len(regra[1])))
        b = struct.unpack_from('<h', e, 6)[0]
        if b != -1:
            feito.append('regra %d: suspensao %d (%s)' % (rs, b, descrever(d['bans'][b])))
            release_ban(d, mapa, s, b)
    return feito


def descrever(ban):
    jogos, cumpr, dias, dia, ano = struct.unpack_from('<bbhhH', ban)
    return '%s, inicio %d/%d, regras %s, motivo %d, bits %d' % (
        'por tempo %d dias' % dias if jogos == -1 else '%d jogo(s), %d cumprido(s)' % (jogos, cumpr),
        dia, ano, list(struct.unpack_from('<4b', ban, 0xc)), ban[0x10], ban[0x13])


def abrir(caminho):
    dados = open(caminho, 'rb').read()
    tipo, _, itens = ps.indice(dados)
    nomes = [x[0].lower() for x in itens]
    alvo = nomes.index('discipline.dat')
    N = itens[nomes.index('staff.dat')][2] // 110
    D = ps.conteudo(dados, tipo, itens[alvo][1], itens[alvo][2])
    d = ler(D, N)
    assert gravar(d) == D, 'ida e volta do discipline.dat'
    return dados, alvo, N, d


def listar(d, mapa, N):
    for s in range(N):
        for k in range(4):
            rs, _ = vaga(d, s, k)
            e = entrada(d, mapa, s, rs, k) if rs != -1 else None
            b = struct.unpack_from('<h', e, 6)[0] if e is not None else -1
            if b != -1:
                print('pessoa %d vaga %d regra %d: suspensao %d, %s' % (s, k, rs, b, descrever(d['bans'][b])))
    for rs, pos in mapa.items():
        for p in d['regras'][pos][1]:
            print('pendente na regra %d: pessoa %d, %s' % (rs, struct.unpack_from('<i', p, 0x1c)[0], descrever(p)))
    print('indice por nacao:', d['idx'])


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('entrada')
    ap.add_argument('saida', nargs='?')
    ap.add_argument('--pessoa', type=int, action='append', default=[])
    a = ap.parse_args()
    dados, alvo, N, d = abrir(a.entrada)
    mapa = mapa_regras(d, N)
    if not a.saida:
        listar(d, mapa, N)
        return 0
    if os.path.normcase(os.path.abspath(a.entrada)) == os.path.normcase(os.path.abspath(a.saida)):
        sys.exit('a SAIDA nao pode ser a ENTRADA')
    md5_antes = ps.md5(a.entrada)
    for s in a.pessoa:
        print('pessoa %d: %s' % (s, '; '.join(remover(d, mapa, s)) or 'nada a remover'))
    novo = gravar(d)
    open(a.saida, 'wb').write(regravar(dados, alvo, novo))
    conferir(dados, open(a.saida, 'rb').read(), alvo, novo)
    assert ps.md5(a.entrada) == md5_antes, 'a ENTRADA mudou'
    print('%s: discipline.dat %d -> %d bytes; conferido; MD5 %s' % (
        a.saida, ps.indice(dados)[2][alvo][2], len(novo), ps.md5(a.saida)))
    return 0


if __name__ == '__main__':
    sys.exit(main())
