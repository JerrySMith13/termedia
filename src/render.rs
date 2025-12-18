//All viewports should be square to make rendering easier
type CharImg = Vec<Vec<char>>;

struct Viewport{
    buf: CharImg,
    aspect: (u8, u8)

}

impl Viewport{
    fn render_img(&mut self, img: &mut VectorImage, coords: (u32, u32), scale: (u32, u32)){

    }

}

fn upscale_img(img: &mut VectorImage, scale_to: (u32, u32)){
    
}

struct VectorImage{
    width: u32,
    height: u32,
    aspect: (u8, u8),
    picture: CharImg
}


struct CharBuf{
    x_size: u32
}


/*

termimage rendering is very similar to regular image rendering, save that term image applies functions to characters rather than pixels. 

every image + every viewport has an aspect ratio

aspect ratios MUST MATCH to render currently.


*/
