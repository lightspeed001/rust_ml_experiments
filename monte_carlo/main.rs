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
  
}
