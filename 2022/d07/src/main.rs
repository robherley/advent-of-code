use std::collections::{HashMap};
use std::fmt;

enum Node {
  Dir(Dir),
  File(File),
}

impl Node {
  fn as_dir(&self) -> &Dir {
    match self {
      Node::Dir(dir) => dir,
      Node::File(file) => panic!("not a dir, file:{:?}", file.name),
    }
  }

  fn as_mut_dir(&mut self) -> &mut Dir {
    match self {
      Node::Dir(dir) => dir,
      Node::File(file) => panic!("not a dir, file:{:?}", file.name),
    }
  }

  fn pretty_print(&self, nodes: &Vec<Node>) {
    fn pp(node: &Node, nodes: &Vec<Node>, pad: usize) {
      match node {
        Node::Dir(dir) => {
          println!("{} - {}", " ".repeat(pad), dir);
          for child_id in &dir.children {
            let child = &nodes[*child_id];
            pp(child, nodes, pad + 2);
          }
        },
        Node::File(file) => {
          println!("{} - {}", " ".repeat(pad), file);
        },
      }
    }

    pp(self, nodes, 0);
  }
}

struct Dir {
  id: usize,
  name: String,
  parent: Option<usize>,
  children: Vec<usize>,
}

impl fmt::Display for Dir {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} (dir)", self.name)
  }
}

struct File {
  name: String,
  size: u64
}

impl fmt::Display for File {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} (file, size={})", self.name, self.size)
  }
}

fn build_nodes(input: &str) -> Vec<Node> {
  let root = Dir{
    id: 0,
    parent: None,
    name: "/".to_string(),
    children: vec![],
  };

  // originally i tried using a tree structure, but this is rust :)
  // this is a really "dumb" arena that store nodes, and each node hold index references
  // some nice readings:
  //  https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
  //  https://manishearth.github.io/blog/2021/03/15/arenas-in-rust/
  let mut nodes = vec![Node::Dir(root)];

  let mut curr_id = 0;
  for line in input.lines() {
    match line {
      line if line.starts_with("$ cd") => {
        let cmd = line.split(' ').collect::<Vec<&str>>()[2];
        match cmd {
          ".." => {
            curr_id = nodes.get(curr_id).unwrap().as_dir().parent.expect("parent node not found");
          }
          "/" => {
            curr_id = 0;
          }
          dir_name => {
            let dir = nodes[curr_id].as_dir();

            let child_id = dir.children.iter().find(|child_id| {
              match &nodes[**child_id] {
                Node::Dir(dir) => dir.name == dir_name,
                Node::File(_) => false,
              }
            }).expect("dir not found");

            curr_id = *child_id;
          }
        }
      },
      line if line.starts_with("$ ls") => {
        // noop
      }
      line => {
        let mut parts = line.split(' ');
        let first = parts.next().unwrap();
        let name = parts.next().unwrap();

        let new_child_id = nodes.len();

        match first {
          "dir" => {
            nodes.push(Node::Dir(Dir{
              id: new_child_id,
              name: name.to_string(),
              parent: Some(curr_id),
              children: vec![],
            }));
          },
          size => {
            nodes.push(Node::File(File{
              name: name.to_string(),
              size: size.parse::<u64>().unwrap(),
            }));
          }
        };

        let dir = nodes.get_mut(curr_id).unwrap().as_mut_dir();
        dir.children.push(new_child_id);
      },
    }
  }

  nodes
}

fn calc_sizes(nodes: &Vec<Node>) -> HashMap<usize,u64> {
  let mut map: HashMap<usize,u64> = HashMap::new();

  fn calc_size(node: &Node, nodes: &Vec<Node>, map: &mut HashMap<usize,u64>) -> u64 {
    match node {
      Node::Dir(dir) => {
        let mut size = 0;
        for child_id in &dir.children {
          let child = &nodes[*child_id];
          size += calc_size(child, nodes, map);
        }
        map.insert(dir.id, size);
        size
      },
      Node::File(file) => {
        file.size
      },
    }
  }

  calc_size(&nodes[0], &nodes, &mut map);

  map
}

fn pt1(dir_map: &HashMap<usize,u64>) -> u64 {
  dir_map.values().filter(|size| **size <= 100000).sum()
}

fn pt2(dir_map: &HashMap<usize,u64>) -> u64 {
  let unused = 70000000 - dir_map[&0];
  let remaining = 30000000 - unused;

  *dir_map.values().filter(|size| **size >= remaining).min().unwrap_or(&(0 as u64))
}

fn main() {
  let input = include_str!("../assets/input.txt");
  let nodes = build_nodes(input);
  let dir_map = calc_sizes(&nodes);

  nodes[0].pretty_print(&nodes);

  println!("[pt1]: {}", pt1(&dir_map));
  println!("[pt2]: {}", pt2(&dir_map));
}
