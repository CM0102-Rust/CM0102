"""Unicorn harness: runs an original routine of cm0102.exe with stubbed calls and records the calls made.

emulate(entry, args, ecx, stubs, setup) -> list of (name, args)
stubs: {address: (name, ret_bytes, handler(uc, args) -> eax or None)}; args read lazily (8 dwords).
"""
import struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_32, UC_HOOK_CODE, UC_HOOK_MEM_UNMAPPED
from unicorn.x86_const import UC_X86_REG_ESP, UC_X86_REG_EIP, UC_X86_REG_EAX, UC_X86_REG_ECX, UC_X86_REG_EDX, UC_X86_REG_FS

EXE = r'D:\Sistemas\CM0102\jogo\cm0102.exe'
_pe = pefile.PE(EXE, fast_load=True)
_img = _pe.get_memory_mapped_image()
BASE = 0x400000
STACK, STACK_SIZE = 0x08000000, 0x100000
HEAP = 0x10000000
END = 0x0badf00d


def new_uc():
    uc = Uc(UC_ARCH_X86, UC_MODE_32)
    size = (len(_img) + 0xfff) & ~0xfff
    uc.mem_map(BASE, size)
    uc.mem_write(BASE, _img)
    uc.mem_map(STACK, STACK_SIZE)
    uc.mem_map(HEAP, 0x1000000)
    uc.mem_map(END & ~0xfff, 0x1000)
    return uc


def u32(uc, a):
    return struct.unpack('<I', uc.mem_read(a, 4))[0]


def cstr(uc, a, n=400):
    b = bytes(uc.mem_read(a, n))
    return b.split(b'\0')[0]


def emulate(uc, entry, args, ecx, stubs, limit=2_000_000):
    calls = []
    esp = STACK + STACK_SIZE - 0x1000
    frame = struct.pack('<I', END) + b''.join(struct.pack('<I', a & 0xffffffff) for a in args)
    esp -= len(frame)
    uc.mem_write(esp, frame)
    uc.reg_write(UC_X86_REG_ESP, esp)
    uc.reg_write(UC_X86_REG_ECX, ecx)

    def hook(uc, address, size, _):
        if address == END:
            uc.emu_stop()
            return
        st = stubs.get(address)
        if st is None:
            return
        name, ret, h = st
        sp = uc.reg_read(UC_X86_REG_ESP)
        ra = u32(uc, sp)
        a = [u32(uc, sp + 4 + 4 * k) for k in range(10)]
        this = uc.reg_read(UC_X86_REG_ECX)
        r = h(uc, a, this)
        calls.append((name, a, this))
        if r is not None:
            uc.reg_write(UC_X86_REG_EAX, r & 0xffffffff)
        uc.reg_write(UC_X86_REG_ESP, sp + 4 + ret)
        uc.reg_write(UC_X86_REG_EIP, ra)

    h1 = uc.hook_add(UC_HOOK_CODE, hook)
    try:
        uc.emu_start(entry, END, count=limit)
    finally:
        uc.hook_del(h1)
    return calls
