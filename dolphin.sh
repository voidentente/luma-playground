#!/bin/bash
QT_QPA_PLATFORM=xcb dolphin-emu \
    -C Dolphin.Display.Fullscreen=true \
    -be $1
