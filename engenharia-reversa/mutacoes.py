r"""Autoteste do motor (CM_AUTOTESTE) com mutacoes propositais: cada mutacao tem de ser acusada.

Uso: python mutacoes.py <lista.py> [--sem-base] [--so-base] [--base-completa] [--casos N] [--jogo AAAA-MM-DD]
                         [--ligas "ENG ESP ..."] [--limite SEGUNDOS]
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
comparacao ate a data (simular.py, com a trava do laboratorio; a mutacao para na primeira
divergencia, a base vai ate o fim). Para caminhos que o autoteste
sintetico nao alcanca (o que depende do banco carregado, como a tatica do clube).

--carreira AAAA-MM-DD: cada mutacao (e a base) roda a carreira automatica em modo Rust (sem
comparacao, ~15x mais rapido) ate a data; a mutacao e acusada se a impressao ou a contagem de
sorteios muda em relacao a base (ou se o jogo cai). A base tem de bater com a do original na mesma
data (conferido pela cadeia). A impressao inclui a das partidas (hash de cada motor de partida ao
ser liberado) e a do banco (hash diario dos blocos do banco de dados, ver simulacao.rs): pega o que nao
chega aos sorteios. Nao diz onde divergiu: para isso, o modo --jogo.
--confirmar AAAA-MM-DD (com --carreira): a mutacao que escapar da carreira e refeita no jogo em
modo de comparacao ate a data (esquema hibrido: triagem rapida, confirmacao so do que escapou). No fim (inclusive com erro ou Ctrl+C) restaura o motor.rs e recompila.

--prova-save N: para as levas do bloco do save (a carreira nao passa por elas). A base sao duas gravacoes
do .qst em modo rust com as rotinas da leva N desligadas (prova_save.base_da_leva: os bytes que variam
entre elas viram mascara); sem mutacao, a gravacao com a leva tem de dar IGUAL fora da mascara. Cada
mutacao grava 1 .qst: ACUSADA se diferir fora da mascara ou se o jogo cair/estourar o tempo (sem .qst),
IGUAL A BASE (escapou) se igual. Leva de carregar (levas/N/CARGA): a base e cada mutacao tambem carregam o
save de ferias (prova_save: --sav, --ate); com a gravacao igual, ACUSADA se a impressao da carga mudar ou se
o jogo cair/estourar o tempo na carga.

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
JOGO = os.path.join(RAIZ, 'jogo_testes' if os.path.isdir(os.path.join(RAIZ, 'jogo_testes')) else 'jogo')  # ver simular.py
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
    return [l for l in texto.splitlines() if l.startswith('banco v1') or l.startswith('motor: AUTOTESTE') or 'motor: DIVERGENCIA' in l or 'EXCECAO' in l]


# --ligas "ENG ESP ...": a carreira ampla (de madrugada); --limite: segundos por execucao (padrao 900)
LIGAS = ['--ligas', sys.argv[sys.argv.index('--ligas') + 1]] if '--ligas' in sys.argv else []
LIMITE_CARREIRA = sys.argv[sys.argv.index('--limite') + 1] if '--limite' in sys.argv else '900'
PARAR_NA_DIVERGENCIA = False  # ligado depois da base: a mutacao para na primeira divergencia


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
    # a mutacao ja e acusada na primeira divergencia: a rodada para ali (a base vai ate o fim)
    extra = ['--parar-na-divergencia'] if PARAR_NA_DIVERGENCIA else []
    r = subprocess.run([sys.executable, os.path.join(AQUI, 'simular.py'), '--ate', data, '--motor', 'sim', '--modo', 'comparar', '--limite', LIMITE_CARREIRA] + LIGAS + extra,
                       cwd=AQUI, capture_output=True, text=True)
    if r.returncode != 0 and not os.path.exists(LOG):
        return ['SIMULACAO RECUSADA: ' + (r.stdout + r.stderr).strip().splitlines()[-1:][0] if (r.stdout + r.stderr).strip() else 'SIMULACAO RECUSADA']
    texto = open(LOG, encoding='utf-8', errors='replace').read() if os.path.exists(LOG) else ''
    achados = [l for l in texto.splitlines() if 'DIVERGENCIA' in l or 'EXCECAO' in l]
    return achados[:1] or [l for l in texto.splitlines() if l.startswith('motor: ') and 'comparacoes' in l][-1:]


def carreira(data):
    # a impressao e os sorteios ao fim da carreira Rust (ou a queda); ver --carreira
    import laboratorio
    for _ in range(120):
        ok, motivo = laboratorio.pode_rodar()
        if ok:
            break
        print('  aguardando memoria: %s' % motivo, flush=True)
        time.sleep(60)
    if os.path.exists(LOG):
        os.remove(LOG)
    r = subprocess.run([sys.executable, os.path.join(AQUI, 'simular.py'), '--ate', data, '--motor', 'sim', '--modo', 'rust', '--limite', LIMITE_CARREIRA] + LIGAS,
                       cwd=AQUI, capture_output=True, text=True)
    texto = open(LOG, encoding='utf-8', errors='replace').read() if os.path.exists(LOG) else ''
    quedas = [l for l in texto.splitlines() if 'EXCECAO' in l]
    fim = [l.split(': ', 1)[1].rsplit(', ', 1)[0] for l in r.stdout.splitlines() if l.startswith('execucao 1:')]
    return quedas[:1] or fim or ['SEM RESULTADO: ' + ' '.join((r.stdout + r.stderr).strip().splitlines()[-2:])]


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


def jogo_aberto():
    """Os pids dos jogos deste projeto abertos (jogo/ ou jogo_testes/). So os deste projeto: outros projetos da
    maquina (ex.: o CMRTE em D:/Sistemas) tem exe de teste com o mesmo nome; a carga deles fica a cargo da trava
    do laboratorio."""
    import psutil
    raiz = os.path.normcase(RAIZ)
    return [q.pid for q in psutil.process_iter(['name', 'exe'])
            if (q.info['name'] or '').lower().startswith('cm0102') and os.path.normcase(q.info['exe'] or '').startswith(raiz)]


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    abertos = jogo_aberto()
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
    fim = sys.argv[sys.argv.index('--carreira') + 1] if '--carreira' in sys.argv else None
    rodar = (lambda: carreira(fim)) if fim else (lambda: jogo(data)) if data else None
    if '--autoteste-banco' in sys.argv:  # só os autotestes do banco (CM_AUTOTESTE=banco, 232a)
        rodar = lambda: autoteste('banco')
    prova = sys.argv[sys.argv.index('--prova-save') + 1] if '--prova-save' in sys.argv else None
    if prova:
        import prova_save
        opcoes = prova_save.padrao()
        base_save = prova_save.base_da_leva(opcoes, prova)  # mexe no motor.rs e o devolve antes de seguir
        if base_save is None:
            sys.exit('base da prova do save sem .qst: mutacoes canceladas')
        rodar = lambda: [prova_save.contra_a_base(opcoes, base_save, 'mutacao')[1]]
    base = None
    confirmar = sys.argv[sys.argv.index('--confirmar') + 1] if '--confirmar' in sys.argv else None
    original = open(SRC, encoding='utf-8').read()
    ruins = [(m[0], original.count(m[1])) for m in mutacoes if original.count(m[1]) != 1]
    if ruins:
        sys.exit('trechos que nao aparecem exatamente uma vez: %s' % ruins)
    try:
        compilar()
        if '--sem-base' not in sys.argv:
            t0 = time.time()
            r = rodar() if rodar else autoteste('1' if '--base-completa' in sys.argv else 'motor:%d' % casos)
            print('sem mutacao (%.0fs):' % (time.time() - t0), r, flush=True)
            if prova and not r[0].startswith('IGUAL'):
                sys.exit('base da prova do save diferente: mutacoes canceladas')
            if fim and not r[0].startswith('parou em'):
                sys.exit('base sem resultado: mutacoes canceladas')
            base = r[0]
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
        global PARAR_NA_DIVERGENCIA
        PARAR_NA_DIVERGENCIA = True
        for m in mutacoes:
            nome, velho, novo = m[:3]
            if len(m) > 3 and ramos is not None and ramos.get(m[3], 0) == 0:
                print('mutacao %-18s    0s ramo "%s" nao rodou na base: lacuna (pulada)' % (nome, m[3]), flush=True)
                continue
            open(SRC, 'w', encoding='utf-8').write(original.replace(velho, novo))
            compilar()
            t0 = time.time()
            r = rodar() if rodar else autoteste('motor:%d' % casos)
            if prova:
                r = [('IGUAL A BASE (escapou): ' if r[0].startswith('IGUAL') else 'ACUSADA: ') + r[0]]
            if fim and base:
                r = [('IGUAL A BASE (escapou): ' if r[0] == base else 'ACUSADA: ') + r[0]]
                if confirmar and r[0].startswith('IGUAL'):
                    print('mutacao %-18s %4.0fs %s' % (nome, time.time() - t0, r[0][:150]), flush=True)
                    t0 = time.time()
                    r = ['comparando: ' + (jogo(confirmar) or ['SEM RESULTADO'])[0]]
            print('mutacao %-18s %4.0fs %s' % (nome, time.time() - t0, r[0][:150] if r else 'SEM RESULTADO'), flush=True)
    finally:
        open(SRC, 'w', encoding='utf-8').write(original)
        compilar()
        print('restaurado e recompilado')


if __name__ == '__main__':
    main()
