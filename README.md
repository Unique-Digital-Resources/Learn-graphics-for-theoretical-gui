
![UDR_github_graphics_examples_banner](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/assets/144396669/eead490c-597d-4a14-9ae0-7754477b2ad8)

# Learn graphics for theoretical gui with rust

Examples of GUI element graphics, written and illustrated with pictures to better understand the algorithm for drawing an element in the user interface.

### note:
- This project is a community project, meaning it depends on community participation to a large extent
- This README.md file was written by [dillinger](https://dillinger.io) and [readme.so](https://readme.so).


## the goal:
The main goal of this repository is to help understand:
- How to draw and paint every shape (especially in the graphical user interface) using libraries whose function is mainly graphics.
- Dealing with the window management library to link graphics to events (mouse, keyboard, joystick, system events, etc.)
- You can consider it as a challenge or training to improve your programming skills.


## Features

- Examples using different libraries or algorithm to implement the same operation
- Detailed explanation with pictures suitable for beginners and professionals as well
- Additional resources and external links for further information

## roadmap
We currently use two graphics libraries: [skia(rust)](https://github.com/rust-skia/rust-skia) and [tiny-skia](https://github.com/RazrFalcon/tiny-skia).
In the future, examples will be developed for other libraries, but the algorithm will not differ much.

There are levels of difficulty (the expected difficulty of writing the required example in terms of knowing the appropriate algorithm):
##### Easy: 
It can be written once you know how to work with the graphical library.
##### Difficult:
In addition to the previous requirement, it requires skill, resourcefulness, and some mathematical knowledge to know how to reach the required algorithm.
- Tiny-Skia
    - [x] Rounded rectangle (Difficult) 
    - [x] Simple pie chart (Difficult)
    - [x] Simple graph (Easy)
    - [ ] [Circular ends pie chart](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/blob/main/roadmap_images/circular_ends_pie_chart.png) (Difficult)

    - [ ] Animation/Animated shapes in [winit](https://github.com/rust-windowing/winit) window and [softbuffer](https://github.com/rust-windowing/softbuffer) (Difficult)
    - [ ] Animation/Animated shapes in [winit](https://github.com/rust-windowing/winit) window and [pixels](https://github.com/parasyte/pixels/tree/main) (Difficult)
    - [x] mouse hover on shape event in [winit](https://github.com/rust-windowing/winit) window and [softbuffer](https://github.com/rust-windowing/softbuffer) (Difficult)
    - [ ] mouse hover on shape event in [winit](https://github.com/rust-windowing/winit) window and [pixels](https://github.com/parasyte/pixels/tree/main) (Difficult)

- Skia
    - [ ] Rounded rectangle (Difficult) 
    - [ ] Simple pie chart (Difficult)
    - [ ] Simple graph (Easy)
    - [ ] [Circular ends pie chart](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/blob/main/roadmap_images/circular_ends_pie_chart.png) (Difficult)
    - [ ] Animation/Animated shapes in [winit](https://github.com/rust-windowing/winit) window and [softbuffer](https://github.com/rust-windowing/softbuffer) (Difficult)
    - [ ] Animation/Animated shapes in [winit](https://github.com/rust-windowing/winit) window and [pixels](https://github.com/parasyte/pixels/tree/main) (Difficult)
    - [ ] mouse hover on shape event in [winit](https://github.com/rust-windowing/winit) window and [softbuffer](https://github.com/rust-windowing/softbuffer) (Difficult)
    - [ ] mouse hover on shape event in [winit](https://github.com/rust-windowing/winit) window and [pixels](https://github.com/parasyte/pixels/tree/main) (Difficult)

##### Notes on the roadmap:
- You can suggest new goals or other libraries for graphics in the Issues section
- The level of difficulty is based on my personal scale and does not necessarily have to be actually easy or difficult. It varies between one person and another.
- Often the same algorithm can be used in a specific function with multiple libraries, meaning the code can be copied and used with another library, and the code can be changed as necessary to be compatible with the library.
- It is possible to write advanced algorithm/code that based on algorithm/code that already exists in this repository.

## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's [code of conduct](https://github.com/Unique-Digital-Resources/Learn-graphics-for-theoretical-gui/blob/main/CODE_OF_CONDUCT.md#contributor-covenant-code-of-conduct).
