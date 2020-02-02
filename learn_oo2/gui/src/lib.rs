pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, //trait对象，使用dyn关键字
}

//void func(A *p){
//}
//
//pub struct Screen<T: Draw> {
//    pub components: Vec<T>,
//}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

//impl<T> Screen <T>
//    where T: Draw {
//    pub fn run(&self) {
//        for comp in self.components.iter() {
//            comp.draw();
//        }
//    }
//}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button, width = {}, height = {}, label = {}", 
                 self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self) {
        println!("draw selectBox, width = {}, height = {}, option = {:?}", 
                 self.width, self.height, self.option);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
