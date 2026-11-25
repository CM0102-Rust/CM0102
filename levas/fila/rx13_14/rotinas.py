# O destrutor escalar do Campeonato de Clubes da Oceania (oceania_club_champ.cpp; leitor rx13, rodada 6, 25/11/2026).
# Item de TELAS: roda quando o jogo é desmontado (carregar outro save com um jogo aberto, ou sair da carreira), pela
# vtable 0x96e018 (slot 0). Caminho no jogo para o Lucas: com uma carreira aberta que tenha a Austrália (a Oceania
# carregada), carregar outro save, jogando comparando.
# - 0x797150 OceaniaClubChamp::scalar_deleting_destructor thiscall(flags) ret 4: push esi; mov esi, ecx;
#   call ~OceaniaClubChamp 0x797170 (thiscall, ret 0); test byte [esp+8], 1: operator delete 0x944c94 (cdecl);
#   mov eax, esi. O prólogo tem 3 bytes antes do call rel32: entra pelas RELATIVAS com (0x797150, 3)
#   (INTEGRAR_ANTES.txt), como os do cx03_4. Usa o delete_through do cx03_3: integrar depois dele.
# Igual no cm0102-1080.exe. Comparador: só a original (libera memória).
SAVE = False
TELAS = True
ROTINAS = [('delete_oceania_club_champ', 0x797150, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('o destrutor escalar do Campeonato de Clubes da Oceania',
        "the Oceania Club Championship's scalar deleting destructor",
        [('delete_oceania_club_champ', 'OceaniaClubChamp::scalar_deleting_destructor', '0x797150', 'oceania_club_champ.cpp', 'thiscall, ret 4',
          'destrói o Campeonato de Clubes da Oceania (o destrutor da classe) e, com o bit 0 das marcas, libera o objeto',
          'destroys the Oceania Club Championship (the class destructor) and, with bit 0 of the flags, frees the object')])
CAMPOS = []
INTERESSA = []
