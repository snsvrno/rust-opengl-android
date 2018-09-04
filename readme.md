This is intended to be a minimal working example of desktop + android cross platform graphic app using `glutin` and `gl`

# What it Does
Simple opengl example taken from [tomka/glutin's window example](https://github.com/tomaka/glutin) to draw on both desktop and mobile.

# Requirements
- [docker](https://www.docker.com/) (sorta optional)
- [just](https://github.com/casey/just) (optional)

## Docker

This example uses of [tomaka/cargo-apk](https://github.com/tomaka/android-rs-glue)'s docker image to build the apk. You can do it without docker, but it is more intensive and not as portable of a setup. I haven't testing it without the docker, but I assume it would work as its the same stuff going on inside and outside the docker image. I would recommend the docker image though, might be a hastle to setup but it does help in the future with building for specific environments.

I use [docker-compose](https://docs.docker.com/compose/) instead of a Dockerfile or command because it makes it easier to declare specific tasks for the docker image.

## Just

I also use [just](https://github.com/casey/just) for all the commands to run. You don't need this as the `justfile` is fairly easy to see whats actually going on, but for convinence I recommend checking it out in your own projects. 

# Using this Example

## Building for Android

First you need to build for android, everything is setup in the docker image so nothing additional is required. If you will use libraries that need to compile for android, you will need to do that in a custom docker image (or you can have then prebuilt as *.so files), and list their location in a `.cargo/config` file in the project directory. In the case of the example there are no libraries.

To build for android using the _just_ runner

```
just build-android
```

Or if you just want to use docker, with _docker-compose_,

```
docker-compose run android
```
Or plain docker if you don't want to use _docker-compose_,

```
docker run --rm -v .:/root/src tomaka/cargo-apk cargo apk build
```

Sometimes the above command doesn't work in windows because of a docker pathing issue. If this gives you an error about path being too short, then just change `.` to the absolute path to the directory: `C:\Users\parent\directory\of\toml`.

## Installing

Then you can install to android using `ADB`

The command is already listed in _just_, which will uninstall the previous version and install the newer version.

```
just install-android
```

# Random list of references & Links

Not all links here are part of this working example, but they all got me here. I wanted to keep them because I found them useful on the journey and might need to reference them again as I go further down this rabbithole.

OpenGL
- https://docs.rs/gl_generator/0.9.0/gl_generator/
- http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html

SDL2
- https://rust-sdl2.github.io/rust-sdl2/sdl2/
- https://rust-sdl2.github.io/rust-sdl2/sdl2/video/index.html

Rust & Graphics on Mobile
- https://michaelfairley.com/blog/i-made-a-game-in-rust/
- https://www.reddit.com/r/rust/comments/9bq7df/iphone8_screenshot_of_gfxhal_with_a_metal_backend/

Building Rust for Android
- https://github.com/tomaka/android-rs-glue


