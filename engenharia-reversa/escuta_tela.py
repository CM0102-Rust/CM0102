r"""Abre o jogo e registra as reconfiguracoes de tela (WM_DISPLAYCHANGE) que ele provoca: e a medida da "piscada".

Uso: python escuta_tela.py <segundos> <exe> [argumentos]     ex.: escuta_tela.py 10 ..\jogo\cm0102-portatil.exe -windowed
"""
import ctypes, ctypes.wintypes as wt, subprocess, sys, time
u = ctypes.windll.user32; k = ctypes.windll.kernel32
u.SetProcessDPIAware()
WNDPROC = ctypes.WINFUNCTYPE(ctypes.c_ssize_t, wt.HWND, wt.UINT, wt.WPARAM, wt.LPARAM)
t0 = None
NOMES = {0x7e: 'WM_DISPLAYCHANGE', 0x1c: 'WM_ACTIVATEAPP', 0x1a: 'WM_SETTINGCHANGE', 0x31a: 'WM_THEMECHANGED',
         0x320: 'WM_DWMCOLORIZATIONCHANGED', 0x31e: 'WM_DWMCOMPOSITIONCHANGED', 0x219: 'WM_DEVICECHANGE', 0x2e0: 'WM_DPICHANGED'}
def proc(h, msg, wp, lp):
    if msg in NOMES and t0:
        extra = ' %dx%d %dbpp' % (lp & 0xffff, lp >> 16, wp) if msg == 0x7e else ''
        print('%.2fs %s%s' % (time.time() - t0, NOMES[msg], extra), flush=True)
    return u.DefWindowProcW(h, msg, wp, ctypes.c_ssize_t(lp))
cb = WNDPROC(proc)
class WNDCLASS(ctypes.Structure):
    _fields_ = [('style', wt.UINT), ('lpfnWndProc', WNDPROC), ('cbClsExtra', ctypes.c_int), ('cbWndExtra', ctypes.c_int),
                ('hInstance', wt.HINSTANCE), ('hIcon', wt.HICON), ('hCursor', wt.HANDLE), ('hbrBackground', wt.HBRUSH),
                ('lpszMenuName', wt.LPCWSTR), ('lpszClassName', wt.LPCWSTR)]
wc = WNDCLASS(lpfnWndProc=cb, hInstance=k.GetModuleHandleW(None), lpszClassName='escuta_cm')
u.RegisterClassW(ctypes.byref(wc))
u.DefWindowProcW.argtypes = [wt.HWND, wt.UINT, wt.WPARAM, ctypes.c_ssize_t]
u.DefWindowProcW.restype = ctypes.c_ssize_t
h = u.CreateWindowExW(0, 'escuta_cm', 'escuta', 0, 0, 0, 10, 10, None, None, wc.hInstance, None)  # top-level oculta
dur = float(sys.argv[1])
t0 = time.time()
p = subprocess.Popen(sys.argv[2:], cwd='C:\\')
msg = wt.MSG()
while time.time() - t0 < dur:
    while u.PeekMessageW(ctypes.byref(msg), None, 0, 0, 1):
        u.TranslateMessage(ctypes.byref(msg)); u.DispatchMessageW(ctypes.byref(msg))
    time.sleep(0.01)
p.kill()
t1 = time.time()
while time.time() - t1 < 2:
    while u.PeekMessageW(ctypes.byref(msg), None, 0, 0, 1):
        u.TranslateMessage(ctypes.byref(msg)); u.DispatchMessageW(ctypes.byref(msg))
    time.sleep(0.01)
print('fim')
