r"""Prova do save: o mesmo save gravado e carregado pela original e pela Rust (levas S1..S4 do
bloco do save, docs/MAPA_SAVE.md §4). A carreira automatica nao passa pelo conteiner (zipdir.cpp):
esta prova e o banco de provas dele.

Uso:
  python prova_save.py [--so-gravar] [--tipo 4|3] [--semente 12345] [--pasta DIR]
                       [--sav SAVE.sav] [--ate 2001-09-01] [--exe cm0102-1080.exe] [--limite 900]
  python prova_save.py --comparar A B          (so o passo 2, sem abrir o jogo)

1. Gravar: em jogo_testes, o inicio rapido (quick_start.cfg com AUTO_SETUP=PROVA.cfg, que tem as
   linhas da carreira do simular.py) cria a carreira e grava qstart\PROVA.qst, uma vez em cada
   modo (original e rust), com a semente fixa e o modo simulacao da DLL (relogio, heap e discos
   fixos: docs/VERSAO_SEM_TELA.md §5). O tipo do conteiner vem do bit 0x8000 de [0xae3138]+4, que
   e o dword +4 do game.cfg (0x5d7636 le o arquivo direto nele): bit LIGADO = tipo 3 (sem
   compressao), DESLIGADO = tipo 4 (RLE) (0x82e6b1 em save_game_data). O game.cfg dos testes tem o
   bit ligado (por isso o quick_start dava tipo 3); --tipo 4 (o padrao) desliga o bit so durante a
   prova. Os dois .qst vao para a pasta de trabalho (%TEMP%\cm0102_prova_save: original.qst, rust.qst).
2. Comparar: byte a byte; se diferirem, diz o item do conteiner (indice: tipo, id 0x16/0x17,
   quantidade e 1000 entradas de 0x10c = posicao u32, tamanho u32, nome 260) e o deslocamento
   dentro do item ja descomprimido (tipo 4). IGNORAR lista os campos sabidamente de relogio.
3. Carregar: so com --sav. A chave -load <arquivo> (tratador 0x81bd70) carrega e, se o save nao
   tem tecnico humano (count_human_managers 0x838d10 = 0, o caso de todo .qst), abre a tela
   "Manager Status" e o "Enter Name" do novo tecnico (0x81fea0, 0x820d40): pede cliques. E o -load
   e procurado antes do quick_start.cfg/auto_setup.cfg (0x81bdb1): com ele, a carreira automatica
   nao e criada. Por isso a carga sem cliques precisa de um .sav COM tecnico (de ferias), feito uma
   vez pelo Lucas numa carreira automatica (menu Salvar). A prova copia o --sav para a pasta de
   trabalho, abre o jogo com -load nos dois modos, deixa a carreira andar ate --ate e compara a
   impressao (a mesma da linha "execucao" do simular.py).
4. Saida: linhas curtas ("gravar rust: ...", "comparar: IGUAL" ou "comparar: DIFERENTE item ...",
   "carregar original: impressao ..."); codigo 0 so se tudo bater.

Cuidados (os do simular.py, pela mesma bancada): trava do laboratorio, vigia de memoria, game.cfg
copiado e restaurado, *.tmp guardados e devolvidos, PROVA.cfg/quick_start.cfg apagados no fim.
"""
import argparse
import os
import shutil
import struct
import sys
import tempfile
from types import SimpleNamespace

import laboratorio
from simular import JOGO, bancada, falha, linhas_carreira, rodar

CABECALHO = 12 + 1000 * 0x10c  # 0x416ec: tipo, id, quantidade e as 1000 entradas
# Campos que dependem do relogio da maquina, ignorados na comparacao. general.dat = versao (4) +
# resumo de build_save_game_summary 0x591f80 (0xf74 bytes) + ...; no resumo, +0xf6c e o tempo de
# jogo acumulado ([0xdd7140] += time() - [0xdd7144]) e +0xf70 o time() da gravacao (0x592232).
# No modo simulacao o time() e fixo e os dois deveriam sair iguais; ficam fora mesmo assim porque
# nao sao estado do jogo e, fora do modo simulacao, mudam sempre. Diferenca neles e so informada.
# O "id unico" (+0xf78, [0xdd7120]) e a semente do -seed (0x81b9aa): fica na comparacao.
IGNORAR = {'general.dat': [(4 + 0xf6c, 4, 'tempo de jogo no resumo'), (4 + 0xf70, 4, 'time() da gravacao no resumo')]}


def indice(dados):
    """(tipo, id, [(nome, posicao, tamanho)])."""
    tipo, ident, n = struct.unpack_from('<3I', dados)
    itens = []
    for i in range(min(n, 1000)):
        e = 12 + i * 0x10c
        pos, tam = struct.unpack_from('<2I', dados, e)
        itens.append((dados[e + 8:e + 0x10c].split(b'\0')[0].decode('latin-1'), pos, tam))
    return tipo, ident, itens


def trechos(dados, itens):
    """Fim (no arquivo) de cada item: a posicao do item seguinte, ou o fim do arquivo."""
    posicoes = sorted({p for _, p, _ in itens} | {len(dados)})
    return [next(q for q in posicoes if q > p) if p < len(dados) else p for _, p, _ in itens]


def conteudo(dados, tipo, pos, tam):
    """O item descomprimido. RLE do tipo 4: b <= 0x80 literal; b > 0x80 = b - 0x80 copias do byte seguinte."""
    if tipo != 4:
        return dados[pos:pos + tam]
    saida, j = bytearray(), pos
    while len(saida) < tam and j < len(dados):
        b = dados[j]
        if b > 0x80:
            saida += dados[j + 1:j + 2] * (b - 0x80)
            j += 2
        else:
            saida.append(b)
            j += 1
    return bytes(saida)


def variaveis(a, b):
    """{item: bytes que diferem} entre duas gravacoes da mesma base: lixo de heap e de ponteiro que o jogo
    grava sem inicializar (23/10/2026: 13 bytes do general.dat mudam de uma rodada para outra no mesmo modo;
    entre os modos ainda ha 161 do staff history.tmp, por isso a prova compara a mesma DLL com e sem a leva,
    nao a original com a Rust)."""
    da, db = open(a, 'rb').read(), open(b, 'rb').read()
    ta, _, la = indice(da)
    tb, _, lb = indice(db)
    v = {}
    for (nome, pa, sa), (_, pb, sb) in zip(la, lb):
        ca, cb = conteudo(da, ta, pa, sa), conteudo(db, tb, pb, sb)
        if ca != cb:
            v[nome] = {i for i in range(min(len(ca), len(cb))) if ca[i] != cb[i]}
    return v


def comparar(a, b, extra=None):
    """(igual, [linhas]) entre dois arquivos de conteiner; `extra` = {item: bytes a ignorar} (ver variaveis)."""
    extra = extra or {}
    da, db = open(a, 'rb').read(), open(b, 'rb').read()
    if da == db:
        return True, ['comparar: IGUAL (%d bytes)' % len(da)]
    ta, ia, la = indice(da)
    tb, ib, lb = indice(db)
    if (ta, ia, len(la)) != (tb, ib, len(lb)):
        return False, ['comparar: DIFERENTE cabecalho: tipo %d/%d, id %#x/%#x, %d/%d itens' % (ta, tb, ia, ib, len(la), len(lb))]
    linhas = []
    fa, fb = trechos(da, la), trechos(db, lb)
    for k, ((nome, pa, sa), (nome_b, pb, sb)) in enumerate(zip(la, lb)):
        if nome != nome_b:
            return False, linhas + ['comparar: DIFERENTE item %d: nome %s / %s' % (k, nome, nome_b)]
        if sa == sb and da[pa:fa[k]] == db[pb:fb[k]]:
            continue
        ca, cb = conteudo(da, ta, pa, sa), conteudo(db, tb, pb, sb)
        if len(ca) != sa or len(cb) != sb:
            return False, linhas + ['comparar: DIFERENTE item %d %s: descomprimido %d/%d bytes, indice diz %d/%d' % (
                k, nome, len(ca), len(cb), sa, sb)]
        mascara = IGNORAR.get(nome, [])
        primeira = None
        for i in range(min(sa, sb)):
            if ca[i] != cb[i]:
                campo = next((m for m in mascara if m[0] <= i < m[0] + m[1]), None)
                if campo is None and i in extra.get(nome, ()):
                    campo = (i, 1, 'varia entre gravacoes da base')
                if campo is None:
                    primeira = i
                    break
                aviso = 'comparar: ignorado %s +%#x (%s)' % (nome, campo[0], campo[2])
                if aviso not in linhas:
                    linhas.append(aviso)
        if primeira is None and sa != sb:
            primeira = min(sa, sb)
        if primeira is not None:
            return False, linhas + ['comparar: DIFERENTE item %d %s +%#x (tamanho %d/%d)' % (k, nome, primeira, sa, sb)]
    # itens iguais (fora das mascaras); sobram as entradas alem da quantidade (lixo de heap do zopen,
    # MAPA_SAVE §5) e, sem campo ignorado, as posicoes do indice e o que vier depois dos itens
    resto = 12 + len(la) * 0x10c
    if da[resto:CABECALHO] != db[resto:CABECALHO]:
        i = next(i for i in range(resto, CABECALHO) if da[i] != db[i])
        return False, linhas + ['comparar: DIFERENTE indice +%#x (entrada %d, alem da quantidade)' % (i, (i - 12) // 0x10c)]
    if linhas:  # um campo ignorado muda o tamanho comprimido e desloca as posicoes seguintes
        return True, linhas + ['comparar: IGUAL fora dos campos ignorados']
    i = next((i for i in range(min(len(da), len(db))) if da[i] != db[i]), min(len(da), len(db)))
    return False, ['comparar: DIFERENTE fora dos itens +%#x (%s)' % (i, 'indice' if i < CABECALHO else 'arquivo')]


def opcoes(a, modo, ate):
    """Os argumentos que o simular.rodar espera."""
    return SimpleNamespace(ate=ate, modo=modo, exe=a.exe, limite=a.limite, visivel=False, som=False,
                           cobertura=False, parar_na_divergencia=False)


def gravar(a, modo):
    """Grava qstart\\PROVA.qst no modo pedido e o move para a pasta de trabalho. Devolve o caminho ou None."""
    qst = os.path.join(JOGO, 'qstart', 'PROVA.qst')
    destino = os.path.join(a.pasta, modo + '.qst')
    if os.path.exists(destino):
        os.remove(destino)
    with bancada('CM0102 prova_save.py', {'PROVA.cfg': linhas_carreira(), 'quick_start.cfg': ['AUTO_SETUP=PROVA.cfg']}):
        ok, motivo = laboratorio.pode_rodar()
        if not ok:
            print('gravar %s: nao iniciado: %s' % (modo, motivo))
            return None
        cfg = os.path.join(JOGO, 'game.cfg')
        dados = bytearray(open(cfg, 'rb').read())
        bandeiras = struct.unpack_from('<I', dados, 4)[0]
        bandeiras = bandeiras | 0x8000 if a.tipo == 3 else bandeiras & ~0x8000
        struct.pack_into('<I', dados, 4, bandeiras)
        open(cfg, 'wb').write(dados)  # a bancada devolve a copia original no fim
        if os.path.exists(qst):
            os.remove(qst)  # o inicio rapido so grava o .qst que ainda nao existe
        # Data de parada longe: o inicio rapido fecha o jogo sozinho depois de gravar; CM_PARAR_EM so
        # liga o modo simulacao (relogio e heap fixos). Sem a linha SIMULACAO o rodar devolve "erro".
        _, seg, _ = rodar(opcoes(a, modo, '2099-12-31'), a.semente)
        if not os.path.exists(qst):
            print('gravar %s: o .qst nao foi gravado (%.0f s); veja jogo_testes\\ddraw.log' % (modo, seg))
            return None
        shutil.move(qst, destino)
    tipo, ident, itens = indice(open(destino, 'rb').read(CABECALHO))
    print('gravar %s: %s, %d bytes, tipo %d, id %#x, %d itens, %.0f s' % (
        modo, destino, os.path.getsize(destino), tipo, ident, len(itens), seg))
    if tipo != a.tipo:
        print('gravar %s: ERRO: pedi tipo %d (bit 0x8000 do game.cfg) e saiu %d' % (modo, a.tipo, tipo))
        return None
    return destino


def carregar(a, modo):
    """-load de uma copia do --sav e carreira ate --ate. Devolve a tupla da impressao ou None."""
    copia = os.path.join(a.pasta, 'carga_%s%s' % (modo, os.path.splitext(a.sav)[1]))
    shutil.copy2(a.sav, copia)
    with bancada('CM0102 prova_save.py', {}):
        ok, motivo = laboratorio.pode_rodar()
        if not ok:
            print('carregar %s: nao iniciado: %s' % (modo, motivo))
            return None
        info, seg, erro = rodar(opcoes(a, modo, a.ate), a.semente, ['-load', copia])
    if erro:
        print('carregar %s: %s (%.0f s)' % (modo, erro, seg))
        return None
    print('carregar %s: parou em %s, %d sorteios, impressao %s, partidas %s, banco %s, %.0f s' % (
        modo, info['data'], info['sorteios'], info['impressao'], info['partidas'], info['banco'], seg))
    return info['data'], info['sorteios'], info['impressao'], info['partidas'], info['banco']


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--so-gravar', action='store_true', help='so grava e compara os .qst (sem a carga)')
    ap.add_argument('--tipo', type=int, choices=[3, 4], default=4, help='4 = RLE (padrao), 3 = sem compressao')
    ap.add_argument('--semente', type=int, default=12345)
    ap.add_argument('--pasta', default=os.path.join(tempfile.gettempdir(), 'cm0102_prova_save'), help='pasta de trabalho')
    ap.add_argument('--sav', help='save COM tecnico de ferias para o passo 3 (o .qst nao serve: pede o nome do tecnico)')
    ap.add_argument('--ate', default='2001-09-01', help='data de parada da carga AAAA-MM-DD')
    ap.add_argument('--exe', default='cm0102-1080.exe')
    ap.add_argument('--limite', type=int, default=900, help='segundos maximos por execucao')
    ap.add_argument('--comparar', nargs=2, metavar=('A', 'B'), help='so compara dois arquivos de conteiner')
    ap.add_argument('--variaveis', nargs=2, metavar=('BASE1', 'BASE2'),
                    help='com --comparar: ignora os bytes que diferem entre duas gravacoes da base')
    a = ap.parse_args()

    if a.comparar:
        extra = variaveis(*a.variaveis) if a.variaveis else None
        if extra:
            print('variaveis: %s' % ', '.join('%s %d bytes' % (n, len(v)) for n, v in extra.items()))
        igual, linhas = comparar(*a.comparar, extra=extra)
        print('\n'.join(linhas))
        return 0 if igual else 1
    if len(a.ate) != 10:
        falha('--ate tem de ser AAAA-MM-DD')
    os.makedirs(a.pasta, exist_ok=True)
    tudo = True
    arquivos = [gravar(a, modo) for modo in ('original', 'rust')]
    if None in arquivos:
        tudo = False
    else:
        igual, linhas = comparar(*arquivos)
        print('\n'.join(linhas))
        tudo &= igual
    if a.so_gravar:
        pass
    elif not a.sav:
        print('carregar: pulado (sem --sav; -load de .qst para na tela do nome do tecnico)')
    else:
        impressoes = [carregar(a, modo) for modo in ('original', 'rust')]
        bate = None not in impressoes and impressoes[0] == impressoes[1]
        print('carregar: %s' % ('IGUAL' if bate else 'DIFERENTE'))
        tudo &= bate
    print('prova do save: %s' % ('OK' if tudo else 'FALHOU'))
    return 0 if tudo else 1


if __name__ == '__main__':
    sys.exit(main())
