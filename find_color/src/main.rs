use opencv::{
    core::{self, Mat, Scalar},
    highgui, // 导入 highgui 模块用于显示窗口
    imgcodecs,
    imgproc,
    prelude::*,
};

// 创建一个结构体来保存我们想要可视化的所有结果
struct DetectionResult {
    original: Mat,
    mask: Mat,
    result: Mat,
}

// 这个函数现在会返回一个包含所有结果图像的结构体
fn detect_blue_color(image_path: &str) -> opencv::Result<Option<DetectionResult>> {
    // 1. 读取图片
    let img = imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR)?;

    // 检查图片是否成功加载
    if img.empty() {
        return Err(opencv::Error::new(
            -1,
            format!("Failed to load image: {}", image_path),
        ));
    }

    // 2. 将颜色空间从 BGR 转换为 HSV
    let mut hsv_img = Mat::default();
    imgproc::cvt_color_def(&img, &mut hsv_img, imgproc::COLOR_BGR2HSV)?;

    // 3. 定义蓝色的 HSV 范围
    let lower_blue = Scalar::new(100.0, 50.0, 50.0, 0.0);
    let upper_blue = Scalar::new(140.0, 255.0, 255.0, 0.0);

    // 4. 根据颜色范围创建掩码
    let mut mask = Mat::default();
    core::in_range(&hsv_img, &lower_blue, &upper_blue, &mut mask)?;

    // 5. 创建一个只显示蓝色部分的结果图
    let mut result_img = Mat::default();
    // 使用 bitwise_and 将掩码应用到原始图像上
    core::bitwise_and(&img, &img, &mut result_img, &mask)?;

    // 6. 检查掩码中是否有非零像素
    if core::count_non_zero(&mask)? > 0 {
        // 如果找到了蓝色，返回包含所有图像的结构体
        Ok(Some(DetectionResult {
            original: img,
            mask,
            result: result_img,
        }))
    } else {
        Ok(None)
    }
}

fn main() -> opencv::Result<()> {
    // main 函数现在也返回一个 Result
    let image_path = "../find_color.jpg"; // 确保这个路径是正确的

    match detect_blue_color(image_path) {
        // 函数成功执行并找到了颜色
        Ok(Some(detection)) => {
            println!("Detected blue color in image: {}", image_path);

            // 创建窗口
            highgui::named_window("Original Image", highgui::WINDOW_AUTOSIZE)?;
            highgui::named_window("Mask", highgui::WINDOW_AUTOSIZE)?;
            highgui::named_window("Detected Blue", highgui::WINDOW_AUTOSIZE)?;

            // 在窗口中显示图像
            highgui::imshow("Original Image", &detection.original)?;
            highgui::imshow("Mask", &detection.mask)?;
            highgui::imshow("Detected Blue", &detection.result)?;

            // 等待用户按键，否则窗口会一闪而过。0 表示无限等待。
            highgui::wait_key(0)?;
            // 关闭所有窗口
            highgui::destroy_all_windows()?;
        }
        // 函数成功执行但未找到颜色
        Ok(None) => {
            println!("Blue color not found in the image.");
        }
        // 函数执行出错
        Err(e) => {
            eprintln!("An error occurred: {}", e);
        }
    }
    Ok(()) // main 函数成功结束
}
