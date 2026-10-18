"""Lacuna da 158a leva em docs/PENDENCIAS (pt/en), as das 154a-157a exercitadas no jogo do Lucas em
17/10/2026 e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = ("| `conteiner` | 0x9326e0, 0x933080, 0x933170 (158ª) | — | o contêiner do save só roda ao salvar, ao carregar e no início rápido (a carreira automática não salva; as 4 mutações escapam). Provado à parte: `save_grande.py` (`.qst` de 2,38 GB, original × Rust) e no jogo do Lucas (17/10/2026): save de 2,55 GB gravado e carregado com a DLL; com as originais o carregamento falha (`Friendly.cpp`) |",
      {'eventos gravados': "sim (17/10/2026, `COMPARAR_800.bat`: contador \"eventos lidos\" ao abrir o relatório da partida, 0 divergências)",
       'passo em rede': "sim (17/10/2026, `JOGAR.bat` e `JOGAR_COMPARANDO.bat`)",
       'texto e partida em rede': "sim (17/10/2026: \"partida em rede confirmada\" no `JOGAR.bat`; \"texto do evento\" só no relatório da partida, comparado no `COMPARAR_800.bat` com 0 divergências)",
       'narracao no painel': "sim (17/10/2026, `JOGAR.bat`: contadores \"narracao no painel\" e \"sons dos eventos\"; cores, quebra de linha e sons normais na tela)"})
EN = ("| `conteiner` | 0x9326e0, 0x933080, 0x933170 (batch 158) | — | the save container only runs when saving, loading and in the quick start (the automatic career does not save; the 4 mutations escape). Proven separately: `save_grande.py` (2.38 GB `.qst`, original × Rust) and in Lucas's game (2026-10-17): a 2.55 GB save saved and loaded with the DLL; with the originals loading fails (`Friendly.cpp`) |",
      {'eventos gravados': "yes (2026-10-17, `COMPARAR_800.bat`: counter \"eventos lidos\" when opening the match report, 0 divergences)",
       'passo em rede': "yes (2026-10-17, `JOGAR.bat` and `JOGAR_COMPARANDO.bat`)",
       'texto e partida em rede': "yes (2026-10-17: \"partida em rede confirmada\" in `JOGAR.bat`; \"texto do evento\" only in the match report, compared in `COMPARAR_800.bat` with 0 divergences)",
       'narracao no painel': "yes (2026-10-17, `JOGAR.bat`: counters \"narracao no painel\" and \"sons dos eventos\"; colours, line wrap and sounds normal on screen)"})
for arq, (nova, cobertos) in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    linhas = open(p, encoding='utf-8').read().split('\n')
    for i, l in enumerate(linhas):
        for nome, txt in cobertos.items():
            pre = f'| `{nome}` | '
            if l.startswith(pre):
                c = l.split(' | ')
                assert c[2] == '—', l
                c[2] = txt
                linhas[i] = ' | '.join(c)
    k = [i for i, l in enumerate(linhas) if l.startswith('| `narracao no painel`')][0]
    linhas.insert(k + 1, nova)
    open(p, 'w', encoding='utf-8').write('\n'.join(linhas))
for arq, a, b in (('index.md', '(50 hoje)', '(51 hoje)'), ('index.en.md', '(50 today)', '(51 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
