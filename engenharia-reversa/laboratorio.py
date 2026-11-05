r"""Cuidados com a maquina para testes pesados (usado pelo simular.py; pode ser usado por outras sessoes).

1. Vigia de memoria: le, pelo GetPerformanceInfo do Windows, o pool nao paginado e o paginado do
   kernel e a memoria fisica livre. Um driver que vaza a cada operacao de disco faz os pools crescerem
   ate travar a maquina; com a maquina ja pressionada, nao se comeca um teste pesado.
2. Trava compartilhada: D:\Sistemas\.laboratorio.lock. Quem segura a trava roda; os outros esperam.
   Convencao para as varias sessoes do Claude no mesmo PC: so um teste pesado por vez.

Uso direto:  python laboratorio.py          (mostra o estado da memoria e da trava)
"""
import ctypes
import ctypes.wintypes as wt
import msvcrt
import os
import sys
import time

TRAVA = r'D:\Sistemas\.laboratorio.lock'
LIMITE_POOL_NAO_PAGINADO_GB = 3.0
MINIMO_RAM_LIVRE_GB = 4.0


class _Info(ctypes.Structure):
    _fields_ = [('cb', wt.DWORD), ('CommitTotal', ctypes.c_size_t), ('CommitLimit', ctypes.c_size_t),
                ('CommitPeak', ctypes.c_size_t), ('PhysicalTotal', ctypes.c_size_t), ('PhysicalAvailable', ctypes.c_size_t),
                ('SystemCache', ctypes.c_size_t), ('KernelTotal', ctypes.c_size_t), ('KernelPaged', ctypes.c_size_t),
                ('KernelNonpaged', ctypes.c_size_t), ('PageSize', ctypes.c_size_t), ('HandleCount', wt.DWORD),
                ('ProcessCount', wt.DWORD), ('ThreadCount', wt.DWORD)]


def memoria():
    """Dicionario em GB: pool_nao_paginado, pool_paginado, ram_livre, comprometida, limite."""
    i = _Info()
    i.cb = ctypes.sizeof(i)
    ctypes.windll.psapi.GetPerformanceInfo(ctypes.byref(i), i.cb)
    gb = lambda paginas: paginas * i.PageSize / 2 ** 30
    return {'pool_nao_paginado': gb(i.KernelNonpaged), 'pool_paginado': gb(i.KernelPaged),
            'ram_livre': gb(i.PhysicalAvailable), 'comprometida': gb(i.CommitTotal), 'limite': gb(i.CommitLimit)}


def texto(m):
    return 'pool nao paginado %.2f GB, paginado %.2f GB, RAM livre %.1f GB, comprometida %.1f/%.1f GB' % (
        m['pool_nao_paginado'], m['pool_paginado'], m['ram_livre'], m['comprometida'], m['limite'])


def pode_rodar():
    """(sim/nao, motivo)."""
    m = memoria()
    if m['pool_nao_paginado'] > LIMITE_POOL_NAO_PAGINADO_GB:
        return False, 'pool nao paginado do kernel em %.2f GB (limite %.1f): provavel vazamento de driver; reinicie o PC' % (
            m['pool_nao_paginado'], LIMITE_POOL_NAO_PAGINADO_GB)
    if m['ram_livre'] < MINIMO_RAM_LIVRE_GB:
        return False, 'so %.1f GB de RAM livre (minimo %.1f)' % (m['ram_livre'], MINIMO_RAM_LIVRE_GB)
    return True, texto(m)


class Trava:
    """Trava exclusiva entre processos (e entre sessoes). Espera ate `espera` segundos."""

    def __init__(self, dono, espera=6 * 3600):  # 05/11: com agentes e bancadas juntos, 30 min de espera não bastavam
        self.dono, self.espera, self.f = dono, espera, None

    def __enter__(self):
        os.makedirs(os.path.dirname(TRAVA), exist_ok=True)
        # CM_VAGAS=N (31/10/2026, Lucas): até N testes pesados deste projeto ao mesmo tempo; a vaga 1 é a trava
        # compartilhada com as outras sessões, as outras são .laboratorio_2.lock, _3... (cada bancada na sua pasta)
        vagas = [TRAVA] + [TRAVA.replace('.lock', '_%d.lock' % k) for k in range(2, int(os.environ.get('CM_VAGAS', '1')) + 1)]
        t0, avisou = time.time(), False
        while True:
            for v in vagas:
                self.f = open(v, 'a+')
                try:
                    self.f.seek(0)
                    msvcrt.locking(self.f.fileno(), msvcrt.LK_NBLCK, 1)
                    break
                except OSError:
                    self.f.close()
            else:
                self.f = None
            if self.f:
                break
            else:
                if not avisou:
                    print('esperando a trava do laboratorio (outro teste pesado rodando)...', flush=True)
                    avisou = True
                if time.time() - t0 > self.espera:
                    raise TimeoutError('trava do laboratorio ocupada ha mais de %d s' % self.espera)
                time.sleep(5)
        self.f.seek(0)
        self.f.truncate()
        self.f.write('%s pid %d desde %s\n' % (self.dono, os.getpid(), time.strftime('%Y-%m-%d %H:%M:%S')))
        self.f.flush()
        return self

    def __exit__(self, *a):
        try:
            self.f.seek(0)
            msvcrt.locking(self.f.fileno(), msvcrt.LK_UNLCK, 1)
        finally:
            self.f.close()


if __name__ == '__main__':
    print(texto(memoria()))
    ok, motivo = pode_rodar()
    print('teste pesado:', 'pode rodar' if ok else 'NAO: ' + motivo)
    try:
        with Trava('consulta', espera=0):
            print('trava do laboratorio: livre')
    except TimeoutError:
        try:
            dono = open(TRAVA).read().strip()
        except OSError:
            dono = '(arquivo travado pelo dono; o Windows nao deixa ler)'
        print('trava do laboratorio: ocupada por', dono)
