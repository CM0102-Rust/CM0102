r"""Typed record variables (readability, migration to records.rs, 09/11/2026): in each function, a variable `x` used as
`x + club::FIELD` holds a club; it becomes a `Club` and its field accesses become methods:

    read_i16(x + club::REPUTATION)          ->  x.reputation()
    write_u8(x + club::OWN_STADIUM, 1)      ->  x.set_own_stadium(1)
    *((x + club::OWN_STADIUM) as *mut u8) = 1;  ->  x.set_own_stadium(1);
    read_usize(x + club::STADIUM)           ->  x.stadium().addr()   (a pointer field keeps its address for now)

A parameter keeps its `usize` signature and is rebound at the top of the body (`let x = Club(x);`); a `let x = e;` becomes
`let x = Club(e);`. Every other use of `x` becomes `x.addr()` (`x == 0` becomes `x.is_null()`). An access with a width
other than the field's stays raw (`read_u8(x.addr() + club::REPUTATION)`). Only variables bound once and never
reassigned. A function that does not compile goes back to its previous text (traduzir_locais), and the mutation lists
get the same change, snippet by snippet.
Usage: python tipar_registros.py MODULE [--gravar]      (MODULE = a fields.rs module, e.g. club)
[pt] Variáveis de registro tipadas: a variável usada como `x + club::CAMPO` vira `Club` e os acessos viram métodos.
"""
import collections
import json
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import involucros_exe as ie  # noqa: E402
import mover_rotinas  # noqa: E402
import traduzir_locais as tl  # noqa: E402

LARG = {'u8': 1, 'i8': 1, 'u16': 2, 'i16': 2, 'u32': 4, 'i32': 4, 'usize': 4, 'f32': 4, 'f64': 8, 'i64': 8, 'u64': 8}


def campos_do_registro(mod):
    """{FIELD: (method, type or record type, is_pointer)} from records.rs; type None = address only."""
    t = open(os.path.join(SRC, 'motor', 'records.rs'), encoding='utf-8').read()
    out, tipo = {}, None
    for m in re.finditer(r'/// `%s::(\w+)` \(\+0x[0-9a-f]+\)\.\n\s*#\[inline\(always\)\]\n\s*pub\(crate\) (?:unsafe )?fn (r#)?(\w+)\(self\) -> (\w+)' % mod, t):
        campo, nome, rt = m.group(1), m.group(3), m.group(4)
        if nome.endswith('_addr') and rt == 'usize':
            out[campo] = (nome, None, False)
        else:
            out[campo] = ((m.group(2) or '') + nome, rt, rt not in LARG)
    m = re.search(r'/// A `%s` record in the game' % mod, t)
    tipo = re.search(r'pub\(crate\) struct (\w+)\(', t[m.end():]).group(1)
    return tipo, out


def mascara(s):
    """The text with comments and string/char literals blanked (same length): positions stay valid."""
    out = list(s)
    for m in re.finditer(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)\'', s):
        for k in range(m.start(), m.end()):
            if out[k] != '\n':
                out[k] = ' '
    return ''.join(out)


def fim_da_expressao(s, i):
    """Index of the `;` that ends the expression starting at s[i] (depth 0)."""
    nivel = 0
    while i < len(s):
        c = s[i]
        if c in '([{':
            nivel += 1
        elif c in ')]}':
            nivel -= 1
        elif c == ';' and nivel == 0:
            return i
        i += 1
    return -1


class Conversor:
    def __init__(self, mod):
        self.mod = mod
        self.tipo, self.campos = campos_do_registro(mod)

    def leitura(self, x, campo, t):
        if campo not in self.campos:
            return None
        nome, ft, ptr = self.campos[campo]
        if ft is None:
            return None
        if ptr:
            if LARG.get(t) != 4:
                return None
            return '%s.%s().addr()' % (x, nome) if t == 'usize' else '(%s.%s().addr() as %s)' % (x, nome, t)
        if t == ft:
            return '%s.%s()' % (x, nome)
        if LARG.get(t) == LARG.get(ft) and 'f' not in (t[0] + ft[0]):
            return '(%s.%s() as %s)' % (x, nome, t)
        # a narrower read of an integer field is its low bytes (little-endian, no side effect): the same value
        if LARG.get(t, 9) < LARG.get(ft, 0) and 'f' not in (t[0] + ft[0]) and ft not in ('i64', 'u64'):
            return '(%s.%s() as %s)' % (x, nome, t)
        return None

    def escrita(self, x, campo, t, valor):
        if campo not in self.campos:
            return None
        nome, ft, ptr = self.campos[campo]
        if ft is None or ptr:
            return None
        if t == ft:
            return '%s.set_%s(%s)' % (x, nome.replace('r#', ''), valor)
        if LARG.get(t) == LARG.get(ft) and 'f' not in (t[0] + ft[0]):
            return '%s.set_%s((%s) as %s)' % (x, nome.replace('r#', ''), valor, ft)
        return None

    def variaveis(self, bloco, mk):
        """The variables of this block that hold the record, bound once and never reassigned: {name: kind}."""
        nomes = set(re.findall(r'(?<![\w.])([a-z_][a-z0-9_]*)\s*\+\s*%s::[A-Z]' % self.mod, mk))
        cab = mk[:mk.find('{')] if '{' in mk else ''
        out = {}
        for x in nomes:
            if x == 'self':
                continue
            param = re.search(r'[(,]\s*%s\s*:\s*usize\b' % re.escape(x), cab)
            lets = re.findall(r'\blet\s+(mut\s+)?%s\b' % re.escape(x), mk)
            atrib = re.search(r'(?<![\w.])(?<!let )(?<!let mut )%s\s*(\+|-|\*|/|&|\||\^)?=(?!=)' % re.escape(x), mk)
            # a closure parameter with the same name: `|x: usize|` or `|a, x|` right after `=`, `(`, `,` or `move`
            fecho = any(re.search(r'(?<![\w.])%s\b' % re.escape(x), ps)
                        for ps in re.findall(r'(?:[=(,]|\bmove)\s*\|([^|]*)\|', mk))
            if atrib or fecho or (param and lets) or len(lets) > 1 or (lets and lets[0]):
                continue
            if param:
                out[x] = 'param'
            elif len(lets) == 1:
                out[x] = 'let'
        return out

    def converter_trecho(self, s, xs, so_acessos=False):
        """The accesses and the other uses of the variables xs in the text s (a function or a mutation snippet). With
        so_acessos, the variables are already typed (`x.addr() + mod::F`): only the accesses change."""
        mk = mascara(s)
        edicoes = []
        ocupado = []
        nomes = '|'.join(re.escape(x) for x in sorted(xs, key=len, reverse=True))
        if not nomes:
            return s
        # read_T(x + mod::F) / write_T(x + mod::F, v)
        for m in re.finditer(r'\b(read|write)_(u8|i8|u16|i16|u32|i32|usize|f32|f64|i64|u64)(_as_i32)?\(', mk):
            args, fim = ie.args_da_chamada(mk, m.end() - 1)
            if not args:
                continue
            a0 = re.fullmatch(r'\s*(%s)(?:\.addr\(\))?\s*\+\s*%s::([A-Z][A-Z0-9_]*)\s*' % (nomes, self.mod), args[0])
            if not a0:
                continue
            x, campo, t = a0.group(1), a0.group(2), m.group(2)
            if m.group(3) and m.group(1) != 'read':
                continue
            if m.group(1) == 'read' and len(args) == 1:
                novo = self.leitura(x, campo, t)
                if novo and m.group(3):
                    novo = '(%s as i32)' % novo
            elif m.group(1) == 'write' and len(args) == 2:
                # the value text from the real string (it may hold a comment or a literal)
                ini_v = m.end() + mk[m.end():fim].index(',') + 1
                novo = self.escrita(x, campo, t, self.converter_trecho(s[ini_v:fim - 1].strip(), xs, so_acessos))
            else:
                novo = None
            if novo and not any(a <= m.start() < b for a, b in ocupado):
                edicoes.append((m.start(), fim, novo))
                ocupado.append((m.start(), fim))
        # core::ptr::read_unaligned((x + mod::F) as *const T)  and  core::ptr::write_unaligned((x + mod::F) as *mut T, v)
        for m in re.finditer(r'core::ptr::(read|write)_unaligned\(', mk):
            if any(a <= m.start() < b for a, b in ocupado):
                continue
            args, fim = ie.args_da_chamada(mk, m.end() - 1)
            if not args:
                continue
            a0 = re.fullmatch(r'\s*\(\s*(%s)(?:\.addr\(\))?\s*\+\s*%s::([A-Z][A-Z0-9_]*)\s*\)\s*as\s*\*(?:const|mut)\s+(\w+)\s*' % (nomes, self.mod), args[0])
            if not a0:
                continue
            x, campo, t = a0.group(1), a0.group(2), a0.group(3)
            if m.group(1) == 'read' and len(args) == 1:
                novo = self.leitura(x, campo, t)
            elif m.group(1) == 'write' and len(args) == 2:
                ini_v = m.end() + len(mk[m.end():fim]) - len(mk[m.end():fim].lstrip()) + mk[m.end():fim].index(args[1], len(args[0]))
                novo = self.escrita(x, campo, t, self.converter_trecho(s[ini_v:ini_v + len(args[1])].strip(), xs, so_acessos))
            else:
                novo = None
            if novo:
                edicoes[:] = [e for e in edicoes if not (m.start() <= e[0] and e[1] <= fim)]
                ocupado[:] = [o for o in ocupado if not (m.start() <= o[0] and o[1] <= fim)]
                edicoes.append((m.start(), fim, novo))
                ocupado.append((m.start(), fim))
        # *((x + mod::F) as *const T)  and  *((x + mod::F) as *mut T) = v;
        for m in re.finditer(r'\*\(\(\s*(%s)(?:\.addr\(\))?\s*\+\s*%s::([A-Z][A-Z0-9_]*)\s*\)\s*as\s*\*(const|mut)\s+(\w+)\)' % (nomes, self.mod), mk):
            if any(a <= m.start() < b for a, b in ocupado):
                continue
            x, campo, kind, t = m.group(1), m.group(2), m.group(3), m.group(4)
            depois = mk[m.end():]
            atr = re.match(r'\s*=(?!=)\s*', depois)
            if atr:
                fim = fim_da_expressao(mk, m.end() + atr.end())
                if fim < 0:
                    continue
                novo = self.escrita(x, campo, t, self.converter_trecho(s[m.end() + atr.end():fim].strip(), xs, so_acessos))
                if novo:
                    # the value was converted with it: the reads already edited inside this span go
                    edicoes[:] = [e for e in edicoes if not (m.start() <= e[0] and e[1] <= fim)]
                    ocupado[:] = [o for o in ocupado if not (m.start() <= o[0] and o[1] <= fim)]
                    edicoes.append((m.start(), fim, novo))
                    ocupado.append((m.start(), fim))
                continue
            if re.match(r'\s*(\+|-|\*|/|&|\||\^|<<|>>)=', depois):
                continue  # compound assignment: stays raw
            novo = self.leitura(x, campo, t)
            if novo:
                edicoes.append((m.start(), m.end(), novo))
                ocupado.append((m.start(), m.end()))
        # the other uses
        for m in ([] if so_acessos else re.finditer(r'(?<![\w.:])(%s)\b(?!\s*::)(?!\s*:(?!:))' % nomes, mk)):
            if any(a <= m.start() < b for a, b in ocupado):
                continue
            x = m.group(1)
            depois = mk[m.end():]
            antes = mk[:m.start()]
            if re.search(r'\blet\s+(mut\s+)?$', antes) or re.match(r'\s*\.', depois):
                continue
            z = re.match(r'\s*(==|!=)\s*0(?![\w.x])', depois)
            if z:
                novo = ('%s.is_null()' if z.group(1) == '==' else '!%s.is_null()') % x
                edicoes.append((m.start(), m.end() + z.end(), novo))
                continue
            z = re.search(r'\b0\s*(==|!=)\s*$', antes)
            if z:
                novo = ('%s.is_null()' if z.group(1) == '==' else '!%s.is_null()') % x
                edicoes.append((m.start() - (len(antes) - z.start()), m.end(), novo))
                continue
            # x == y between two variables of the same record type stays as it is
            z = re.match(r'\s*(==|!=)\s*(%s)\b(?!\s*\.)' % nomes, depois)
            if z:
                continue
            if re.search(r'(?<![\w.])(%s)\s*(==|!=)\s*$' % nomes, antes):
                continue
            edicoes.append((m.start(), m.end(), '%s.addr()' % x))
        out = s
        for a, b, novo in sorted(edicoes, reverse=True):
            out = out[:a] + novo + out[b:]
        return out

    def converter_bloco(self, bloco):
        mk = mascara(bloco)
        xs = self.variaveis(bloco, mk)
        if not xs:
            return bloco, {}
        novo = self.converter_trecho(bloco, xs)
        # the bindings (after the uses, on the new text)
        mk2 = mascara(novo)
        for x, kind in xs.items():
            if kind == 'let':
                m = re.search(r'\blet\s+%s\s*(:\s*usize\s*)?=\s*' % re.escape(x), mk2)
                if not m:
                    return bloco, {}
                fim = fim_da_expressao(mk2, m.end())
                expr = novo[m.end():fim]
                novo = novo[:m.start()] + 'let %s = %s(%s)' % (x, self.tipo, expr.strip()) + novo[fim:]
            else:
                i = mk2.find('{')
                j = novo.find('\n', i)
                ind = re.match(r'\s*', novo[j + 1:]).group(0) or '    '
                novo = novo[:j + 1] + '%slet %s = %s(%s);\n' % (ind, x, self.tipo, x) + novo[j + 1:]
            mk2 = mascara(novo)
        return novo, xs


def tirar_addr_exe(grava):
    """`exe::x(club.addr())` -> `exe::x(club)` where the wrapper's parameter is `impl Addr` (a pointer in the original)."""
    calls = open(os.path.join(SRC, 'motor', 'exe_calls.rs'), encoding='utf-8').read()
    params = {}
    for m in re.finditer(r'pub\(crate\) unsafe fn (\w+)\(([^)]*)\)', calls):
        params[m.group(1)] = ['impl Addr' in p for p in m.group(2).split(',')] if m.group(2).strip() else []
    n = 0
    arqs = [os.path.join(raiz, f) for raiz, _, fs in os.walk(SRC) for f in fs if f.endswith('.rs') and f != 'exe_calls.rs']
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    arqs += [os.path.join(pasta, f) for f in os.listdir(pasta) if f.endswith('.py')]
    for p in arqs:
        if True:
            t = open(p, encoding='utf-8').read()
            mk = t if p.endswith('.py') else mascara(t)  # the mutation lists hold the code inside strings
            edicoes = []
            for m in re.finditer(r'(?<![\w:])exe::(\w+)\(', mk):
                ps = params.get(m.group(1))
                if not ps:
                    continue
                args, fim = ie.args_da_chamada(mk, m.end() - 1)
                if not args or len(args) != len(ps):
                    continue
                pos = m.end()
                for k, a in enumerate(args):
                    ini = mk.index(a, pos) if a else pos
                    pos = ini + len(a)
                    if ps[k] and re.fullmatch(r'[a-z_][a-z0-9_]*\.addr\(\)', a):
                        edicoes.append((ini, ini + len(a), a[:-len('.addr()')]))
            if edicoes:
                n += len(edicoes)
                for a, b, novo in sorted(edicoes, reverse=True):
                    t = t[:a] + novo + t[b:]
                if grava:
                    open(p, 'w', encoding='utf-8').write(t)
    print('%d `.addr()` tirados de argumentos exe::' % n)


def assinaturas(tipo, grava):
    """Our helpers that rebind a parameter (`fn f(club: usize) { let club = Club(club); ..`) take the type instead
    (`fn f(club: Club)`); then the callers, driven by the compiler: an argument `y.addr()` becomes `y`, any other
    becomes `Club(..)`. A function used as a function pointer (a callback, a table) goes back to its old signature."""
    import json as js
    import subprocess
    trocadas = {}  # (file, function) -> (param, old head text)
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            p = os.path.join(raiz, f)
            if not f.endswith('.rs') or f in ('records.rs', 'exe_calls.rs'):
                continue
            t = open(p, encoding='utf-8').read()
            partes, pos = [], 0
            for nome, a, b in mover_rotinas.blocos(t):
                bloco = t[a:b]
                m = re.search(r'^((?:pub\(crate\) )?unsafe fn \w+\()([^)]*)\)([^{]*)\{\n([ \t]*)let (\w+) = %s\(\5\);\n' % tipo, bloco, re.M)
                if not m or 'extern' in bloco[:m.start()].split('\n')[-1]:
                    continue
                x = m.group(5)
                ps = m.group(2)
                ps2 = re.sub(r'\b%s\s*:\s*usize\b' % re.escape(x), '%s: %s' % (x, tipo), ps, count=1)
                if ps2 == ps:
                    continue
                novo = bloco[:m.start()] + m.group(1) + ps2 + ')' + m.group(3) + '{\n' + bloco[m.end():]
                trocadas[(p, nome)] = x
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and partes != [t]:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d funções passam a receber %s' % (len(trocadas), tipo))
    if not grava:
        return
    ajustar_pelo_compilador([tipo])


def ajustar_pelo_compilador(tipos):
    """Builds and fixes, until it compiles, the calls the type changes broke: `expected Club, found usize` (an argument
    `y.addr()` becomes `y`, any other `Club(..)`) and `expected usize, found Club` (`x` becomes `x.addr()`)."""
    import json as js
    import subprocess
    for volta in range(40):
        r = subprocess.run(['cargo', 'build', '--release', '--message-format=json'], cwd=os.path.join(R, 'rust'),
                           capture_output=True, text=True, encoding='utf-8', errors='replace')
        erros = []
        for l in r.stdout.splitlines():
            try:
                d = js.loads(l)
            except ValueError:
                continue
            if d.get('reason') == 'compiler-message' and d['message']['level'] == 'error':
                erros.append(d['message'])
        if not erros:
            print('compila (volta %d)' % volta)
            return True
        edicoes = collections.defaultdict(list)
        for e in erros:
            # the span to fix is the one whose label says the mismatch (for an argument the primary span can be the
            # called function's name)
            sp = next((x for x in e['spans'] if 'expected `' in (x.get('label') or '') and 'found' in (x.get('label') or '')), None) \
                or next((x for x in e['spans'] if x['is_primary']), None)
            if not sp:
                continue
            arq = os.path.normpath(os.path.join(R, 'rust', sp['file_name']))
            txt = e['message'] + ' ' + (sp.get('label') or '')
            feito = False
            for tipo in tipos:
                if ('expected `%s`, found `usize`' % tipo) in txt or ('expected `%s`, found integer' % tipo) in txt:
                    edicoes[arq].append((sp['byte_start'], sp['byte_end'], 'para_tipo', tipo))
                    feito = True
                    break
                if ('expected `usize`, found `%s`' % tipo) in txt:
                    edicoes[arq].append((sp['byte_start'], sp['byte_end'], 'para_usize', tipo))
                    feito = True
                    break
            if not feito:
                print('erro não tratado:', sp['file_name'], sp['line_start'], e['message'][:140])
        n = 0
        for arq, spans in edicoes.items():
            b = open(arq, 'rb').read()
            for a_, b_, kind, tipo in sorted(set(spans), reverse=True):
                x = b[a_:b_].decode('utf-8')
                if kind == 'para_tipo':
                    novo = x[:-len('.addr()')] if re.fullmatch(r'[a-z_][a-z0-9_]*\.addr\(\)', x) else '%s(%s)' % (tipo, x)
                else:
                    novo = '%s.addr()' % x if re.fullmatch(r'[a-z_][a-z0-9_]*', x) else '(%s).addr()' % x
                b = b[:a_] + novo.encode('utf-8') + b[b_:]
                n += 1
            open(arq, 'wb').write(b)
        print('volta %d: %d erros, %d ajustes' % (volta, len(erros), n))
        if n == 0:
            print('parou: erros que a ferramenta não trata')
            return False
    return False


def propagar(tipos, grava):
    """Our functions that take a `usize` parameter that most callers fill with a typed record (`f(club.addr())`) take
    the type, even when they only pass it on; the body is converted like a typed variable and the callers are fixed by
    the compiler. Repeats until no function is left (each round types new calls). Functions used as values
    (callbacks, tables), `extern` ones and reassigned parameters stay."""
    mods = {}
    t_rec = open(os.path.join(SRC, 'motor', 'records.rs'), encoding='utf-8').read()
    for m in re.finditer(r'/// A `(\w+)` record in the game.*\n(?:.*\n){0,3}pub\(crate\) struct (\w+)\(', t_rec):
        if m.group(2) in tipos:
            mods[m.group(2)] = m.group(1)
    conv = {t: Conversor(m) for t, m in mods.items()}
    nomes_t = '|'.join(tipos)
    for rodada in range(10):
        fontes = {}
        for raiz, _, fs in os.walk(SRC):
            for f in fs:
                if f.endswith('.rs') and f not in ('records.rs', 'exe_calls.rs', 'mem.rs', 'fields.rs'):
                    q = os.path.join(raiz, f)
                    fontes[q] = open(q, encoding='utf-8').read()
        tudo = '\n'.join(mascara(t) for t in fontes.values())
        defs = {}
        for q, t in fontes.items():
            for nome, a, b in mover_rotinas.blocos(t):
                mk = mascara(t[a:b])
                m = re.search(r'^(?:pub\(crate\) )?unsafe fn (\w+)\(', mk, re.M)
                if not m or m.group(1) != nome:
                    continue
                args, fim = ie.args_da_chamada(mk, m.end() - 1)
                defs[nome] = (q, a, b, [x.split(':')[0].strip() for x in (args or [])], [x.split(':', 1)[1].strip() if ':' in x else '' for x in (args or [])])
        # used as a value somewhere (not called): stays
        # one pass: every identifier that appears without a call (and not as `fn name`)
        soltos = {m.group(1) for m in re.finditer(r'(?<![\w.:])(?<!fn )([a-z_][a-z0-9_]*)\b(?!\s*\()(?!\s*::)', tudo)}
        valor = {n for n in defs if n in soltos}
        votos = collections.defaultdict(lambda: collections.Counter())
        for q, t in fontes.items():
            for nome, a, b in mover_rotinas.blocos(t):
                mk = mascara(t[a:b])
                tipadas = {}
                for m in re.finditer(r'(?<![\w.])([a-z_][a-z0-9_]*)\s*:\s*(%s)\b' % nomes_t, mk):
                    tipadas[m.group(1)] = m.group(2)
                for m in re.finditer(r'\blet\s+([a-z_][a-z0-9_]*)\s*=\s*(%s)\(' % nomes_t, mk):
                    tipadas[m.group(1)] = m.group(2)
                for m in re.finditer(r'(?<![\w.:])(\w+)\(', mk):
                    if m.group(1) not in defs or m.group(1) in valor:
                        continue
                    args, _ = ie.args_da_chamada(mk, m.end() - 1)
                    for k, x in enumerate(args or []):
                        mm = re.fullmatch(r'\s*([a-z_][a-z0-9_]*)\.addr\(\)\s*', x)
                        votos[(m.group(1), k)][tipadas.get(mm.group(1), '-') if mm else '-'] += 1
        cand = {}
        for (nome, k), c in votos.items():
            q, a, b, ps, ts = defs[nome]
            if k >= len(ps) or ts[k] != 'usize':
                continue
            top, n = max(((t, v) for t, v in c.items() if t != '-'), key=lambda tv: tv[1], default=('-', 0))
            if top == '-' or n * 2 < sum(c.values()):
                continue
            x = ps[k]
            corpo = mascara(fontes[q][a:b])
            if re.search(r'(?<![\w.])%s\s*(\+|-|\*|/|&|\||\^)?=(?!=)' % re.escape(x), corpo) or re.search(r'\blet\s+(mut\s+)?%s\b' % re.escape(x), corpo):
                continue
            cand.setdefault(nome, []).append((k, x, top))
        print('rodada %d: %d funções, %d parâmetros' % (rodada, len(cand), sum(len(v) for v in cand.values())))
        if not cand or not grava:
            return
        por_arq = collections.defaultdict(list)
        for nome, ks in cand.items():
            por_arq[defs[nome][0]].append((nome, ks))
        for q, lista in por_arq.items():
            t = fontes[q]
            for nome, ks in sorted(lista, key=lambda v: -defs[v[0]][1]):
                _, a, b, ps, ts = defs[nome]
                bloco = t[a:b]
                mk = mascara(bloco)
                i = mk.index('fn %s(' % nome) + len('fn %s' % nome)
                args, fim = ie.args_da_chamada(mk, i)
                cab, corpo = bloco[:fim], bloco[fim:]
                for k, x, tipo in ks:
                    cab = re.sub(r'(?<![\w.])%s\s*:\s*usize\b' % re.escape(x), '%s: %s' % (x, tipo), cab, count=1)
                    corpo = conv[tipo].converter_trecho(corpo, {x})
                t = t[:a] + cab + corpo + t[b:]
            open(q, 'w', encoding='utf-8').write(t)
        if not ajustar_pelo_compilador(list(tipos)):
            return
        tirar_addr_exe(True)


def acessos(mod, grava):
    """The accesses `read_T(x.addr() + mod::F)` of variables already typed become methods (a narrower read, a sign
    cast, `read_i8_as_i32`); the mutation snippets follow."""
    cv = Conversor(mod)
    mutacoes = carregar_mutacoes()
    trocas_mut, n = {}, 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            p = os.path.join(raiz, f)
            if not f.endswith('.rs') or f in ('records.rs', 'fields.rs', 'exe_calls.rs', 'mem.rs'):
                continue
            t = open(p, encoding='utf-8').read()
            tl.ANTES[p] = t
            partes, pos = [], 0
            for _, a, b in mover_rotinas.blocos(t):
                bloco = t[a:b]
                xs = set(re.findall(r'(?<![\w.])([a-z_][a-z0-9_]*)\.addr\(\)\s*\+\s*%s::[A-Z]' % mod, mascara(bloco)))
                if not xs:
                    continue
                novo = cv.converter_trecho(bloco, xs, True)
                if novo == bloco:
                    continue
                n += 1
                for lista, mu in mutacoes:
                    if bloco.count(mu[1]) == 1:
                        trocas_mut[(lista, mu[0])] = (mu, (lambda s, xs=xs: cv.converter_trecho(s, xs, True)))
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and ''.join(partes) != t:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d funções com acessos convertidos (%s)' % (n, mod))
    if grava:
        tl.voltar_funcoes_com_erro(trocas_mut)
        gravar_mutacoes(trocas_mut)


def carregar_mutacoes():
    mutacoes = []
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in sorted(os.listdir(pasta)):
        if f.endswith('.py'):
            cwd = os.getcwd()
            try:
                os.chdir(os.path.join(R, 'engenharia-reversa'))
                mutacoes += [(f, m) for m in runpy.run_path(os.path.join(pasta, f)).get('MUTACOES', [])]
            except Exception:
                pass
            finally:
                os.chdir(cwd)
    return mutacoes


def gravar_mutacoes(trocas_mut):
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    por = collections.defaultdict(list)
    for (lista, _), v in trocas_mut.items():
        por[lista].append(v)
    for lista, ms in por.items():
        pl = os.path.join(pasta, lista)
        s = open(pl, encoding='utf-8').read()
        for mu, g in ms:
            ini = max(s.find("('%s'," % mu[0]), s.find('("%s",' % mu[0]))
            if ini < 0:
                continue
            pos = ini
            for k in (1, 2):
                nv = g(mu[k])
                if nv == mu[k]:
                    continue
                for v, n2 in ((repr(mu[k]), repr(nv)), (json.dumps(mu[k], ensure_ascii=False), json.dumps(nv, ensure_ascii=False))):
                    j = s.find(v, pos)
                    if j >= 0:
                        s = s[:j] + n2 + s[j + len(v):]
                        pos = j + len(n2)
                        break
        open(pl, 'w', encoding='utf-8').write(s)


def main():
    if '--propagar' in sys.argv:
        return propagar(sys.argv[sys.argv.index('--propagar') + 1].split(','), '--gravar' in sys.argv)
    if '--acessos' in sys.argv:
        return acessos(sys.argv[sys.argv.index('--acessos') + 1], '--gravar' in sys.argv)
    if '--assinaturas' in sys.argv:
        return assinaturas(sys.argv[sys.argv.index('--assinaturas') + 1], '--gravar' in sys.argv)
    if '--exe' in sys.argv:
        return tirar_addr_exe('--gravar' in sys.argv)
    mod = sys.argv[1]
    grava = '--gravar' in sys.argv
    cv = Conversor(mod)
    print('registro %s -> %s, %d campos' % (mod, cv.tipo, len(cv.campos)))
    mutacoes = []
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in sorted(os.listdir(pasta)):
        if f.endswith('.py'):
            try:
                cwd = os.getcwd()
                os.chdir(os.path.join(R, 'engenharia-reversa'))
                mutacoes += [(f, m) for m in runpy.run_path(os.path.join(pasta, f)).get('MUTACOES', [])]
            except Exception:
                pass
            finally:
                os.chdir(cwd)
    trocas_mut, n_fn, n_var = {}, 0, 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            p = os.path.join(raiz, f)
            if not f.endswith('.rs') or f in ('records.rs', 'fields.rs', 'exe_calls.rs', 'mem.rs'):
                continue
            t = open(p, encoding='utf-8').read()
            tl.ANTES[p] = t
            partes, pos = [], 0
            for _, a, b in mover_rotinas.blocos(t):
                bloco = t[a:b]
                novo, xs = cv.converter_bloco(bloco)
                if novo == bloco:
                    continue
                n_fn += 1
                n_var += len(xs)
                for lista, mu in mutacoes:
                    if bloco.count(mu[1]) == 1:
                        trocas_mut[(lista, mu[0])] = (mu, (lambda s, xs=xs: cv.converter_trecho(s, xs)))
                partes += [t[pos:a], novo]
                pos = b
            partes.append(t[pos:])
            if grava and ''.join(partes) != t:
                open(p, 'w', encoding='utf-8').write(''.join(partes))
    print('%d funções, %d variáveis; %d trechos de mutação' % (n_fn, n_var, len(trocas_mut)))
    if not grava:
        return
    tl.voltar_funcoes_com_erro(trocas_mut)
    por = collections.defaultdict(list)
    for (lista, _), v in trocas_mut.items():
        por[lista].append(v)
    for lista, ms in por.items():
        pl = os.path.join(pasta, lista)
        s = open(pl, encoding='utf-8').read()
        for mu, g in ms:
            ini = max(s.find("('%s'," % mu[0]), s.find('("%s",' % mu[0]))
            if ini < 0:
                continue
            pos = ini
            for k in (1, 2):
                nv = g(mu[k])
                if nv == mu[k]:
                    continue
                for v, n2 in ((repr(mu[k]), repr(nv)), (json.dumps(mu[k], ensure_ascii=False), json.dumps(nv, ensure_ascii=False))):
                    j = s.find(v, pos)
                    if j >= 0:
                        s = s[:j] + n2 + s[j + len(v):]
                        pos = j + len(n2)
                        break
        open(pl, 'w', encoding='utf-8').write(s)
    print('gravado')


if __name__ == '__main__':
    main()
