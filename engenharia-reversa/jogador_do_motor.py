r"""The match engine's player block by name (10/11/2026, the outside reader's round 10, item 5: "engine sub-structures").
Every hand-computed address of a match player (`m + 0x4796 + 8920 * side + 0x1be * k`, `side_player_slot(m, side, k)`
and their variants) becomes `m.player(side, k)`, which returns the typed `MatchPlayer`. The receiver is the typed
`Engine` (`m.addr()` loses the `.addr()`; a raw `usize` gets `Engine(..)`); where the old expression was a `usize`, the
result gets `.addr()`. The argument casts are then fixed by the compiler's own messages (`as i8` / `as usize`).
Usage: python jogador_do_motor.py [--gravar | --casts]
[pt] O jogador do bloco do motor com nome: os endereços calculados à mão viram `m.player(lado, k)`.
"""
import json
import os
import re
import subprocess
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import involucros_exe as ie  # noqa: E402

PAR = r'\((?:[^()]|\((?:[^()]|\([^()]*\))*\))*\)'
ATOM = r'(?:\w[\w.:]*(?:%s)?(?:\.\w+(?:%s)?)*|%s)(?: as \w+)*' % (PAR, PAR, PAR)
BASE = r'(\w+(?:\.addr\(\))?)'
LADO = r'(?:0x22d8|8920)'
BLOCO = r'(?:engine::PLAYERS|0x4796)'
FORMAS = [
    # E + PLAYERS + 8920 * S + 0x1be * K  /  K * 0x1be
    (BASE + r' \+ ' + BLOCO + r' \+ ' + LADO + r' \* (' + ATOM + r') \+ 0x1be \* (' + ATOM + r')', 'sk'),
    (BASE + r' \+ ' + BLOCO + r' \+ ' + LADO + r' \* (' + ATOM + r') \+ (' + ATOM + r') \* 0x1be', 'sk'),
    (BASE + r' \+ ' + BLOCO + r' \+ (' + ATOM + r') \* ' + LADO + r' \+ (' + ATOM + r') \* 0x1be', 'sk'),
    (BASE + r' \+ ' + LADO + r' \* (' + ATOM + r') \+ ' + BLOCO + r' \+ 0x1be \* (' + ATOM + r')', 'sk'),
    # (E as isize + 0x4796 + 0x1be * (K + 20 * S)) as usize  /  (20 * S + K)
    (r'\(' + BASE + r' as isize \+ 0x4796 \+ 0x1be \* \((' + ATOM + r') \+ 20 \* (' + ATOM + r')\)(?: as isize)?\) as usize', 'ks'),
    (r'\(' + BASE + r' as isize \+ 0x4796 \+ 0x1be \* \(20 \* (' + ATOM + r') \+ (' + ATOM + r')\)(?: as isize)?\) as usize', 'sk'),
    (r'\(' + BASE + r' as isize \+ 0x4796 \+ 0x1be \* \((' + ATOM + r') \* 20 \+ (' + ATOM + r')\)(?: as isize)?\) as usize', 'sk'),
    (r'\(' + BASE + r' as isize \+ 0x4796 \+ \((' + ATOM + r') \+ (' + ATOM + r') \* 20\) \* 0x1be\) as usize', 'ks'),
    # (E as isize + 0x4796 + 8920 * S) as usize + K * 0x1be
    (r'\(' + BASE + r' as isize \+ 0x4796 \+ 8920 \* (' + ATOM + r')\) as usize \+ (' + ATOM + r') \* 0x1be', 'sk'),
    # E + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be  (the self-test generators)
]


def sem_cast(x):
    x = x.strip()
    while True:
        m = re.fullmatch(r'(.*\S) as (?:isize|usize|i32|i64)', x, re.S)
        if not m:
            return x
        x = m.group(1)


def receptor(e):
    e = e.strip()
    if e.endswith('.addr()'):
        return e[:-len('.addr()')]
    return 'Engine(%s)' % e


def trocar(t):
    n = 0
    # 1. the raw forms become side_player_slot(E, S, K) with the casts dropped
    for rx, ordem in FORMAS:
        def f(m):
            e = m.group(1)
            a, b = m.group(2), m.group(3)
            s, k = (a, b) if ordem == 'sk' else (b, a)
            return 'side_player_slot(%s, %s, %s)' % (e, sem_cast(s), sem_cast(k))
        t, c = re.subn(rx, f, t)
        n += c
    # 2. side_player_slot(E, S, K) -> E.player(S, K)
    out, pos = [], 0
    for m in re.finditer(r'(MatchPlayer\()?\bside_player_slot\(', t):
        if m.start() < pos:
            continue
        args, fim = ie.args_da_chamada(t, m.end() - 1)
        if not args or len(args) != 3:
            continue
        novo = '%s.player(%s, %s)' % (receptor(args[0]), args[1].strip(), args[2].strip())
        if m.group(1):
            if t[fim:fim + 1] != ')':
                continue
            fim += 1
        else:
            novo += '.addr()'
        out += [t[pos:m.start()], novo]
        pos = fim
        n += 1
    out.append(t[pos:])
    return ''.join(out), n


def casts():
    """Fix the argument types from the compiler's E0308 messages on .player(..) calls."""
    for _ in range(6):
        r = subprocess.run(['cargo', 'build', '--release', '--message-format=json'], cwd=os.path.join(R, 'rust'),
                           capture_output=True, text=True, encoding='utf-8')
        ed = {}
        for l in r.stdout.splitlines():
            try:
                d = json.loads(l)
            except ValueError:
                continue
            msg = d.get('message') or {}
            if msg.get('level') != 'error':
                continue
            for sp in msg.get('spans', []):
                lab = sp.get('label') or ''
                mm = re.match(r'expected `(i8|usize)`, found `[^`]+`', lab)
                if mm:
                    ed.setdefault(os.path.join(R, 'rust', sp['file_name']), []).append(
                        (sp['byte_start'], sp['byte_end'], mm.group(1)))
        if not ed:
            return r.returncode == 0
        for f, es in ed.items():
            b = open(f, 'rb').read()
            for a, z, tp in sorted(set(es), reverse=True):
                x = b[a:z].decode('utf-8')
                if not re.fullmatch(r'[\w.]+(?:\(\))?', x):
                    x = '(%s)' % x
                b = b[:a] + ('%s as %s' % (x, tp)).encode('utf-8') + b[z:]
            open(f, 'wb').write(b)
    return False


def main():
    if '--casts' in sys.argv:
        print('compila' if casts() else 'NAO compila: ver cargo build')
        return
    total = 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs') or f in ('fields.rs', 'records.rs'):
                continue
            p = os.path.join(raiz, f)
            t = open(p, encoding='utf-8').read()
            novo, n = trocar(t)
            if n:
                total += n
                if '--gravar' in sys.argv:
                    open(p, 'w', encoding='utf-8').write(novo)
    print('%d trocas' % total)
    if '--gravar' in sys.argv:
        print('compila' if casts() else 'NAO compila: ver cargo build')


if __name__ == '__main__':
    main()
