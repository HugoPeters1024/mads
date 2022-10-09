use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::time::Instant;

const X : u32 = 32432;
const Y : u32 = 2310024; 


fn main() {
//    const X : u32 = 400;
//    const Y : u32 = 410; 
//    bfs(X, Y);
    bfs_pruned(X, Y);
    bfs_simul(X, Y);
}

fn bfs(x: u32, y: u32) {
    let start = Instant::now();
    let mut work : VecDeque<(u32, u32)> = VecDeque::from([(x, 0)]);
    let mut ticks = 0;

    while let Some((value,nr_steps)) = work.pop_front() {
        ticks +=1;

        if value == y {
            println!("BFS: answer is {} in {} ticks or {:?}", nr_steps, ticks, start.elapsed());
            break;
        }

        if value % 2 == 0 { work.push_back((value/2, nr_steps+1)); }
        if value % 3 == 0 { work.push_back((value/3, nr_steps+1)); }
        work.push_back((value-1, nr_steps+1));
        // prevents overflow
        work.push_back((value+1, nr_steps+1));
        if value < u32::MAX/5 { work.push_back((value*5, nr_steps+1)); }
    }
}

fn bfs_pruned(x: u32, y: u32) {
    let start = Instant::now();
    let mut work : VecDeque<(u32, u32)> = VecDeque::from([(x, 0)]);
    let mut visited : HashSet<u32> = HashSet::new();
    let mut ticks = 0;

    while let Some((value,nr_steps)) = work.pop_front() {
        ticks +=1;

        if visited.contains(&value) { continue; }
        visited.insert(value);

        if value == y {
            println!("BFS_pruned: answer is {} in {} ticks or {:?}", nr_steps, ticks, start.elapsed());
            break;
        }

        if value % 2 == 0 { work.push_back((value/2, nr_steps+1)); }
        if value % 3 == 0 { work.push_back((value/3, nr_steps+1)); }
        work.push_back((value-1, nr_steps+1));
        work.push_back((value+1, nr_steps+1));
        if value < u32::MAX/5 { work.push_back((value*5, nr_steps+1)); }
    }
}

#[derive(PartialEq, Clone)]
enum Direction {
    Forward,
    Backward,
}

fn bfs_simul(x: u32, y: u32) {
    let start = Instant::now();
    let mut work : VecDeque<(u32, u32, Direction)> = VecDeque::from([(x, 0, Direction::Forward), (y, 0, Direction::Backward)]);
    let mut forward_visited : HashMap<u32, u32> = HashMap::new();
    let mut backward_visited : HashMap<u32, u32> = HashMap::new();

    let mut ticks = 0;

    while let Some((value,nr_steps,dir)) = work.pop_front() {
        ticks += 1;

        match dir {
            Direction::Forward => {
                if let Some(nt) = backward_visited.get(&value) {
                    println!("BFS_simul: answer is {} in {} ticks or {:?}", nr_steps+nt, ticks, start.elapsed());
                    break;
                }

                if forward_visited.contains_key(&value) {
                    continue;
                }

                forward_visited.insert(value, nr_steps);

                if value % 2 == 0 { work.push_back((value/2, nr_steps+1, dir.clone())); }
                if value % 3 == 0 { work.push_back((value/3, nr_steps+1, dir.clone())); }
                if value < u32::MAX/5 { work.push_back((value*5, nr_steps+1, dir.clone())); }
                work.push_back((value+1, nr_steps+1, dir.clone()));
                work.push_back((value-1, nr_steps+1, dir.clone()));
            }
            Direction::Backward => {
                if let Some(nt) = forward_visited.get(&value) {
                    println!("BFS_simul: answer is {} in {} ticks or {:?}", nr_steps+nt, ticks, start.elapsed());
                    break;
                }

                if backward_visited.contains_key(&value) {
                    continue;
                }

                backward_visited.insert(value, nr_steps);

                if value % 5 == 0 { work.push_back((value/5, nr_steps+1, dir.clone())); }
                if value < u32::MAX/2 { work.push_back((value*2, nr_steps+1, dir.clone())); }
                if value < u32::MAX/3 { work.push_back((value*3, nr_steps+1, dir.clone())); }
                work.push_back((value+1, nr_steps+1, dir.clone()));
                work.push_back((value-1, nr_steps+1, dir.clone()));
            }
        }
    }
}
