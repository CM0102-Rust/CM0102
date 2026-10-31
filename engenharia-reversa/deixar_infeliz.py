r"""Prova do "Deixar infeliz" do editor CMRTE (pedido do Orquestrador, 31/10/2026): numa COPIA de save, o contrato
principal de um jogador recebe as gravacoes do editor e o jogo, carregado, deve subir o pedido de transferencia.

O contrato (contract.dat, item do CONTRACT_MANAGER, save 0x4dc3a0: i32 pre, pre x 21, i32 n, n x 0x50 gravados crus
da memoria, logo o leiaute do arquivo e o da memoria): +0x40 u32 as marcas (0x867560 faz OR), +0x44 i8 a forca do
pedido, +0x45 u32 as queixas (0x867700 faz OR), +0x49 u8, +0x4e bits (0x02, 0x08 = listado), +0x4f status (& 0xf0).
O indice da pessoa (general.dat +0xfa7 + 79 x pessoa): +0 = contrato principal, +4 = emprestimo.
Gravacoes do editor: +0x3f..+0x46 = 0xff, menos o +0x44 (marcas todas e queixas 0xffff), e +0x44 = +100. Controle:
o mesmo com +0x44 = -1 (o que o editor gravava antes). Pela desmontagem, 0x867700 so chama o pedido
(0x4d8610) com +0x44 >= 0 e 0x867560 com +0x44 > 0 (e +0x49 = 0, +0x4e & 9 = 0, 0x866f60, 0x8cca10, 0x8c1870).

Uso:
  python deixar_infeliz.py ENTRADA --listar                      (tecnicos humanos e o elenco do clube deles)
  python deixar_infeliz.py ENTRADA --pessoa P [--pasta DIR]      (gera infeliz_100.sav e infeliz_menos1.sav)
  python deixar_infeliz.py --ler SAV --pessoa P                  (os campos do contrato num save)
  python deixar_infeliz.py --carregar --pessoa P [--ate AAAA-MM-DD] [--pasta DIR]
      (PESADO: abre o jogo. Carrega os dois saves em modo rust com CM_FERIAS pelo prova_save.carregar e guarda o
       ddraw.log de cada carga como DIR\ddraw_<nome>.log; mostra as linhas "ramo cps_1 ...")
A ENTRADA nunca muda (MD5 conferido antes e depois).
"""
import argparse
import os
import shutil
import struct
import sys

import prova_save as ps
from renomear_no_save import conferir, regravar

RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PASTA = os.path.join(RAIZ, 'copias', 'deixar_infeliz')  # copias/ fica fora do git
# (forca, com as marcas e queixas do editor): a 3a so com a forca, para ver se as queixas pre-ligadas (+0x45 = 0xffff)
# impedem a queixa nova que chama o pedido (31/10/2026)
SAVES = {'infeliz_100': (100, True), 'infeliz_menos1': (-1, True), 'so_forca_100': (100, False)}


def abrir(caminho):
    dados = open(caminho, 'rb').read()
    tipo, _, itens = ps.indice(dados)
    I = {n.lower(): (k, ps.conteudo(dados, tipo, p, s)) for k, (n, p, s) in enumerate(itens)}
    return dados, I


def base_contratos(K):
    pre = struct.unpack_from('<i', K, 0)[0]
    base = 4 + pre * 21
    return base + 4, struct.unpack_from('<i', K, base)[0]


def contrato_de(I, p):
    """(posicao do contrato principal de p no contract.dat, indice) ou sai."""
    G, K = I['general.dat'][1], I['contract.dat'][1]
    i = struct.unpack_from('<i', G, 0xfa7 + 79 * p)[0]
    kb, n = base_contratos(K)
    if not 0 <= i < n:
        sys.exit('pessoa %d sem contrato principal (indice %d de %d)' % (p, i, n))
    o = kb + 0x50 * i
    if struct.unpack_from('<i', K, o)[0] != p:
        sys.exit('contrato %d nao e da pessoa %d' % (i, p))
    return o, i


def campos(K, o):
    return '+0x3f..+0x48 %s | +0x44 %d, +0x45 %#x, +0x40 %#x, +0x49 %d, +0x4e %#x, +0x4f %#x, clube %d' % (
        K[o + 0x3f:o + 0x49].hex(), struct.unpack_from('<b', K, o + 0x44)[0], struct.unpack_from('<I', K, o + 0x45)[0],
        struct.unpack_from('<I', K, o + 0x40)[0], K[o + 0x49], K[o + 0x4e], K[o + 0x4f], struct.unpack_from('<i', K, o + 4)[0])


def editar(K, o, forca, tudo=True):
    """As gravacoes do editor no contrato em K (bytearray) na posicao o (tudo=False: so a forca)."""
    for j in range(0x3f, 0x47) if tudo else ():
        K[o + j] = 0xff
    struct.pack_into('<b', K, o + 0x44, forca)


def listar(I):
    S, C, K, G = (I[x][1] for x in ('staff.dat', 'club.dat', 'contract.dat', 'general.dat'))
    N = len(S) // 110
    print('data', struct.unpack_from('<hh', G, 0xf68), 'pessoas', N)
    for h in [p for p in range(N - 16, N) if S[p * 110 + 0x5f] == 1]:  # os tecnicos humanos sao as ultimas 16 vagas
        c = struct.unpack_from('<i', S, h * 110 + 57)[0]
        print('tecnico humano %d, clube %d' % (h, c))
        if c < 0:
            continue
        for v in range(50):
            p = struct.unpack_from('<i', C, c * 581 + 215 + 4 * v)[0]
            if 0 <= p < N:
                i = struct.unpack_from('<i', G, 0xfa7 + 79 * p)[0]
                kb, n = base_contratos(K)
                extra = campos(K, kb + 0x50 * i) if 0 <= i < n else 'sem contrato'
                print('  jogador %d cargo %d idade? %d emprestimo %d: contrato %d %s' % (
                    p, S[p * 110 + 0x3d], S[p * 110 + 0x18], struct.unpack_from('<i', G, 0xfa7 + 79 * p + 4)[0], i, extra))


def gerar(a):
    md5_antes = ps.md5(a.entrada)
    dados, I = abrir(a.entrada)
    alvo, K0 = I['contract.dat']
    o, i = contrato_de(I, a.pessoa)
    assert struct.unpack_from('<i', K0, o + 4)[0] >= 0, 'contrato sem clube'
    print('pessoa %d, contrato %d, antes: %s' % (a.pessoa, i, campos(K0, o)))
    os.makedirs(a.pasta, exist_ok=True)
    for nome, (forca, tudo) in SAVES.items():
        K = bytearray(K0)
        editar(K, o, forca, tudo)
        saida = os.path.join(a.pasta, nome + '.sav')
        open(saida, 'wb').write(regravar(dados, alvo, bytes(K)))
        conferir(dados, open(saida, 'rb').read(), alvo, bytes(K))  # so o contract.dat mudou, e como pedido
        _, J = abrir(saida)
        print('%s: %s; MD5 %s' % (saida, campos(J['contract.dat'][1], o), ps.md5(saida)))
    assert ps.md5(a.entrada) == md5_antes, 'a ENTRADA mudou'
    print('entrada intocada: MD5 %s' % md5_antes)


def ler(a):
    _, I = abrir(a.ler)
    o, i = contrato_de(I, a.pessoa)
    print('%s: pessoa %d contrato %d: %s' % (a.ler, a.pessoa, i, campos(I['contract.dat'][1], o)))


def carregar(a):
    from simular import JOGO
    op = ps.parser().parse_args(['--ate', a.ate])
    op.pasta = os.path.join(a.pasta, 'trabalho')  # longe da pasta de trabalho das bancadas
    os.makedirs(op.pasta, exist_ok=True)
    dias = {}
    for nome in SAVES:
        op.sav = os.path.join(a.pasta, nome + '.sav')
        ps.conferir_sav(op)
        imp = ps.carregar(op, 'rust')
        log = os.path.join(a.pasta, 'ddraw_%s.log' % nome)
        shutil.copy2(os.path.join(JOGO, 'ddraw.log'), log)
        linhas = [l.rstrip() for l in open(log, encoding='utf-8', errors='replace')]
        dias[nome] = [l for l in linhas if l.startswith('dia ')]
        # "cps_1 ...": os ramos do clube do computador; "pedido de transferencia no clube humano": a linha
        # proposta para o motor.rs (o clube humano vai direto para a original, sem ramo)
        achadas = [l for l in linhas if 'cps_1 ' in l or 'clube humano' in l]
        print('%s: %s\n  %s' % (nome, imp and ps.resumo_carga(imp), '\n  '.join(achadas) or 'nenhum ramo cps_1'))
    nomes = list(dias)
    a1 = dias[nomes[0]]
    for outro in nomes[1:]:  # cada carga contra a primeira
        a2 = dias[outro]
        dif = next((i for i, (x, y) in enumerate(zip(a1, a2)) if x != y), None)
        print('diario %s x %s: %s' % (nomes[0], outro, 'igual nos %d dias' % len(a1) if dif is None else 'primeiro dia diferente: %s' % a1[dif].split()[1]))


def autoteste():
    K = bytearray(range(0x50))
    editar(K, 0, -1)
    assert K[0x3f:0x47] == b'\xff' * 8 and K[0x47] == 0x47 and K[0x3e] == 0x3e
    editar(K, 0, 100)
    assert K[0x44] == 100 and struct.unpack_from('<I', K, 0x45)[0] & 0xffff == 0xffff


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('entrada', nargs='?')
    ap.add_argument('--listar', action='store_true')
    ap.add_argument('--pessoa', type=int)
    ap.add_argument('--pasta', default=PASTA)
    ap.add_argument('--ler')
    ap.add_argument('--carregar', action='store_true')
    ap.add_argument('--ate', default='2001-08-28')
    a = ap.parse_args()
    autoteste()
    if a.carregar:
        return carregar(a)
    if a.ler:
        return ler(a)
    if a.listar:
        return listar(abrir(a.entrada)[1])
    if a.entrada is None or a.pessoa is None:
        ap.error('ENTRADA e --pessoa')
    gerar(a)


if __name__ == '__main__':
    sys.exit(main())
