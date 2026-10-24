r"""Troca o nome de um registro dentro de uma COPIA de save (.sav/.qst) e, opcionalmente, carrega a copia no jogo
dos testes para ver se o jogo aceita (docs/TESTE_CHAVES.md: renomear uma chave fixa de clube/nacao impede a carga?).

Uso:
  python renomear_no_save.py ENTRADA SAIDA --item club.dat --de "F.C. Barcelona" --para "F.C. Barcelonx" [--curto]
  python renomear_no_save.py ENTRADA SAIDA --item eng.lng --tabela nacao --de Brazil --para Brazix
  python renomear_no_save.py ENTRADA SAIDA                       (ida e volta: regrava sem mudar nada)
  python renomear_no_save.py --carregar COPIA.sav [--modo original] [--ate 2001-07-31]

Itens: club.dat / nat_club.dat (581 bytes por registro) e nation.dat (290): nome em +4 (51), curto em +0x38 (26).
Um *.lng: cabecalho de 8 contagens (0x20 bytes; clubes, continentes, competicoes de nacao, nacoes, ...) e as
tabelas em sequencia: clubes de 0x4f, continentes de 0xba, competicoes de nacao de 0x53, nacoes de 0x6d; nome
em +0 (51), curto em +0x34 (26) (lidos assim por set_key_ids 0x601a40 em 0x603c5e..0x604208).
Troca todo registro cujo campo e exatamente --de (bytes latin-1 ate o zero); --para nao pode passar do campo.
Grava --para + um zero por cima; o resto do campo fica. Tipo 3 (sem compressao): troca no lugar. Tipo 4 (RLE):
o item e recodificado (b <= 0x80 literal, b > 0x80 vira 0x81 b) e as posicoes seguintes do indice andam.
No fim confere a SAIDA: todos os itens descomprimidos iguais aos da ENTRADA, menos o trocado, que tem de ser o
esperado. A ENTRADA nunca e escrita (e a SAIDA nao pode ser ela).

--carregar: a carga da prova_save (-load, CM_FERIAS, modo simulacao, trava do laboratorio) com um vigia que le a
memoria do jogo (sem cliques): a caixa de erro 0x5e8290 espera clique, entao se o texto dela (0xae24d0, "v3.9.68
arquivo.cpp linha") e o detalhe (0xde1f64) aparecerem, o vigia registra e encerra o jogo.
"""
import argparse
import ctypes
import os
import struct
import sys
import threading
import time

import prova_save as ps

REGISTROS = {'club.dat': 581, 'nat_club.dat': 581, 'nation.dat': 290}
LNG = [('clube', 0x4f), ('continente', 0xba), ('competicao', 0x53), ('nacao', 0x6d)]  # na ordem do arquivo


def campos(item, dados, tabela, curto):
    """As posicoes (no item) do campo pedido em cada registro, e o tamanho do campo."""
    if item in REGISTROS:
        tam = REGISTROS[item]
        ini, n = 0, len(dados) // tam
        desl, largura = (0x38, 26) if curto else (4, 51)
    elif item.endswith('.lng'):
        contagens = struct.unpack_from('<8I', dados)
        ini = 0x20
        for k, (nome, t) in enumerate(LNG):
            if nome == tabela:
                tam, n = t, contagens[k]
                break
            ini += contagens[k] * t
        else:
            sys.exit('tabela desconhecida: %s' % tabela)
        desl, largura = (0x34, 26) if curto else (0, 51)
    else:
        sys.exit('item sem layout: %s' % item)
    return [ini + i * tam + desl for i in range(n)], largura


def renomear(conteudo, item, tabela, curto, de, para):
    """(novo conteudo, quantos registros trocados)."""
    de, para = de.encode('latin-1'), para.encode('latin-1')
    novo = bytearray(conteudo)
    posicoes, largura = campos(item, conteudo, tabela, curto)
    if len(para) >= largura:
        sys.exit('--para tem %d bytes; o campo tem %d (com o zero)' % (len(para), largura))
    n = 0
    for p in posicoes:
        if conteudo[p:p + largura].split(b'\0')[0] == de:
            novo[p:p + len(para) + 1] = para + b'\0'
            n += 1
    return bytes(novo), n


def rle(dados):
    """O RLE do tipo 4 mais simples que o leitor aceita: literal ate 0x80, acima disso 1 copia (0x81 b)."""
    saida = bytearray()
    for b in dados:
        if b > 0x80:
            saida.append(0x81)
        saida.append(b)
    return bytes(saida)


def regravar(dados, alvo, novo_conteudo):
    """O conteiner com o item `alvo` (indice na lista) trocado por `novo_conteudo` (mesmo tamanho descomprimido)."""
    tipo, _, itens = ps.indice(dados)
    nome, pos, tam = itens[alvo]
    assert len(novo_conteudo) == tam
    if tipo != 4:
        return dados[:pos] + novo_conteudo + dados[pos + tam:]
    fim = ps.trechos(dados, itens)[alvo]
    enc = rle(novo_conteudo)
    delta = len(enc) - (fim - pos)
    saida = bytearray(dados[:pos] + enc + dados[fim:])
    for i, (_, p, _) in enumerate(itens):
        if p > pos:
            struct.pack_into('<I', saida, 12 + i * 0x10c, p + delta)
    return bytes(saida)


def conferir(antes, depois, alvo, esperado):
    """Todos os itens de `depois` iguais aos de `antes`, menos o `alvo`, que tem de ser `esperado`."""
    ta, ia, la = ps.indice(antes)
    td, idd, ld = ps.indice(depois)
    assert (ta, ia, len(la)) == (td, idd, len(ld)), 'cabecalho mudou'
    for k, ((na, pa, sa), (nd, pd, sd)) in enumerate(zip(la, ld)):
        assert (na, sa) == (nd, sd), 'entrada %d mudou' % k
        cd = ps.conteudo(depois, td, pd, sd)
        assert cd == (esperado if k == alvo else ps.conteudo(antes, ta, pa, sa)), 'item %d %s diferente' % (k, nd)


def principal_renomear(a):
    if os.path.normcase(os.path.abspath(a.entrada)) == os.path.normcase(os.path.abspath(a.saida)):
        sys.exit('a SAIDA nao pode ser a ENTRADA')
    md5_antes = ps.md5(a.entrada)
    dados = open(a.entrada, 'rb').read()
    tipo, _, itens = ps.indice(dados)
    if not a.item:  # ida e volta
        open(a.saida, 'wb').write(dados)
        print('ida e volta: tipo %d, %d itens, %s' % (tipo, len(itens), 'IGUAL' if open(a.saida, 'rb').read() == dados else 'DIFERENTE'))
        return 0
    alvo = next((k for k, x in enumerate(itens) if x[0] == a.item), None)
    if alvo is None:
        sys.exit('item %s nao esta no conteiner' % a.item)
    _, pos, tam = itens[alvo]
    antigo = ps.conteudo(dados, tipo, pos, tam)
    novo, n = renomear(antigo, a.item, a.tabela, a.curto, a.de, a.para)
    if n == 0:
        sys.exit('nenhum registro com %r em %s' % (a.de, a.item))
    saida = regravar(dados, alvo, novo)
    open(a.saida, 'wb').write(saida)
    conferir(dados, open(a.saida, 'rb').read(), alvo, novo)
    assert ps.md5(a.entrada) == md5_antes, 'a ENTRADA mudou'
    print('%s: %d registro(s) %r -> %r em %s%s (tipo %d); conferido; MD5 %s' % (
        a.saida, n, a.de, a.para, a.item, ' (' + a.tabela + ')' if a.item.endswith('.lng') else '', tipo, ps.md5(a.saida)))
    return 0


def ler_memoria(pid, endereco, n):
    k32 = ctypes.windll.kernel32
    h = k32.OpenProcess(0x0010 | 0x0400, False, pid)  # PROCESS_VM_READ | QUERY_INFORMATION
    if not h:
        return b''
    try:
        buf, lido = ctypes.create_string_buffer(n), ctypes.c_size_t()
        ok = k32.ReadProcessMemory(h, ctypes.c_void_p(endereco), buf, n, ctypes.byref(lido))
        return buf.raw[:lido.value] if ok else b''
    finally:
        k32.CloseHandle(h)


def vigiar(fim, achado):
    """Enquanto o jogo dos testes roda: le o texto da caixa de erro; se aparecer, guarda e encerra o jogo."""
    import psutil
    while not fim.is_set():
        for q in psutil.process_iter(['name', 'exe']):
            if (q.info['name'] or '').lower().startswith('cm0102') and \
                    os.path.normcase(q.info['exe'] or '').startswith(os.path.normcase(ps.JOGO)):
                caixa = ler_memoria(q.pid, 0xae24d0, 80).split(b'\0')[0]
                if caixa.startswith(b'v3.9') and b'.cpp' in caixa:
                    time.sleep(3)  # o detalhe ja foi escrito antes da caixa; a pausa so confirma que ela ficou
                    achado.append((caixa.decode('latin-1'), ler_memoria(q.pid, 0xde1f64, 120).split(b'\0')[0].decode('latin-1')))
                    q.kill()
                    return
        fim.wait(2)


def principal_carregar(a):
    if ps.mutacoes.jogo_aberto():
        sys.exit('ja ha um jogo deste projeto aberto: um teste por vez')
    o = ps.padrao()
    o.sav, o.ate, o.limite = os.path.abspath(a.carregar), a.ate, a.limite
    o.pasta = os.path.join(os.path.dirname(o.sav), 'carga')
    os.makedirs(o.pasta, exist_ok=True)
    fim, achado = threading.Event(), []
    t = threading.Thread(target=vigiar, args=(fim, achado), daemon=True)
    t.start()
    imp = ps.carregar(o, a.modo)
    fim.set()
    t.join()
    if achado:
        print('CAIXA: %s | %s' % achado[0])
    print('resultado %s: %s' % (os.path.basename(o.sav), 'CARREGOU (%s)' % ps.resumo_carga(imp) if imp else
                                'CAIXA DE ERRO' if achado else 'NAO CARREGOU (sem caixa vista)'))
    return 0 if imp else 1


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('entrada', nargs='?')
    ap.add_argument('saida', nargs='?')
    ap.add_argument('--item', help='club.dat, nat_club.dat, nation.dat ou um *.lng (ex.: eng.lng)')
    ap.add_argument('--tabela', default='clube', choices=[n for n, _ in LNG], help='so para *.lng')
    ap.add_argument('--curto', action='store_true', help='troca o nome curto em vez do nome')
    ap.add_argument('--de')
    ap.add_argument('--para')
    ap.add_argument('--carregar', metavar='COPIA', help='carrega a copia no jogo dos testes (vigiando a caixa de erro)')
    ap.add_argument('--modo', default='original', choices=['original', 'rust'])
    ap.add_argument('--ate', default='2001-07-31')
    ap.add_argument('--limite', type=int, default=600)
    a = ap.parse_args()
    if a.carregar:
        return principal_carregar(a)
    if not (a.entrada and a.saida) or (a.item and not (a.de and a.para)):
        ap.error('ENTRADA SAIDA [--item X --de A --para B]')
    return principal_renomear(a)


if __name__ == '__main__':
    sys.exit(main())
