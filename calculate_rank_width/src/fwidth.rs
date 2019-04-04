/*
This implementation is based on bellow paper.
Oum, Sang-il. "Computing rank-width exactly." Information Processing Letters 109.13 (2009): 745-748.
https://mathsci.kaist.ac.kr/~sangil/pdf/2008exp.pdf
*/

use rayon::prelude::*;
use graph::*;

//smap matrix elements (i1, j1) and (i2, j2)
fn mat_swap_default<T: Default>(v: &mut Vec<Vec<T>>, i1: usize, j1: usize, i2: usize, j2: usize) {
    let mut t = T::default();
    std::mem::swap(&mut v[i1][j1], &mut t);
    std::mem::swap(&mut v[i2][j2], &mut t);
    std::mem::swap(&mut v[i1][j1], &mut t);
}

fn subset_all(v: usize, i: usize) -> Vec<u32> {
    return subset(v, i, 0);
}

fn subset(v: usize, i: usize, j: usize) -> Vec<u32> {
    if i == 0 {
        return vec![v as u32];
    }

    if (v >> j) == 0 {
        return Vec::new();
    }

    let mut k = j + 1;
    while false {
        if (v >> k & 1) == 1 {
            break;
        }

        k += 1;
    }
    let mut sub1 = subset(v, i, k);
    let mut sub2 = subset(v - (1 << j), i - 1, k);

    sub1.append(&mut sub2);
    return sub1;

}

impl Graph {
   //Calculate cut carving with subset X.
    fn cut_carving(&self, x: usize) -> usize {
        let mut cut : usize = 0;
        for (i,j) in self.edge.clone() {
            if ((x >> i & 1) ^ (x >> j & 1)) == 1 {
                cut += 1;
            }
        }

        return cut;
    }

   //Calculate cut rank with subset X.
    fn cut_rank(&self, x: usize) -> usize {
        let mut r = 0;
        let mut c = 0;
        let mut map = vec![0; self.v];
        for i in 0..self.v {
            if (x >> i & 1) == 1 {
                map[i] = r.clone();
                r += 1;
            }
            else {
                map[i] = c.clone();
                c += 1;
            }
        }


        let mut rho : Vec<Vec<bool>>;
        let mut rank : usize;
        let ro : usize;

        if c < r {
            rho = vec![vec![false; c]; r];
            for (i,j) in self.edge.clone() {
                if (x >> i & 1) == 1 && (x >> j & 1) == 0 {
                    rho[map[i]][map[j]] = true;
                }
                else if (x >> i & 1) == 0 && (x >> j & 1) == 1 {
                    rho[map[j]][map[i]] = true;
                }
            }

            ro = r.clone();
            rank = c.clone();
        }

        else {
            rho = vec![vec![false; r]; c];
            for (i,j) in self.edge.clone() {
                if (x >> i & 1) == 1 && (x >> j & 1) == 0 {
                    rho[map[j]][map[i]] = true;
                }
                else if (x >> i & 1) == 0 && (x >> j & 1) == 1 {
                    rho[map[i]][map[j]] = true;
                }
            }

            ro = c.clone();
            rank = r.clone();
        }


        let mut row = 0;
        while row < rank {
            if rho[row][row] {
                for col in 0..ro {
                    if col != row {
                        for i in 0..rank {
                            rho[col][i] = rho[col][i] ^ rho[row][i];
                        }
                    }
                }
                row += 1;
            }

            else {
                let mut reduce = true;
                for i in (row + 1)..ro {
                    if rho[i][row] {
                        for j in 0..rank {
                            mat_swap_default(&mut rho, row, j, i, j);
                        }
                        reduce = false;
                        break;
                    }
                }

                if reduce {
                    rank -= 1;
                    for i in 0..ro {
                        rho[i][row] = true && rho[i][rank];
                    }
                }
            }

        }

        return rank;
    }


    //Return rank-width of input graph is bigger than k or not.
    fn fwidth_bigger_k (&self, f : &str, k: usize) -> bool {
        let mut map_g = vec![false; 1 << self.v]; //storage of g_i
        let mut map_g_hat = vec![vec![0; 1 << self.v]; self.v + 1];
        let mut map_gg_hat = vec![vec![0; 1 << self.v]; self.v + 1];
        let mut map_gg = vec![vec![0; 1 << self.v]; self.v + 1];
        let mut target_subset = Vec::<u32>::new();

        let map_cut_rank : Vec<_>; //storage of cut rank of all subset
        //calculate cut rank of all subset
        if f == "rank" {
            map_cut_rank = (0..(1 << self.v) as u32).into_par_iter().map(|i| self.cut_rank(i as usize)).collect();
        }
        else {
            map_cut_rank = (0..(1 << self.v) as u32).into_par_iter().map(|i| self.cut_carving(i as usize)).collect();
        }

        for i in 0..self.v {
            map_g[1 << i] = map_cut_rank[1 << i] <= k;
        }

        //main loop for calculating g_i
        for i in 1..self.v {
            let mut sub = subset_all((1 << self.v) - 1, self.v - i - 1);
            target_subset.append(&mut sub.clone());

            //calculate \hat{g}_i (i, X) of all subset
            {
                let tmp : Vec<usize> = target_subset.par_iter().map(
                    |&y|
                    {
                        let x = y as usize;
                        let mut obj = 0;
                        for j in 0..self.v {
                            obj += x >> j & 1;
                        }
    
                        if obj >= i {
                            return subset_all(x, obj - i).par_iter().map(
                                |&s|
                                map_g[s as usize] as usize
                            ).sum();
                        }
                        
                        else {
                            return 0;
                        }
                    }
                ).collect();
    
                for j in 0..target_subset.len() {
                    map_g_hat[i][target_subset[j] as usize] = tmp[j];
                }
            }

            {
                let tmp : Vec<_> = target_subset.par_iter().map(
                    |&y|
                    {
                        let x = y as usize;
                        return (0..((i as u32) + 1)).into_par_iter().map(
                            |j|
                            map_g_hat[j as usize][x] * map_g_hat[i + 1 - (j as usize)][x]
                        ).sum();
                    }
                ).collect(); 

                for j in 0..target_subset.len() {
                    map_gg_hat[i][target_subset[j] as usize] = tmp[j as usize];
                }
            }

            //calculate (g_i * g_i) (i + 1, X) of all subset
            {
                let mut map_gg_i = vec![vec![0; 1 << self.v]; self.v + 1];
                target_subset.iter().for_each(
                    |&x|
                    map_gg_i[0][x as usize] = map_gg_hat[i][x as usize] as i64
                );

                for j in 0..self.v {
                    for &y in target_subset.iter() {
                        let x = y as usize;
                        if (x >> j & 1) == 1 {
                            map_gg_i[j + 1][x] = map_gg_i[j][x] - map_gg_i[j][x ^ (1 << j)];
                        }
                        else {
                            map_gg_i[j + 1][x] = map_gg_i[j][x];
                        }
                    }
                }

                map_gg[i] = (0..(1 << self.v) as u32).into_par_iter().map(|x| map_gg_i[self.v][x as usize]).collect();
            }


            for &y in sub.iter() {
                let x = y as usize;
                map_g[x] = (map_gg[i][x] != 0) && ((map_cut_rank[x] as usize <= k) || (x == (1 << self.v) -1));
            }
        }

        return map_g[(1 << self.v) - 1];
    }

    pub fn rank_width_bigger_k (&self, k : usize) -> bool {
        self.fwidth_bigger_k("rank", k)
    }

    pub fn carving_width_bigger_k (&self, k : usize) -> bool {
        self.fwidth_bigger_k("carving", k)
    }
}

