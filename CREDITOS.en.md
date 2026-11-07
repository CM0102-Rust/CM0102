*English version of [CREDITOS.md](CREDITOS.md). Both are kept in sync; the Portuguese file is the original.*

# Credits

This project rewrites Championship Manager 01/02 (3.9.68) in Rust, routine by routine. It stands on the work of many people in the community.

## maxevans60's function map

The [cm0102-function-map](https://github.com/maxevans60/cm0102-function-map), by **maxevans60**, is distributed under the [Creative Commons Attribution 4.0 (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/) licence, with extra permission for use with AI.

The map gives a name, source file, calling convention, purpose and confidence to the 6,863 functions of `cm0102.exe` 3.9.68, and a name, type and readers/writers to its 4,428 globals. It was made for the "Bosman Edition", a recompiled port of the game.

Here, the map:
- is read by the tools in `engenharia-reversa/` (the map itself stays outside the repository, in `referencias/`);
- is the source, with changes (our coverage measurements and our names), of [`docs/mapa_funcoes.csv`](docs/mapa_funcoes.csv), [`docs/PLACAR_ARQUIVOS.en.md`](docs/PLACAR_ARQUIVOS.en.md) and [`docs/MAPA_COMPETICOES.en.md`](docs/MAPA_COMPETICOES.en.md);
- gives the routines' original names quoted in the code and the documentation.

Thanks to maxevans60, and to those he credits: John Locke, Tapani, Saturn and the Offset Index topic.

## Forum and community patches

The addresses, patches and old forum topics that served as leads are listed, with their authors, in [`docs/ENDERECOS_COMUNIDADE.en.md`](docs/ENDERECOS_COMUNIDADE.en.md). Thanks to all of them and to whoever keeps the archive of the old topics at nic.opik.net.

## The game

Championship Manager 01/02 belongs to Sports Interactive and Eidos. The game was released for free in 2008. This repository contains nothing from the game (neither the executable nor the data); to play, you need the original installation.
