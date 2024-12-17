# Ray tracer
A ray tracer for me to learn more about rendering and rust.

![Cornell box scene rendered with 1000 samples per pixel.](cornell_box_1000.png)
Cornell box scene rendered with 500 samples per pixel. ($\sim$ 15min 17s)
This project does not aim to be at the cutting edge of its domain. However, it utilizes some of major techniques of optimisation for faster rendre times.
Currently, spacial partitioning is implemented to accelerate ray intersection tests. The algorithm used is Binned SAH BVH.
