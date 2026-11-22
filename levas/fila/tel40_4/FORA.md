# tel40: o que ficou de fora (leitor tel40, 22/11/2026)

Sem rotinas.py de proposito: esta pasta so registra o que nao foi portado do grupo tel40.

- **ScreenManager::monitor 0x7fc7e0** (5,1 KB, tabela de saltos 0x7fdbcc com 13 codigos -13..-1): o laco de telas de TODO
  jogo (nao so rede). Lido na desmontagem (estado completo: inicio, rodizio dos canais, pacote de rede, pausa por tempo
  +0x26279c/+0x26279e, gancho de tecla +0x37ee, os 13 codigos de saida, fechamento com "Processing, Please Wait ...").
  Nao portado: nao roda duas vezes (so a original no comparador), entao a copia so se provaria em modo Rust, e um erro
  deixa o jogo inteiro sem telas. Merece item proprio, com a cadeia confirmando se a carreira automatica passa por ele
  (se passar, as carreiras Rust x2 provam). Notas: o ultimo codigo -5 da o retorno; -12 manda o pacote 4 sem conferir a
  sessao (defeito: com sessao nula cairia).
- **ScreenManager::run_network_client 0x7fdc00** (2,2 KB) e **Display::send_screen_section 0x562370** (7,4 KB: secoes
  0 cabecalho, 1 widgets com diferenca contra o pool de copias 0xae2918 e mascara de campos, 2 o buffer de pacotes da
  tela): so rede, sem como provar sem duas maquinas; o tamanho (e as dezenas de vagas de pilha das chamadas a
  Guio::get_fields) pede um leitor so para eles.
- **thunk_area_release 0x55f1a0 e thunk_guio_free_images 0x55f1b0**: um `jmp rel32` de 5 bytes cada; o RELATIVAS nao trata
  `jmp` e a copia so repetiria o salto. O destrutor do Display ja passa os mesmos thunks.
- **process_command_packet_epilogue_567000 e _567260**: nao sao rotinas (fragmentos dentro do process_command_packet
  0x564080..0x567a1b, sem nenhuma referencia); o process_command_packet inteiro ja e Rust (tel37_5).
