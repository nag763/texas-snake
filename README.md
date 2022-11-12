[![texas-snake-stars](https://img.shields.io/github/stars/nag763/texas-snake?style=social)](https://github.com/nag763/texas-snake/stargazers)
[![tchatchers-license](https://img.shields.io/github/license/nag763/texas-snake)](https://raw.githubusercontent.com/nag763/texas-snake/main/LICENSE.md)
[![github-issues](https://img.shields.io/github/issues/nag763/texas-snake)](https://github.com/nag763/texas-snake/issues)

<p align="center"><img height="300" src="https://raw.githubusercontent.com/nag763/texas-snake/main/webapp/favicon.ico"></img></p>

<h2 align="center">Texas Snake :snake:</h2>
<h4 align="center">A blazingly fast WASM snake game built with Bevy, running on the browser</h4>

<p align="center"><img src=""></img></p>

## TL;DR

* :speech_balloon: Texas Snake is a game built in Rust with bevy framework. 
* :globe_with_meridians: It's running on the browser thanks to WASM.
* :rocket: Blazing fast, completely built on Rust.
* :moon: Runs integraly in dark mode.
* :abacus: It is inspired from the Snake game that was on my Texas Instrument calculator back in high school.

## How to access the game

The game is deployed on  and should be compatible with any modern navigator.

## Long story, short story : another snake game ?

Back in highschol, back in a time where games on mobile phone were starting to be popular, there was a need to kill the time during some courses.

My mates and I, who were mostly bored by courses and waiting for the evening to play online games, had to kill the time.

The only electronic stuff that was allowed during the courses were the calculators. I remember my school forced us to buy the same model, I think it was the TI 82 (mine was looking like this [one](https://www.amazon.fr/Texas-Instruments-Calculatrice-TI-Stats/dp/B0074AV98Q)).

And then someday, someone bragged about having successfully installed a Snake game on his calculator, and that he could copy this game on other calculators. It was wonderful, eventually a way to kill the time during the class hours.

Started a long period where me and my schoolmates were competiting on who will have the highest score. It was pretty fun, and at the same time, pretty tough. I moreover remember the setup with the borders as a '+', this one was particulary hard but also particulary rewarding.

This project is a tribute to this time, and also an excuse to learn how making games in Rust works :happy:.

## Project structure

```
.
├── Cargo.lock
├── Cargo.toml 
├── Makefile.toml => Cargo make file, used to build the app
|── README.md
├── snake => the snake game
│   ├── assets => the assets used in the game
│   │   └── score_font.otf
│   ├── Cargo.toml => The dependencies definitions
│   └── src
│       ├── common.rs => common constants
│       ├── components => components used in the app
│       ├── main.rs => runnable entry point
│       ├── resources => resources used in the app
│       └── systems => every system per game_state
└── webapp => the webapp running the wasm exe 
    ├── assets => the assets of the webapp
    │   ├── score_font.otf
    │   └── tailwind.css => style sheet
    ├── favicon.ico
    ├── games => target directory of the executable
    │   ├── snake_bg.wasm
    │   ├── snake.js
    │   └── snake.wasm
    ├── index.html => entry point of the webapp
    ├── input.css
    ├── tailwind.config.js
    └── wasm_loader.html => iframe called by the index
```

## Technologies used

|Technology/Framework|Utility                     |Version|
|--------------------|----------------------------|-------|
|Rust                |Programming language        |1.65   |
|bevy                |Game engine                 |0.7.1  |
|Tailwind            |Stylesheets                 |3.1.18 |

## In game logic schema

![process](https://raw.githubusercontent.com/nag763/texas-snake/main/.github/process-graph.webp)

## Feedback

My goal behind this project was to first learn about bevy and in general how to make simple games in Rust. I must say bevy is a pretty easy to use (at least for 2D games) game engine and adding the comfort of developping in Rust to make games is a pretty cool thing. 