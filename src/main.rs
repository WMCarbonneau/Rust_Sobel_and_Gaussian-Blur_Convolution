extern crate image;

// use std::io::set_output_capture;
use image::{
    GenericImageView,
    ImageBuffer,
    Luma
};
use ndarray::Array2;
use ndarray::arr2;

/**Convolves over an image using the 3x3 matrix parameter as a kernel.**/
fn convolve(input: &ImageBuffer<Luma<u8>, Vec<u8>>, inputMatrix: &Array2<f64>) -> ImageBuffer<Luma<u8>, Vec<u8>>{
    let width: u32 = input.width();
    let height: u32 = input.height();
    let mut buff: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // println!("Top left pixel value {}", input.get_pixel(0,0).data[0]); // debug

    for i in 0..width-2{
        for j in 0..height-2{
            // let val0 = input.get_pixel(i,j).data[0] as i32;
            if i == 0 {}
            else if j == 0 {}
            else if i >= width-1{}
            else if j >= height-1{}
            // else if j > width-1{}
            else {
                // println!("{} {}",i,j);  // test coordinates
                let matrix = arr2(&[
                    [input.get_pixel(i-1, j-1).data[0] as i32,
                        input.get_pixel(i, j-1).data[0] as i32 ,
                        input.get_pixel(i+1, j-1).data[0] as i32],
                    [input.get_pixel(i-1, j).data[0] as i32,
                        input.get_pixel(i, j).data[0] as i32 ,
                        input.get_pixel(i+1, j).data[0] as i32],
                    [input.get_pixel(i-1, j+1).data[0] as i32,
                        input.get_pixel(i, j+1).data[0] as i32 ,
                        input.get_pixel(i+1, j+1).data[0] as i32]
                ]);
                let mut finalValue = (matrix[[0,0]] as f64* &inputMatrix[[0,0]])+
                                     (matrix[[0,1]] as f64* &inputMatrix[[0,1]])+
                                     (matrix[[0,2]] as f64* &inputMatrix[[0,2]])+
                                     (matrix[[1,0]] as f64* &inputMatrix[[1,0]])+
                                     (matrix[[1,1]] as f64* &inputMatrix[[1,1]])+
                                     (matrix[[1,2]] as f64* &inputMatrix[[1,2]])+
                                     (matrix[[2,0]] as f64* &inputMatrix[[2,0]])+
                                     (matrix[[2,1]] as f64* &inputMatrix[[2,1]])+
                                     (matrix[[2,2]] as f64* &inputMatrix[[2,2]]);
                if finalValue > 255.0 {finalValue = 255.0;}
                // println!("{} {}",i,j);
                buff.put_pixel(i,j,Luma([finalValue as u8])); // move this outside when other conditions are met
            }
            // buff.put_pixel(i,j,Luma([val0 as u8]));
        }
    }
    return buff;
}

//does nothing at the moment
// fn sobel(input: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>>{
//     let width: u32 = input.width();
//     let height: u32 = input.height();
//     let mut buff: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
//
//
//     for i in 0..width-1{
//         for j in 0..height-1{
//             // let val0 = input.get_pixel(i,j).data[0] as i32;
//             if i == 0{
//
//             }
//             else if j == 0{
//
//             }
//             else if i == width-1{
//
//             }
//             else if j == height -1{
//
//             }
//             else {
//                 let mut matrix = arr2(&[
//                     [input.get_pixel(i-1, j-1).data[0] as i32,input.get_pixel(i, j-1).data[0] as i32 ,input.get_pixel(i+1, j-1).data[0] as i32],
//                     [input.get_pixel(i-1, j).data[0] as i32,input.get_pixel(i, j).data[0] as i32 ,input.get_pixel(i+1, j).data[0] as i32],
//                     [input.get_pixel(i-1, j+1).data[0] as i32,input.get_pixel(i, j+1).data[0] as i32 ,input.get_pixel(i+1, j+1).data[0] as i32]
//                 ]);
//                 let gx = (matrix[[0,0]] * -1) + (matrix[[0,1]] * -2) + (matrix[[0,2]] * -1) + matrix[[2,0]] + matrix[[2,1]] + matrix[[2,2]];
//                 let gy = (matrix[[0,0]] * -1) + (matrix[[1,0]] * -2) + (matrix[[2,0]] * -1) + matrix[[2,2]] + matrix[[1,2]] + matrix[[0,2]];
//
//                 let mut total = f64::sqrt(((gx * gx) + (gy * gy)) as f64);
//
//                 if total > 255.0 {total = 255.0;} // limit pixels to 255
//
//                 buff.put_pixel(i,j,Luma([total as u8])); // move this outside when other conditions are met
//             }
//             // buff.put_pixel(i,j,Luma([val0 as u8]));
//         }
//     }
//
//     return buff;
// }

/**"noise.jpg" is the image that is being processed at the moment. "Converged.jpg" is the result. **/
fn main() {
    let BLUR_ITER = 5; // Edit to change blur steps amount

    let img = image::open("images/noise.jpg").unwrap();
    let (imgx, imgy) = img.dimensions();
    println!("{} {}", imgx, imgy);

    let matrixGaussian = arr2(&[
        [0.0625, 0.125, 0.0625],
        [0.125, 0.25, 0.125],
        [0.0625, 0.125, 0.0625]
    ]);

    let matrixSobelx = arr2(&[
        [-1.0, -2.0, -1.0],
        [0.0, 0.0, 0.0],
        [1.0, 2.0, 1.0]
    ]);
    let matrixSobely = arr2(&[
        [-1.0, 0.0, 1.0],
        [-2.0, 0.0, 2.0],
        [-1.0, 0.0, 1.0]
    ]);

    //     Grayscale image
    let Gray = img.to_luma();
    // Gray.save("Gray.jpg").unwrap();

    let mut GaussianBlur = convolve(&Gray, &matrixGaussian);
    GaussianBlur.save("testBlur.png").unwrap();


    for _ in 0..BLUR_ITER-1{
        GaussianBlur = convolve(&GaussianBlur, &matrixGaussian);
    }

    GaussianBlur.save(format!("testBlur{}Times.png", BLUR_ITER)).unwrap();

    //
    // let sobel = sobel(&GaussianBlur);
    // GaussianBlur.save("testBlur.png").unwrap();
    // sobel.save("testSobel.png").unwrap();

    let Sobelx = convolve(&GaussianBlur, &matrixSobelx);

    Sobelx.save("SobelX.jpg").unwrap();

    let Sobely = convolve(&GaussianBlur, &matrixSobely);

    Sobely.save("SobelY.jpg").unwrap();

    let mut buff: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(img.width(), img.height());

    for i in 0..img.width(){
        for j in 0..img.height(){
            buff.put_pixel(i,j,Luma([((Sobelx.get_pixel(i,j).data[0] as f64).powi(2) + (Sobely.get_pixel(i,j).data[0] as f64).powi(2)).sqrt() as u8]));
        }
    }

    buff.save("Converged.jpg").unwrap();

    let imgbuff : ImageBuffer<Luma<u8>, Vec<u8>> = image::ImageBuffer::new(imgx,imgy);
    println!("Dynamic image dimensions: {:?}, {:?}", imgx, imgy);
    println!("ImgBuff Dimensions: {:?}", imgbuff.dimensions());

}