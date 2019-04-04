extern crate rayon;
pub mod fwidth;
pub mod graph;

#[cfg(test)]
mod tests {
    use fwidth::*;
    use graph::*;
    //case 1 (petersen graph, rank-width = 3)
    #[test]
    fn test_petersen () {
        let (info, data) = read_graph("sample/petersen.dat");
        let graph = Graph{v: info.1, e: info.0, edge: data};

        for i in 1..graph.v {
            if i < 3 {
                assert!(!graph.rank_width_bigger_k(i));
            }
            else {
                assert!(graph.rank_width_bigger_k(i));
            }
        }
    }

    //case 2 (ring graph, rank-width = 2)
    #[test]
    fn test_ring () {
        let (info, data) = read_graph("sample/ring.dat");
        let graph = Graph{v: info.1, e: info.0, edge: data};

        for i in 1..graph.v {
            if i < 2 {
                assert!(!graph.rank_width_bigger_k(i));
            }
            else {
                assert!(graph.rank_width_bigger_k(i));
            }
        }
    }

    //case 3 (clique graph, rank-width = 2)
    #[test]
    fn test_clique () {
        let (info, data) = read_graph("sample/clique.dat");
        let graph = Graph{v: info.1, e: info.0, edge: data};

        for i in 1..graph.v {
            if i < 1 {
                assert!(!graph.rank_width_bigger_k(i));
            }
            else {
                assert!(graph.rank_width_bigger_k(i));
            }
        }
    }
}
