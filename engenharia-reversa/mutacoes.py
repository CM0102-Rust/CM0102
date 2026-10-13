r"""Autoteste do motor (CM_AUTOTESTE) com mutacoes propositais: cada mutacao tem de ser acusada.

Uso: python mutacoes.py <lista.py> [--sem-base] [--so-base] [--base-completa] [--casos N] [--jogo AAAA-MM-DD]
     ex.: python mutacoes.py mutacoes\leva43.py

A lista e um arquivo Python com MUTACOES = [(nome, trecho_original, trecho_mutado[, ramo]), ...]; cada
trecho original tem de aparecer uma unica vez em rust/src/motor.rs (ou no fonte de ARQUIVO = 'data.rs'
na lista; conferido antes de comecar). O `ramo` opcional e o nome de uma marca `ramo("...")` da copia
Rust: a base registra quantas vezes cada ramo rodou (linhas "ramo <nome>: N" no ddraw.log, a cada
potencia de 2) e a mutacao cujo ramo nao rodou na base e pulada (lacuna medida, sem gastar a rodada).
Primeiro roda o autoteste sem mutacao com --casos casos (a base tem de dar 0 divergencias; com
--base-completa, todos os autotestes com 2 milhoes de casos no motor, a cada 5 levas); depois, para
cada mutacao, troca o trecho, recompila, copia a DLL para jogo/ e roda so o autoteste do motor
com --casos casos (padrao 200 mil; CM_AUTOTESTE=motor:N), que para na primeira divergencia. Uma
mutacao que passa com poucos casos pode precisar de mais: refazer so ela com --casos 2000000.

--jogo AAAA-MM-DD: em vez do autoteste, cada mutacao (e a base) roda o jogo real em modo de
comparacao ate a data (simular.py, com a trava do laboratorio). Para caminhos que o autoteste
sintetico nao alcanca (o que depende do banco carregado, como a tatica do clube). No fim (inclusive com erro ou Ctrl+C) restaura o motor.rs e recompila.

Se a base divergir, para sem rodar as mutacoes (codigo de saida 1).
Uma mutacao que passa sem divergencia e uma lacuna do teste (melhorar o gerador de casos) ou uma
mutacao equivalente pelos valores possiveis (documentar em docs/ROTINAS.md por que).
O autoteste abre o jogo oculto e em janela (CM_OCULTO, CM_JANELA=960x540): um jogo por vez.
"""
import os
import shutil
import subprocess
import sys
import time

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.dirname(AQUI)
SRC = os.path.join(RAIZ, 'rust', 'src', 'motor.rs')
DLL = os.path.join(RAIZ, 'rust', 'target', 'i686-pc-windows-msvc', 'release', 'ddraw.dll')
JOGO = os.path.join(RAIZ, 'jogo')
LOG = os.path.join(JOGO, 'ddraw.log')
LIMITE = 2400  # segundos; o autoteste completo passa de 20 minutos (cresce a cada leva)


def compilar():
    os.utime(SRC)  # o cargo nao recompila se a data nao mudar (ver CLAUDE.md)
    for tentativa in range(3):
        r = subprocess.run(['cargo', 'build', '--release'], cwd=os.path.join(RAIZ, 'rust'), capture_output=True, text=True)
        if 'Finished' in r.stderr:
            break
        # falha sem mensagem (cargo interrompido ao lado de outra compilacao): tenta de novo
        if r.stderr.strip():
            raise RuntimeError(r.stderr[-2000:])
        time.sleep(10)
    else:
        raise RuntimeError('cargo falhou 3 vezes sem mensagem (codigo %s)' % r.returncode)
    shutil.copy2(DLL, JOGO)


def autoteste(valor='1'):
    if os.path.exists(LOG):
        os.remove(LOG)
    amb = dict(os.environ, CM_AUTOTESTE=valor, CM_OCULTO='1', CM_JANELA='960x540')
    p = subprocess.Popen([os.path.join(JOGO, 'cm0102-1080.exe'), '-windowed'], cwd=JOGO, env=amb)
    try:
        for _ in range(LIMITE):
            time.sleep(1)
            if p.poll() is not None:
                break
            texto = open(LOG, encoding='utf-8', errors='replace').read() if os.path.exists(LOG) else ''
            if 'motor: AUTOTESTE' in texto or 'motor: DIVERGENCIA' in texto:
                break
    finally:
        p.kill()
        p.wait()
    texto = open(LOG, encoding='utf-8', errors='replace').read() if os.path.exists(LOG) else ''
    return [l for l in texto.splitlines() if l.startswith('motor: AUTOTESTE') or 'motor: DIVERGENCIA' in l or 'EXCECAO' in l]


def jogo(data):
    # Sem o log da rodada anterior: se a simulacao for recusada (vigia de memoria), nao ha resultado
    # velho para ler como se fosse desta mutacao. Recusada por falta de memoria, espera e tenta de novo.
    import laboratorio
    for _ in range(120):
        ok, motivo = laboratorio.pode_rodar()
        if ok:
            break
        print('  aguardando memoria: %s' % motivo, flush=True)
        time.sleep(60)
    if os.path.exists(LOG):
        os.remove(LOG)
    r = subprocess.run([sys.executable, os.path.join(AQUI, 'simular.py'), '--ate', data, '--motor', 'sim', '--modo', 'comparar', '--limite', '900'],
                       cwd=AQUI, capture_output=True, text=True)
    if r.returncode != 0 and not os.path.exists(LOG):
        return ['SIMULACAO RECUSADA: ' + (r.stdout + r.stderr).strip().splitlines()[-1:][0] if (r.stdout + r.stderr).strip() else 'SIMULACAO RECUSADA']
    texto = open(LOG, encoding='utf-8', errors='replace').read() if os.path.exists(LOG) else ''
    achados = [l for l in texto.splitlines() if 'DIVERGENCIA' in l or 'EXCECAO' in l]
    return achados[:1] or [l for l in texto.splitlines() if l.startswith('motor: ') and 'comparacoes' in l][-1:]


def ramos_da_base():
    """Quantas vezes cada ramo marcado rodou na base (o maior valor registrado de cada um)."""
    texto = open(LOG, encoding='utf-8', errors='replace').read() if os.path.exists(LOG) else ''
    vistos = {}
    for l in texto.splitlines():
        if l.startswith('ramo ') and ': ' in l:
            nome, n = l[5:].rsplit(': ', 1)
            if n.strip().isdigit():
                vistos[nome] = max(vistos.get(nome, 0), int(n))
    return vistos


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    import psutil
    abertos = [q.pid for q in psutil.process_iter(['name']) if (q.info['name'] or '').lower().startswith('cm0102')]
    if abertos:
        sys.exit('ja ha um jogo aberto (pid %s): um teste por vez (outra rodada ainda rodando?)' % abertos)
    lista = {}
    exec(open(sys.argv[1], encoding='utf-8').read(), lista)
    mutacoes = lista['MUTACOES']
    # a lista pode mutar outro fonte (ex.: ARQUIVO = 'data.rs'); o padrao e o motor.rs
    global SRC
    SRC = os.path.join(RAIZ, 'rust', 'src', lista.get('ARQUIVO', 'motor.rs'))
    casos = int(sys.argv[sys.argv.index('--casos') + 1]) if '--casos' in sys.argv else 200000
    data = sys.argv[sys.argv.index('--jogo') + 1] if '--jogo' in sys.argv else None
    original = open(SRC, encoding='utf-8').read()
    ruins = [(m[0], original.count(m[1])) for m in mutacoes if original.count(m[1]) != 1]
    if ruins:
        sys.exit('trechos que nao aparecem exatamente uma vez: %s' % ruins)
    try:
        compilar()
        if '--sem-base' not in sys.argv:
            t0 = time.time()
            r = jogo(data) if data else autoteste('1' if '--base-completa' in sys.argv else 'motor:%d' % casos)
            print('sem mutacao (%.0fs):' % (time.time() - t0), r, flush=True)
            # base com divergencia: as mutacoes nao provam nada (e a cadeia que chamou para aqui)
            if any('DIVERGENCIA' in l or 'EXCECAO' in l for l in r):
                sys.exit('base com divergencia: mutacoes canceladas')
            ramos = ramos_da_base()
            if ramos:
                print('ramos na base:', ', '.join('%s %d' % x for x in sorted(ramos.items())), flush=True)
        else:
            ramos = None
        if '--so-base' in sys.argv:
            return
        for m in mutacoes:
            nome, velho, novo = m[:3]
            if len(m) > 3 and ramos is not None and ramos.get(m[3], 0) == 0:
                print('mutacao %-18s    0s ramo "%s" nao rodou na base: lacuna (pulada)' % (nome, m[3]), flush=True)
                continue
            open(SRC, 'w', encoding='utf-8').write(original.replace(velho, novo))
            compilar()
            t0 = time.time()
            r = jogo(data) if data else autoteste('motor:%d' % casos)
            print('mutacao %-18s %4.0fs %s' % (nome, time.time() - t0, r[0][:150] if r else 'SEM RESULTADO'), flush=True)
    finally:
        open(SRC, 'w', encoding='utf-8').write(original)
        compilar()
        print('restaurado e recompilado')


if __name__ == '__main__':
    main()
