# rust_ml_experiments
## Practical Rust & CUDA ML experiments 

### Projects: :spiral_notepad:
  
  > * __Matrix Multiplication Benchmark__ (CPU vs GPU) [rust_gemm.md](rust_gemm/README.md)
  > * __GPU-Accelerated Image Blur__ (Box Filter) [box_filter.md](box_filter/README.md)
  > * __Mandelbrot Set Rendering__ [mandelbrot.md](mandelbrot/README.md)
  > * __N-Body Physics Simulation__ [physics.md](physics/README.md)
  > * __Burn & Cuda: Resnet-18 on CIFAR-10__ [restnet-18.md](resnet-18/README.md)
  > * __Custom CUDA Kenernel__ with ``` rustacuda ``` (Parallel Reduction) [parrallel_reduction.md](parallel_reduction/README.md)
  > * __Monte Carlo Pi Estimation__  [monte_carlo.md](monte_carlo/README.md)

### Key Optimisations :chart_with_upwards_trend:  
  
  > * __Matrix Multiply (GEMM)__ : Shared Memory & tiling. 50 -100x expected speedup (vs CPU).
  > * __Image Blur__ : Shared memory for 3x3 neighborhood. 20-50x expected speedup (vs CPU).
  > * __Mandelbrot__ : Coalesced memory access. 10-30x expected speedup (vs CPU).
  > * __N-Body__ : Shared memory & tree methods. 30-80x expected speedup (vs CPU).
  > * __Parallel Reduction__ : Bank conflict avoidence. 50-150x expected speedup (vs CPU).
  > * __Monte Carlo Pi__ : Minimize atomic ops. 40-100x expected speedup (vs CPU).

### Build and Run it (Generic) :hammer_and_pick:

  > * __Run it Locally__:
    
    ```bash
          cargo run --release
     ```  

  > * __Build and Run it Docker__:
    
    ```bash
          docker build -t ${PROJECT_TITLE} .
          docker run -it ${PROJECT_TITLE}
     ```  

