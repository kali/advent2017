use itertools::Itertools;
use std::collections::HashMap;

type Tree<'a> = HashMap<&'a str, (usize, Vec<&'a str>)>;

fn rec_cum_weight<'a>(tree: &Tree<'a>, cum_weight: &mut HashMap<&'a str, usize>, name: &'a str) {
    let node = &tree[name];
    for kid in &node.1 {
        rec_cum_weight(tree, cum_weight, kid)
    }
    cum_weight.insert(
        name,
        node.0 + node.1.iter().map(|kid| cum_weight[kid]).sum::<usize>(),
    );
}

fn balance(
    tree: &Tree,
    cum_weight: &HashMap<&str, usize>,
    node: &str,
    delta: isize,
) -> Option<usize> {
    let (weight, kids) = &tree[node];
    let kid_weights = kids.iter().map(|k| cum_weight[k]).collect_vec();
    let wanted_weight = kid_weights.iter().sorted().duplicates().next().unwrap();
    if let Some(wrong_ix) = kid_weights.iter().position(|w| w != wanted_weight) {
        balance(
            tree,
            cum_weight,
            kids[wrong_ix],
            *wanted_weight as isize - kid_weights[wrong_ix] as isize,
        )
    } else {
        Some((*weight as isize + delta) as usize)
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let tree: Tree = input
        .lines()
        .map(|line| {
            let (node, kids) = line
                .split_once(" -> ")
                .map(|(node, kids)| (node, kids.split(", ").collect()))
                .unwrap_or((line, vec![]));
            let (name, weight) = node.split_once(' ').unwrap();
            let weight: usize = weight
                .trim_start_matches('(')
                .trim_end_matches(')')
                .parse()
                .unwrap();
            (name, (weight, kids))
        })
        .collect();
    let root = tree
        .keys()
        .find(|root| !tree.iter().any(|node| node.1 .1.contains(root)))
        .unwrap();
    dbg!(root);

    let mut cum_weight = HashMap::<&str, usize>::default();
    rec_cum_weight(&tree, &mut cum_weight, root);

    let p2 = balance(&tree, &cum_weight, root, 0).unwrap();
    dbg!(p2);
    Ok(())
}
