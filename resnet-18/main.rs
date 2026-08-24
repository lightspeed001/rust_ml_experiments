use burn::{
  backend::Cuda,
  nn::{resnet::ResNet, ResNetConfig},
  tensor::Tensor,
};

type MyBackend = Cuda;
type MyAutodiff = burn::autodiff::ADBackend<MyBackend>;

fn main() {
  let device = burn::backend::Cuda::default();
  let model: ResNet<MyBackend> = ResNetConfig::new(18, 10).init(&device); // 18 layers, 10 classes (CIFAR-10)

  //.. (load CIFAR-10, train loop)
}
