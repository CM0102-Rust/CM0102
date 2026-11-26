"""rxdb_1: the whole save_database (0x52dad0) against the Rust copy (test export rxdb_save) on a random fake world:
the call traces (arguments, strings and the bytes each write sends) and the memory of the world must match.

python check_routine.py DLL MAP (emu.py from levas/fila/gra_2c; the DLL built by apply_test_build.py with /MAP) [N] [first]
"""
import random
import re
import struct
import sys
import os
import pefile
sys.path.insert(0, r'D:\Sistemas\CM0102\levas\fila\gra_2c')
from emu import new_uc, u32, HEAP, STACK, STACK_SIZE, END
from unicorn import UC_HOOK_CODE, UcError
from unicorn.x86_const import UC_X86_REG_ESP, UC_X86_REG_EIP, UC_X86_REG_EAX, UC_X86_REG_ECX

DLL, MAP = sys.argv[1], sys.argv[2]
N = int(sys.argv[3]) if len(sys.argv) > 3 else 500
FIRST = int(sys.argv[4]) if len(sys.argv) > 4 else 0
MAPTXT = open(MAP, errors='replace').read()


def sym(pat):
    return int(re.search(pat + r'\s+([0-9a-f]{8}) f', MAPTXT).group(1), 16)


RUST = sym(r'_rxdb_save')
RUST_RESOLVE = sym(r'5motor8database24resolve_database_indexes')
BRANCH = sym(r'10comparison6branch')
pe = pefile.PE(DLL, fast_load=True)
dll_img = pe.get_memory_mapped_image()
DLL_BASE = pe.OPTIONAL_HEADER.ImageBase

W = HEAP
TARGETS = W + 0x1000      # 64 target records of 0x40 (ids at +0 and +0x33)
TABLES = W + 0x10000      # the tables
STRS = W + 0xf0000
WORLD_END = W + 0x100000

# (global table, global count or None, record size)
TABLE_GLOBALS = {
    0xae23a4: (0xae234c, 0xc6), 0xae23a8: (0xae2350, 0x122), 0xae23ac: (0xae2354, 0x38), 0xae23b0: (0xae2358, 0x4e),
    0xae23b4: (0xae235c, 0x245), 0xae23bc: (0xae2364, 0x6e), 0xae23c0: (0xae2368, 0x44), 0xae23c4: (0xae236c, 0x46),
    0xae23c8: (0xae2370, 0x34), 0xae23cc: (0xae2374, 0x11), 0xae23d0: (0xae2378, 0x6b), 0xae23d4: (0xae237c, 0x1a),
    0xae23e0: (0xae2388, 0x65), 0xae23e4: (0xae238c, 0x3a), 0xae23e8: (0xae2390, 0x2b), 0xae23ec: (0xae2394, 0x3a),
    0xae23f0: (0xae2398, 0x3c), 0xae23f4: (0xae239c, 0x3c), 0xae23f8: (0xae23a0, 0x3c),
}
# pointer fields per table (offsets, as listed by the pseudocode of the original)
PTRS = {
    0xae23d0: [0x59, 0x5d, 0x61, 0x65],
    0xae23b4: [0x53, 0x57, 0x5b, 0x60, 0x69, 0x6e] + list(range(0x83, 0xbf, 4)) + [0xbf, 0xc3, 0xc7, 0xcb, 0xcf, 0xd3]
              + [0xd7 + 4 * k for k in range(50)] + [0x19f + 4 * k for k in range(15)] + [0x1cf + 4 * k for k in range(3)]
              + [0x1e0 + 4 * k for k in range(20)],
    0xae23a8: [0x71, 0x7a, 0x80, 0x90, 0x94, 0x98, 0x9c, 0xa0, 0xa4, 0x110, 0x114, 0x118],
    0xae23e8: [4, 8, 0x16, 0x1a], 0xae23b0: [0x38, 0x48], 0xae23c8: list(range(4, 0x34, 4)),
    0xae23c0: list(range(0x23, 0x43, 4)), 0xae23bc: [4, 8, 0xc, 0x1a, 0x1e, 0x24, 0x39, 0x61, 0x65, 0x69],
    0xae23e0: [0x53, 0x57, 0x5b, 0x5f], 0xae23cc: [4, 0xa], 0xae23ac: [0x1f], 0xae23d4: [4, 0xa, 0xe, 0x12, 0x16],
}


def w32(uc, a, v):
    uc.mem_write(a, struct.pack('<I', v & 0xffffffff))


def cstr(uc, a, n=300):
    try:
        return bytes(uc.mem_read(a, n)).split(b'\0')[0]
    except UcError:
        return 'BAD'


def make_uc():
    uc = new_uc()
    uc.mem_map(DLL_BASE, (len(dll_img) + 0xfff) & ~0xfff)
    uc.mem_write(DLL_BASE, dll_img)
    return uc


def world(uc, rnd):
    uc.mem_write(W, bytes(rnd.getrandbits(8) for _ in range(0x10000)))
    uc.mem_write(TABLES, b'\0' * (WORLD_END - TABLES))
    targets = [TARGETS + 0x40 * k for k in range(64)]
    cur = TABLES
    small = rnd.random() < 0.7
    for g, (cnt, size) in TABLE_GLOBALS.items():
        n = rnd.choice([0, 1, 2, 3, 4]) if small else rnd.randrange(0, 9)
        if rnd.random() < 0.06:
            w32(uc, g, 0)
            n = 0
        else:
            w32(uc, g, cur)
        w32(uc, cnt, n)
        uc.mem_write(cur, bytes(rnd.getrandbits(8) for _ in range(n * size)))
        for i in range(n):
            for o in PTRS.get(g, []):
                w32(uc, cur + i * size + o, 0 if rnd.random() < 0.3 else rnd.choice(targets))
        cur += n * size + 0x40
    # nat clubs inside the club table, nation comps inside the club comps, nation comp history inside the history
    for g, base, cg in ((0xae23b8, 0xae23b4, 0xae2360), (0xae23d8, 0xae23d0, 0xae2380), (0xae23dc, 0xae23d4, 0xae2384)):
        w32(uc, g, 0 if rnd.random() < 0.06 else u32(uc, base) + 0x20)
        w32(uc, cg, rnd.randrange(0, 3))
    w32(uc, 0xae2344, rnd.choice([0, 0, 1, 2, -1]))
    w32(uc, 0xae2348, rnd.getrandbits(32))
    w32(uc, 0xae2340, rnd.random() < 0.7)
    w32(uc, 0x9b5984, rnd.getrandbits(32))
    if rnd.random() < 0.5:
        w32(uc, 0xae23fc, 0)
    else:
        w32(uc, 0xae23fc, cur)
        uc.mem_write(cur, bytes(rnd.getrandbits(8) for _ in range(0x245 * 4)))
    # the loaded index.dat: 22 entries
    ids = list(range(0x17))
    rnd.shuffle(ids)
    ids = ids[:22]
    for k in range(22):
        if rnd.random() < 0.08:
            ids[k] = rnd.choice([0x30, ids[(k + 1) % 22]])
    staffname = b'staff.dat'
    for k in range(22):
        e = 0xae1a80 + k * 0x43
        name = rnd.choice([staffname, b'file%d.dat' % rnd.randrange(99)]) if ids[k] in (8, 9, 10, 0x16) else b'f%d_%d.dat' % (k, ids[k])
        if ids[k] == 6:
            name = staffname
        body = bytes(rnd.getrandbits(8) for _ in range(0x43))
        uc.mem_write(e, body)
        uc.mem_write(e, name + b'\0')
        w32(uc, e + 0x33, ids[k])
    uc.mem_write(STRS, b'C:\\TEMP\\cm3\\\0')
    w32(uc, 0xb67a34, 0x55)
    return 6 in ids


def run(uc, entry, args, rnd, resolve):
    calls = []

    def rec(*x):
        calls.append(x)

    def content(a, size, n):
        t = (size * n) & 0xffffffff
        if t > 0x40000:
            return ('BIG', size, n)
        try:
            return bytes(uc.mem_read(a, t))
        except UcError:
            return ('BAD', size, n)

    fail = rnd.random() < 0.35

    def ok():
        return not fail or rnd.random() > 0.08

    def h_langlib(a, this):
        uc.mem_write(a[0], b'Saving database\0')
        rec('langlib', a[1])

    def h_progress(a, this):
        rec('progress', this, cstr(uc, a[0]), a[1], a[2], a[3])

    def h_zopen(a, this):
        rec('zopen', a[0], cstr(uc, a[1]), a[2], a[3], a[4])
        return 0x7700 + len(calls) if ok() else 0

    def h_makepath(a, this):
        p = cstr(uc, a[2]) + b'\\' + cstr(uc, a[3])
        uc.mem_write(a[0], p + b'\0')
        rec('makepath', a[0], a[1], a[2], cstr(uc, a[3]), a[4])

    def h_fopen(a, this):
        rec('fopen', cstr(uc, a[0]), cstr(uc, a[1]))
        return 0x8800 + len(calls) if ok() else 0

    def h_write(name):
        def h(a, this):
            rec(name, content(a[0], a[1], a[2]), a[1], a[2], a[3])
            return a[2] if ok() else 0
        return h

    def h_zclose(a, this):
        rec('zclose', u32(uc, a[0]))

    def h_fclose(a, this):
        rec('fclose', a[0])

    def h_tell(name):
        def h(a, this):
            rec(name, a[0])
            return rnd.getrandbits(16)
        return h

    def h_sprintf(a, this):
        f = cstr(uc, a[1])
        rec('sprintf', a[0], f, cstr(uc, a[2]) if b'valid' in f else None)

    def h_split(a, this):
        rec('splitpath', a[0], a[1], a[2])

    def h_version(a, this):
        rec('version', a[2])
        return STRS

    def h_box(a, this):
        rec('box', a[0], a[1], a[2])

    def h_resolve(a, this):
        rec('resolve', a[0])

    def h_build(a, this):
        rec('build_path', cstr(uc, a[0]), cstr(uc, a[1]))
        return STRS

    def h_zadd(a, this):
        rec('zadd', a[0], cstr(uc, a[1]), cstr(uc, a[2]), a[3], a[4])

    stubs = {
        0x66f4e0: (0, h_langlib), 0x802f80: (0x10, h_progress), 0x9326e0: (0, h_zopen), 0x945b13: (0, h_makepath),
        0x945b00: (0, h_fopen), 0x932e10: (0, h_write('zwrite')), 0x945ce6: (0, h_write('fwrite')),
        0x932b00: (0, h_zclose), 0x945a09: (0, h_fclose), 0x933170: (0, h_tell('ztell')), 0x946aae: (0, h_tell('ftell')),
        0x944c9f: (0, h_sprintf), 0x944cff: (0, h_split), 0x90d130: (0, h_version), 0x5e8290: (0, h_box),
        resolve: (0, h_resolve), 0x90bde0: (0, h_build), 0x933610: (0, h_zadd),
    }

    def hook(uc, address, size, _):
        if address == END:
            uc.emu_stop()
            return
        if 0x52dad0 <= address < 0x5315a0:
            COVER.add(address)
        if address == BRANCH:
            sp = uc.reg_read(UC_X86_REG_ESP)
            uc.reg_write(UC_X86_REG_ESP, sp + 4)
            uc.reg_write(UC_X86_REG_EIP, u32(uc, sp))
            return
        if address == 0x52dad0 and entry != 0x52dad0:
            rec('ORIGINAL ENTERED')
        st = stubs.get(address)
        if st is None:
            return
        ret, h = st
        sp = uc.reg_read(UC_X86_REG_ESP)
        ra = u32(uc, sp)
        a = [u32(uc, sp + 4 + 4 * k) for k in range(8)]
        r = h(a, uc.reg_read(UC_X86_REG_ECX))
        if r is not None:
            uc.reg_write(UC_X86_REG_EAX, r & 0xffffffff)
        uc.reg_write(UC_X86_REG_ESP, sp + 4 + ret)
        uc.reg_write(UC_X86_REG_EIP, ra)

    esp = STACK + STACK_SIZE - 0x3000
    uc.mem_write(STACK, b'\0' * STACK_SIZE)
    esp -= 4 + 4 * len(args)
    uc.mem_write(esp, struct.pack('<I', END) + b''.join(struct.pack('<I', x & 0xffffffff) for x in args))
    uc.reg_write(UC_X86_REG_ESP, esp)
    h = uc.hook_add(UC_HOOK_CODE, hook)
    try:
        uc.emu_start(entry, END, count=5_000_000)
    except UcError as ex:
        rec('EXC', str(ex)); os.environ.get('DBG') and print('EXC at %x' % uc.reg_read(UC_X86_REG_EIP))
    finally:
        uc.hook_del(h)
    rec('RET', uc.reg_read(UC_X86_REG_EAX) if uc.reg_read(UC_X86_REG_EIP) == END else 'eip %x' % uc.reg_read(UC_X86_REG_EIP))
    return calls


def snapshot(uc):
    g = bytes(uc.mem_read(0xae1a80, 0x980)) + bytes(uc.mem_read(0xb67a34, 4)) + bytes(uc.mem_read(0xde1f64, 0x100))
    return bytes(uc.mem_read(W, WORLD_END - W)) + g


COVER = set()
uc = make_uc()
bad = 0
defect = 0
double = 0
seen = {}
for i in range(FIRST, FIRST + N):
    if i % 50 == 0:
        uc = make_uc()
    res = []
    for side in ('o', 'r'):
        wr = random.Random(i)
        staff_present = world(uc, wr)
        args = [0x9ab000 if wr.random() < 0.5 else STRS, wr.random() < 0.6, wr.getrandbits(8), wr.getrandbits(8)]
        rnd = random.Random(i * 7 + 1)
        calls = run(uc, 0x52dad0 if side == 'o' else RUST, args, rnd, 0x5315a0 if side == 'o' else RUST_RESOLVE)
        res.append((calls, snapshot(uc)))
    for c in res[0][0]:
        seen[c[0]] = seen.get(c[0], 0) + 1
    if res[0] != res[1] and not staff_present and any(c[0] == 'EXC' for c in res[0][0]):
        defect += 1
        continue
    oc = res[0][0]
    dbl = [k for k in range(1, len(oc)) if oc[k][0] in ('zclose', 'fclose') and oc[k] == oc[k - 1]]
    if res[0] != res[1] and dbl and oc[:dbl[0]] + oc[dbl[0] + 1:] == res[1][0] and res[0][1] == res[1][1]:
        double += 1
        continue
    if res[0] != res[1]:
        bad += 1
        if bad <= 4:
            print('DIFF case', i)
            a, b = res[0][0], res[1][0]
            for k in range(max(len(a), len(b))):
                x = a[k] if k < len(a) else None
                y = b[k] if k < len(b) else None
                if x != y:
                    print('  call', k, '\n   o', str(x)[:300], '\n   r', str(y)[:300])
                    print('   before:', [str(c)[:80] for c in a[max(0, k - 3):k]])
                    break
            if res[0][1] != res[1][1]:
                ma, mb = res[0][1], res[1][1]
                d = [k for k in range(len(ma)) if ma[k] != mb[k]]
                print('   mem differs at', [hex(W + k) if k < WORLD_END - W else 'g+%x' % (k - (WORLD_END - W)) for k in d[:12]])
print('cases %d differences %d (staff entry missing, the original reads out of its frame: %d; index.dat entries failed, the original closes twice: %d)' % (N, bad, defect, double))
allins = [int(l.split()[0], 16) for l in open(r'D:\Sistemas\CM0102\pacotes\0x52dad0\desmontagem.txt') if l[:1].isdigit()]
miss = [a for a in allins if a not in COVER]
print('original instructions covered %d of %d' % (len(allins) - len(miss), len(allins)))
open('miss.txt', 'w').write(chr(10).join(hex(a) for a in miss))
print('calls seen:', sorted(seen.items(), key=lambda x: -x[1]))
