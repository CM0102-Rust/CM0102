"""gra_2b: the contracted player's day of game_daily_staff_update (0x5ce4b6..0x5d352d) against the Rust copy
(staff_day_player through the scratch export gra2b_player_day) on a stubbed random world: the call traces (with
their arguments) and the memory written must match.

python check_player.py DLL MAP [N] [first]
"""
import random
import re
import struct
import sys
import pefile
sys.path.insert(0, __import__('os').path.dirname(__import__('os').path.abspath(__file__)))
import emu
from emu import new_uc, u32, HEAP, STACK, STACK_SIZE, END
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import (UC_X86_REG_ESP, UC_X86_REG_EIP, UC_X86_REG_EAX, UC_X86_REG_ECX, UC_X86_REG_EBP,
                               UC_X86_REG_ESI, UC_X86_REG_EDI, UC_X86_REG_EBX, UC_X86_REG_EDX, UC_X86_REG_FPCW)

DLL, MAP = sys.argv[1], sys.argv[2]
N = int(sys.argv[3]) if len(sys.argv) > 3 else 300
FIRST = int(sys.argv[4]) if len(sys.argv) > 4 else 0
MAPTXT = open(MAP, errors='replace').read()
BRANCH = int(re.search(r'10comparison6branch (\w+)', MAPTXT).group(1), 16)
RUST = int(re.search(r'_gra2b_player_day\s+(\w+)', MAPTXT).group(1), 16)
pe = pefile.PE(DLL, fast_load=True)
dll_img = pe.get_memory_mapped_image()
DLL_BASE = pe.OPTIONAL_HEADER.ImageBase

W = HEAP
P, MATE = W + 0x1000, W + 0x1100
OTHER = W + 0x1200          # 8 more people, 0x80 apart
PD = W + 0x2000
CONTRACT = W + 0x3000
OCONTRACT = W + 0x3100      # 6 other contracts, 0x60 apart
STAFFNP = W + 0x5000        # chairman/manager/assistant/coaches: 8 people, 0x80 apart
NP = W + 0x6000             # their non-player sheets, 0x40 apart
SLOT70 = W + 0x7000
CT = W + 0x10000            # 10 clubs
NAT = W + 0x20000           # 4 nations, 0x130 apart
CONT = W + 0x21000          # 3 continents
DIVS = W + 0x22000          # 4 divisions / comps, 0x80 apart
ST = W + 0x40000            # staff state table
CST = W + 0x50000           # club state table
COT = W + 0x51000           # comp object table
COBJ = W + 0x52000
TM = W + 0x53000
PSTATS = W + 0x54000
MGR = W + 0x55000           # dummy managers
OUT = W + 0x60000           # stub records (injuries, finance, entries...)
CAVE = W + 0x70000
WORLD_END = W + 0x71000
NCLUB = 10


def w32(uc, a, v):
    uc.mem_write(a, struct.pack('<I', v & 0xffffffff))


def w16(uc, a, v):
    uc.mem_write(a, struct.pack('<H', v & 0xffff))


def w8(uc, a, v):
    uc.mem_write(a, bytes([v & 0xff]))


def wf64(uc, a, v):
    uc.mem_write(a, struct.pack('<d', v))


def make_uc():
    uc = new_uc()
    uc.mem_map(DLL_BASE, (len(dll_img) + 0xfff) & ~0xfff)
    uc.mem_write(DLL_BASE, dll_img)
    uc.mem_map(0, 0x1000)
    return uc


def club(k):
    return CT + k * 0x245


def staff_rec(uc, a, rnd, sid):
    uc.mem_write(a, bytes(rnd.randrange(256) for _ in range(0x6e)))
    w32(uc, a, sid)
    w8(uc, a + 0x18, rnd.randrange(15, 40))           # age
    w32(uc, a + 0x1a, NAT + 0x130 * rnd.randrange(4))  # nation
    w32(uc, a + 0x1e, NAT + 0x130 * rnd.randrange(4) if rnd.random() < 0.3 else 0)
    w8(uc, a + 0x22, rnd.choice([0, 0, 0, 3, 12, 40]))  # caps
    w32(uc, a + 0x24, club(rnd.randrange(NCLUB)) if rnd.random() < 0.3 else 0)  # national job
    w8(uc, a + 0x28, rnd.choice([11, 11, 3]))
    w32(uc, a + 0x39, club(rnd.randrange(NCLUB)) if rnd.random() < 0.9 else 0)
    w8(uc, a + 0x3d, rnd.choice([11, 11, 11, 5, 12, 8, 3]))
    w16(uc, a + 0x3e, rnd.randrange(365))
    w16(uc, a + 0x40, rnd.choice([1995, 1999, 2000, 2001]))
    w32(uc, a + 0x52, rnd.choice([0, 100000, 300000, 2000000]))
    for k in range(0x56, 0x5e):
        w8(uc, a + k, rnd.randrange(1, 21))
    w32(uc, a + 0x61, 0)
    w32(uc, a + 0x65, 0)
    w32(uc, a + 0x69, 0)


def world(uc, rnd):
    uc.mem_write(W, b'\0' * (WORLD_END - W))
    # people
    pid = rnd.randrange(0, 500) if rnd.random() < 0.85 else 0  # id 0 opens the gates (id % k) + day == 0
    staff_rec(uc, P, rnd, pid)
    staff_rec(uc, MATE, rnd, rnd.randrange(500, 600))
    for k in range(8):
        staff_rec(uc, OTHER + 0x80 * k, rnd, 600 + k)
        staff_rec(uc, STAFFNP + 0x80 * k, rnd, 700 + k)
        w32(uc, STAFFNP + 0x80 * k + 0x69, NP + 0x40 * k if rnd.random() < 0.9 else 0)
        uc.mem_write(NP + 0x40 * k, bytes(rnd.randrange(1, 21) for _ in range(0x40)))
        w16(uc, NP + 0x40 * k + 4, rnd.randrange(0, 200))
    w32(uc, P + 0x61, PD)
    w32(uc, P + 0x65, OTHER + 0x400 if rnd.random() < 0.6 else 0)  # preferences (+0x28/+0x2c/+0x30)
    prefs = OTHER + 0x400
    for k, off in enumerate((0x28, 0x2c, 0x30)):
        w32(uc, prefs + off, OTHER + 0x80 * rnd.randrange(8) if rnd.random() < 0.6 else 0)
    uc.mem_write(PD, bytes(rnd.randrange(1, 21) for _ in range(0x46)))
    for off in (5, 7):
        w16(uc, PD + off, rnd.randrange(1, 200))
    for off in (9, 0xb, 0xd):
        w16(uc, PD + off, rnd.choice([0, 500, 3000, 4500, 5300, 5800, 6800, 7500, 9500]) + rnd.randrange(300))
    w8(uc, PD + 0xf, rnd.choice([1, 20, 5]))
    w8(uc, PD + 0x11, rnd.choice([0, 0, 5, 16]))
    w8(uc, PD + 0x45, rnd.randrange(0, 21))
    # nations, continents, divisions
    for k in range(3):
        w32(uc, CONT + 0x10 * k, [0xffffffff, 2, 3][k])
    w32(uc, 0x9cfa10, 0xffffffff)
    for k in range(4):
        n = NAT + 0x130 * k
        uc.mem_write(n, bytes(rnd.randrange(256) for _ in range(0x130)))
        w32(uc, n + 0x71, CONT + 0x10 * rnd.randrange(3) if rnd.random() < 0.9 else 0)
        w8(uc, n + 0x75, rnd.randrange(3))
        w8(uc, n + 0x77, rnd.randrange(40))
        w8(uc, n + 0x85, rnd.randrange(1, 20))
        w8(uc, n + 0x11c, rnd.randrange(2))
    for k in range(4):
        dv = DIVS + 0x80 * k
        w32(uc, dv, k)
        w16(uc, dv + 0x69, rnd.randrange(0, 200))
        w32(uc, COT + 4 * k, COBJ + 0x40 * k if rnd.random() < 0.9 else 0)
        w32(uc, COBJ + 0x40 * k + 8, OUT + 0x100 if rnd.random() < 0.9 else 0)
    # clubs
    for k in range(NCLUB):
        c = club(k)
        uc.mem_write(c, bytes(rnd.randrange(256) for _ in range(0x245)))
        w32(uc, c, k)
        w32(uc, c + 0x53, NAT + 0x130 * rnd.randrange(4) if rnd.random() < 0.95 else 0)
        w32(uc, c + 0x57, DIVS + 0x80 * rnd.randrange(4) if rnd.random() < 0.9 else 0)
        w8(uc, c + 0x7f, rnd.choice([0, 0, 5, 15]))
        w16(uc, c + 0x80, rnd.randrange(500, 10000))
        for off, j in ((0xbf, 0), (0xcf, 1), (0xd3, 2)):
            w32(uc, c + off, STAFFNP + 0x80 * j if rnd.random() < 0.85 else 0)
        for j in range(5):
            w32(uc, c + 0x19f + 4 * j, STAFFNP + 0x80 * (3 + j) if rnd.random() < 0.6 else 0)
        w32(uc, c + 0x1db, rnd.choice([0xffffffff, 0xffffffff, 3]))
        uc.mem_write(CST + 6 * k, bytes([rnd.choice([0xff, 0xff, 0, 1]) for _ in range(4)]) + bytes([rnd.randrange(256) for _ in range(2)]))
        w8(uc, CST + 6 * k + 4, rnd.choice([0xfd, 0xff, 0, 1, 2, 3, 5]))
        w8(uc, CST + 6 * k + 5, rnd.choice([0, 0, 2, 4, 9, 12]))
    w32(uc, 0xae23b4, CT)
    w32(uc, 0xae2d14, CST)
    w32(uc, 0xadadfc, COT)
    w32(uc, 0xae2d0c, ST)
    uc.mem_write(ST + pid * 0x4f, bytes(rnd.randrange(256) for _ in range(0x4f)))
    w8(uc, ST + pid * 0x4f + 0xa, rnd.choice([0, 0, 3, 10, 14, 30, 60]))
    w8(uc, ST + pid * 0x4f + 0xb, rnd.choice([0, 0, 1, 2, 0x40]))
    # contract
    ck = rnd.randrange(NCLUB)
    uc.mem_write(CONTRACT, bytes(rnd.randrange(256) for _ in range(0x50)))
    w32(uc, CONTRACT + 4, ck)
    w32(uc, CONTRACT + 0xc, rnd.choice([1000, 20000, 80000]))
    w16(uc, CONTRACT + 0x25, rnd.randrange(365))
    w16(uc, CONTRACT + 0x27, rnd.choice([1999, 2000, 2001, 2002]))
    w16(uc, CONTRACT + 0x2d, rnd.randrange(365))
    w16(uc, CONTRACT + 0x2f, rnd.choice([2000, 2001, 2002, 2003, 2004]))
    w8(uc, CONTRACT + 0x35, rnd.choice([1, 2, 1, 5]))
    w8(uc, CONTRACT + 0x3d, rnd.choice([-60, -5, 20, 50, 80, 99]) + rnd.randrange(-3, 1))
    w8(uc, CONTRACT + 0x3e, rnd.randrange(0, 6))
    w8(uc, CONTRACT + 0x3f, rnd.randrange(0, 6))
    w32(uc, CONTRACT + 0x40, rnd.getrandbits(32) & rnd.choice([0, 0x7f, 0x13ff]))
    w8(uc, CONTRACT + 0x44, rnd.randrange(-100, 101))
    g = 0
    for _ in range(rnd.choice([0, 1, 2, 4, 8])):
        g |= 1 << rnd.randrange(32)
    w32(uc, CONTRACT + 0x45, g)
    w8(uc, CONTRACT + 0x49, 0 if rnd.random() < 0.95 else 1)
    w8(uc, CONTRACT + 0x4e, rnd.randrange(16))
    w8(uc, CONTRACT + 0x4f, rnd.choice([0, 0x11, 0x23, 0x50, 0x65, 0x70, 0x7a, 0x14]))
    for k in range(6):
        oc = OCONTRACT + 0x60 * k
        w32(uc, oc + 4, rnd.choice([ck, rnd.randrange(NCLUB)]))
        w8(uc, oc + 0x35, rnd.choice([1, 2, 5]))
    # managers and globals
    for k, gl in enumerate((0xb63d28, 0xad9c6c, 0xae1050, 0xae28f0, 0xae2a30, 0xae2a38, 0xb6b7c8, 0xb74318,
                            0xb7431c, 0xb7434c, 0xde1f44)):
        w32(uc, gl, MGR + 0x100 * k)
    if rnd.random() < 0.1:
        w32(uc, 0xb63d28, 0)
    if rnd.random() < 0.1:
        w32(uc, 0xae2a30, 0)
    w32(uc, 0xde1f54, TM)
    w32(uc, TM + 0x856, MGR + 0x2000)
    w32(uc, 0xb74350, PSTATS)
    wf64(uc, PSTATS + 0x10, rnd.choice([0.0, 680.0, 7100.0]))
    wf64(uc, PSTATS + 0x18, rnd.choice([0.0, 100.0, 1000.0]))
    # the day: mostly one that opens one of the routine's day gates for this id
    gate = rnd.choice([(14, 9, 5), (30, 13, 5), (5, 3, 0), (14, 0, 8), (7, 0, 0), (12, 4, 0), (14, 5, 0), (8, 0, 0), (45, 0, 0), (10, 0, 0), (14, 0, 0), (30, 13, 5), None])
    day = rnd.randrange(365) if gate is None else next(x for x in range(365) if (x + pid + gate[2]) % gate[0] == gate[1])
    w16(uc, 0xae2c90, day)
    w16(uc, 0xae2c92, rnd.choice([2001, 2002]))
    w32(uc, 0xae2c94, rnd.getrandbits(8))
    w16(uc, 0xdd6ec8, rnd.randrange(365))
    w16(uc, 0xdd6eca, rnd.choice([2001, 2002]))
    w32(uc, 0xdd7120, rnd.getrandbits(16))
    clubv = club(ck) if rnd.random() < 0.85 else (club(rnd.randrange(NCLUB)) if rnd.random() < 0.5 else 0)
    contract = CONTRACT if rnd.random() < 0.9 else 0
    if clubv and rnd.random() < 0.7:
        w32(uc, P + 0x39, clubv)  # the club job is the club
    if rnd.random() < 0.5:
        w8(uc, P + 0x3d, 11)
    b_team = rnd.choice([0, 0, 1])
    slot70 = rnd.getrandbits(32)
    w32(uc, SLOT70, slot70)
    return pid, clubv, contract, b_team, slot70


# stubs: address -> (name, ret bytes, number of args, kind)
STUBS = {
    0x90cfc0: ('random', 0, 1, 'random'), 0x548da0: ('pick_mate', 0, 2, 'mate'),
    0x62c3d0: ('fatigue', 4, 1, 'fatigue'), 0x600a50: ('human_club', 0, 5, 'bool'),
    0x7c1950: ('recent_form', 4, 1, 'form'), 0x544520: ('listed', 0, 2, 'rarebool'),
    0x8676f0: ('get_flags', 0, 0, 'getflags'), 0x8676d0: ('clear_flags', 4, 1, 'clearflags'),
    0x867560: ('set_flags', 4, 1, 'setflags'), 0x8674e0: ('clear_grievances', 4, 1, 'clearg'),
    0x867700: ('apply_event_flags', 8, 2, 'applyg'), 0x7c1e30: ('get_stat', 16, 4, 'stat'),
    0x7c1a60: ('last_rating', 4, 1, 'stat'), 0x62c400: ('injury_status', 4, 1, 'injst'),
    0x62ed70: ('get_injury', 4, 1, 'injury'), 0x418f60: ('awol', 8, 2, 'rarebool'),
    0x547af0: ('add_disliked_staff', 0, 3, 'void'), 0x544410: ('np_rating', 0, 2, 'rating'),
    0x8b39f0: ('schedule', 4, 1, 'schedule'), 0x588cd0: ('training_report', 0, 4, 'void'),
    0x569e30: ('in_dispute', 8, 2, 'bool'), 0x56a090: ('remove_dispute', 4, 1, 'void'),
    0x56a030: ('dispute_club', 4, 1, 'clubid'), 0x56a3b0: ('dispute_news', 0x14, 5, 'void'),
    0x866f60: ('wants_to_leave', 0, 0, 'bool'), 0x4a4850: ('comp_stats_entry', 4, 1, 'entry4'),
    0x547790: ('club_favours', 0, 2, 'bool'), 0x5477d0: ('club_dislikes', 0, 2, 'bool'),
    0x544710: ('adjust_morale', 0, 3, 'void'), 0x54cdb0: ('days_since', 8, 2, 'days'),
    0x8cca10: ('future_transfer', 8, 2, 'rarebool'), 0x842c00: ('add_shortlists', 8, 2, 'void'),
    0x5a0590: ('club_finance', 4, 1, 'finance'), 0x72c1f0: ('media_story', 0x14, 5, 'void'),
    0x547770: ('in_season', 0, 2, 'bool'), 0x6e86d0: ('judge', 0, 5, 'judge'),
    0x7c3a40: ('apps', 8, 2, 'apps'), 0x744240: ('praise', 0x10, 4, 'void'),
    0x7751e0: ('current_comp', 4, 1, 'comp'), 0x76d940: ('is_finals', 4, 1, 'bool'),
    0x54c5b0: ('add_days', 8, 2, 'adddays'), 0x48f340: ('first_date', 4, 1, 'firstdate'),
    0x735840: ('has_story', 4, 1, 'bool'), 0x547810: ('favoured_by', 0, 2, 'bool'),
    0x547840: ('disliked_by', 0, 2, 'bool'), 0x589300: ('thanks', 0, 2, 'void'),
    0x4d0c60: ('calc_value', 0x14, 5, 'value'), 0x547d60: ('staff_favours_club', 0, 2, 'bool'),
    0x547d30: ('staff_dislikes_club', 0, 2, 'bool'), 0x5986b0: ('wealth_band', 0, 0, 'band'),
    0x7a7eb0: ('retirement', 4, 1, 'rarebool'), 0x8c1870: ('queued', 4, 1, 'rarebool'),
    0x545e90: ('club_rep_for_staff', 0, 2, 'rep'), 0x4d9840: ('seeks_assurances', 4, 1, 'void'),
    0x849ce0: ('shortlist_entry', 0x10, 4, 'slentry'), 0x866620: ('satisfaction', 4, 1, 'sat'),
    0x4cda70: ('offer', 0x1c, 7, 'bool'), 0x4cc9e0: ('find_offer', 8, 2, 'bool'),
    0x4d9f80: ('wants_news', 4, 1, 'void'), 0x547e70: ('add_disliked_club', 0, 3, 'void'),
    0x4ca470: ('deserved', 8, 2, 'deserved'), 0x4cf6d0: ('loan_contract', 4, 1, 'bool'),
    0x8c5cf0: ('offers', 0xc, 3, 'rarebool'), 0x8c5e40: ('exchange', 8, 2, 'rarebool'),
    0x53d980: ('is_foreign', 0, 3, 'bool'), 0x75a640: ('queue_story', 8, 2, 'void'),
    0x4cf5a0: ('get_contract', 4, 1, 'contract'), 0x547c90: ('count_disliked', 0, 2, 'count'),
    0x547ce0: ('count_favoured', 0, 2, 'count'), 0x5480f0: ('get_disliked', 0, 3, 'disliked'),
    0x55d270: ('banned', 0x10, 4, 'rarebool'), 0x8b4db0: ('fast_training', 4, 1, 'bool'),
    0x7c6a30: ('add_language', 0x10, 4, 'void'), 0x7c6ca0: ('improve_language', 0x10, 4, 'void'),
    0x419840: ('start_absence', 0xc, 3, 'void'),
}
# arguments read narrower by the callee (the rest of the dword is register garbage in the original)
NARROW = {'np_rating': {1: 0xff}, 'add_language': {1: 0xff}, 'improve_language': {1: 0xff}, 'offer': {6: 0xff},
          'judge': {3: 0xff, 4: 0xffff}, 'add_days': {1: 0xffff}}


def run(uc, entry, regs, frame, args, rnd, stop):
    calls = []
    outp = [OUT + 0x1000]

    def alloc(n):
        a = outp[0]
        outp[0] += (n + 15) & ~15
        return a

    def handler(name, kind, a, this):
        r = rnd
        if kind == 'random':
            n = a[0] if a[0] < 0x80000000 else a[0] - (1 << 32)
            if n <= 0:
                return 0
            x = r.random()
            return 0 if x < 0.5 else (n - 1 if x < 0.6 else r.randrange(n))
        if kind == 'bool':
            return r.choice([0, 1])
        if kind == 'rarebool':
            return 1 if r.random() < 0.15 else 0
        if kind == 'void':
            return None
        if kind == 'mate':
            return r.choice([0, MATE, OTHER])
        if kind == 'fatigue':
            return r.randrange(-6000, 3000) & 0xffff
        if kind == 'form':
            return r.randrange(0, 80)
        if kind == 'getflags':
            return u32(uc, this + 0x40)
        if kind == 'clearflags':
            w32(uc, this + 0x40, u32(uc, this + 0x40) & ~a[0])
            return None
        if kind == 'setflags':
            w32(uc, this + 0x40, u32(uc, this + 0x40) | a[0])
            return None
        if kind == 'clearg':
            w32(uc, this + 0x45, u32(uc, this + 0x45) & ~a[0])
            return None
        if kind == 'applyg':
            w32(uc, this + 0x45, u32(uc, this + 0x45) | a[0])
            return None
        if kind == 'stat':
            return ('fp', r.choice([0.0, 0.5, 5.5, 6.4, 6.6, 7.2, 8.0, 9.5, 11.0, 14.0, 16.0]) + r.random() * 0.1)
        if kind == 'injst':
            return r.choice([0, 0, 0, 1, 2])
        if kind == 'injury':
            if r.random() < 0.6:
                return 0
            i = alloc(0x40)
            w16(uc, i + 0x2c, r.choice([5, 20, 40, 70]))
            return i
        if kind == 'rating':
            return r.randrange(1, 21)
        if kind == 'schedule':
            if r.random() < 0.3:
                return 0
            s = alloc(0x20)
            w8(uc, s + 0x15, r.choice([0, 1]))
            return s
        if kind == 'clubid':
            return r.randrange(NCLUB)
        if kind == 'entry4':
            if r.random() < 0.3:
                return 0
            e = alloc(0x10)
            w8(uc, e + 4, r.randrange(1, 20))
            return e
        if kind == 'days':
            return r.choice([-50, 10, 40, 70, 150, 200, 400, 600, 900, 1500]) & 0xffffffff
        if kind == 'finance':
            if r.random() < 0.2:
                return 0
            f = alloc(0x20)
            w32(uc, f + 0x10, r.choice([5000, 50000, 200000]))
            return f
        if kind == 'judge':
            return r.randrange(50, 250)
        if kind == 'apps':
            return r.randrange(0, 150)
        if kind == 'comp':
            if r.random() < 0.2:
                return 0
            c = alloc(0x10)
            w32(uc, c, r.randrange(4))
            return c
        if kind in ('adddays', 'firstdate'):
            out = a[0]
            w16(uc, out, r.randrange(365))
            w16(uc, out + 2, r.choice([2000, 2001, 2002, 2003]))
            w32(uc, out + 4, 0)
            return out
        if kind == 'value':
            return r.choice([100000, 400000, 1000000, 3000000, 7000000])
        if kind == 'band':
            return r.randrange(-2, 4) & 0xff
        if kind == 'rep':
            return r.randrange(500, 9500)
        if kind == 'slentry':
            if r.random() < 0.2:
                return 0
            e = alloc(0x30)
            w8(uc, e + 0x24, r.randrange(256))
            w32(uc, e + 0xf, r.choice([u32(uc, P), 12345]))
            return e
        if kind == 'sat':
            return r.choice([0, 1, 2, 3])
        if kind == 'deserved':
            return r.randrange(0, 9)
        if kind == 'contract':
            return r.choice([0, OCONTRACT + 0x60 * r.randrange(6)])
        if kind == 'count':
            return r.choice([0, 0, 1, 2])
        if kind == 'disliked':
            n = r.choice([0, 1, 3])
            for k in range(n):
                w32(uc, a[1] + 4 * k, r.choice([0, OTHER + 0x80 * r.randrange(8)]))
            w8(uc, a[2], n)
            return n
        raise Exception('kind ' + kind)

    def hook(uc, address, size, _):
        if 0x5ce4b6 <= address < 0x5d352d:
            COVER.add(address)
            if TRACE is not None and address in ():
                spp = uc.reg_read(UC_X86_REG_ESP)
                print('at', hex(address), 'esp', hex(spp), '[1c]', hex(u32(uc, spp + 0x1c)), 'edi', hex(uc.reg_read(UC_X86_REG_EDI)))
            if TRACE is not None:
                TRACE.append(address)
        if address == END or address in stop:
            uc.emu_stop()
            return
        if address == BRANCH:
            sp = uc.reg_read(UC_X86_REG_ESP)
            uc.reg_write(UC_X86_REG_ESP, sp + 4)
            uc.reg_write(UC_X86_REG_EIP, u32(uc, sp))
            return
        st = STUBS.get(address)
        if st is None:
            return
        name, ret, nargs, kind = st
        sp = uc.reg_read(UC_X86_REG_ESP)
        ra = u32(uc, sp)
        a = [u32(uc, sp + 4 + 4 * k) for k in range(max(nargs, 3))]
        this = uc.reg_read(UC_X86_REG_ECX)
        rec = []
        for k in range(nargs):
            v = a[k]
            if STACK <= v < STACK + STACK_SIZE:
                v = 'S'
            elif name in NARROW and k in NARROW[name]:
                v &= NARROW[name][k]
            rec.append(v)
        thisrec = this if ret or kind in ('getflags', 'band') or name in ('wants_to_leave',) else None
        r = handler(name, kind, a, this)
        calls.append((name, tuple(rec), thisrec))
        if isinstance(r, tuple):
            struct_addr = CAVE + 0x100
            uc.mem_write(struct_addr, struct.pack('<d', r[1]))
            # one fixed piece of code per return size (unicorn keeps translated code: never rewrite it)
            cave = CAVE + 0x10 * (ret // 4)
            code = b'\xdd\x05' + struct.pack('<I', struct_addr) + (b'\xc2' + struct.pack('<H', ret) if ret else b'\xc3')
            if bytes(uc.mem_read(cave, len(code))) != code:
                uc.mem_write(cave, code)
            uc.reg_write(UC_X86_REG_EIP, cave)
            return
        if r is not None:
            uc.reg_write(UC_X86_REG_EAX, r & 0xffffffff)
        uc.reg_write(UC_X86_REG_ESP, sp + 4 + ret)
        uc.reg_write(UC_X86_REG_EIP, ra)

    esp = STACK + STACK_SIZE - 0x3000
    uc.mem_write(esp - 0x100, b'\0' * 0x1000)
    for off, v in frame.items():
        w32(uc, esp + off, v)
    if args is not None:
        esp -= 4 + 4 * len(args)
        uc.mem_write(esp, struct.pack('<I', END) + b''.join(struct.pack('<I', x & 0xffffffff) for x in args))
    uc.reg_write(UC_X86_REG_ESP, esp)
    for reg, v in regs.items():
        uc.reg_write(reg, v & 0xffffffff)
    uc.reg_write(UC_X86_REG_FPCW, 0x27f)
    h = uc.hook_add(UC_HOOK_CODE, hook)
    try:
        uc.emu_start(entry, END, count=3_000_000)
    except Exception as ex:
        calls.append(('EXC', str(ex), hex(uc.reg_read(UC_X86_REG_EIP))))
    finally:
        uc.hook_del(h)
    eip = uc.reg_read(UC_X86_REG_EIP)
    return calls, eip, esp


def snapshot(uc, slot):
    uc.mem_write(SLOT70, b'\0' * 8)  # the copy's slot70 lives here, the original's in its frame
    return bytes(uc.mem_read(W, OUT - W)) + bytes(uc.mem_read(0xae1080, 4)), slot


COVER = set()
import os
TRACE = [] if os.environ.get('GRA_TRACE') else None
uc = make_uc()
bad = 0
seen = {}
for i in range(FIRST, FIRST + N):
    if i % 100 == 0:
        uc = make_uc()
    res = []
    for side in ('o', 'r'):
        wr = random.Random(i)
        pid, clubv, contract, b_team, slot70 = world(uc, wr)
        rnd = random.Random(i * 7 + 1)
        if side == 'o':
            frame = {0x1c: clubv, 0x28: pid * 0x4f, 0x30: pid, 0x40: PD, 0x68: contract, 0x6c: P, 0x70: slot70, 0x74: 0,
                     0x7c: b_team}
            regs = {UC_X86_REG_EBP: P, UC_X86_REG_ESI: contract, UC_X86_REG_EDI: pid, UC_X86_REG_EBX: 0}
            calls, eip, esp = run(uc, 0x5ce4b6, regs, frame, None, rnd, {0x5d352d, 0x5d3539})
            slot = u32(uc, esp + 0x70)
            if eip not in (0x5d352d, 0x5d3539):
                calls.append(('END', hex(eip)))
        else:
            w32(uc, SLOT70 + 4, 0)
            calls, eip, esp = run(uc, RUST, {}, {}, [P, PD, clubv, contract, b_team, SLOT70], rnd, set())
            slot = u32(uc, SLOT70)
            if eip != END:
                calls.append(('END', hex(eip)))
        res.append((calls, snapshot(uc, slot)))
    for c in res[0][0]:
        seen[c[0]] = seen.get(c[0], 0) + 1
    if res[0][0] != res[1][0] or res[0][1] != res[1][1]:
        bad += 1
        if bad <= 3:
            print('DIFF case', i)
            a, b = res[0][0], res[1][0]
            for k in range(max(len(a), len(b))):
                x = a[k] if k < len(a) else None
                y = b[k] if k < len(b) else None
                if x != y:
                    print('  call', k, '\n   o', x, '\n   r', y)
                    print('   before:', a[max(0, k - 3):k])
                    break
            if res[0][1] != res[1][1]:
                ma, mb = res[0][1][0], res[1][1][0]
                d = [hex(W + k) for k in range(len(ma)) if ma[k] != mb[k]]
                print('   mem differs at', d[:12], 'slot70', hex(res[0][1][1]), hex(res[1][1][1]))
print('cases %d differences %d' % (N, bad))
if TRACE is not None:
    k = TRACE.index(0x5d0e47) if 0x5d0e47 in TRACE else len(TRACE)
    print(' '.join(hex(a) for a in TRACE[k - 50:k + 3]))
import re as _re
allins = [int(l.split()[0], 16) for l in open(sys.argv[5] if len(sys.argv) > 5 else 'c.txt') if 0x5ce4b6 <= int(l.split()[0], 16) < 0x5d352d]
miss = [a for a in allins if a not in COVER]
print('original instructions covered %d of %d' % (len(allins) - len(miss), len(allins)))
open('miss.txt', 'w').write('\n'.join(hex(a) for a in miss))
print('calls seen:', sorted(seen.items(), key=lambda x: -x[1]))
