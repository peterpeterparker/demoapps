#!/usr/bin/env bash

APP_NAME=Papyrs APP_THEME=#C4CDFF APP_LOGO=https://daviddalbusco.com/images/portfolio/icons/papyrs-icon.png dfx build
cp .dfx/local/canisters/demoapps/demoapps.wasm ./demoapps_1.wasm

APP_NAME=DeckDeckGo APP_THEME=#3a81fe APP_DESCRIPTION="An open source web editor for presentations." APP_URL=https://deckdeckgo.com APP_LOGO=https://daviddalbusco.com/images/portfolio/icons/deckdeckgo-icon.png dfx build
cp .dfx/local/canisters/demoapps/demoapps.wasm ./demoapps_2.wasm

APP_NAME="Tie Tracker" APP_THEME=#230f29 APP_LOGO=https://daviddalbusco.com/images/portfolio/icons/tietracker-icon.png dfx build
cp .dfx/local/canisters/demoapps/demoapps.wasm ./demoapps_3.wasm

APP_NAME="Rebel Scan" APP_THEME="linear-gradient(135deg, #00a5cf, #9fffcb)" APP_LOGO=https://daviddalbusco.com/images/portfolio/icons/rebelscan-icon.png dfx build
cp .dfx/local/canisters/demoapps/demoapps.wasm ./demoapps_4.wasm

APP_NAME=StyloJS APP_THEME=#ff65a9 APP_LOGO=https://daviddalbusco.com/images/portfolio/icons/stylo-icon.png dfx build
cp .dfx/local/canisters/demoapps/demoapps.wasm ./demoapps_5.wasm