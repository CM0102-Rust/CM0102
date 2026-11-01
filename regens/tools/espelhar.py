"""Espelha regens/ do projeto CM0102 no repositório separado (padrão D:/Sistemas/cm0102-regens), como o save2gb:
- os arquivos de regens/ (sem target/, sem os LEIA e sem este script);
- src/conteiner.rs copiado de rust/src/conteiner.rs, e o #[path] tirado de src/lib.rs;
- README.md (inglês) e README.pt.md a partir dos LEIA, com os caminhos do repositório separado;
- docs/DISTRIBUICAO_DOS_REGENS(.en).md.
Não faz commit nem push. Uso: python regens/tools/espelhar.py [DESTINO]"""
import os, re, shutil, sys

AQUI = os.path.dirname(os.path.abspath(__file__))
REGENS = os.path.dirname(AQUI)
PROJETO = os.path.dirname(REGENS)
DESTINO = sys.argv[1] if len(sys.argv) > 1 else r'D:\Sistemas\cm0102-regens'


def ler(c):
    return open(c, encoding='utf-8').read()


def gravar(c, texto):
    os.makedirs(os.path.dirname(c), exist_ok=True)
    open(c, 'w', encoding='utf-8', newline='').write(texto)


def caminhos(t):
    return (t.replace('../docs/', 'docs/').replace('regens/tools/', 'tools/')
             .replace('`cd regens && cargo build --release`', '`cargo build --release`').replace('jogo/TesteTelas.sav', 'TesteTelas.sav')
             .replace('`rust/src/conteiner.rs`', '`src/conteiner.rs`'))


for raiz, pastas, arquivos in os.walk(REGENS):
    pastas[:] = [p for p in pastas if p not in ('target', '__pycache__')]
    for a in arquivos:
        if a.startswith('LEIA') or a == 'espelhar.py':
            continue
        origem = os.path.join(raiz, a)
        alvo = os.path.join(DESTINO, os.path.relpath(origem, REGENS))
        os.makedirs(os.path.dirname(alvo), exist_ok=True)
        shutil.copyfile(origem, alvo)

lib = ler(os.path.join(REGENS, 'src', 'lib.rs'))
lib, n = re.subn(r'// O save acima de 2 GB.*?\n#\[path = "\.\./\.\./rust/src/conteiner\.rs"\]\n', '// O save acima de 2 GB (o do save2gb): as três rotinas do contêiner.\n', lib, flags=re.S)
if n != 1:
    sys.exit('src/lib.rs: o #[path] do conteiner não foi achado')
gravar(os.path.join(DESTINO, 'src', 'lib.rs'), caminhos(lib))
shutil.copyfile(os.path.join(PROJETO, 'rust', 'src', 'conteiner.rs'), os.path.join(DESTINO, 'src', 'conteiner.rs'))

pt = caminhos(ler(os.path.join(REGENS, 'LEIA.md'))).replace('*English version: [LEIA.en.md](LEIA.en.md)*', '*English version: [README.md](README.md)*')
en = caminhos(ler(os.path.join(REGENS, 'LEIA.en.md'))).replace('*Versão em português: [LEIA.md](LEIA.md)*', '*Versão em português: [README.pt.md](README.pt.md)*')
gravar(os.path.join(DESTINO, 'README.pt.md'), pt)
gravar(os.path.join(DESTINO, 'README.md'), en)
for d in ('DISTRIBUICAO_DOS_REGENS.md', 'DISTRIBUICAO_DOS_REGENS.en.md'):
    shutil.copyfile(os.path.join(PROJETO, 'docs', d), os.path.join(DESTINO, 'docs', d))
if not os.path.exists(os.path.join(DESTINO, '.gitignore')):
    gravar(os.path.join(DESTINO, '.gitignore'), 'target/\n')
print('espelhado em', DESTINO)
