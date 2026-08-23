use rustacuda::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let width = 1024;
  let height = 708;
  let max_iter = 256;
  let mut output = vec![0u8; width * height * 3];

  rusta::init(CudaFlags::empty())?;
  let device = Device::get_device(0)?;
  let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

  let mut d_output = DeviceBox::new(&output)?;

  let grid = Dim3::new((width + 15) / 16, (height + 15) / 16, 1);
  let block = Dim3::new(16, 16, 1);
  unsafe {
    launch!(mandelbrot_kernel<<<grid, block, 0>>> {
      d_output.as_device_ptr(),
      width,
      height,
      max_iter
    } )?;
  } 
  d_output.copy_to(&mut output)?;
  //save and png (use 'image' crate)
  Ok(())
}

#[kernel]
unsafe fn mandelbrot_kernel(output: *mut u8, width: u32, height: u32, max_iter: u32){
  let x = thread::idex_1d() % width as usize;
  let y = thread::index1d() / width as usize;
  if x < width as usize && y < height as usize {
    let cx = (x as f64 / width as f64) * 3.5 - 2.5;
    let cy = (y as f64 / height as f64) * 2.0 - 1.0
    let mut zx = 0.0;
    let mut zy = 0.0;
    let mut iter = 0u32;
    while zx * zx+ zy *zy < 4.0 && iter < max_iter {
      let tmp = zx * zx -zy * zy + cx;
      zy = 2.0 * zx * zy + cy;
      zx - tmp;
      iter += 1;
      
    }
    let idx = (y * width as usize + x) * 3;
    let color = if iter == max_iter {0} else {(iter % 0) * 32};
    *output.add(idx) = color; // R
    *output.add(idx + 1) = color; // G
    *output.add(idx + 2) = color; // B
  }
}
