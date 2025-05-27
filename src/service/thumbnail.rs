use ::base64::{Engine, engine::general_purpose};
use ::image::DynamicImage;
use ::std::{io, mem, path::Path};
use ::windows::{
    Win32::{
        Graphics::Gdi::{
            BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, DeleteObject, GetDC, GetDIBits,
            GetObjectW, ReleaseDC,
        },
        System::Com::{CoInitialize, CoUninitialize},
        UI::Shell::{
            IShellItem, IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_RESIZETOFIT,
        },
    },
    core::*,
};

pub fn thumbnail(path: impl AsRef<Path>) -> Option<String> {
    if let Some(image) = extract_thumbnail(path) {
        let mut buffer = io::Cursor::new(Vec::new());

        image.write_to(&mut buffer, image::ImageFormat::Png).ok()?;
        let buffer = buffer.into_inner();

        let base64 = general_purpose::STANDARD.encode(&buffer);
        Some(format!("data:image/png;base64,{base64}"))
    } else {
        None
    }
}

fn extract_thumbnail(path: impl AsRef<Path>) -> Option<DynamicImage> {
    unsafe {
        let com_initialized = CoInitialize(None).is_ok();

        let result = (|| {
            let path_str = path.as_ref().to_string_lossy();
            let shell_item: IShellItem =
                SHCreateItemFromParsingName(&HSTRING::from(path_str.as_ref()), None).ok()?;

            let image_factory: IShellItemImageFactory = shell_item.cast().ok()?;

            let hbitmap = image_factory
                .GetImage(
                    windows::Win32::Foundation::SIZE { cx: 256, cy: 256 },
                    SIIGBF_RESIZETOFIT,
                )
                .ok()?;

            let mut bitmap: BITMAP = mem::zeroed();
            if GetObjectW(
                hbitmap.into(),
                size_of::<BITMAP>() as i32,
                Some(&mut bitmap as *mut _ as *mut _),
            ) == 0
            {
                let _ = DeleteObject(hbitmap.into());
                return None;
            }
            let width = bitmap.bmWidth as u32;
            let height = bitmap.bmHeight.unsigned_abs();

            let mut info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width as i32,
                    biHeight: -(height as i32),
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: 0,
                    ..Default::default()
                },
                bmiColors: [Default::default(); 1],
            };

            let mut bits = vec![0u8; (width * height * 4) as usize];

            let hdc = GetDC(None);
            let success = GetDIBits(
                hdc,
                hbitmap,
                0,
                height,
                Some(bits.as_mut_ptr() as *mut _),
                &mut info,
                DIB_RGB_COLORS,
            ) != 0;
            let _ = ReleaseDC(None, hdc);

            let _ = DeleteObject(hbitmap.into());

            if !success {
                return None;
            }

            for chunk in bits.chunks_exact_mut(4) {
                chunk.swap(0, 2);
            }

            image::RgbaImage::from_raw(width, height, bits).map(DynamicImage::ImageRgba8)
        })();

        if com_initialized {
            CoUninitialize();
        }

        result
    }
}
