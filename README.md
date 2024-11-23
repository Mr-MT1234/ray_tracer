# Ray tracer
A ray tracer for me to learn more about rendering and rust.

![Cornell box scene rendered with 100 samples per pixel.](cornell_box_500.png)
Cornell box scene rendered with 500 samples per pixel. ($\sim$ 1h 16min)
This project does not aim to be at the cutting edge of its domain. However, it utilizes some of major techniques of optimisation for faster rendre times.
Currently, spacial partitioning is implemented to accelerate ray intersection tests. The algorithm used is Binned SAH BVH.
