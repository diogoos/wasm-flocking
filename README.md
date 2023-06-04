# Boids Simulation in Rust + WebAssembly
This project is a simulation of the "boids" algorithm, originally developed by [Craig Reynolds](https://www.red3d.com/cwr/boids/), implemented using Rust and WebAssembly. The boids algorithm simulates the coordinated motion of a group of entities, such as bird flocks or fish schools, based on three simple steering behaviors: separation, alignment, and cohesion.

The purpose of this project is to showcase the capabilities of Rust and WebAssembly for creating efficient and interactive simulations in the browser. It was also inspired by The Coding Train's [video](https://www.youtube.com/watch?v=mhjuuHl6qHM).


## [View online here](https://diogoos.github.io/wasm-flocking)
<a href="https://diogoos.github.io/wasm-flocking"><img src="https://raw.githubusercontent.com/diogoos/wasm-flocking/dist/flocking.png" alt="Flocking demo"/></a>


## Behavior description
* **Cohesion:** steers each boid towards the average position of its nearby flockmates, promoting a sense of togetherness and group cohesion.
* **Separation:** ensures that each boid maintains a safe distance from its neighbors, avoiding crowding and collisions.
* **Alignment:** aligns each boid's heading with the average heading of its nearby flockmates, resulting in synchronized motion within the group.

## QuadTree storage
This simulation utilizes a QuadTree data structure to provides a hierarchical representation of the simulation area, partitioning it into smaller quadrants recursively. Each quadrant can either be further divided into four sub-quadrants or contain boids within its bounds.

The QuadTree structure offers several advantages for boid data storage and spatial querying:

* **Efficient spatial partitioning:** The QuadTree divides the simulation area into smaller regions, allowing for efficient organization and retrieval of boid data based on their spatial positions. This enables faster neighbor search operations required for the separation, alignment, and cohesion behaviors.

* **Reduction in search space:** With a QuadTree, it is possible to limit the search for neighboring boids to a specific region of interest rather than considering all boids in the simulation. This optimization significantly reduces the computational cost, especially as the number of boids increases.

* **Dynamic adaptation:** The QuadTree can dynamically adjust its structure based on the distribution of boids in the simulation. If a quadrant becomes too crowded, it can be subdivided into smaller quadrants to maintain an appropriate balance between subdivision depth and boid density.

## Running locally
To run the flocking simulation locally, you will need to have Rust and Node.js installed on your machine.

1. Clone this repository: `git clone https://github.com/diogoos/wasm-flocking.git`
2. Install node & webpack dependencies: `npm install`
3. Install cargo dependencies by building from source: `cargo build`
4. Run the app: `npm run serve`
