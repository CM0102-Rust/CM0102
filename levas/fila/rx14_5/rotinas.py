# O destrutor escalar da fase de mata-mata da copa (cup_stage.cpp; leitor rx14, rodada 6, 25/11/2026). Item de
# simulacao: as fases de mata-mata sao apagadas na carreira (o Cup_Stage::~Cup_Stage 0x523300 tem 23 chamadas ate
# 2002-06-01, pelo slot 0 da vtable 0x969468, que e esta rotina).
# Os destrutores escalares (push esi; mov esi, ecx; call ~X; test byte [esp+8], 1; operator delete 0x944c94;
# mov eax, esi; ret 4): o prologo tem 3 bytes antes de um call rel32 e entra pelas RELATIVAS (endereco, 3) do
# rust/src/detours.rs, ver INTEGRAR_ANTES.txt. O destrutor da classe e chamado pelo desvio (delete_through, do
# cx03_3: integrar depois dele). Convencoes conferidas: thiscall(flags) ret 4 -> this; o destrutor da classe
# thiscall, ret 0; operator delete cdecl. Iguais no cm0102-1080.exe (todas inteiras).
# Comparador: so a original (libera memoria); as mutacoes vazam memoria no alocador da simulacao (os enderecos das
# alocacoes seguintes mudam, o que a impressao diaria do banco acusa).
SAVE = False
TELAS = False
ROTINAS = [('delete_cup_stage', 0x5230c0, [0x56, 0x8b, 0xf1, 0xe8, 0x38, 0x02, 0x00, 0x00])]
DOCS = ('o destrutor escalar da fase de mata-mata da copa',
        'the cup stage scalar deleting destructor',
        [('delete_cup_stage', 'Cup_Stage::scalar_deleting_destructor', '0x5230c0', 'cup_stage.cpp', 'thiscall, ret 4',
          'roda o destrutor da fase de mata-mata de uma copa e, com o bit 0, apaga o objeto',
          'runs the destructor of a cup knockout stage and, with bit 0, deletes the object')])
CAMPOS = []
INTERESSA = []
