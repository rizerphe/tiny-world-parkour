# [WIP] Tiny world parkour

This is an implementation of a tiny parkour minecraft server in rust and valence. It is a work in progress, pushed pretty much in the middle of a refactoring :3

## How to run

```bash
cargo run path/to/minecraft/world
```

This will start a server on `localhost:25565` that will serve the world at `path/to/minecraft/world`, with a parkour course procedurally generated on top of it, starting at 0,128,0. The world, as well as the client, should be in 1.20.1. Make sure enough of the world is generated before starting the server.

A known bug is the server not responding for the first ~half a minute after joining; I already fixed it once accidentally, but it came back, and I don't at all know what changed. I'll get to it eventually.

Note about lighting: minecraft calculates lighting server-side. Valence does not support this, therefore the default client just renders everything as flat. It's all great, but problems start to happen when you throw shaders into the mix. Most iris shaders I've tried have generated pitch black chunks etc. A workaround is using a reimplementation of the lighting system, such as starlight. This works great after the world has already been generated, however, worlds created with starlight will not get loaded properly by this server. I have no clue why, oh well though. So the current workflow looks like this:

-   boot client with chunky
-   create a new world, generate a large area with chunky
-   close the client
-   start the server
-   boot the client with starlight
-   connect to the server and play

That's just what I do, you can do whatever you want ofc.
