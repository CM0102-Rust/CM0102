r"""Prova do save: o mesmo save gravado e carregado pela original e pela Rust (levas S1..S4 do
bloco do save, docs/MAPA_SAVE.md §4). A carreira automatica nao passa pelo conteiner (zipdir.cpp):
esta prova e o banco de provas dele.

Uso:
  python prova_save.py [--so-gravar] [--tipo 4|3] [--semente 12345] [--pasta DIR]
                       [--sav SAVE.sav] [--ate 2001-08-28] [--exe cm0102-1080.exe] [--limite 1800]
  python prova_save.py --comparar A B          (so o passo 2, sem abrir o jogo)
  python prova_save.py --leva N                (a prova de uma leva do save, ver abaixo)

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

--leva N: a prova de uma leva do bloco do save. Os .qst nunca saem identicos (lixo de ponteiro no
general.dat muda a cada execucao; entre os modos ha ainda o staff history.tmp), entao a prova compara a
MESMA DLL no modo rust COM e SEM as rotinas da leva: le os nomes do ROTINAS de levas/N/integrar.py (sem
executa-lo), desliga os desvios delas no motor.rs (`continue` no laco `for i in 0..N {` da instalacao:
fica a original), compila, grava a base 2 vezes (a mascara: os bytes que variam sozinhos), restaura o
motor.rs (sempre, com recompilacao), grava 1 vez com a leva e compara fora da mascara. Saida:
"prova da leva N: IGUAL" ou "prova da leva N: DIFERENTE item X nome +0x..." (ou FALHOU se nao gravou).
O mutacoes.py --prova-save N usa a mesma base (base_da_leva).
Leva de carregar (levas/N/CARGA, criado pelo fila.py pelo CARGA = True do item): a base e a leva tambem
CARREGAM (modo rust, CM_FERIAS) uma copia do --sav (padrao jogo\FeriasTesteSave.sav, nunca alterado: MD5
conferido antes e depois) ate --ate (padrao 2001-08-28); "IGUAL" so se a gravacao E a impressao da carga
baterem. Nas mutacoes, a carga so roda se a gravacao deu igual; carga diferente, queda ou tempo estourado =
ACUSADA.

Cuidados (os do simular.py, pela mesma bancada): trava do laboratorio, vigia de memoria, game.cfg
copiado e restaurado, *.tmp guardados e devolvidos, PROVA.cfg/quick_start.cfg apagados no fim.
"""
import argparse
import ast
import contextlib
import os
import shutil
import struct
import sys
import tempfile
from types import SimpleNamespace

import laboratorio
import mutacoes
from simular import JOGO, bancada, falha, linhas_carreira, rodar, sufixo_da_pasta  # o sufixo: uma pasta de trabalho por bancada (31/10/2026)

JOGO_LUCAS = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'jogo')  # o FeriasTesteSave.sav

CABECALHO = 12 + 1000 * 0x10c  # 0x416ec: tipo, id, quantidade e as 1000 entradas
# Campos que dependem do relogio da maquina, ignorados na comparacao. general.dat = versao (4) +
# resumo de build_save_game_summary 0x591f80 (0xf74 bytes) + ...; no resumo, +0xf6c e o tempo de
# jogo acumulado ([0xdd7140] += time() - [0xdd7144]) e +0xf70 o time() da gravacao (0x592232).
# No modo simulacao o time() e fixo e os dois deveriam sair iguais; ficam fora mesmo assim porque
# nao sao estado do jogo e, fora do modo simulacao, mudam sempre. Diferenca neles e so informada.
# O "id unico" (+0xf78, [0xdd7120]) e a semente do -seed (0x81b9aa): fica na comparacao.
IGNORAR = {'general.dat': [(4 + 0xf6c, 4, 'tempo de jogo no resumo'), (4 + 0xf70, 4, 'time() da gravacao no resumo')],
           # desde 31/10/2026 (sem a zeragem da 0x5fb810 reset_human_record): o rascunho de contrato de cada um dos 16
           # humanos (registros de 0x4a80 a partir de +0x3dc0) leva as datas sem valor do STAFF_CONTRACT temporario
           'human_manager.dat': [(0x3dc0 + k * 0x4a80 + o, n, 'datas sem valor do rascunho de contrato do humano %d' % k)
                                 for k in range(16) for o, n in ((0, 12), (0x24, 2))]}


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


MASCARA_GUARDADA = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'mascara_save.json')


def acumular(v):
    """Junta a mascara aprendida agora com a guardada (engenharia-reversa/mascara_save.json, versionada) e
    grava a uniao: duas gravacoes da base nem sempre mostram todo o lixo que varia (na 219a o +0x51c do
    general.dat, ja visto em outras provas, calhou de sair igual nas duas). So cresce com bytes que variaram
    sozinhos numa base, nunca com diferencas de leva."""
    import json
    try:
        guardada = {k: set(x) for k, x in json.load(open(MASCARA_GUARDADA, encoding='utf-8')).items()}
    except (OSError, ValueError):
        guardada = {}
    for k, x in v.items():
        guardada.setdefault(k, set()).update(x)
    json.dump({k: sorted(x) for k, x in sorted(guardada.items())}, open(MASCARA_GUARDADA, 'w', encoding='utf-8'))
    return guardada


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
            # o lixo e de ponteiro: o campo de 4 bytes inteiro varia, mesmo quando so alguns bytes mudaram
            # nas duas gravacoes (23/10: +0x711 escapou da mascara de +0x710 na prova da 218a)
            v[nome] = {j for i in range(min(len(ca), len(cb))) if ca[i] != cb[i] for j in range(i & ~3, (i & ~3) + 4)}
    return v


def cauda_do_nome(d, i):
    """No index.dat (8 bytes de cabeçalho, entradas de 67: nome de 51 terminado em zero, FORMATOS.md): o byte i está
    depois do zero que termina o nome da entrada. Desde 31/10/2026 (sem a zeragem da save_database 0x52dad0) esses
    bytes são as locais sem valor que ela grava, e variam com o lixo de pilha: não são estado do jogo."""
    import re
    if i < 8:
        return False
    e, p = divmod(i - 8, 67)
    # da primeira entrada sem nome de arquivo em diante (as vagas além das 22 tabelas) tudo é sobra
    for k in range(e + 1):
        n = d[8 + 67 * k:8 + 67 * k + 51].split(b'\0')[0]
        if not re.fullmatch(rb'[\w ]+\.\w+', n):
            return True
    nome = d[8 + 67 * e:8 + 67 * e + 51]
    z = nome.find(b'\0')
    return p < 51 and z >= 0 and p > z


def sobra_do_amistoso(da, db, i):
    """No fixtures_<ano>.tmp (u32 quantos e registros de partida de 0x4f, MAPA_SAVE §7): o byte i é sobra de pilha de
    um amistoso (+0x30 = 0, categoria +0x3a 1 ou 9). Friendly::process_friendly_reply (0x5c5f40) monta a partida num
    local de 0x4f (esp+0x14) e a manda ao calendário (0x5aa680) sem gravar +0x24..+0x27 e, na palavra +0x4d, só limpa
    os bits 8..10 (and 0xf8ff, 0x5c61a6) e põe 0x99 no byte baixo: os bits 0xf8 do +0x4e ficam com o que havia na pilha.
    Desde 31/10/2026 (sem zeragens) variam entre duas gravações da mesma base (bases de 31/10: +0x4e 0xf8/0x00/0x08,
    +0x24 0x5d4e19f3/0x5e511863). A cópia em Rust (cqa_2, 263a) grava zero neles; a máscara vale para a original."""
    if i < 4:
        return False
    r, o = divmod(i - 4, 0x4f)
    x, y = da[4 + r * 0x4f:4 + r * 0x4f + 0x4f], db[4 + r * 0x4f:4 + r * 0x4f + 0x4f]
    if len(x) < 0x4f or len(y) < 0x4f or not all(z[0x30:0x32] == b'\0\0' and z[0x3a] in (1, 9) for z in (x, y)):
        return False
    return 0x24 <= o < 0x28 or (o == 0x4e and not (x[o] ^ y[o]) & 0x07)


def sobra_da_copa(da, db, i):
    """No fixtures_<ano>.tmp: o byte i é o +0x4e de um jogo de rodada de copa (+0x30, o código da fase, diferente de
    zero) e só os bits 0xf0 diferem. A Cup cria a partida num local de 0x4f (Cup
    0x51a150: esp+0x34; Cup::create_match 0x51cb90: esp+0x18) e a manda ao calendário (0x5aa680) mexendo na palavra
    +0x4d só com and 0xf4ff/0xfbff/0xf0ff e or 4 no +0x4e (0x51a999, 0x51aa30, 0x51ae53, 0x51cd21, 0x51cdfc, 0x51ce03):
    os bits 12..15 (0xf0 do +0x4e) ficam com o que havia na pilha; a 0x51ebe0 (as datas da rodada) copia o registro
    inteiro (rep movsd) e o leva junto. Ninguém lê os bits 14-15; os 12-13 o motor da partida regrava ao começar o jogo
    (0x6b8089, do 0x923630) e só o fechamento do mesmo jogo os lê (0x6b6062, a única leitura): no registro gravado são
    estado morto, jogado ou não (o placar não serve de marca: a carreira nova já traz +0x43 = 0 nos jogos da
    repescagem, e resultados da base de dados nos da fase de grupos). A sobra muda com o
    que rodou antes na pilha: na 265a, as cargas em Rust da carreira nova (0x5aee90, 0x59b2a0, na pilha particular) não
    deixam o rastro da original, e as quartas da Copa América (0x1c2, +0x30 = 0x78) e a repescagem da Oceania (0x1b5,
    +0x30 = 0x99/0x9a) saíram com 0x40 na base e 0x10/0x00 na leva (1/11/2026)."""
    if i < 4:
        return False
    r, o = divmod(i - 4, 0x4f)
    x, y = da[4 + r * 0x4f:4 + r * 0x4f + 0x4f], db[4 + r * 0x4f:4 + r * 0x4f + 0x4f]
    if o != 0x4e or len(x) < 0x4f or len(y) < 0x4f:
        return False
    return all(z[0x30:0x32] != b'\0\0' for z in (x, y)) and not (x[o] ^ y[o]) & 0x0f


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
                if campo is None and nome == 'index.dat' and cauda_do_nome(ca, i) and cauda_do_nome(cb, i):
                    campo = (i, 1, 'cauda do nome da entrada (locais sem valor da save_database 0x52dad0)')
                if campo is None and nome.startswith('fixtures_') and sobra_do_amistoso(ca, cb, i):
                    campo = (i, 1, 'sobra de pilha do amistoso (+0x24 e bits 0xf8 do +0x4e, 0x5c5f40)')
                if campo is None and nome.startswith('fixtures_') and sobra_da_copa(ca, cb, i):
                    campo = (i, 1, 'sobra de pilha da copa (bits 0xf0 do +0x4e, 0x51a150/0x51cb90)')
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


def gravar(a, modo, nome=None):
    """Grava qstart\\PROVA.qst no modo pedido e o move para a pasta de trabalho (como <nome ou modo>.qst).
    Devolve o caminho ou None."""
    qst = os.path.join(JOGO, 'qstart', 'PROVA.qst')
    destino = os.path.join(a.pasta, (nome or modo) + '.qst')
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


MOTOR = os.path.join(mutacoes.RAIZ, 'rust', 'src', 'motor.rs')
LACO = '\n    ];\n    for i in 0..N {\n'  # o laco da instalacao dos desvios (a ancora do integrar.py)


def rotinas_da_leva(n):
    """Os nomes do motor.rs ("motor_<nome>") do ROTINAS = [...] de levas/N/integrar.py, sem executa-lo."""
    arv = ast.parse(open(os.path.join(mutacoes.RAIZ, 'levas', str(n), 'integrar.py'), encoding='utf-8').read())
    for no in arv.body:
        if isinstance(no, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'ROTINAS' for t in no.targets):
            return ['motor_' + (r[0] if isinstance(r, (tuple, list)) else r) for r in ast.literal_eval(no.value)]
    falha('levas/%s/integrar.py sem ROTINAS = [...]' % n)


@contextlib.contextmanager
def desligadas(nomes):
    """O motor.rs com os desvios de `nomes` desligados (a original roda), compilado e copiado para o jogo
    dos testes; no fim (mesmo com erro ou Ctrl+C) o motor.rs volta e e recompilado."""
    original = open(MOTOR, encoding='utf-8').read()
    assert original.count(LACO) == 1, 'laco da instalacao nao achado no motor.rs'
    # (até 31/10/2026 a rotina que estava em ZERADAS rodava a original com a zeragem; não há mais zeragem)
    trava = ('        if [%s].contains(&nomes[i]) {\n'
             '            continue;\n'
             '        }\n') % ', '.join('"%s"' % x for x in nomes)
    try:
        open(MOTOR, 'w', encoding='utf-8').write(original.replace(LACO, LACO + trava))
        mutacoes.compilar()
        yield
    finally:
        open(MOTOR, 'w', encoding='utf-8').write(original)
        mutacoes.compilar()  # toca a data (os.utime) antes do cargo
        print('motor.rs restaurado e recompilado', flush=True)


def e_de_carga(n):
    return os.path.exists(os.path.join(mutacoes.RAIZ, 'levas', str(n), 'CARGA'))


def base_da_leva(a, n):
    """(base1, base2, mascara, carga): duas gravacoes rust com as rotinas da leva desligadas, os bytes que
    variam entre elas e, se a leva for de carregar (levas/N/CARGA), a impressao da carga da base (senao None).
    None se alguma gravacao (ou a carga) falhar."""
    nomes = rotinas_da_leva(n)
    carga = e_de_carga(n)
    if carga:
        a.sav = a.sav or FERIAS
        conferir_sav(a)
    print('leva %s: desligando %s%s' % (n, ', '.join(nomes), ' (com a carga de %s ate %s)' % (a.sav, a.ate) if carga else ''), flush=True)
    with desligadas(nomes):
        bases = [gravar(a, 'rust', 'base%d' % k) for k in (1, 2)]
        imp = carregar(a, 'rust') if carga and None not in bases else None
    if None in bases or (carga and imp is None):
        return None
    mascara = acumular(variaveis(*bases))
    print('variaveis: %s' % (', '.join('%s %d bytes' % (x, len(v)) for x, v in mascara.items()) or 'nenhuma'), flush=True)
    return bases[0], bases[1], mascara, imp


def contra_a_base(a, base, nome):
    """Grava 1 vez em modo rust (com o motor.rs como esta) e compara com a base fora da mascara; leva de
    carregar (base[3] = impressao da carga da base): se a gravacao bateu, carrega e compara a impressao.
    (igual, texto); texto = 'IGUAL ...', 'DIFERENTE item ...', 'DIFERENTE carga ...' ou 'sem .qst/carga ...'."""
    q = gravar(a, 'rust', nome)
    if q is None:
        return False, 'sem .qst (caiu ou estourou o tempo)'
    igual, linhas = comparar(base[0], q, extra=base[2])
    texto = linhas[-1].replace('comparar: ', '', 1)
    if not igual or base[3] is None:
        return igual, texto
    imp = carregar(a, 'rust')
    if imp is None:
        return False, 'sem carga (caiu ou estourou o tempo); gravacao ' + texto
    if imp != base[3]:
        return False, 'DIFERENTE carga: %s, base %s; gravacao %s' % (resumo_carga(imp), resumo_carga(base[3]), texto)
    return True, '%s; carga IGUAL (%s)' % (texto, resumo_carga(imp))


def resumo_carga(imp):
    return 'parou em %s, %d sorteios, impressao %s' % imp[:3]


FERIAS = os.path.join(JOGO_LUCAS, 'FeriasTesteSave.sav')
MD5_FERIAS = '33fda465193b5e8e6d7ace93ecd44559'


def md5(p):
    import hashlib
    return hashlib.md5(open(p, 'rb').read()).hexdigest()


def conferir_sav(a):
    """O --sav existe e, se for o FeriasTesteSave do Lucas, esta intocado (a prova so carrega copias)."""
    if not os.path.isfile(a.sav):
        falha('save de carga nao achado: %s' % a.sav)
    if os.path.normcase(os.path.abspath(a.sav)) == os.path.normcase(FERIAS) and md5(a.sav) != MD5_FERIAS:
        falha('%s mudou (MD5 %s, esperado %s): pare e avise o Lucas' % (a.sav, md5(a.sav), MD5_FERIAS))


def prova_da_leva(a, n):
    if mutacoes.jogo_aberto():
        falha('ja ha um jogo deste projeto aberto: um teste por vez')
    base = base_da_leva(a, n)
    if base is None:
        print('prova da leva %s: FALHOU (a base nao gravou)' % n)
        return 1
    igual, texto = contra_a_base(a, base, 'leva')
    if base[3] is not None:
        conferir_sav(a)  # o save do Lucas continua intocado
    print('prova da leva %s: %s' % (n, texto if igual or texto.startswith('DIFERENTE') else 'FALHOU ' + texto))
    return 0 if igual else 1


def carregar(a, modo):
    """-load de uma copia do --sav e carreira ate --ate. Devolve a tupla da impressao ou None."""
    copia = os.path.join(a.pasta, 'carga_%s%s' % (modo, os.path.splitext(a.sav)[1]))
    antes = md5(a.sav)
    shutil.copy2(a.sav, copia)
    with bancada('CM0102 prova_save.py', {}):
        ok, motivo = laboratorio.pode_rodar()
        if not ok:
            print('carregar %s: nao iniciado: %s' % (modo, motivo))
            return None
        # CM_FERIAS: a DLL poe o tecnico de ferias logo depois do -load (o jogo nao grava de ferias; simulation.rs)
        os.environ['CM_FERIAS'] = '1'
        # a carga regrava o hall_of_fame.bin do jogo de testes (o do save): guarda e devolve, para nao mudar o
        # que as carreiras e as gravacoes seguintes leem
        hof = os.path.join(JOGO, 'hall_of_fame.bin')
        hof_guardado = open(hof, 'rb').read() if os.path.exists(hof) else None
        try:
            info, seg, erro = rodar(opcoes(a, modo, a.ate), a.semente, ['-load', copia])
        finally:
            if hof_guardado is not None:
                open(hof, 'wb').write(hof_guardado)
            os.environ.pop('CM_FERIAS', None)
    if md5(a.sav) != antes:
        falha('%s mudou durante a carga: pare e avise o Lucas' % a.sav)
    if erro:
        print('carregar %s: %s (%.0f s)' % (modo, erro, seg))
        return None
    print('carregar %s: parou em %s, %d sorteios, impressao %s, partidas %s, banco %s, %.0f s' % (
        modo, info['data'], info['sorteios'], info['impressao'], info['partidas'], info['banco'], seg))
    return info['data'], info['sorteios'], info['impressao'], info['partidas'], info['banco']


def parser():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--so-gravar', action='store_true', help='so grava e compara os .qst (sem a carga)')
    ap.add_argument('--tipo', type=int, choices=[3, 4], default=4, help='4 = RLE (padrao), 3 = sem compressao')
    ap.add_argument('--semente', type=int, default=12345)
    ap.add_argument('--pasta', default=os.path.join(tempfile.gettempdir(), 'cm0102_prova_save' + sufixo_da_pasta()), help='pasta de trabalho')
    ap.add_argument('--sav', help='save COM tecnico de ferias para o passo 3 (o .qst nao serve: pede o nome do tecnico); '
                    'na leva de carregar o padrao e jogo\\FeriasTesteSave.sav')
    ap.add_argument('--ate', default='2001-08-28', help='data de parada da carga AAAA-MM-DD (2001-08-28: original e rust '
                    'dao a mesma impressao com o FeriasTesteSave)')
    ap.add_argument('--exe', default='cm0102-1080.exe')
    ap.add_argument('--limite', type=int, default=1800, help='segundos maximos por execucao (1800 desde 01/11/2026: com bancadas em paralelo a carga passa de 900)')
    ap.add_argument('--comparar', nargs=2, metavar=('A', 'B'), help='so compara dois arquivos de conteiner')
    ap.add_argument('--variaveis', nargs=2, metavar=('BASE1', 'BASE2'),
                    help='com --comparar: ignora os bytes que diferem entre duas gravacoes da base')
    ap.add_argument('--leva', help='a prova da leva N do save: a mesma DLL com e sem as rotinas dela')
    return ap


def padrao():
    """As opcoes padrao (para o mutacoes.py --prova-save)."""
    a = parser().parse_args([])
    os.makedirs(a.pasta, exist_ok=True)
    return a


def main():
    a = parser().parse_args()

    if a.comparar:
        extra = variaveis(*a.variaveis) if a.variaveis else None
        if extra:
            print('variaveis: %s' % ', '.join('%s %d bytes' % (n, len(v)) for n, v in extra.items()))
        igual, linhas = comparar(*a.comparar, extra=extra)
        print('\n'.join(linhas))
        return 0 if igual else 1
    if a.leva:
        os.makedirs(a.pasta, exist_ok=True)
        return prova_da_leva(a, a.leva)
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
