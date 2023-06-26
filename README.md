# sudoku-rust-sdl2
An implementation of sudoku using rust with the sdl2 library.

## Installation
### 1.) Clone the repo to your local machine.
`git clone https://github.com/CCappsDevelopment/sudoku-rust-sdl2.git`<br>
### 2.) Install Rust along with a C++ compiler: I leave this as an excersize to the reader.<br>
### 3.) Install SDL2:<br>
#### Windows
- Navigate to `https://github.com/libsdl-org/SDL/releases/tag/release-2.28.0` <br>(NOTE: Replace 2.28.0 with the latest version of SDL2)
- Download the development kit that matches your C++ complier. The file will be in the form: <br>`SDL2-devel-[version]-[compiler].zip`
- Open the .zip file and navigate to `SDL2-devel-[version]-[compiler].zip\SDL2-[version]\lib\[your pc's architecture]`
- Copy the .lib files to the following location in your .rustup folder: <br>`C:\Users\USERNAME\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`
- Ensure the SDL2.dll is located in the base of the cloned `sudoku-rust-sdl2` repo, if not copy the .dll from the downloaded .zip file to the base directory.
#### MacOS
- Ensure you have homebrew installed
- Type the following command into Terminal: `$ brew install sdl2`
#### Linux

##### apt package mananger:
`$ sudo apt-get install libsdl2-dev`

##### yum package manager:
`$ yum install SDL2-devel`

##### pacman package manager:
`$ pacman -S sdl2`

### 4.) Install sdl2-ttf
#### Windows
- Navigate to `https://github.com/libsdl-org/SDL_ttf/releases`
- Download the development kit that matches your C++ complier. The file will be in the form: <br>`SDL2-devel-[version]-[compiler].zip`
- Open the .zip file and navigate to `SDL2_ttf-devel-[version]\lib\[your pc's architecture]`
- Copy the .lib file to the following location in your .rustup folder: <br>`C:\Users\USERNAME\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`
- Ensure the SDL2_ttf.dll is located in the base of the cloned `sudoku-rust-sdl2` repo, if not copy the .dll from the downloaded .zip file to the base directory.
#### MacOS
- Ensure you have homebrew installed
- Type the following command into Terminal: `$ brew install sdl2_ttf`
#### Linux

##### apt package mananger:
`$ sudo apt-get install libsdl2-ttf-dev`

##### yum package manager:
`$ yum install SDL2_ttf`

##### pacman package manager:
`$ pacman -S sdl2_ttf`
