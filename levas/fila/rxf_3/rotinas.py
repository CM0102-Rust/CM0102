# The award base class's destructors (award_manager.cpp; reader rxf, round 6, 25/11/2026). Simulation (they run when
# the game frees its data: in the career's creation, after the .qst is written, and before a load).
# Proven in the disassembly:
# - 0x412ef0 AWARD::~AWARD(): thiscall, ret 0: `mov dword [ecx], 0x967864; ret` (7 bytes; the prologue is the 6-byte
#   mov, no relative call). Not a thunk nor an empty body: it writes the vtable. Called or jumped to by the derived
#   award destructors (0x92ffcd call; 0x766f46, 0x8af656 jmp; and the unwind funclets 0x95db83, 0x9637a3, 0x966073,
#   0x966093).
# - 0x412ed0 AWARD scalar deleting destructor(flags): thiscall, ret 4, returns this; slot 0 of the vtable 0x967864 (its
#   only reference). Prologue `mov al, [esp+4]; push esi` (5 bytes, no relative call). Only an object whose dynamic
#   class is the base AWARD reaches it: count 0 expected in the careers (kept because it is a real routine).
# Same bytes in cm0102-1080. Comparers: side by side over the vtable dword (the scalar one with the delete bit: only the
# original).
SAVE = False
ROTINAS = [('award_destruct', 0x412ef0, [0xc7, 0x01, 0x64, 0x78, 0x96, 0x00]),
           ('award_scalar_destruct', 0x412ed0, [0x8a, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('os destrutores da classe base dos prêmios',
        'the award base class destructors',
        [('award_destruct', 'AWARD::~AWARD', '0x412ef0', 'award_manager.cpp', 'thiscall',
          'devolve ao objeto a tabela virtual da classe base dos prêmios; nada mais a liberar (é o fim dos destrutores de cada classe de prêmio)',
          "puts the award base class's vtable back on the object; nothing else to free (it is the tail of each award class's destructor)"),
         ('award_scalar_destruct', 'AWARD::scalar_deleting_destructor', '0x412ed0', 'award_manager.cpp', 'thiscall, ret 4',
          'o destrutor da base e, com o bit 0, a liberação do objeto; só um objeto da classe base pura chega aqui (cada classe derivada tem o seu)',
          'the base destructor and, with bit 0, freeing the object; only an object of the plain base class gets here (every derived class has its own)')])
