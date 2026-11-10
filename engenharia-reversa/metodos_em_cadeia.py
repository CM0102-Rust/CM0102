r"""A typed record's field read through its address becomes the field's method (10/11/2026, the outside reader's round
11, obstacle 1: "the typed records leak back to raw addresses through `.addr() + module::FIELD`"):

    read_usize(person.club_job().addr() + club::MANAGER)   ->  person.club_job().manager().addr()
    read_i8(m.pending_play_offender().addr() + match_player::SIDE)  ->  m.pending_play_offender().side()
    write_i16(x.addr() + club::REPUTATION, v)               ->  x.set_reputation(v)

Two phases, so a field is never read through the method of a different record type that happens to share its name:
first the call is written in the checked form `Club::manager(EXPR)`, which only compiles when EXPR is a Club (a
function that does not compile goes back to its previous text); then those calls become the method form
`EXPR.manager()`. A read narrower than the field is its low bytes (`(EXPR.f() as i8)`, little-endian, no side
effect); a write must have the field's exact type. The mutation snippets follow.
Usage: python metodos_em_cadeia.py [--gravar]
[pt] O campo de um registro tipado lido pelo endereço vira o método dele, em duas fases (a forma conferida
`Club::manager(EXPR)` só compila se EXPR for Club; depois vira `EXPR.manager()`).
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import involucros_exe as ie  # noqa: E402
import mover_rotinas  # noqa: E402
import tipar_registros as tr  # noqa: E402
import traduzir_locais as tl  # noqa: E402

LARG = {'u8': 1, 'i8': 1, 'u16': 2, 'i16': 2, 'u32': 4, 'i32': 4, 'usize': 4, 'f32': 4, 'f64': 8, 'i64': 8, 'u64': 8}
INTS = {'u8', 'i8', 'u16', 'i16', 'u32', 'i32', 'usize', 'i64', 'u64'}
PAR = r'\((?:[^()]|\((?:[^()]|\([^()]*\))*\))*\)'
EXPR = r'[a-z_][a-z0-9_]*(?:%s)?(?:\.[a-z_][a-z0-9_]*%s)*' % (PAR, PAR)
ARG = re.compile(r'\s*(%s)\.addr\(\)\s*\+\s*([a-z_0-9]+)::([A-Z][A-Z0-9_]*)\s*' % EXPR)


def metodos_dos_registros():
    """{Type: {method names}} of records.rs (the field getters and setters, with self)."""
    t = open(os.path.join(SRC, 'motor', 'records.rs'), encoding='utf-8').read()
    out, atual = {}, None
    for l in t.split('\n'):
        m = re.match(r'impl (\w+) \{', l)
        if m:
            atual = m.group(1)
        m = re.match(r'\s*pub\(crate\) (?:unsafe )?fn (?:r#)?(\w+)\(self', l)
        if m and atual:
            out.setdefault(atual, set()).add(m.group(1))
    return out


METODOS = metodos_dos_registros()


def registros():
    t = open(os.path.join(SRC, 'motor', 'records.rs'), encoding='utf-8').read()
    mods = re.findall(r'/// A `(\w+)` record in the game', t)
    out = {}
    for mod in mods:
        tipo, campos = tr.campos_do_registro(mod)
        out[mod] = (tipo, campos)
    return out


def converter(bloco, regs, fase):
    """fase 1: Type::f(EXPR); fase 2 is done globally afterwards."""
    mk = tr.mascara(bloco)
    ed = []
    for m in re.finditer(r'(?<![\w.:])(read|write)_(u8|i8|u16|i16|u32|i32|usize|f32|f64|i64|u64)(_as_i32)?\(', mk):
        args, fim = ie.args_da_chamada(mk, m.end() - 1)
        if not args or fim is None:
            continue
        a0 = m.end()
        n0 = mk.index(args[0], a0)
        mm = ARG.fullmatch(bloco[n0:n0 + len(args[0])])
        if not mm or mm.group(2) not in regs:
            continue
        tipo, campos = regs[mm.group(2)]
        c = campos.get(mm.group(3))
        if not c or c[1] is None:
            continue
        metodo, rt, ponteiro = c
        expr, T = mm.group(1), m.group(2)
        nome = metodo.replace('r#', '')
        chamada = '%s::%s(%s)' % (tipo, metodo, expr)
        if m.group(1) == 'read':
            if len(args) != 1:
                continue
            if ponteiro:
                if T != 'usize' or m.group(3):
                    continue
                novo = '%s.addr()' % chamada
            elif rt == T:
                novo = chamada
            elif rt in INTS and T in INTS and LARG[T] <= LARG[rt]:
                novo = '(%s as %s)' % (chamada, T)
            else:
                continue
            if m.group(3):
                novo = '(%s as i32)' % novo
        else:
            if len(args) != 2 or m.group(3) or ponteiro or rt != T:
                continue
            v = bloco[mk.index(args[1], n0 + len(args[0])):][:len(args[1])].strip()
            novo = '%s::set_%s(%s, %s)' % (tipo, nome, expr, v)
        ed.append((m.start(), fim, novo))
    if not ed:
        return bloco, 0
    # innermost first: an outer access containing an inner one comes back on the next pass
    livres = [e for e in ed if not any(o is not e and e[0] <= o[0] and o[1] <= e[1] for o in ed)]
    for a, b, txt in sorted(livres, reverse=True):
        bloco = bloco[:a] + txt + bloco[b:]
    return bloco, len(livres)


def fase2(s, tipos):
    """`Type::f(EXPR)` -> `EXPR.f()` and `Type::set_f(EXPR, v)` -> `EXPR.set_f(v)`: the compiler already checked EXPR's type."""
    out, pos = [], 0
    for m in re.finditer(r'(?<![\w:])(%s)::(r#)?([a-z_][a-z0-9_]*)\(' % '|'.join(sorted(tipos, key=len, reverse=True)), s):
        if m.start() < pos:
            continue
        args, fim = ie.args_da_chamada(s, m.end() - 1)
        if not args or fim is None or not re.fullmatch(r'\s*%s\s*' % EXPR, args[0]):
            continue
        meth = (m.group(2) or '') + m.group(3)
        if m.group(3) not in METODOS.get(m.group(1), ()):
            continue  # only a record's field methods (an associated constructor stays `Type::new(..)`)
        resto = ', '.join(a.strip() for a in args[1:])
        out += [s[pos:m.start()], '%s.%s(%s)' % (args[0].strip(), meth, resto)]
        pos = fim
    out.append(s[pos:])
    return ''.join(out)


def main():
    grava = '--gravar' in sys.argv
    regs = registros()
    tipos = {t for t, _ in regs.values()}
    mutacoes = tr.carregar_mutacoes()
    trocas_mut, n = {}, 0
    for _ in range(3):
        n_volta = 0
        for raiz, _, fs in os.walk(SRC):
            for f in fs:
                if not f.endswith('.rs') or f in ('records.rs', 'fields.rs', 'exe_calls.rs', 'mem.rs'):
                    continue
                p = os.path.join(raiz, f)
                t = open(p, encoding='utf-8').read()
                tl.ANTES[p] = t
                partes, pos = [], 0
                for _, a, b in mover_rotinas.blocos(t):
                    bloco = t[a:b]
                    novo, k = converter(bloco, regs, 1)
                    if not k:
                        continue
                    n_volta += k
                    for lista, mu in mutacoes:
                        if bloco.count(mu[1]) == 1:
                            trocas_mut[(lista, mu[0])] = (mu, (lambda s: fase2(converter(s, regs, 1)[0], tipos)))
                    partes += [t[pos:a], novo]
                    pos = b
                partes.append(t[pos:])
                if grava and ''.join(partes) != t:
                    open(p, 'w', encoding='utf-8').write(''.join(partes))
        print('passada: %d acessos na forma conferida' % n_volta)
        n += n_volta
        if not grava or not n_volta:
            break
        tl.voltar_funcoes_com_erro(trocas_mut)
    print('%d acessos convertidos (antes de voltar as funções que não compilam)' % n)
    if grava:
        for raiz, _, fs in os.walk(SRC):
            for f in fs:
                if f.endswith('.rs') and f not in ('records.rs', 'fields.rs', 'exe_calls.rs', 'mem.rs'):
                    p = os.path.join(raiz, f)
                    t = open(p, encoding='utf-8').read()
                    t2 = fase2(t, tipos)
                    if t2 != t:
                        open(p, 'w', encoding='utf-8').write(t2)
        tr.gravar_mutacoes(trocas_mut)


if __name__ == '__main__':
    main()
