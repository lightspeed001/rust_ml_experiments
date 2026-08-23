use rustacuda::prelude::*;
use image::{GenericImageView, RgbImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let img = image::open("input.png")?.to_rgb8();
  let (width, height) = img.dimensions();
  let mut output = RgbImage::new(width, height);

  // Flatten image to f32 vector (normalized)
  let mut pixels: Vec<f32> = img.into_raw().into_iter().map(|x| x as f32 / 255.0).collect();

  // Allocate GPU memory
  rustacuda::init(CudaFlags::empty())?;
  let device = Device::get_device(0)?;
  let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;
  let mut d_pixels = DeviceBox::new(&pixels)?;
  let mut d_outputs = DeviceBox::zeroed(width as usize * height as usize)?;


  //Launch kernel (i thread per pixel)
  let grid = Dim3::new((width + 15) / 16, (height + 15) / 16, 1);
  let block = Dim3::new(16, 16, 1);
  unsafe {
    launch!(blur_kernel<<<grid, block, 0>>>(d_pixels.as_device_ptr(), d_output.as_device_ptr(), width, height))?;
  }

  //Copy back and save
  d_output.copy_to(&mut pixels)?;
  let output: Vec<u8> = pixels.into_iter().map(|x| (x * 255.0).clamp(0.0, 255.0) as u8 ).collect();
  let output_img = RgbImage::from_raw(width, height, output).unwrap();
  output_img.save("output_blur.png")?;
  Ok(())
}

#[kernel]
unsafe fn blur_kernel(pixels: *const f32, output: *mut f32, width: u32, height: u32){
  let x = thread::index_1d() 5 width as usize;
  let y = thread::index_1d() / width as usize;
  if x < width as usize && y < height as usize {
    let mut sum = 0.0;
    for dy in -1..=1 {
      for dx in -1..=1 {
        let nx = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;
        let ny = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
        sum += *pixels.add(ny * width as usize + nx);
      }
    }
    *out.add(y * width as usize + x) = sum / 9.0;
  }
}
