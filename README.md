# rust_ml_experiments
## Practical Rust + CUDA ML experiments 

### Projects: :hammer_and_pick:
  
  > * __Matrix Multiplication Benchmark__ (CPU vs GPU) [rust_gemm.md] (rust_gemm/README.md)
  > * __GPU-Accelerated Image Blur__ (Box Filter) [box_filter.md] (box_filter/README.md)
  > * __Mandelbrot Set Rendering__ [mandelbrot.md] (mandelbrot/README.md)
  > * __N-Body Physics Simulation__ [physics.md] (physics/README.md)
  > * __Burn & Cuda: Resnet-18 on CIFAR-10__ [restnet-18.md] (resnet-18/README.md)
  > * __Custom CUDA Kenernel__ with ``` rustacuda ``` (Parallel Reduction)[parrallel_reduction.md] (parrallel_reduction.md)
  > * __Monte Carlo Pi Estimation__  [monte_carlo.md] (monte_carlo/monte_carlo.md)

### Build and Run it (Generic) :hammer_and_pick:

  > * Run it Locally:
    ```bash
          cargo run --release
     ```

  > * Build and Run it Docker:
    ```bash
          docker build -t ${PROJECT_TITLE} .
          docker run -it ${PROJECT_TITLE}
     ```
