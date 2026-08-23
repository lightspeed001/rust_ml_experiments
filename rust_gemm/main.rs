use rustacuda::prelude::*;
use ndarray::(Array2, Array);

fn main -> Result<(), Box<dyn std::error::Error>> {
  let n - 1024;
  let a_cpu: Array2<f32> = Array::random([n, n]);
  let b_cpu: Array2<f32> Array::random([n, n]);

  // CPU (ndarray)
  let start = std::time::Instant::now();
  let c_cpu = a_cpu.dot(&b_cpu);
  println!("CPU: {}", start.elapsed());

  // GPU (rustacuda)
  rustacuda::init(CudaFlags::empty())?;
  let device = Device::get_device(0)?;
  let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;
  let a_gpu = DeviceBox::new(&a_cpu.to_owned())?;
  let b_gpu = DeviceBox::new(&b_cpu.to_owned())?;
  let mut c_gpu = DeviceBox::zeroed([n, n])?;

  let grid = Dim3::new(16, 16, 1);
  let block = Dim3::new(16, 16, 1);
  unsafe {
    launch(matmul_kernel<<<grid>, block, 0>>(a_gpu.as_device_ptr(), b_gpu.as_device_ptr(), c_gpu.as_device_ptr(), n ) )?;
    let start = std::time::Instant::now();
    let_c_result = c_gpu.copy_to(&mut vec![0.0; n * n])?;
    println!("GPU: {:/} ", start.elapsed());
    Ok(())
  }

  #[kernel]
  unsafe fn matmul_kernel(a: *const f32, b: *const f32, c: *mut f32, n: usize) {
    // Naive kernel (optimize with shared memory later)
  }
}
