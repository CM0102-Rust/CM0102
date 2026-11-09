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
            if x in ('self', 'this'):
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

    def converter_trecho(self, s, xs):
        """The accesses and the other uses of the variables xs in the text s (a function or a mutation snippet)."""
        mk = mascara(s)
        edicoes = []
        ocupado = []
        nomes = '|'.join(re.escape(x) for x in sorted(xs, key=len, reverse=True))
        if not nomes:
            return s
        # read_T(x + mod::F) / write_T(x + mod::F, v)
        for m in re.finditer(r'\b(read|write)_(u8|i8|u16|i16|u32|i32|usize|f32|f64|i64|u64)\(', mk):
            args, fim = ie.args_da_chamada(mk, m.end() - 1)
            if not args:
                continue
            a0 = re.fullmatch(r'\s*(%s)\s*\+\s*%s::([A-Z][A-Z0-9_]*)\s*' % (nomes, self.mod), args[0])
            if not a0:
                continue
            x, campo, t = a0.group(1), a0.group(2), m.group(2)
            if m.group(1) == 'read' and len(args) == 1:
                novo = self.leitura(x, campo, t)
            elif m.group(1) == 'write' and len(args) == 2:
                # the value text from the real string (it may hold a comment or a literal)
                ini_v = m.end() + mk[m.end():fim].index(',') + 1
                novo = self.escrita(x, campo, t, self.converter_trecho(s[ini_v:fim - 1].strip(), xs))
            else:
                novo = None
            if novo and not any(a <= m.start() < b for a, b in ocupado):
                edicoes.append((m.start(), fim, novo))
                ocupado.append((m.start(), fim))
        # *((x + mod::F) as *const T)  and  *((x + mod::F) as *mut T) = v;
        for m in re.finditer(r'\*\(\(\s*(%s)\s*\+\s*%s::([A-Z][A-Z0-9_]*)\s*\)\s*as\s*\*(const|mut)\s+(\w+)\)' % (nomes, self.mod), mk):
            if any(a <= m.start() < b for a, b in ocupado):
                continue
            x, campo, kind, t = m.group(1), m.group(2), m.group(3), m.group(4)
            depois = mk[m.end():]
            atr = re.match(r'\s*=(?!=)\s*', depois)
            if atr:
                fim = fim_da_expressao(mk, m.end() + atr.end())
                if fim < 0:
                    continue
                novo = self.escrita(x, campo, t, self.converter_trecho(s[m.end() + atr.end():fim].strip(), xs))
                if novo:
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
        for m in re.finditer(r'(?<![\w.:])(%s)\b(?!\s*::)(?!\s*:(?!:))' % nomes, mk):
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


def main():
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
