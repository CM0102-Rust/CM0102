r"""Troca o nome de um registro dentro de uma COPIA de save (.sav/.qst) e, opcionalmente, carrega a copia no jogo
dos testes para ver se o jogo aceita (docs/TESTE_CHAVES.md: renomear uma chave fixa de clube/nacao impede a carga?).

Uso:
  python renomear_no_save.py ENTRADA SAIDA --item club.dat --de "F.C. Barcelona" --para "F.C. Barcelonx" [--curto]
  python renomear_no_save.py ENTRADA SAIDA --item eng.lng --tabela nacao --de Brazil --para Brazix
  python renomear_no_save.py ENTRADA SAIDA                       (ida e volta: regrava sem mudar nada)
  python renomear_no_save.py --carregar COPIA.sav|COPIA.qst [--idioma 0|9] [--modo original] [--ate 2001-07-31]
  python renomear_no_save.py --gerar-qst SAIDA.qst [--language ENG_UK|POR]

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
import contextlib
import ctypes
import glob
import os
import shutil
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


# O que o vigia le da memoria a cada 2 s (docs/TESTE_CHAVES.md, "Matriz por idioma")
IDIOMA_NOMES = 0x9b5984      # idioma dos nomes da base (-1 na carga 0x52851c; do index.dat do save)
CONFIG = 0xae3138            # ponteiro das preferencias (o game.cfg lido direto: +8 = idioma do jogo)
CHAVE_BARCELONA = 0x9d0138   # chave fixa de F.C. Barcelona (levas/227/codigo.rs); -1 ate o set_key_ids casar
CHAVE_BRAZIL = 0x9cf260      # chave fixa de Brazil
CLUBES = 0xae23b4            # registros de clube na memoria (0x245 bytes, nome em +4)


def u32(pid, endereco):
    b = ler_memoria(pid, endereco, 4)
    return struct.unpack('<i', b)[0] if len(b) == 4 else None


def retrato(pid):
    """(idioma dos nomes, idioma do jogo, chave Barcelona, nome do clube dessa chave na memoria, chave Brazil)."""
    cfg = u32(pid, CONFIG)
    chave = u32(pid, CHAVE_BARCELONA)
    clubes = u32(pid, CLUBES)
    nome = None
    if chave is not None and chave >= 0 and clubes:
        nome = ler_memoria(pid, (clubes & 0xffffffff) + chave * 0x245 + 4, 51).split(b'\0')[0].decode('latin-1')
    return (u32(pid, IDIOMA_NOMES), u32(pid, (cfg & 0xffffffff) + 8) if cfg else None, chave, nome, u32(pid, CHAVE_BRAZIL))


def vigiar(fim, achado, retratos, qst=False):
    """Enquanto o jogo dos testes roda: le o texto da caixa de erro; se aparecer, guarda e encerra o jogo.
    Guarda tambem o retrato da memoria (retrato()) a cada leitura. qst: o -load de um .qst para na tela do nome
    do tecnico (sem data andando), entao a carga conta como feita quando a chave de Barcelona (ou a de Brazil)
    sai de -1 e 15 s passam sem caixa (o set_key_ids abre a caixa na hora); o vigia encerra o jogo."""
    import psutil
    feitas = None
    while not fim.is_set():
        for q in psutil.process_iter(['name', 'exe']):
            if (q.info['name'] or '').lower().startswith('cm0102') and \
                    os.path.normcase(q.info['exe'] or '').startswith(os.path.normcase(ps.JOGO)):
                caixa = ler_memoria(q.pid, 0xae24d0, 80).split(b'\0')[0]
                if caixa.startswith(b'v3.9') and b'.cpp' in caixa:
                    time.sleep(3)  # o detalhe ja foi escrito antes da caixa; a pausa so confirma que ela ficou
                    achado.append((caixa.decode('latin-1'), ler_memoria(q.pid, 0xde1f64, 120).split(b'\0')[0].decode('latin-1')))
                    retratos.append(retrato(q.pid))
                    q.kill()
                    return
                r = retrato(q.pid)
                if r[0] is not None:
                    retratos.append(r)
                if qst and ((r[2] or -1) >= 0 or (r[4] or -1) >= 0):
                    feitas = feitas or time.time()
                    if time.time() - feitas > 15:
                        retratos.append(retrato(q.pid))
                        q.kill()
                        return
        fim.wait(2)


def mudar_idioma_cfg(idioma):
    """Poe o idioma do jogo (dword +8 do game.cfg dos testes: 0x5d8020 grava [0xae3138]+8; 0 = ingles, 9 = POR).
    So dentro da bancada (que copiou o game.cfg e o devolve no fim)."""
    cfg = os.path.join(ps.JOGO, 'game.cfg')
    dados = bytearray(open(cfg, 'rb').read())
    struct.pack_into('<I', dados, 8, idioma)
    open(cfg, 'wb').write(dados)


@contextlib.contextmanager
def lng_guardados():
    """A carga extrai os *.lng do save em CM3_TEMP (sem a variavel, a pasta do jogo dos testes) e o conteudo deles
    entra na impressao das carreiras (docs/VERSAO_SEM_TELA.md): guarda e devolve, para nao mudar a referencia."""
    guardados = {n: open(n, 'rb').read() for n in glob.glob(os.path.join(ps.JOGO, '*.lng'))}
    try:
        yield
    finally:
        for n in glob.glob(os.path.join(ps.JOGO, '*.lng')):
            if n not in guardados:
                os.remove(n)
        for n, d in guardados.items():
            open(n, 'wb').write(d)


def carregar(sav, modo, ate, limite, idioma):
    """A carga da prova_save.carregar (-load de uma copia, CM_FERIAS, ate `ate`), com o idioma do jogo `idioma`
    (None = o do game.cfg) e os *.lng do jogo dos testes devolvidos. Devolve a impressao ou None."""
    o = ps.padrao()
    o.sav, o.ate, o.limite = sav, ate, limite
    o.pasta = os.path.join(os.path.dirname(sav), 'carga')
    os.makedirs(o.pasta, exist_ok=True)
    copia = os.path.join(o.pasta, 'carga_%s%s' % (modo, os.path.splitext(sav)[1]))
    antes = ps.md5(sav)
    shutil.copy2(sav, copia)
    with ps.bancada('CM0102 renomear_no_save.py', {}), lng_guardados():
        ok, motivo = ps.laboratorio.pode_rodar()
        if not ok:
            sys.exit('nao iniciado: %s' % motivo)
        if idioma is not None:
            mudar_idioma_cfg(idioma)
        os.environ['CM_FERIAS'] = '1'
        hof = os.path.join(ps.JOGO, 'hall_of_fame.bin')
        hof_guardado = open(hof, 'rb').read() if os.path.exists(hof) else None
        try:
            info, seg, erro = ps.rodar(ps.opcoes(o, modo, ate), o.semente, ['-load', copia])
        finally:
            if hof_guardado is not None:
                open(hof, 'wb').write(hof_guardado)
            os.environ.pop('CM_FERIAS', None)
    assert ps.md5(sav) == antes, '%s mudou' % sav
    if erro:
        print('carregar %s: %s (%.0f s)' % (modo, erro, seg))
        return None
    return info['data'], info['sorteios'], info['impressao'], info['partidas'], info['banco']


def principal_carregar(a):
    if ps.mutacoes.jogo_aberto():
        sys.exit('ja ha um jogo deste projeto aberto: um teste por vez')
    sav = os.path.abspath(a.carregar)
    qst = sav.lower().endswith('.qst')
    fim, achado, retratos = threading.Event(), [], []
    t = threading.Thread(target=vigiar, args=(fim, achado, retratos, qst), daemon=True)
    t.start()
    imp = carregar(sav, a.modo, a.ate, a.limite if not qst else min(a.limite, 240), a.idioma)
    fim.set()
    t.join()
    if retratos:
        print('memoria no fim: idioma dos nomes [0x9b5984]=%s, idioma do jogo [cfg+8]=%s, chave Barcelona=%s (%r), '
              'chave Brazil=%s' % retratos[-1])
    if achado:
        print('CAIXA: %s | %s' % achado[0])
    if imp:
        res = 'CARREGOU (%s)' % ps.resumo_carga(imp)
    elif achado:
        res = 'CAIXA DE ERRO'
    elif qst and retratos and ((retratos[-1][2] or -1) >= 0 or (retratos[-1][4] or -1) >= 0):
        res = 'CARREGOU (.qst: chaves feitas, sem caixa em 15 s; para na tela do nome do tecnico)'
    else:
        res = 'NAO CARREGOU (sem caixa vista)'
    print('resultado %s (idioma do jogo %s): %s' % (os.path.basename(sav), 'do game.cfg' if a.idioma is None else a.idioma, res))
    return 0 if res.startswith('CARREGOU') else 1


def principal_gerar(a):
    """Um .qst genuino gravado pelo inicio rapido (o da prova_save.gravar, modo --modo) com LANGUAGE=a.language
    no PROVA.cfg; o .qst vai para a.gerar_qst."""
    if ps.mutacoes.jogo_aberto():
        sys.exit('ja ha um jogo deste projeto aberto: um teste por vez')
    o = ps.padrao()
    qst = os.path.join(ps.JOGO, 'qstart', 'PROVA.qst')
    linhas = ['LANGUAGE=' + a.language] + [x for x in ps.linhas_carreira() if not x.startswith('LANGUAGE=')]
    with ps.bancada('CM0102 renomear_no_save.py', {'PROVA.cfg': linhas, 'quick_start.cfg': ['AUTO_SETUP=PROVA.cfg']}), \
            lng_guardados():
        ok, motivo = ps.laboratorio.pode_rodar()
        if not ok:
            sys.exit('nao iniciado: %s' % motivo)
        if os.path.exists(qst):
            os.remove(qst)
        _, seg, _ = ps.rodar(ps.opcoes(o, a.modo, '2099-12-31'), o.semente)
        if not os.path.exists(qst):
            sys.exit('o .qst nao foi gravado (%.0f s)' % seg)
        shutil.move(qst, a.gerar_qst)
    dados = open(a.gerar_qst, 'rb').read()
    tipo, _, itens = ps.indice(dados)
    idx = next(x for x in itens if x[0] == 'index.dat')
    idioma = struct.unpack('<I', ps.conteudo(dados, tipo, idx[1], idx[2])[-4:])[0]
    print('%s: tipo %d, %d itens, idioma no index.dat %d, .lng: %s, MD5 %s (%.0f s)' % (
        a.gerar_qst, tipo, len(itens), idioma, ' '.join(x[0] for x in itens if x[0].endswith('.lng')) or 'nenhum',
        ps.md5(a.gerar_qst), seg))
    return 0


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
    ap.add_argument('--idioma', type=int, help='com --carregar: idioma do jogo (+8 do game.cfg) so nesta carga (0 = ingles, 9 = POR)')
    ap.add_argument('--gerar-qst', metavar='SAIDA', help='grava um .qst pelo inicio rapido com LANGUAGE=--language')
    ap.add_argument('--language', default='ENG_UK', help='com --gerar-qst: ENG_UK, POR... (docs/VERSAO_SEM_TELA.md)')
    a = ap.parse_args()
    if a.gerar_qst:
        return principal_gerar(a)
    if a.carregar:
        return principal_carregar(a)
    if not (a.entrada and a.saida) or (a.item and not (a.de and a.para)):
        ap.error('ENTRADA SAIDA [--item X --de A --para B]')
    return principal_renomear(a)


if __name__ == '__main__':
    sys.exit(main())
