# RustTorch

Template for loading PyTorch projects into Rust

## Requirements

- [libtorch](https://github.com/LaurentMazare/tch-rs)

This template is specifically for loading CNN models with image data inputs. Other architectures may require additional changes to the code.

## Usage 

Add test images to `/data` and change the values in `config.rs`:
```
test_image = "test_image.png",
```

If the API for the model store requires authentication, change the values in `config.rs`:
```
username: "username",
path_to_pass = "../../password.txt",
url = "https://nce.org/remote.php/dav",
```

Run the program:
```
$ cargo run
```