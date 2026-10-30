# As suspensões na troca de clube (discipline.cpp): acrescentar a suspensão (Discipline::add_ban 0x556940, 344
# chamadas na carreira até 2002-06-01: estende a velha ou ocupa uma vaga do vetor, liga as regras e o índice das
# nações), soltar as vagas que não valem no clube novo (Discipline::clear_bans_invalid_for_club 0x556270, 749, da troca
# de clube 0x8c9720) e devolver as suspensões guardadas no fim do empréstimo (Discipline::restore_held_bans 0x556570,
# 155, de 0x4d07e0). A 0x556c70 (copiar a suspensão), a 0x556d00 (soltar) e a 0x4cf5a0 (contrato) já em Rust; 0x7e82e0
# (sem criar, só lê), 0x7e8510 (ligar), 0x68d1f0 (tirar da lista, libera), malloc/realloc/qsort originais. Sem SEH,
# sem sorteio, sem leitura de lixo (o alto do eax empurrado para a 0x556d00 é o do ponteiro da entrada, reproduzido).
# Caixas "Error" pela aviso_em (capacidade no máximo, sem memória; na 0x556270 com suspensões guardadas, e segue).
# Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo real comparando.
# Comparadores: os três pelo plano (realocam, soltam suspensões e tiram itens da lista: não rodam duas vezes).
SAVE = False
ROTINAS = [('acrescentar_suspensao', 0x556940, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('tirar_suspensoes_de_fora_do_clube', 0x556270, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('restaurar_suspensoes_guardadas', 0x556570, [0x83, 0xec, 0x20, 0x53, 0x55])]
DOCS = ('as suspensões na troca de clube: acrescentar a suspensão, soltar as vagas que não valem no clube novo e devolver as suspensões guardadas no fim do empréstimo',
        'bans on a club move: adding a ban, dropping the slots that do not apply at the new club and restoring the held bans when a loan ends',
        [('acrescentar_suspensao', 'Discipline::add_ban', '0x556940', 'discipline.cpp', 'thiscall, ret 8',
          'com suspensão da pessoa numa das 4 regras, estende a velha (jogos ou dias) e marca as duas; senão copia a nova na primeira vaga livre do vetor (ou no fim, crescendo 100 até 0x7d00), liga cada regra a ela (0x7e8510) e, com a quarta regra, põe {id, nação} no índice das nações (qsort)',
          "when the person already has a ban under one of the 4 rules, extends the old one (matches or days) and flags both; otherwise copies the new ban into the vector's first free slot (or the end, growing by 100 up to 0x7d00), links each rule to it (0x7e8510) and, with the fourth rule, adds {id, nation} to the nation index (qsort)"),
         ('tirar_suspensoes_de_fora_do_clube', 'Discipline::clear_bans_invalid_for_club', '0x556270', 'discipline.cpp', 'thiscall, ret 0xc',
          'na troca de clube, libera cada vaga das 4 do registro da pessoa cujo órgão (nação +0x17 ou continente +0x1b) não vale para o clube novo: solta a suspensão, vaga e entrada livres; com suspensões guardadas, a caixa "Error"',
          'on a club move, frees each of the person\'s 4 record slots whose ruling body (nation +0x17 or continent +0x1b) does not cover the new club: releases the ban, frees the slot and the entry; with held bans, shows the "Error" box'),
         ('restaurar_suspensoes_guardadas', 'Discipline::restore_held_bans', '0x556570', 'discipline.cpp', 'thiscall, ret 4',
          'no fim do empréstimo, devolve às vagas as suspensões guardadas da pessoa (a vaga ocupada é liberada antes; o item sai da lista) e libera as vagas 0..2 não devolvidas cujo órgão não vale para o clube do contrato',
          "when a loan ends, puts the person's held bans back into their slots (freeing the occupied slot first; the item leaves the list) and frees slots 0..2 not restored whose ruling body does not cover the contract club")])
CAMPOS = [('disciplina +4/+8/+0xa: vetor das suspensões (20 bytes), capacidade e conta (palavras); ponteiro e valor; lê e grava (acrescentar_suspensao)',
           'discipline +4/+8/+0xa: ban vector (20 bytes), capacity and count (words); pointer and value; reads and writes (acrescentar_suspensao)'),
          ('suspensão +0 jogos (0xff por dias), +2 dias, +0xc..+0xf as 4 regras, +0x13 bit 0 estendida; valor; lê e grava (acrescentar_suspensao)',
           'ban +0 matches (0xff for days), +2 days, +0xc..+0xf the 4 rules, +0x13 bit 0 extended; value; reads and writes (acrescentar_suspensao)'),
          ('disciplina +0xc/+0x10/+0x12: índice das nações (8 bytes: id, nação), capacidade e conta (palavras); ponteiro e valor; lê e grava (acrescentar_suspensao)',
           'discipline +0xc/+0x10/+0x12: nation index (8 bytes: id, nation), capacity and count (words); pointer and value; reads and writes (acrescentar_suspensao)'),
          ('disciplina +0x14: registro de 29 bytes por pessoa (4 vagas de 5 bytes: regra e índice da entrada; +0x1c guardadas, com sinal); lê e grava (tirar_suspensoes_de_fora_do_clube, restaurar_suspensoes_guardadas)',
           'discipline +0x14: 29-byte record per person (4 slots of 5 bytes: rule and entry index; +0x1c held count, signed); reads and writes (tirar_suspensoes_de_fora_do_clube, restaurar_suspensoes_guardadas)'),
          ('disciplina +0x1c: lista das suspensões guardadas (item: id, +4 vaga, +5 os 5 bytes da vaga); lê e tira (restaurar_suspensoes_guardadas)',
           'discipline +0x1c: list of held bans (item: id, +4 slot, +5 the slot\'s 5 bytes); reads and removes (restaurar_suspensoes_guardadas)'),
          ('órgão disciplinar +0x17 nação, +0x1b continente (−1 sem); id; lê; entrada de 14 bytes da pessoa (+0 = −1 livre, +6 índice da suspensão); grava −1 em +0',
           'ruling body +0x17 nation, +0x1b continent (−1 none); id; reads; the person\'s 14-byte entry (+0 = −1 free, +6 ban index); writes −1 to +0'),
          ('clube +0x53 nação, nação +0x71 continente; ponteiro; lê. pessoa +0x1a nação; lê. [0xae1080] o contrato; grava (restaurar_suspensoes_guardadas)',
           'club +0x53 nation, nation +0x71 continent; pointer; reads. person +0x1a nation; reads. [0xae1080] the contract; writes (restaurar_suspensoes_guardadas)')]
INTERESSA = ['CMRTE', 'SGE']
