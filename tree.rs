// Define a simple binary tree structure
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Constructor for TreeNode
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

// Function to find the maximum depth of a binary tree
fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1 + left_depth.max(right_depth)
        }
    }
}

// Example usage
fn main() {
    // Create a sample binary tree
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode::new(9))),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode::new(15))),
            right: Some(Box::new(TreeNode::new(7))),
        })),
    }));

    // Calculate the maximum depth of the binary tree
    let depth = max_depth(&root);
    println!("Maximum depth of the binary tree: {}", depth);
}
