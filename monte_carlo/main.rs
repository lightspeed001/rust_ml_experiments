use rustacuda::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
  let samples = 10_000_000;
  let mut counts = vec![0u32; 1]; // [inside_circle]

  rustacuda::init(CudaFlags::empty())?;
  let device = Device::get_device(0)?;
  let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;
  let mut d_counts = DeviceBox::new(&counts)?;

  let grid = Dim3::new((samples + 1023) / 1024, 1, 1);
  let block = Dim3::new(1024, 1, 1);
  unsafe {
    launch!(monte_carlo_kernel<<<grid, block, 0>>>(d_counts.as_device_ptr(), samples, thread::seed()))?;
  }
  d_counts.copy_to(&mut counts)?;
  let pi_estimate = 4.0 * (counts[0] as f64) / (samples as f64);
  println!("pi = {}", pi_estimate);
  Ok(())
}

#[kernel]
unsafe fn monte_carlo_kernel(counts: *mut u32, samples: usize, seed: u64) {
  let tid = thread::index_1d();
  if tid >= samples {return;}
  let mut rng = thread::Rng::new(seed.wrapping_add(tid as u64));
  let x: f64 = rng.gen();
  let y: f64 = rng.gen();
  if x * x + y * y <= 1.0 {
    atomic::add(counts, 1);
  }
}
