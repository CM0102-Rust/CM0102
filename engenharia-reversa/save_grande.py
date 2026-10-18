r"""O contêiner do save acima de 2 GB sem cliques (158a leva).

O início rápido (quick_start.cfg) cria uma carreira e grava qstart\<nome>.qst no mesmo contêiner
dos saves, levando junto os *.tmp da pasta (zipdir.cpp, pack 0x933610). Com um enchimento de
~2,2 GB ("0000_enchimento.tmp", que entra primeiro) os itens de verdade passam de 2 GB. O script
gera o .qst no modo pedido e confere o índice: posições em sequência, sem buracos, terminando no
fim do arquivo. Na original a posição dos itens depois do enchimento sai errada (ftell = -1);
com a 158a leva, certa. Carregar o .qst grande continua sendo pelos cliques do Lucas.

Uso:  python save_grande.py --modo original|rust [--mb 2200] [--manter]
Roda em jogo_testes, com a trava do laboratório; guarda e devolve game.cfg e *.tmp. Sem
--manter, apaga o .qst grande no fim (são ~2,3 GB).
"""
import argparse
import glob
import os
import shutil
import struct
import subprocess
import sys
import tempfile
import time

import laboratorio
from enchimento import NOME, criar

AQUI = os.path.dirname(os.path.abspath(__file__))
JOGO = os.path.normpath(os.path.join(AQUI, '..', 'jogo_testes'))
CFG = ['LANGUAGE=POR', 'DATABASE=MINIMUM', 'MATCH_STATS=NORMAL', 'FOREGROUND_LEAGUES=ENG',
       'BACKGROUND_LEAGUES=', 'ENABLE_MATCH_ENGINE=NO', 'ENABLE_COMPUTER_BIDS=YES', 'ON_HOLIDAY=NO']


def conferir(qst):
    """Lê o índice (12 bytes + 0x10c por item) e confere a sequência das posições."""
    tam = os.path.getsize(qst)
    with open(qst, 'rb') as f:
        tipo, _, n = struct.unpack('<3I', f.read(12))
        itens = []
        for _ in range(n):
            e = f.read(0x10c)
            itens.append((struct.unpack_from('<2I', e)[0], struct.unpack_from('<2I', e)[1],
                          e[8:].split(b'\0')[0].decode('latin-1')))
    erros = []
    # no tipo 4 (comprimido) o tamanho do índice é o descomprimido: só a ordem das posições vale
    for (p, s, a), (q, _, b) in zip(itens, itens[1:]):
        if (p + s != q) if tipo == 3 else not p < q:
            erros.append(f'{a} termina em {p + s:#x}, {b} começa em {q:#x}')
    fim = itens[-1][0] + itens[-1][1] if itens else 0
    if (fim != tam) if tipo == 3 else not itens[-1][0] < tam:
        erros.append(f'último item termina em {fim:#x}, arquivo tem {tam:#x}')
    alem = sum(1 for p, _, _ in itens if p >= 1 << 31)
    ench = [x for x in itens if x[2] == NOME]
    return tipo, n, tam, alem, ench, erros


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--modo', choices=['rust', 'original'], required=True)
    ap.add_argument('--mb', type=int, default=2200)
    ap.add_argument('--manter', action='store_true')
    ap.add_argument('--exe', default='cm0102-1080.exe')
    ap.add_argument('--limite', type=int, default=900)
    a = ap.parse_args()
    for nome in ('auto_setup.cfg', 'quick_start.cfg'):
        if os.path.exists(os.path.join(JOGO, nome)):
            sys.exit(f'ja existe jogo_testes\\{nome}; nao vou sobrescrever')
    if shutil.disk_usage(JOGO).free < (2 * a.mb + 4096) << 20:
        sys.exit('espaço livre insuficiente')
    cfg = os.path.join(JOGO, 'game.cfg')
    copia = os.path.join(tempfile.gettempdir(), 'cm0102_game.cfg.grande.bak')
    shutil.copy2(cfg, copia)
    guarda = os.path.join(tempfile.gettempdir(), 'cm0102_tmp_grande')
    shutil.rmtree(guarda, ignore_errors=True)
    os.makedirs(guarda)
    qst = os.path.join(JOGO, 'qstart', 'GRANDE.qst')
    # o enchimento espera fora da pasta (no mesmo disco): o jogo apaga os *.tmp ao criar a carreira
    espera = os.path.join(JOGO, '..', '_enchimento_espera.bin')
    criados = [os.path.join(JOGO, x) for x in ('GRANDE.cfg', 'quick_start.cfg', NOME)] + [espera]
    with laboratorio.Trava('CM0102 save_grande.py'):
        ok, motivo = laboratorio.pode_rodar()
        if not ok:
            sys.exit(motivo)
        try:
            for nome in glob.glob(os.path.join(JOGO, '*.tmp')):
                shutil.move(nome, guarda)
            if os.path.exists(qst):
                os.remove(qst)
            criar(espera, a.mb)
            with open(criados[0], 'w', newline='\r\n') as f:
                f.write('\n'.join(CFG) + '\n')
            with open(criados[1], 'w', newline='\r\n') as f:
                f.write('AUTO_SETUP=GRANDE.cfg\n')
            raiz = os.environ.get('SystemRoot', r'C:\Windows')
            amb = {'SystemRoot': raiz, 'PATH': os.path.join(raiz, 'system32'), 'CM_JANELA': '960x540',
                   'CM_OCULTO': '1', 'CM_ORIGINAL': '1' if a.modo == 'original' else '0', 'CM_COMPARAR': '0'}
            t0 = time.time()
            p = subprocess.Popen([os.path.join(JOGO, a.exe), '-windowed'], cwd=JOGO, env=amb)
            # entra assim que o jogo gravar o primeiro .tmp da carreira nova (renomear é instantâneo)
            while p.poll() is None and not glob.glob(os.path.join(JOGO, '*.tmp')):
                time.sleep(0.02)
            os.replace(espera, os.path.join(JOGO, NOME))
            print(f'enchimento entrou {time.time() - t0:.1f} s depois de abrir')
            try:
                p.wait(timeout=a.limite)
            except subprocess.TimeoutExpired:
                p.kill()
                p.wait()
                print('tempo esgotado')
            print(f'jogo saiu (codigo {p.returncode}) em {time.time() - t0:.0f} s')
        finally:
            for x in criados:
                if os.path.exists(x):
                    os.remove(x)
            for nome in glob.glob(os.path.join(JOGO, '*.tmp')):
                os.remove(nome)
            for nome in glob.glob(os.path.join(guarda, '*.tmp')):
                shutil.move(nome, JOGO)
            shutil.copy2(copia, cfg)
    if not os.path.exists(qst):
        sys.exit('o .qst nao foi gravado; veja jogo_testes\\ddraw.log')
    tipo, n, tam, alem, ench, erros = conferir(qst)
    print(f'{a.modo}: GRANDE.qst {tam:,} bytes, tipo {tipo}, {n} itens, {alem} alem de 2 GB, enchimento {ench}')
    print('indice OK' if not erros else 'indice ERRADO:\n  ' + '\n  '.join(erros[:5]))
    if not a.manter:
        os.remove(qst)
    return 0 if not erros else 1


if __name__ == '__main__':
    sys.exit(main())
