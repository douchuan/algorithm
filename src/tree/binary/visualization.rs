use crate::tree::binary::Tree;

pub struct Visualization;

impl Visualization {
    /*
               1
           2       3
         4   5   6   7
    */
    pub fn draw_binary(tree: &Tree) {
        unimplemented!()
    }

    //利用等比数列求和
    pub fn sum_nodes(tree: &Tree) -> usize {
        let h = tree.height();
        2usize.pow(h as u32) - 1
    }
}
