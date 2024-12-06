pub fn upward(map: &Vec<Vec<char>>, guard: &mut (usize, usize), distinct: &mut Vec<(usize, usize)>) -> bool {
    let mut stop = false;
    while guard.0 > 0 {
        if map[guard.0 - 1][guard.1] == '#' {
            break;
        }
        guard.0 -= 1;
        if !distinct.contains(guard) {
            distinct.push(*guard);
        }
    }
    if guard.0 == 0 {
        stop = true;
    }
    stop
}

pub fn downward(map: &Vec<Vec<char>>, guard: &mut (usize, usize), distinct: &mut Vec<(usize, usize)>) -> bool {
    let mut stop = false;
    while guard.0 < map.len() - 1 {
        if map[guard.0 + 1][guard.1] == '#' {
            break;
        }
        guard.0 += 1;
        if !distinct.contains(guard) {
            distinct.push(*guard);
        }
    }
    if guard.0 == map.len() - 1 {
        stop = true;
    }
    stop
}

pub fn leftward(map: &Vec<Vec<char>>, guard: &mut (usize, usize), distinct: &mut Vec<(usize, usize)>) -> bool {
    let mut stop = false;
    while guard.1 > 0 {
        if map[guard.0][guard.1 - 1] == '#' {
            break;
        }
        guard.1 -= 1;
        if !distinct.contains(guard) {
            distinct.push(*guard);
        }
    }
    if guard.1 == 0 {
        stop = true;
    }
    stop
}

pub fn rightward(map: &Vec<Vec<char>>, guard: &mut (usize, usize), distinct: &mut Vec<(usize, usize)>) -> bool {
    let mut stop = false;
    while guard.1 < map[guard.0].len() - 1 {
        if map[guard.0][guard.1 + 1] == '#' {
            break;
        }
        guard.1 += 1;
        if !distinct.contains(guard) {
            distinct.push(*guard);
        }
    }
    if guard.1 == map[guard.0].len() - 1 {
        stop = true;
    }
    stop
}