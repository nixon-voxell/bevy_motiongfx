# Bevy MotionGfx

Bevy Motiongfx is a motion graphics creation tool in [Bevy](https://github.com/bevyengine/bevy). It is highly inspired by [Motion Canvas](https://motioncanvas.io/) & [Manim](https://github.com/3b1b/manim).

![hello_world gif](./.github/assets/hello_world.gif)

## Goal

The goal of this tool is to procedurally generate animations with code. Below are some of the core design principles used in Bevy MotionGfx:

- Ease of use.
- Performance by default.
- Real-time preview.
- First class support for Bevy ECS.
- Robust (can be used not just for animation productions, but for all sorts of applications e.g. game development).

## Why this tool?

Procedurally coded animations can be easily scaled to larger projects (e.g. code can be reused/extended/structured).

- **Streamlining timeline editing with code-based animation.** Reorganizing strips and keyframes, particularly when rearranging clips, can be a laborious endeavor. However, with coded animations, the process becomes remarkably efficient. Swapping sections in the timeline is as simple as shifting lines of code up and down, allowing for swift and hassle-free adjustments.

  Here is an image (by [aarthificial](https://youtu.be/WTUafAwrunE)) comparing keyframes and code:
  <img src="https://i.ytimg.com/vi/WTUafAwrunE/maxresdefault.jpg" width="600"/>

- **Seamless version control.** With most of our assets represented as code or serialized values, we seamlessly integrate version control using [Git](https://git-scm.com/)!
- **Unleashing the full power of programming for animation.** In animation, tasks such as handling repetitions, reusing common effects, and more become a breeze by harnessing programming concepts like loops and functions.

## Easing Functions

Bevy MotionGfx also comes with built-in easing functions which are crucial for animation creation.

![easings gif](./.github/assets/easings.gif)

## License

The `bevy_motiongfx` is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
