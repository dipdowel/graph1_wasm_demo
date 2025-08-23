# Grid & 3D Bars

## Grid
  
As of version `0.0.5-alpha`, `Graph1` provides [UniformGrid](https://github.com/dipdowel/graph1/blob/develop/src/utils/grid/uniform/uniform_grid.rs). The `UniformGrid` is a 2D grid structure made of rectangular regions (cells) that are evenly spaced and arranged in rows and columns. It's flexible, allowing you to both access and manipulate the cells as well as reshape the grid itself. 

 



| <!-- -->    | <!-- -->                                                                                                                                                                                                             |
|-------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 💡         | Each cell is powered by the [Region<T>](https://github.com/dipdowel/graph1/blob/develop/src/utils/math/geometry/region.rs) structure, see the [Quadrants](/?demo=10) demo page for some more details on `Region<T>`. |


The grid simplifies positioning and aligning graphical elements on the screen. It can be used to  create layouts,
visualize coordinates, etc.
In this demo the 3D bars are positioned using 3 grids of the following sizes:
- 8 x 2
- 16 x 5
- 24 x 3


src/utils/grid/uniform/render.rs

### Features

#### Construction 
You can create a grid from a prototype cell, with rows and columns, and optionally cycle through a list of colors to give each cell a different color.

#### Cell access and iteration 
You can retrieve individual cells by row, column, or index, and iterate through all cells either immutably or mutably.

#### Resizing
The grid supports resizing in several ways:
    * Changing the number of rows and columns while keeping cell size fixed.
    * Changing the size of each cell while keeping the layout intact.
    * Adjusting both grid dimensions and cell size in one step.
    * Automatically resizing cells when rows/columns change so that the total grid area stays the same.
 
#### Neighborhood queries
For any cell, you can get its neighbors according to different definitions (orthogonal, diagonal, circular, diamond, etc.). This is useful for simulations where changes in one cell influence nearby cells. 

#### Use cases
The grid is well-suited for applications in graphics, simulations, and procedural content generation, e.g.: 
* Spatial partitioning (breaking space into manageable chunks).
* [Cellular automata](https://en.wikipedia.org/wiki/Cellular_automaton) (like Conway's Game of Life).
* Pathfinding or navigation grids.
* Terrain or texture generation.
* Any situation where structured, evenly spaced regions need to be manipulated or queried efficiently.

- - - 
src/sprites/axonometric/bar_3d.rs


## 3D Bars
The [bar_3d](https://github.com/dipdowel/graph1/blob/develop/src/utils/math/geometry/region.rs) module allows drawing pseudo-3D bars in an [axonometric projection](https://en.wikipedia.org/wiki/Axonometric_projection). 

### Features
* Set colors for front, top, and side faces of the bar
* Set width, height, and depth
* Control slant and projection direction
* Efficiently render multiple bars in a batch

---

💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_012_grid.rs)
