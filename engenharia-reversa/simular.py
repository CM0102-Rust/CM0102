r"""Simulacao em lote do CM 01/02, sem tela e sem cliques (ver docs/VERSAO_SEM_TELA.md).

Uso:
  python simular.py --ate 2001-10-01 [--semente 12345] [--ligas ENG] [--motor sim|nao]
                    [--modo rust|original|comparar] [--repetir N] [--visivel] [--limite 900]

Monta uma carreira automatica (auto_setup.cfg, treinador de ferias), abre o jogo com a semente
fixa (-seed) e a nossa DLL em modo oculto, e a DLL encerra o jogo na primeira chamada do sorteio
depois de a data do jogo alcancar --ate, gravando a impressao digital em jogo\ddraw.log:
quantidade de sorteios desde o inicio + estado completo do gerador.

Modos (rotinas do jogo):
  rust      as rotinas ja reescritas rodam em Rust (o normal)
  original  nenhuma rotina substituida; o sorteio so e observado
  comparar  original e Rust lado a lado em cada chamada; divergencias contadas no log
Mesma semente => mesma impressao digital. --repetir roda varias vezes e confere.

Cuidado com a maquina (laboratorio.py): segura a trava D:\Sistemas\.laboratorio.lock durante a
simulacao (so um teste pesado por vez entre as sessoes) e nao inicia execucao se o pool nao
paginado do kernel passar de 3 GB ou a RAM livre ficar abaixo de 4 GB; mostra a variacao do pool.

Seguranca para a instalacao do Lucas: recusa se ja houver auto_setup.cfg/quick_start.cfg na pasta;
copia o game.cfg antes e restaura depois; apaga o auto_setup.cfg; a janela fica fora da tela e
nao rouba o foco; nao grava no janela.cfg (CM_JANELA).
"""
import argparse
import hashlib
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time

import laboratorio

AQUI = os.path.dirname(os.path.abspath(__file__))
JOGO = os.path.normpath(os.path.join(AQUI, '..', 'jogo'))


def falha(msg):
    sys.exit('ERRO: ' + msg)


def rodar(args, semente):
    log = os.path.join(JOGO, 'ddraw.log')
    ambiente = dict(os.environ)
    for k in ('CM_COMPARAR', 'CM_ORIGINAL', 'CM_AUTOTESTE', 'CM_SEM_DESVIOS', 'CM_ESPIAO'):
        ambiente.pop(k, None)
    ambiente['CM_PARAR_EM'] = args.ate
    ambiente['CM_JANELA'] = '960x540'
    if not args.visivel:
        ambiente['CM_OCULTO'] = '1'
    if args.modo == 'original':
        ambiente['CM_ORIGINAL'] = '1'
    elif args.modo == 'comparar':
        ambiente['CM_COMPARAR'] = '1'
    t0 = time.time()
    p = subprocess.Popen([os.path.join(JOGO, args.exe), '-windowed', '-seed', str(semente)], cwd=JOGO, env=ambiente)
    try:
        p.wait(timeout=args.limite)
    except subprocess.TimeoutExpired:
        p.kill()
        p.wait()
        return None, time.time() - t0, 'tempo esgotado (%d s) antes de chegar em %s' % (args.limite, args.ate)
    segundos = time.time() - t0
    texto = open(log, encoding='utf-8', errors='replace').read() if os.path.exists(log) else ''
    m = re.search(r'SIMULACAO parou em (\S+) sorteios=(\d+) ponteiro=(\S+) deslocamento=(\S+) rand=(\S+)', texto)
    if not m:
        return None, segundos, 'o jogo saiu sem chegar na data (codigo %s); veja jogo\\ddraw.log' % p.returncode
    impressao = hashlib.sha1(' '.join(m.groups()[1:]).encode()).hexdigest()[:16]
    info = {'data': m.group(1), 'sorteios': int(m.group(2)), 'impressao': impressao,
            'comparacoes': re.findall(r'^(\w+): (\d+) comparacoes, (\d+) divergencias', texto, re.M),
            'divergencias': len(re.findall(r'DIVERGENCIA', texto))}
    return info, segundos, None


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--ate', required=True, help='data de parada AAAA-MM-DD')
    ap.add_argument('--semente', type=int, default=12345)
    ap.add_argument('--ligas', default='ENG', help='codigos de 3 letras separados por espaco (ex.: "ENG ESP")')
    ap.add_argument('--fundo', default='', help='ligas de fundo, mesmo formato')
    ap.add_argument('--motor', choices=['sim', 'nao'], default='nao', help='motor de lances (nao = muito mais rapido)')
    ap.add_argument('--modo', choices=['rust', 'original', 'comparar'], default='rust')
    ap.add_argument('--repetir', type=int, default=1)
    ap.add_argument('--visivel', action='store_true', help='mostra a janela (por padrao fica fora da tela)')
    ap.add_argument('--limite', type=int, default=900, help='segundos maximos por execucao')
    ap.add_argument('--exe', default='cm0102-1080.exe')
    args = ap.parse_args()

    for nome in ('auto_setup.cfg', 'quick_start.cfg'):
        if os.path.exists(os.path.join(JOGO, nome)):
            falha('ja existe jogo\\%s; nao vou sobrescrever (apague ou mova antes)' % nome)
    cfg = os.path.join(JOGO, 'game.cfg')
    copia = os.path.join(tempfile.gettempdir(), 'cm0102_game.cfg.bak')
    tem_cfg = os.path.exists(cfg)
    if tem_cfg:
        shutil.copy2(cfg, copia)
    auto = os.path.join(JOGO, 'auto_setup.cfg')
    linhas = ['LANGUAGE=POR', 'DATABASE=MINIMUM', 'MATCH_STATS=NORMAL', 'FOREGROUND_LEAGUES=' + args.ligas,
              'BACKGROUND_LEAGUES=' + args.fundo, 'ENABLE_MATCH_ENGINE=' + ('YES' if args.motor == 'sim' else 'NO'),
              'ENABLE_COMPUTER_BIDS=YES', 'ON_HOLIDAY=YES']
    resultados = []
    trava = laboratorio.Trava('CM0102 simular.py')
    try:
        trava.__enter__()
        with open(auto, 'w', newline='\r\n') as f:
            f.write('\n'.join(linhas) + '\n')
        for i in range(args.repetir):
            ok, motivo = laboratorio.pode_rodar()
            if not ok:
                print('execucao %d nao iniciada: %s' % (i + 1, motivo))
                resultados.append(None)
                break
            antes = laboratorio.memoria()
            info, seg, erro = rodar(args, args.semente)
            depois = laboratorio.memoria()
            print('    pool nao paginado %.2f -> %.2f GB, paginado %.2f -> %.2f GB' % (
                antes['pool_nao_paginado'], depois['pool_nao_paginado'], antes['pool_paginado'], depois['pool_paginado']))
            if erro:
                print('execucao %d: %s (%.0f s)' % (i + 1, erro, seg))
                resultados.append(None)
                continue
            print('execucao %d: parou em %s, %d sorteios, impressao %s, %.0f s' % (
                i + 1, info['data'], info['sorteios'], info['impressao'], seg))
            if args.modo == 'comparar':
                for rotina, total, div in info['comparacoes'][-4:]:
                    pass
                ultimos = {}
                for rotina, total, div in info['comparacoes']:
                    ultimos[rotina] = (total, div)
                for rotina, (total, div) in ultimos.items():
                    print('    %s: %s comparacoes, %s divergencias' % (rotina, total, div))
            resultados.append(info)
    finally:
        if os.path.exists(auto):
            os.remove(auto)
        if tem_cfg:
            shutil.copy2(copia, cfg)
        if trava.f and not trava.f.closed:
            trava.__exit__(None, None, None)
    if not tem_cfg or open(cfg, 'rb').read() == open(copia, 'rb').read():
        print('game.cfg restaurado; auto_setup.cfg removido')
    else:
        print('ATENCAO: game.cfg diferente da copia em', copia)
    ok = [r for r in resultados if r]
    if args.repetir > 1 and ok:
        iguais = len({r['impressao'] for r in ok}) == 1 and len(ok) == len(resultados)
        print('determinismo: %s' % ('OK, todas as execucoes iguais' if iguais else 'DIFERENTES'))
    return 0 if ok and len(ok) == len(resultados) else 1


if __name__ == '__main__':
    sys.exit(main())
