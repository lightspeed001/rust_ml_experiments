use rustacuda::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let n = 1 << 20;
  let input: Vec<f32> = (0..n).map(|x| x as f32).collect();
  let mut output = vec![0.0];

  rustacuda::init(CudaFlags::empty())?;
  let device = Device::get_device(0)?;
  let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;
  let d_input = DeviceBox::new(&input)?;

  let threads = 1024;
  let blocks = (n + threads - 1)/ threads;
  unsafe {
    launch!(reduce_kernel<<<blocks, threads, threads * 4>>>{
      d_input.as_device_ptr(),
      d_output.as_device_ptr(),
      n
    })?;
    
  }
  d_output.copy_to(&mut output)?;
    println!("Sum: {}", output[0]); // 500M (0+1+2+.......+999999)
    Ok(())
  
}

#[kernel]
unsafe fn reduce_kernel(input: *const f32, output: *mut f32, n: usize) {
  let tid = thread::index_1d();
  letmut val = if tid <n {*input.add(tid)} else {0.0};
  let mut stride < block::size_1d(){
    if tid % (2 * stride) == 0 {
      val += *block::shared_memory::<f32>().add(tid + stride);
    }
    stride <<= 1;
    block::sync();
  }
  if tid == 0 {
    *output = val;
  }
}
