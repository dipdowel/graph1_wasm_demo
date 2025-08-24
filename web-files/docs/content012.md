# Grid & 3D Bars

## Grids in `Graph1`
  
As of version `0.0.5-alpha`, the following grids are available:<br>
**1. The [UniformGrid](https://github.com/dipdowel/graph1/blob/develop/src/utils/grid/uniform.rs)** is a 2D grid structure made of rectangular regions (cells) that are evenly spaced and arranged in rows and columns. It's flexible, allowing you to both access and manipulate the cells as well as reshape the grid itself. <br>
**2. The [FlexRowGrid](https://github.com/dipdowel/graph1/blob/develop/src/utils/grid/flex_row.rs)** is similar to the `UniformGrid`, however, each row can have its own height and each cell in a row can have its own width.




| <!-- -->    | <!-- -->                                                                                                                                                                                                                         |
|-------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 💡         | Each cell of the grid is powered by the [Region<T>](https://github.com/dipdowel/graph1/blob/develop/src/utils/math/geometry/region.rs) structure, see the [Quadrants](/?demo=10) demo page for some more details on `Region<T>`. |


The grid simplifies positioning and aligning graphical elements on the screen. It can be used to  create layouts,
visualize coordinates, etc.
src/utils/grid/



## Rendering

[grid::render()](https://github.com/dipdowel/graph1/blob/develop/src/utils/grid/render.rs) performs a quick visualization of any kind of grid. The grid can be visualized as cell outlines or filled cells. Such visualization is useful for debugging or understanding the grid structure.

## `UniformGrid`

In this demo utilizes `UniformGrid` for positioning of the 3D bars. 3 grids of the following sizes were used:
- 8 x 2
- 16 x 5
- 24 x 3




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

## `FlexRowGrid`

#### 🚧 Under construction 🚧
Please stay tuned for updates!

- - - 
 


## 3D Bars
The [bar_3d](https://github.com/dipdowel/graph1/blob/develop/src/utils/math/geometry/region.rs) module allows drawing pseudo-3D bars in an [axonometric projection](https://en.wikipedia.org/wiki/Axonometric_projection). 

### Features
* Set colors for front, top, and side faces of the bar
* Set width, height, and depth
* Control slant and projection direction
* Efficiently render multiple bars in a batch

---

💻 [Code of this demo on GitHub](https://github.com/dipdowel/graph1_wasm_demo/blob/develop/src/demo/d_012_grid.rs)
