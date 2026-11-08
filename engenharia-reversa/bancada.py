r"""Bancadas paralelas (31/10/2026, Lucas: "pode paralelizar os testes"): cópias do projeto em copias\bancadaK com o
código, as ferramentas, as levas e um jogo_testes próprio, para rodar a cadeia de uma leva ao lado da principal.
A trava do laboratório dá uma vaga por bancada (CM_VAGAS, laboratorio.py); a vigia de memória vale para todas.

    python bancada.py sincronizar K           copia rust\src, Cargo.*, engenharia-reversa, levas e (na 1a vez) o jogo_testes
    python bancada.py cadeia K N [DATA]       sincroniza, integra a leva N lá se preciso, compila e roda a cadeia;
                                              o cadeia.txt volta para levas\N\ da principal

A bancada não é repositório git: publicar, integrar de vez e documentar continuam na principal. A impressão das
carreiras depende da pasta (o caminho entra na heap): só vale comparar dentro da mesma bancada.
"""
import os, shutil, subprocess, sys

RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
# o bash do Git (o "bash" do PATH pode ser o do WSL, que não entende D:/)
BASH = next((b for b in (r'C:\Program Files\Git\bin\bash.exe', shutil.which('bash')) if b and os.path.exists(b)), 'bash')


def pasta(k):
    return os.path.join(RAIZ, 'copias', 'bancada%d' % k)


def espelhar(origem, destino, *excluir):
    # robocopy /MIR: só copia o que mudou; códigos < 8 são sucesso
    r = subprocess.run(['robocopy', origem, destino, '/MIR', '/NFL', '/NDL', '/NJH', '/NJS', '/NP', '/XD', *(excluir or ['__nada__'])],
                       capture_output=True, text=True)
    if r.returncode >= 8:
        sys.exit('robocopy falhou (%d): %s -> %s\n%s' % (r.returncode, origem, destino, r.stdout[-500:]))


def sincronizar(k):
    b = pasta(k)
    os.makedirs(os.path.join(b, 'rust'), exist_ok=True)
    espelhar(os.path.join(RAIZ, 'rust', 'src'), os.path.join(b, 'rust', 'src'))
    for f in os.listdir(os.path.join(RAIZ, 'rust')):
        if os.path.isfile(os.path.join(RAIZ, 'rust', f)):
            shutil.copy2(os.path.join(RAIZ, 'rust', f), os.path.join(b, 'rust', f))
    if os.path.isdir(os.path.join(RAIZ, 'rust', '.cargo')):
        espelhar(os.path.join(RAIZ, 'rust', '.cargo'), os.path.join(b, 'rust', '.cargo'))
    espelhar(os.path.join(RAIZ, 'cm0102-base'), os.path.join(b, 'cm0102-base'), 'target')  # a base comum (rust/ depende dela)
    espelhar(os.path.join(RAIZ, 'engenharia-reversa'), os.path.join(b, 'engenharia-reversa'), '__pycache__')
    espelhar(os.path.join(RAIZ, 'levas'), os.path.join(b, 'levas'))
    # o save de férias (só lido) que as mutações carregam para confirmar no modo comparação (prova_save.JOGO_LUCAS)
    os.makedirs(os.path.join(b, 'jogo'), exist_ok=True)
    shutil.copy2(os.path.join(RAIZ, 'jogo', 'FeriasTesteSave.sav'), os.path.join(b, 'jogo'))
    if not os.path.isdir(os.path.join(b, 'jogo_testes')):
        # a cópia limpa do jogo de testes, sem os arquivos de trabalho (game.cfg conferido pelo simular.py)
        espelhar(os.path.join(RAIZ, 'jogo_testes'), os.path.join(b, 'jogo_testes'))
    print('bancada %d sincronizada: %s' % (k, b), flush=True)


def cadeia(k, n, data=None):
    b = pasta(k)
    sincronizar(k)
    # as levas ainda não integradas na principal (até a N, em ordem): integra só na bancada (a primeira rotina de cada
    # uma já no motor = integrada); assim a N+1 roda antes de a N ser publicada
    import re
    for lv in range(n - 5, n + 1):  # lv, não k: k é a bancada
        p = os.path.join(b, 'levas', str(lv), 'integrar.py')
        if not os.path.exists(p):
            continue
        src = open(os.path.join(b, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
        primeira = re.search(r"\('\w+', (\d+), \[", open(p, encoding='utf-8').read())
        if primeira and ('address: %#x,' % int(primeira.group(1))) not in src:
            print('integrando a %d' % lv, flush=True)
            subprocess.run([sys.executable, p], check=True)
    r = subprocess.run(['cargo', 'build', '--release'], cwd=os.path.join(b, 'rust'), capture_output=True, text=True)
    if r.returncode:
        sys.exit('cargo falhou na bancada %d:\n%s' % (k, r.stderr[-1500:]))
    shutil.copy2(os.path.join(b, 'rust', 'target', 'i686-pc-windows-msvc', 'release', 'ddraw.dll'), os.path.join(b, 'jogo_testes'))
    print('cadeia da %d rodando: acompanhe %s' % (n, os.path.join(b, 'levas', str(n), 'cadeia.txt')), flush=True)
    amb = dict(os.environ, CM_VAGAS=os.environ.get('CM_VAGAS', '2'))
    subprocess.run([BASH, os.path.join(b, 'engenharia-reversa', 'cadeia.sh').replace('\\', '/'), str(n)] + ([data] if data else []), env=amb)
    shutil.copy2(os.path.join(b, 'levas', str(n), 'cadeia.txt'), os.path.join(RAIZ, 'levas', str(n), 'cadeia.txt'))
    print('cadeia da %d na bancada %d: levas\\%d\\cadeia.txt' % (n, k, n))


if __name__ == '__main__':
    a = sys.argv[1:]
    if a[:1] == ['sincronizar']:
        sincronizar(int(a[1]))
    elif a[:1] == ['cadeia']:
        cadeia(int(a[1]), int(a[2]), a[3] if len(a) > 3 else None)
    else:
        sys.exit(__doc__)
