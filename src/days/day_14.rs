use std::{
    collections::HashMap,
    io::{BufWriter, Write},
    str::FromStr,
};

use aoclib_rs::{prep_io, printwriteln};

use {once_cell::sync::Lazy, regex::Regex};

const ORE: &str = "ORE";
const FUEL: &str = "FUEL";

type ChemAndAmt = (String, u64);

// map of chemical name to (amount of chemical name produced, vector of inputs)
type Deps = HashMap<String, (u64, Vec<ChemAndAmt>)>;

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 14).unwrap();

    let mut deps: Deps = HashMap::new();
    for line in contents {
        let (name, amt, d) = parse_dep(line);
        deps.insert(name, (amt, d));
    }

    let ore_per_fuel = part1(&mut writer, &deps);
    part2(&mut writer, &deps, ore_per_fuel);
}

fn parse_dep(line: &str) -> (String, u64, Vec<ChemAndAmt>) {
    const RE2_STR: &str = r"(\d+) (\w+)(, )?";

    let mut deps: Vec<ChemAndAmt> = Vec::new();

    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(format!(r"^(({})+) => (\d+) (\w+)$", RE2_STR).as_str()).unwrap());
    let Some(caps) = RE.captures(line) else {
        panic!("bad input");
    };

    let inputs = &caps[1];
    let amt = u64::from_str(&caps[6]).unwrap();
    let chem_name = &caps[7];

    static RE2: Lazy<Regex> = Lazy::new(|| Regex::new(RE2_STR).unwrap());
    let caps2: Vec<_> = RE2.captures_iter(inputs).collect();
    if caps2.is_empty() {
        panic!("bad input");
    }
    for cap2 in caps2 {
        let c2_amt = u64::from_str(&cap2[1]).unwrap();
        let c2_chem_name = &cap2[2];
        deps.push((c2_chem_name.to_string(), c2_amt));
    }

    (chem_name.to_string(), amt, deps)
}

fn part1<W: Write>(writer: &mut BufWriter<W>, deps: &Deps) -> u64 {
    let req_ore = get_req_ore(deps, 1 /* desired_fuel */, true /* verbose */);
    printwriteln!(writer, "{}", req_ore).unwrap();
    req_ore
}

fn get_req_ore(deps: &Deps, desired_fuel: u64, verbose: bool) -> u64 {
    let mut reqs: HashMap<String, u64> = HashMap::new();
    reqs.insert(FUEL.to_string(), desired_fuel);

    let mut available: HashMap<String, u64> = HashMap::new();

    while reqs.len() > 1 || !reqs.contains_key(ORE) {
        let mut keys = reqs.keys();
        let mut chem = keys.next().expect("").to_string();
        if chem == ORE {
            chem = keys.next().expect("").to_string();
        }

        let req_amt = *reqs.get(&chem).expect("");
        reqs.remove(&chem);

        if verbose {
            println!("examining {} (required: {})...", chem, req_amt);
        }

        let (amt, d) = deps.get(&chem).expect("");
        let multiplier = req_amt.div_ceil(*amt);
        if verbose {
            println!(
                "  production rule produces {} - therefore we need to multiply the recipe by {}",
                amt, multiplier
            );
        }

        let extra = multiplier * amt - req_amt;
        if extra > 0 {
            let mut total_extra = extra;
            if let Some(&prev_extra) = available.get(&chem) {
                total_extra += prev_extra;
            }
            available.insert(chem.clone(), total_extra);
            if verbose {
                println!(
                    "    but this leaves {} extra (total extra {}: {})",
                    extra,
                    chem.clone(),
                    total_extra
                );
            }
        }

        for dep in d {
            let mut need = dep.1 * multiplier;
            if verbose {
                println!("  we need {} more {}", need, dep.0);
            }
            if let Some(&avail) = available.get(&dep.0) {
                let need2 = need.saturating_sub(avail);
                let avail2 = avail - (need - need2);
                if avail2 == 0 {
                    available.remove(&dep.0);
                } else {
                    available.insert(dep.0.clone(), avail2);
                }
                need = need2;
                if verbose {
                    println!(
                        "    but we already have {} available, so we only actually need to acquire {}",
                        avail, need
                    );
                    println!("    this leaves {} {} available", avail2, dep.0);
                }
            }

            let mut total_need = need;
            if let Some(prev) = reqs.get(&dep.0) {
                total_need += prev;
            }
            if total_need == 0 {
                reqs.remove(&dep.0);
            } else {
                reqs.insert(dep.0.clone(), total_need);
            }
            if verbose {
                println!(
                    "  in total, we've now planned to acquire {} more {}",
                    total_need, dep.0
                );
            }
        }
    }

    *reqs.get(ORE).unwrap()
}

fn part2<W: Write>(writer: &mut BufWriter<W>, deps: &Deps, ore_per_fuel: u64) {
    const TRILLION: u64 = 1000000000000_u64;

    let mut lower_bound = TRILLION / ore_per_fuel;
    let mut upper_bound = lower_bound * 2;
    let mut max_under_trillion = 0;
    let mut answer = 0;

    while upper_bound > lower_bound {
        let mid_point = (lower_bound + upper_bound) / 2;
        let req_ore = get_req_ore(deps, mid_point, false /* verbose */);
        if req_ore > max_under_trillion && req_ore <= TRILLION {
            max_under_trillion = req_ore;
            answer = mid_point;
        }

        const TRILLION_PLUS_ONE: u64 = TRILLION + 1;
        match req_ore {
            TRILLION => break,
            u64::MIN..TRILLION => {
                if lower_bound == mid_point {
                    break;
                }
                lower_bound = mid_point;
            }
            TRILLION_PLUS_ONE..=u64::MAX => upper_bound = mid_point,
        }
    }

    println!("max used ore: {}", max_under_trillion);
    printwriteln!(writer, "{}", answer).unwrap();
}
