r"""Simulacao em lote do CM 01/02, sem tela e sem cliques (ver docs/VERSAO_SEM_TELA.md).

Uso:
  python simular.py --ate 2001-10-01 [--semente 12345] [--ligas ENG] [--motor sim|nao]
                    [--modo rust|original|comparar] [--repetir N] [--visivel] [--limite 900] [--detector]

Monta uma carreira automatica (auto_setup.cfg, treinador de ferias), abre o jogo com a semente
fixa (-seed) e a nossa DLL em modo oculto, e a DLL encerra o jogo na primeira chamada do sorteio
depois de a data do jogo alcancar --ate, gravando a impressao digital em jogo\ddraw.log:
quantidade de sorteios desde o inicio + estado completo do gerador.

Modos (rotinas do jogo):
  rust      as rotinas ja reescritas rodam em Rust (o normal)
  original  nenhuma rotina substituida; o sorteio so e observado
  comparar  original e Rust lado a lado em cada chamada; divergencias contadas no log
Mesma semente => mesma impressao digital. --repetir roda varias vezes e confere. --detector roda mais uma com a
pilha particular envenenada e so registra se saiu diferente (detector de lixo; ver a funcao detector).

Cuidado com a maquina (laboratorio.py): segura a trava D:\Sistemas\.laboratorio.lock durante a
simulacao (so um teste pesado por vez entre as sessoes) e nao inicia execucao se o pool nao
paginado do kernel passar de 3 GB ou a RAM livre ficar abaixo de 4 GB; mostra a variacao do pool.

Seguranca para a instalacao do Lucas: recusa se ja houver auto_setup.cfg/quick_start.cfg na pasta;
copia o game.cfg antes e restaura depois; apaga o auto_setup.cfg; guarda os *.tmp que houver
e os devolve no fim (cada execucao comeca sem eles: sobras de uma execucao mudam a seguinte); a janela fica fora da tela e
nao rouba o foco; nao grava no janela.cfg (CM_JANELA).
"""
import argparse
import contextlib
import glob
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
# Os testes automaticos rodam numa copia so deles (jogo_testes): a carreira automatica depende do que
# esta na pasta (saves, hall_of_fame.bin), e os jogos do Lucas em jogo/ mudavam a referencia (15/10/2026).
JOGO = os.path.normpath(os.path.join(AQUI, '..', 'jogo_testes' if os.path.isdir(os.path.join(AQUI, '..', 'jogo_testes')) else 'jogo'))


def falha(msg):
    sys.exit('ERRO: ' + msg)


def limpar_temporarios():
    """Apaga os arquivos de trabalho do jogo (*.tmp: historicos, estatisticas de partidas...).
    A simulacao encerra o jogo a forca e eles ficam; a execucao seguinte os reaproveita e a
    carreira muda (mesma semente dava resultados diferentes conforme a execucao anterior)."""
    for nome in glob.glob(os.path.join(JOGO, '*.tmp')):
        os.remove(nome)


def mundo():
    """'MUNDO' (12/11/2026): ENG e todas as nacoes do nation.dat (codigo de 3 letras em +0x53, registro de 290 bytes)."""
    d = open(os.path.join(JOGO, 'Data', 'nation.dat'), 'rb').read()
    cods = {d[i + 0x53:i + 0x56].decode('latin1') for i in range(0, len(d) - 289, 290)}
    return ' '.join(['ENG'] + sorted(c for c in cods if c.isalpha() and c.isupper() and c != 'ENG'))


def linhas_carreira(ligas='ENG', fundo='', motor='nao'):
    """As linhas do auto_setup.cfg da carreira automatica de ferias (formato em docs/VERSAO_SEM_TELA.md)."""
    ligas = mundo() if ligas == 'MUNDO' else ligas
    return ['LANGUAGE=POR', 'DATABASE=MINIMUM', 'MATCH_STATS=NORMAL', 'FOREGROUND_LEAGUES=' + ligas,
            'BACKGROUND_LEAGUES=' + fundo, 'ENABLE_MATCH_ENGINE=' + ('YES' if motor == 'sim' else 'NO'),
            'ENABLE_COMPUTER_BIDS=YES', 'ON_HOLIDAY=YES']


def sufixo_da_pasta():
    """Vazio no jogo_testes da principal (os nomes de sempre); outra pasta (worktree, copia em %TEMP%) ganha um sufixo
    proprio, para duas bancadas em paralelo nao se pisarem na copia do game.cfg nem nos .tmp guardados."""
    if os.path.normcase(JOGO) == os.path.normcase(r'D:\Sistemas\CM0102\jogo_testes'):
        return ''
    return '_' + hashlib.md5(os.path.normcase(JOGO).encode()).hexdigest()[:8]


@contextlib.contextmanager
def bancada(dono, arquivos, sem_trava=False):
    """A pasta do jogo preparada para um teste pesado, e devolvida como estava no fim.

    Segura a trava do laboratorio; confere e copia o game.cfg (restaura no fim); guarda os *.tmp
    que houver (devolve no fim); grava `arquivos` ({nome: [linhas]}, ex.: auto_setup.cfg) na pasta
    do jogo e os apaga no fim. Recusa se algum deles (ou auto_setup/quick_start) ja existir.
    Usado tambem pelo prova_save.py."""
    trava = laboratorio.Trava(dono)
    if sem_trava:
        print('rodando SEM a trava do laboratorio (--sem-trava)')
    else:
        trava.__enter__()
    try:
        for nome in set(arquivos) | {'auto_setup.cfg', 'quick_start.cfg'}:
            if os.path.exists(os.path.join(JOGO, nome)):
                falha('ja existe jogo\\%s; nao vou sobrescrever (apague ou mova antes)' % nome)
        cfg = os.path.join(JOGO, 'game.cfg')
        copia = os.path.join(tempfile.gettempdir(), 'cm0102_game.cfg%s.bak' % sufixo_da_pasta())
        tem_cfg = os.path.exists(cfg)
        if tem_cfg:
            # Preferencias validas (marca 0x1553117c) e som desligado (bit 0x80 de +4): uma queda de energia
            # zerou o arquivo em 11/10/2026; sem a marca o jogo usa o padrao (som ligado) e, sem placa de som
            # nos testes, para numa caixa de erro escondida. Nao copiar por cima de uma copia boa.
            dados = open(cfg, 'rb').read()
            if len(dados) < 8 or int.from_bytes(dados[:4], 'little') != 0x1553117C:
                falha('jogo\\game.cfg invalido (zerado ou corrompido); restaure uma copia boa antes')
            if dados[4] & 0x80:
                falha('jogo\\game.cfg com o som ligado: o jogo para na caixa "Unable to initialise the sound" nos testes')
            shutil.copy2(cfg, copia)
        # Arquivos de trabalho que ja estavam na pasta: guardados e devolvidos no fim.
        guarda_tmp = os.path.join(tempfile.gettempdir(), 'cm0102_tmp_guardados' + sufixo_da_pasta())
        shutil.rmtree(guarda_tmp, ignore_errors=True)
        os.makedirs(guarda_tmp)
        for nome in glob.glob(os.path.join(JOGO, '*.tmp')):
            shutil.move(nome, guarda_tmp)
        try:
            for nome, linhas in arquivos.items():
                with open(os.path.join(JOGO, nome), 'w', newline='\r\n') as f:
                    f.write('\n'.join(linhas) + '\n')
            yield
        finally:
            for nome in arquivos:
                if os.path.exists(os.path.join(JOGO, nome)):
                    os.remove(os.path.join(JOGO, nome))
            limpar_temporarios()
            for nome in glob.glob(os.path.join(guarda_tmp, '*.tmp')):
                shutil.move(nome, JOGO)
            if tem_cfg:
                shutil.copy2(copia, cfg)
            if not tem_cfg or open(cfg, 'rb').read() == open(copia, 'rb').read():
                print('game.cfg restaurado; %s removido' % ', '.join(arquivos))
            else:
                print('ATENCAO: game.cfg diferente da copia em', copia)
    finally:
        if trava.f and not trava.f.closed:
            trava.__exit__(None, None, None)


def caixa_do_jogo(log):
    """'caixa do jogo: titulo | texto' da primeira linha `msgbox AAAA-DDD titulo | texto` do ddraw.log (a DLL registra
    toda caixa de mensagem do jogo antes de ela aparecer), ou None."""
    if not os.path.exists(log):
        return None
    with open(log, encoding='utf-8', errors='replace') as f:
        m = re.search(r'^msgbox \S+ (.*)$', f.read(), re.M)
    return m and 'caixa do jogo: ' + m.group(1)


def rodar(args, semente, extra=()):
    """Uma execucao do jogo; `extra` vai no fim da linha de comando (ex.: ['-load', save])."""
    limpar_temporarios()
    log = os.path.join(JOGO, 'ddraw.log')
    if os.path.exists(log):
        os.remove(log)  # nada da execucao anterior (uma linha msgbox velha pararia esta)
    # Ambiente minimo e fixo: a CRT do jogo copia cada variavel para a heap logo no inicio, e
    # variaveis diferentes (do terminal, ou CM_ORIGINAL presente so num modo) deslocavam a heap
    # e mudavam a carreira. Todas as CM_* vao sempre, com valores do mesmo tamanho.
    raiz = os.environ.get('SystemRoot', r'C:\Windows')
    ambiente = {
        'SystemRoot': raiz, 'windir': raiz, 'PATH': raiz + r'\system32;' + raiz,
        'TEMP': tempfile.gettempdir(), 'TMP': tempfile.gettempdir(),
        'CM_PARAR_EM': args.ate, 'CM_JANELA': '960x540',
        'CM_OCULTO': '0' if args.visivel else ('2' if args.som else '1'),
        'CM_ORIGINAL': '1' if args.modo == 'original' else '0',
        'CM_COMPARAR': '1' if args.modo == 'comparar' else '0',
        'CM_RASTRO': os.environ.get('CM_RASTRO', '0000-00-00'),
        'CM_COBERTURA': '1' if args.cobertura else '0',
    }
    # CM_FERIAS (carga da prova do save, prova_save.py): so quando pedida, para nao mexer no ambiente
    # (e na heap) das outras execucoes; vai igual nos dois modos da carga.
    if os.environ.get('CM_FERIAS', '0') != '0':
        ambiente['CM_FERIAS'] = '1'
    # CM_RESOLUCAO=1080 (experimento: o cm0102.exe original vira o 1080 em memoria) e o diagnostico
    # CM_RESOLUCAO_CONFERIR; a sonda de pilha (CM_SONDA_*, sonda_pilha.py): so quando pedidos, pelo mesmo motivo do CM_FERIAS
    for nome in ('CM_RESOLUCAO', 'CM_RESOLUCAO_CONFERIR', 'CM_SONDA_PILHA', 'CM_SONDA_FAIXA', 'CM_SONDA_VENENO', 'CM_SONDA_MOSTRAR', 'CM_SONDA_LISTA', 'CM_DIAG_PESSOA', 'CM_DIAG_CLUBE', 'CM_DIAG_FORCAR'):
        if os.environ.get(nome, '0') != '0':
            ambiente[nome] = os.environ[nome]
    if len(ambiente['CM_RASTRO']) != 10:
        falha('CM_RASTRO tem de ser AAAA-MM-DD')
    referencia = []
    if getattr(args, 'referencia', None):
        with open(args.referencia, encoding='utf-8') as f:
            referencia = [l.rstrip('\n') for l in f if l.startswith(('dia ', 'day '))]
    t0 = time.time()
    p = subprocess.Popen([os.path.join(JOGO, args.exe), '-windowed', '-seed', str(semente), *extra], cwd=JOGO, env=ambiente)
    while True:
        try:
            p.wait(timeout=5)
            break
        except subprocess.TimeoutExpired:
            pass
        # uma caixa do jogo (desvios::caixa_de_mensagem): oculto, o jogo ficaria parado esperando o OK
        caixa = caixa_do_jogo(log)
        if caixa:
            p.kill()
            p.wait()
            return None, time.time() - t0, caixa
        if time.time() - t0 > args.limite:
            p.kill()
            p.wait()
            return None, time.time() - t0, 'tempo esgotado (%d s) antes de chegar em %s' % (args.limite, args.ate)
        # --referencia: o diario (linha "dia" de cada dia do jogo) comparado com o da base; o primeiro dia
        # diferente ja decide (as mutacoes da carreira, 28/10/2026); so linhas completas
        if referencia and os.path.exists(log):
            with open(log, encoding='utf-8', errors='replace') as f:
                dias = [l for l in f.read().split('\n')[:-1] if l.startswith(('dia ', 'day '))]
            for feito, esperado in zip(dias, referencia):
                if feito != esperado:
                    p.kill()
                    p.wait()
                    return None, time.time() - t0, 'divergiu da base em %s' % feito.split()[1]
        # --parar-na-divergencia: a primeira divergencia ja decide (as mutacoes no modo comparacao)
        if args.parar_na_divergencia and os.path.exists(log):
            with open(log, encoding='utf-8', errors='replace') as f:
                if re.search('DIVERGEN(?:CIA|CE)', f.read()):
                    p.kill()
                    p.wait()
                    return None, time.time() - t0, 'parado na primeira divergencia'
    segundos = time.time() - t0
    texto = open(log, encoding='utf-8', errors='replace').read() if os.path.exists(log) else ''
    m = re.search(r'(?:SIMULACAO parou em|SIMULATION stopped at) (\S+) (?:sorteios|draws)=(\d+) (?:ponteiro|pointer)=(\S+) (?:deslocamento|offset)=(\S+) rand=(\S+)', texto)
    if not m and caixa_do_jogo(log):
        return None, segundos, caixa_do_jogo(log)
    if not m:
        return None, segundos, 'o jogo saiu sem chegar na data (codigo %s); veja jogo\\ddraw.log' % p.returncode
    impressao = hashlib.sha1(' '.join(m.groups()[1:]).encode()).hexdigest()[:16]
    mp = re.search(r'^(?:partidas|matches): (\d+) (?:motores|engines), (?:impressao|fingerprint) (\w+)', texto, re.M)
    info = {'data': m.group(1), 'sorteios': int(m.group(2)), 'impressao': impressao,
            'partidas': '%s/%s' % mp.groups() if mp else '-',
            'banco': (re.findall(r'^(?:memoria: banco|memory: bank) ([0-9a-f]{16})', texto, re.M) or ['-'])[-1],
            'comparacoes': re.findall(r'^(\w+): (\d+) (?:comparacoes|comparisons), (\d+) (?:divergencias|divergences)', texto, re.M),
            'divergencias': len(re.findall(r'DIVERGEN(?:CIA|CE)', texto)),
            'avisos': re.findall(r'^(?:alocador|cobertura|allocator|coverage): (?:AVISO|WARNING).*', texto, re.M),
            'cobertura': [(e, a, int(n)) for e, a, n in re.findall(r'^(?:cobertura|coverage) ([0-9a-f]+) (\S*) (\d+)$', texto, re.M)]}
    return info, segundos, None


VENENO = '5a5a5a5a'


def detector(args, normais):
    """Detector de lixo (01/11/2026, so registro): mais uma execucao, com jogo\\veneno.txt, em que a DLL enche a pilha
    particular inteira com VENENO a cada entrada vinda de fora (desvios::VENENO). Uma rotina original que le variavel
    local sem valor passa a ler o veneno: se a carreira sair diferente das normais, ha uma leitura de lixo a achar e
    corrigir. Nao muda o codigo de saida nem a impressao registrada; as linhas nao comecam com "execucao" (gen_docs)."""
    pode, motivo = laboratorio.pode_rodar()
    if not pode:
        print('detector de lixo: nao rodou (%s)' % motivo)
        return
    arq = os.path.join(JOGO, 'veneno.txt')
    if os.path.exists(arq):
        falha('ja existe jogo\\veneno.txt; nao vou sobrescrever')
    with open(arq, 'w') as f:
        f.write(VENENO)
    try:
        info, seg, erro = rodar(args, args.semente)
    finally:
        os.remove(arq)
    if erro:
        print('detector de lixo: DIFERENTE (%s, %.0f s)' % (erro, seg))
        return
    print('detector: parou em %s, %d sorteios, impressao %s, partidas %s, banco %s, %.0f s' % (
        info['data'], info['sorteios'], info['impressao'], info['partidas'], info['banco'], seg))
    chave = lambda r: (r['impressao'], r['partidas'], r['banco'])
    if all(chave(r) == chave(info) for r in normais):
        print('detector de lixo: igual')
    else:
        print('detector de lixo: DIFERENTE (impressao %s, partidas %s, banco %s)' % chave(info))


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
    ap.add_argument('--som', action='store_true', help='mantem o som do jogo (por padrao os testes ocultos ficam mudos)')
    ap.add_argument('--limite', type=int, default=900, help='segundos maximos por execucao')
    ap.add_argument('--parar-na-divergencia', action='store_true', help='encerra a execucao na primeira DIVERGENCIA do ddraw.log')
    ap.add_argument('--referencia', help='diario da base (linhas "dia"): encerra no primeiro dia diferente')
    ap.add_argument('--exe', default='cm0102-1080.exe')
    ap.add_argument('--cobertura', action='store_true', help='conta as chamadas das funcoes de jogo/cobertura.txt (gerado por cobertura.py)')
    ap.add_argument('--sem-trava', action='store_true', help='nao espera a trava do laboratorio (o vigia de memoria continua)')
    ap.add_argument('--detector', action='store_true', help='depois das execucoes, mais uma com a pilha particular envenenada (detector de lixo, so registro)')
    args = ap.parse_args()

    resultados = []
    with bancada('CM0102 simular.py', {'auto_setup.cfg': linhas_carreira(args.ligas, args.fundo, args.motor)}, args.sem_trava):
        for i in range(args.repetir):
            ok, motivo = laboratorio.pode_rodar()
            # a RAM livre oscila perto do mínimo com outras sessões abertas: espera até 30 min antes de desistir
            # (25/10/2026: uma mutação "não iniciada" por 4,0 GB derrubou a cadeia inteira)
            for _ in range(60):
                if ok:
                    break
                print('  aguardando: %s' % motivo, flush=True)
                time.sleep(30)
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
            print('execucao %d: parou em %s, %d sorteios, impressao %s, partidas %s, banco %s, %.0f s' % (
                i + 1, info['data'], info['sorteios'], info['impressao'], info['partidas'], info['banco'], seg))
            for aviso in info['avisos']:
                print('    ' + aviso)
            if info['cobertura']:
                usadas = [c for c in info['cobertura'] if c[2]]
                print('    cobertura: %d de %d funcoes chamadas (lista completa no ddraw.log)' % (len(usadas), len(info['cobertura'])))
                por_arquivo = {}
                for e, a, n in info['cobertura']:
                    t = por_arquivo.setdefault(a, [0, 0])
                    t[0] += 1
                    t[1] += n > 0
                for a, (total, usadas_a) in sorted(por_arquivo.items()):
                    print('      %-24s %3d de %3d' % (a, usadas_a, total))
            if args.modo == 'comparar':
                for rotina, total, div in info['comparacoes'][-4:]:
                    pass
                ultimos = {}
                for rotina, total, div in info['comparacoes']:
                    ultimos[rotina] = (total, div)
                for rotina, (total, div) in ultimos.items():
                    print('    %s: %s comparacoes, %s divergencias' % (rotina, total, div))
            resultados.append(info)
        normais = [r for r in resultados if r]
        if args.detector and normais:
            detector(args, normais)
    ok = [r for r in resultados if r]
    if args.repetir > 1 and ok:
        iguais = len({(r['impressao'], r['partidas'], r['banco']) for r in ok}) == 1 and len(ok) == len(resultados)
        print('determinismo: %s' % ('OK, todas as execucoes iguais' if iguais else 'DIFERENTES'))
    return 0 if ok and len(ok) == len(resultados) else 1


if __name__ == '__main__':
    sys.exit(main())
