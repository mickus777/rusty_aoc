#[derive(Debug)]
struct MonkeyData {
    items: Vec<u64>,
    test_value: u64,
    if_true: usize,
    if_false: usize,
    inspected: usize,
    operation: fn(u64) -> u64
}

fn create_mode1() -> Vec<MonkeyData> {
    let mut monkeys = Vec::new();

    monkeys.push(MonkeyData { items: Vec::from([79, 98]), test_value: 23, if_true: 2, if_false: 3, inspected: 0, operation: |val| val * 19 });
    monkeys.push(MonkeyData { items: Vec::from([54, 65, 75, 74]), test_value: 19, if_true: 2, if_false: 0, inspected: 0, operation: |val| val + 6 });
    monkeys.push(MonkeyData { items: Vec::from([79, 50, 97]), test_value: 13, if_true: 1, if_false: 3, inspected: 0, operation: |val| val * val });
    monkeys.push(MonkeyData { items: Vec::from([74]), test_value: 17, if_true: 0, if_false: 1, inspected: 0, operation: |val| val + 3 });

    monkeys
}

fn create_mode2() -> Vec<MonkeyData> {
    let mut monkeys = Vec::new();

    monkeys.push(MonkeyData { items: Vec::from([99, 67, 92, 61, 83, 64, 98]), test_value: 3, if_true: 4, if_false: 2, inspected: 0, operation: |val| val * 17 });
    monkeys.push(MonkeyData { items: Vec::from([78, 74, 88, 89, 50]), test_value: 5, if_true: 3, if_false: 5, inspected: 0, operation: |val| val * 11 });
    monkeys.push(MonkeyData { items: Vec::from([98, 91]), test_value: 2, if_true: 6, if_false: 4, inspected: 0, operation: |val| val + 4 });
    monkeys.push(MonkeyData { items: Vec::from([59, 72, 94, 91, 79, 88, 94, 51]), test_value: 13, if_true: 0, if_false: 5, inspected: 0, operation: |val| val * val });
    monkeys.push(MonkeyData { items: Vec::from([95, 72, 78]), test_value: 11, if_true: 7, if_false: 6, inspected: 0, operation: |val| val + 7 });
    monkeys.push(MonkeyData { items: Vec::from([76]), test_value: 17, if_true: 0, if_false: 2, inspected: 0, operation: |val| val + 8 });
    monkeys.push(MonkeyData { items: Vec::from([69, 60, 53, 89, 71, 88]), test_value: 19, if_true: 7, if_false: 1, inspected: 0, operation: |val| val + 5 });
    monkeys.push(MonkeyData { items: Vec::from([72, 54, 63, 80]), test_value: 7, if_true: 1, if_false: 3, inspected: 0, operation: |val| val + 3 });

    monkeys
}

fn inspect_items(monkey : &mut MonkeyData, low_worry : bool, limit : u64) -> Vec<(u64, usize)> {
    let mut items = Vec::new();

    while monkey.items.len() > 0 {
        let item = monkey.items.pop().unwrap();
        let mut new_item = (monkey.operation)(item);
        if low_worry {
            new_item /= 3;
        } else {
            new_item %= limit;
        }
        if new_item % monkey.test_value == 0 {
            items.push((new_item, monkey.if_true));
        } else {
            items.push((new_item, monkey.if_false));
        }
    }

    items
}

fn run_round(monkeys : &mut Vec<MonkeyData>, low_worry : bool, limit : u64) {
    for index in 0..monkeys.len() {
        let items = inspect_items(&mut monkeys[index], low_worry, limit);

        monkeys[index].inspected += items.len();

        for item in items {
            monkeys[item.1].items.push(item.0);
        }
    }
}

pub fn execute(input : &String, _file_name: &String) {
    let mode: usize = input.parse().unwrap();

    let mut monkeys : Vec<MonkeyData>;
    let mut limit = 0;
    let mut rounds = 0;
    if mode % 2 == 1 {
        monkeys = create_mode1();
        limit = 96577;
    } else {
        monkeys = create_mode2();
        limit = 9699690;
    }
    if mode < 3 {
        rounds = 20;
    } else {
        rounds = 10000;
    }

    for _i in 0..rounds {
        run_round(&mut monkeys, mode < 3, limit);
    }

    let mut inspects : Vec<usize> = Vec::new();
    for monkey in monkeys {
        inspects.push(monkey.inspected);
    }
    inspects.sort();
    let product = inspects.pop().unwrap() * inspects.pop().unwrap();

    println!("{}", product);
}