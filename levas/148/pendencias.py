"""Lacunas da 148a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("prejogo ceu com chuva", "0x6b7f20 (148ª)", "—", "com a marca de chuva (+0x4d & 0x2000) e o céu fora de 0/2/4, o limite +0xf638 do laço vem de sorteio(25) + 15 ou sorteio(30) + 50; na carreira automática até 2002-06-01 o limite nunca pesou (a mutação que o fixa em 1 não muda nada)"),
      ("prejogo codigo da insatisfacao", "0x6b7f20", "—", "o evento de pré-jogo 0x13 (jogador insatisfeito com o contrato) entra na lista com o código; na carreira automática quem o lê não muda o resultado (a mutação para 0x1e não muda nada)")]
EN = [("prejogo ceu com chuva", "0x6b7f20 (batch 148)", "—", "with the rain flag (+0x4d & 0x2000) and the sky outside 0/2/4, the loop's limit +0xf638 comes from draw(25) + 15 or draw(30) + 50; in the automatic career up to 2002-06-01 the limit never mattered (the mutation that fixes it at 1 changes nothing)"),
      ("prejogo codigo da insatisfacao", "0x6b7f20", "—", "the pre-match event 0x13 (player unhappy with his contract) goes into the list with the code; in the automatic career whoever reads it does not change the result (the mutation to 0x1e changes nothing)")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `chute bloqueado`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(34 hoje)', '(36 hoje)'), ('index.en.md', '(34 today)', '(36 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
