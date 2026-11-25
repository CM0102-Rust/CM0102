# O construtor da competição reserva (record_utils.cpp; leitor rx11, rodada 6, 25/11/2026). Simulação, carreira ENG
# (sem LIGAS): roda na criação do jogo e na carga (o setup, 0x82959a e 0x82c6dc). A espera do CD pela opção (a) do
# Lucas, o find_game_cd (sx02b_1, já no HEAD): o patch sem CD do cm0102-1080 e do portátil troca 3 bytes
# (0x7e603c/0x7e603d e 0x7e607f); a rotina é desviada nos dois exes pelos dois hashes. Fora esses bytes, igual no
# cm0102-1080.exe. Antes ficava de fora (sx18_28: "desligada pelo patch_portatil.py").
# - 0x7e5fc0 ReserveComp::ReserveComp thiscall(a1, não lido) ret 4 -> this (quadro SEH só para o destrutor da base):
#   Comp::Comp 0x48c580, a vtable 0x96ef54, +0xa7 = 0 e +0xa3 = 0 (as competições sub-21 e B), a busca do CD
#   (GetLogicalDriveStringsA, GetDriveTypeA = 5, "<raiz>cm0102.exe" por 0x4c0aa0/0x4c0c10, a caixa "Insert CD" e de
#   novo), +4 = o registro de [KEY_CLUB_COMP_RESERVE] (x 0x6b). Prólogo push -1; push imm32 (7 bytes).
# Comparador: só a original (monta o objeto e pode esperar o CD), com o plano: a vtable, o registro e as duas vagas
# nulas.
SAVE = False
ROTINAS = [('construct_reserve_comp', 0x7e5fc0, [0x6a, 0xff, 0x68, 0xfb, 0x06, 0x96, 0x00])]
DOCS = ('o construtor da competição reserva',
        "the reserve competition's constructor",
        [('construct_reserve_comp', 'ReserveComp::ReserveComp', '0x7e5fc0', 'record_utils.cpp', 'thiscall, ret 4',
          'regra: a competição reserva é uma competição própria, o registro reserva da tabela das competições, ainda sem as competições B e sub-21; antes, pede o CD do jogo',
          'rule: the reserve competition is a competition of its own, the reserve record of the competitions table, with no B or Under 21 sub-competition yet; building it asks for the game CD first')])
CAMPOS = []
INTERESSA = []
