struct Node {
    parent: usize,
}

fn main() {
    let mut nodes = vec![
        Node{parent: 0},
        Node{parent: 1},
        Node{parent: 2},
    ];

    /*for i in 1..nodes.len() {
        if nodes[i].parent == 1 {
            nodes[i-1].parent = 0;
        }
    }*/

    nodes.iter().enumerate().for_each(|(i, node)| {
        if node.parent == 1 && i > 0 {
            nodes[i-1].parent = 0;
        }
    });
}
