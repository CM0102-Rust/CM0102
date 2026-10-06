@echo off
rem Abre o CM 01/02 em janela, sem CD e sem piscar a tela (versao GDI portatil).
rem Para tela cheia, tire o -windowed (a tela vai piscar: o jogo troca o video para 800x600 em 16 bits).
start "" /D "%~dp0jogo" "%~dp0jogo\cm0102_GDI-portatil.exe" -windowed
