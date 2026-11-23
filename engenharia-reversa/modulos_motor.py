r"""Troca os include! do motor.rs por módulos de verdade (etapa 4 da reorganização, 07/11/2026).

Cada arquivo de rust/src/motor/ vira um módulo (`#[path] mod comp_leagues_eng_prm;`), com `use super::*;` no topo; o
motor.rs re-exporta todos (`pub(crate) use self::<m>::*;`), para os caminhos crate::motor::X e os irmãos continuarem
enxergando tudo como antes. Os itens de topo e os campos das structs ganham pub(crate) (irmão não enxerga privado). As
macros continuam por include! antes dos módulos (escopo textual).

Uso: python modulos_motor.py --gravar
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
FICA = {'macros.rs'}  # por include! (escopo textual)
ITEM = re.compile(r'^(?!pub\b)(?!use super::)((?:unsafe |extern "\w+" |const |async )*(?:fn|static|const|struct|enum|type|union|trait|use|mod)\b)', re.M)


def nome_mod(rel):
    return re.sub(r'\W', '_', rel[:-3].replace('/', '_'))


def publicar(t):
    t = ITEM.sub(r'pub(crate) \1', t)
    # campos das structs com chaves (linhas de 4 espaços `nome: Tipo,` dentro de `struct X {`)
    def campos(m):
        return re.sub(r'^(    )(?!pub\b)(\w+\s*:)', r'\1pub(crate) \2', m.group(0), flags=re.M)
    t = re.sub(r'^pub\(crate\) struct \w+(?:<[^>]*>)? \{\n.*?^\}', campos, t, flags=re.M | re.S)
    # the methods of an inherent impl (not a trait's: those take no visibility), 23/11/2026: an impl moved out of
    # motor.rs is used by sibling modules
    def metodos(m):
        return re.sub(r'^(    )(?!pub\b)((?:const |unsafe |extern "\w+" )*fn )', r'\1pub(crate) \2', m.group(0), flags=re.M)
    t = re.sub(r'^(?:pub\(crate\) )?(?:unsafe )?impl\b(?![^{\n]* for )[^{\n]*\{\n.*?^\}', metodos, t, flags=re.M | re.S)
    # structs de tupla: `struct X(A, B);`
    def tupla(m):
        dentro = ', '.join(x if x.strip().startswith('pub') else 'pub(crate) ' + x.strip() for x in m.group(2).split(','))
        return '%s(%s);' % (m.group(1), dentro)
    t = re.sub(r'^(pub\(crate\) struct \w+)\(([^()]*)\);', tupla, t, flags=re.M)
    return t


def main():
    base = os.path.join(SRC, 'motor')
    mods = []
    for d, _, fs in os.walk(base):
        for f in sorted(fs):
            rel = os.path.relpath(os.path.join(d, f), base).replace('\\', '/')
            if f.endswith('.rs') and rel not in FICA:
                mods.append(rel)
    mods.sort()
    if '--gravar' not in sys.argv:
        print(len(mods), 'módulos')
        return
    for rel in mods:
        p = os.path.join(base, rel)
        t = open(p, encoding='utf-8').read()
        # de novo (código novo acrescentado pelo partir_motor.py): publicar é idempotente
        novo = publicar(t if t.startswith('use super::*;') else 'use super::*;\n\n' + t)
        if novo != t:
            open(p, 'w', encoding='utf-8').write(novo)
    m = os.path.join(SRC, 'motor.rs')
    s = open(m, encoding='utf-8').read()
    # a source file whose name is already an inline name module (fields.rs's `mod display { .. }`, the field names of
    # the Display object) gets the _cpp suffix: the file module would shadow it (23/11/2026, display.cpp and guio.cpp)
    inline = set(re.findall(r'^(?:pub\(crate\) )?mod (\w+) \{', s, re.M))
    for rel in mods:
        inline |= set(re.findall(r'^(?:pub\(crate\) )?mod (\w+) \{', open(os.path.join(base, rel), encoding='utf-8').read(), re.M))
    nome = {rel: nome_mod(rel) + ('_cpp' if nome_mod(rel) in inline else '') for rel in mods}
    decl = ''.join('#[path = "motor/%s"]\npub(crate) mod %s;\npub(crate) use self::%s::*;\n' % (rel, nome[rel], nome[rel]) for rel in mods)
    s2 = re.sub(r'^include!\("motor/(?!macros\.rs)[^"]+"\);\n', '', s, flags=re.M)
    # de novo: as declarações antigas saem (a lista é refeita pelos arquivos que existem)
    # (the use line may be missing: cargo fix drops an unused re-export, 23/11/2026)
    s2 = re.sub(r'^#\[path = "motor/[^"]+"\]\npub\(crate\) mod \w+;\n(?:pub\(crate\) use self::\w+::\*;\n)?', '', s2, flags=re.M)
    s2 = s2.replace('include!("motor/macros.rs");\n', 'include!("motor/macros.rs");\n' + decl, 1)
    open(m, 'w', encoding='utf-8').write(s2)
    print('gravado: %d módulos' % len(mods))


if __name__ == '__main__':
    main()
