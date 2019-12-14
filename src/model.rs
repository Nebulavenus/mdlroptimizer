#[derive(Default, Debug)]
pub struct Model {
    pub name: String,
    pub sequences: Vec<Anim>,
    pub bones: Vec<Bone>,
}

#[derive(Default, Debug)]
pub struct Anim {
    pub name: String,
    pub interval: [u32; 2],
}

#[derive(Default, Debug)]
pub struct Bone {
    pub name: String,
    pub translations: Vec<Frame>,
    pub rotation: Vec<Frame>,
}

#[derive(Default, Debug)]
pub struct Frame {
    pub num: u32,
    pub value: [f32; 3],
}