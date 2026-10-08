r"""Pseudocodigo C (Ghidra) de rotinas do cm0102.exe, como apoio de leitura das rotinas grandes.

Uso: python descompilar.py 0x6bc030 [0x6c2be0 ...]     (grava e mostra referencias/ferramentas/saida/<end>.c)
     python descompilar.py --analisar                    (analise completa do exe, uma vez: ~4 min)

O Ghidra (portatil, fora do git) fica em referencias/ferramentas: jdk-21*, ghidra_*_PUBLIC e o
projeto ja analisado em proj/cm0102_completo. Cada rotina leva uns 15 s (o projeto e aberto so para
leitura). O pseudocodigo acerta a estrutura e as variaveis da pilha, mas esconde as expressoes
que entram no `__ftol` e os arredondamentos para float: as contas exatas saem da desmontagem
(desmonta.py), e a prova continua sendo a comparacao no jogo real e as mutacoes.
"""
import glob, os, subprocess, sys

import laboratorio

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.dirname(AQUI)
F = os.path.join(RAIZ, 'referencias', 'ferramentas')
EXE = os.path.join(RAIZ, 'jogo', 'cm0102.exe')
PROJETO = 'cm0102_completo'


def achar(padrao):
    r = sorted(glob.glob(os.path.join(F, padrao)))
    if not r:
        sys.exit('nao achei %s em %s (instalar o Java 21 e o Ghidra portateis ali)' % (padrao, F))
    return r[-1]


def ghidra(args):
    ok, msg = laboratorio.pode_rodar()
    if not ok:
        sys.exit('maquina pressionada, nao rodo: ' + msg)
    env = dict(os.environ, JAVA_HOME=achar('jdk-*'))
    bat = os.path.join(achar('ghidra_*_PUBLIC'), 'support', 'analyzeHeadless.bat')
    os.makedirs(os.path.join(F, 'proj'), exist_ok=True)
    log = os.path.join(F, 'saida', 'ghidra.log')
    os.makedirs(os.path.dirname(log), exist_ok=True)
    with open(log, 'w') as f:
        r = subprocess.run([bat, os.path.join(F, 'proj'), PROJETO] + args, env=env, cwd=F, stdout=f, stderr=subprocess.STDOUT)
    if r.returncode:
        sys.exit('o Ghidra falhou (ver %s)' % log)


def main():
    if sys.argv[1:] == ['--analisar']:
        with laboratorio.Trava('ghidra: analise completa'):
            ghidra(['-import', EXE, '-overwrite', '-max-cpu', '2'])
        return
    ends = [a for a in sys.argv[1:] if a.lower().startswith('0x')]
    if not ends or len(ends) != len(sys.argv[1:]):
        sys.exit(__doc__)
    saida = os.path.join(F, 'saida', '_'.join(e.lower() for e in ends) + '.c')
    ghidra(['-process', os.path.basename(EXE), '-readOnly', '-noanalysis', '-scriptPath', os.path.join(AQUI, 'ghidra'),
            '-postScript', 'Descompilar.java', saida] + ends)
    print(open(saida, encoding='utf-8').read())
    print('// gravado em', saida)


if __name__ == '__main__':
    main()
