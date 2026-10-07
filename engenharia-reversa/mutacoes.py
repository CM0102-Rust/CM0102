r"""Autoteste do motor (CM_AUTOTESTE) com mutacoes propositais: cada mutacao tem de ser acusada.

Uso: python mutacoes.py <lista.py> [--sem-base] [--so-base]
     ex.: python mutacoes.py mutacoes\leva43.py

A lista e um arquivo Python com MUTACOES = [(nome, trecho_original, trecho_mutado), ...]; cada
trecho original tem de aparecer uma unica vez em rust/src/motor.rs (conferido antes de comecar).
Primeiro roda o autoteste sem mutacao (a base tem de dar 0 divergencias); depois, para cada
mutacao, troca o trecho, recompila, copia a DLL para jogo/ e roda o autoteste, que para na
primeira divergencia. No fim (inclusive com erro ou Ctrl+C) restaura o motor.rs e recompila.

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
LIMITE = 1500  # segundos; o autoteste completo passa de 10 minutos


def compilar():
    os.utime(SRC)  # o cargo nao recompila se a data nao mudar (ver CLAUDE.md)
    r = subprocess.run(['cargo', 'build', '--release'], cwd=os.path.join(RAIZ, 'rust'), capture_output=True, text=True)
    if 'Finished' not in r.stderr:
        raise RuntimeError(r.stderr[-2000:])
    shutil.copy2(DLL, JOGO)


def autoteste():
    if os.path.exists(LOG):
        os.remove(LOG)
    amb = dict(os.environ, CM_AUTOTESTE='1', CM_OCULTO='1', CM_JANELA='960x540')
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


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    lista = {}
    exec(open(sys.argv[1], encoding='utf-8').read(), lista)
    mutacoes = lista['MUTACOES']
    original = open(SRC, encoding='utf-8').read()
    ruins = [(n, original.count(a)) for n, a, _ in mutacoes if original.count(a) != 1]
    if ruins:
        sys.exit('trechos que nao aparecem exatamente uma vez: %s' % ruins)
    try:
        compilar()
        if '--sem-base' not in sys.argv:
            print('sem mutacao:', autoteste(), flush=True)
        if '--so-base' in sys.argv:
            return
        for nome, velho, novo in mutacoes:
            open(SRC, 'w', encoding='utf-8').write(original.replace(velho, novo))
            compilar()
            r = autoteste()
            print('mutacao %-18s %s' % (nome, r[0][:150] if r else 'SEM RESULTADO'), flush=True)
    finally:
        open(SRC, 'w', encoding='utf-8').write(original)
        compilar()
        print('restaurado e recompilado')


if __name__ == '__main__':
    main()
