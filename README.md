# rapier_examples
Demonstration of using the [Rapier 2D](https://rapier.rs/) physics engine, without using Bevy.
Using an input format (in JSON) you can setup a scene then run the simulation and render
it to a video output. There is a Docker file and build script if using Windows and don't
want to install FFMPEG (which can be hard on Windows), which are required by the `openh264`
crate.

### Running the Application on Computer
To run the application you need to specify the input file (although there is a default). eg.

```rust
./rapier_examples -f "inputs/box.json" -m 2000 -d
```

Or from Cargo, using the longform args

```rust
cargo run -- --file "inputs/box.json" --max-frames 2000 --debug
```

### Running the Application on Docker
There is a build script....TODO: Add args for Docker

### File layout
The input file is a JSON consists of an array of `Blocks` and `Users`. 
A `Block` object looks like this:

```json
{
    "Location"
}
```