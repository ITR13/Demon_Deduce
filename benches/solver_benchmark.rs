use criterion::{criterion_group, criterion_main, Criterion};
use demon_deduce::roles::*;
use demon_deduce::{brute_force_solve, Role};

fn benchmark_scout_2(c: &mut Criterion) {
    use Role::*;

    let deck = vec![
        Lover,
        Confessor,
        Enlightened,
        Scout,
        Knight,
        Hunter,
        Bombardier,
        Wretch,
        Witch,
        Baa,
    ];

    let visible = vec![
        Some(Enlightened),
        Some(Wretch),
        Some(Knight),
        Some(Scout),
        Some(Confessor),
        Some(Hunter),
        Some(Knight),
        None,
    ];

    let confirmed = vec![None; visible.len()];

    let observed = vec![
        EnlightenedStatement::Clockwise.into(),
        RoleStatement::NoStatement,
        RoleStatement::NoStatement,
        ScoutStatement {
            distance: 2,
            role: Some(Role::Witch),
        }
        .into(),
        ConfessorStatement::IAmDizzy.into(),
        HunterStatement { distance: 1 }.into(),
        RoleStatement::NoStatement,
        RoleStatement::NoStatement,
    ];
    c.bench_function("scout_2_scenario", |b| {
        b.iter(|| {
            brute_force_solve(&deck, &visible, &confirmed, &observed, 5, 1, 1, 1, false);
        })
    });
}

fn benchmark_scout(c: &mut Criterion) {
    use Role::*;
    let deck = vec![Scout, Empress, Judge, Enlightened, Jester, Wretch, Witch];
    let visible = vec![
        Some(Wretch),
        Some(Empress),
        None,
        Some(Jester),
        Some(Scout),
        Some(Enlightened),
    ];
    let confirmed = vec![None; visible.len()];
    let observed = vec![
        RoleStatement::NoStatement,
        EmpressStatement {
            target_indexes: to_bitvec(vec![2, 3, 4]),
        }
        .into(),
        RoleStatement::NoStatement,
        RoleStatement::NoStatement,
        ScoutStatement {
            role: Some(Witch),
            distance: 3,
        }
        .into(),
        EnlightenedStatement::Clockwise.into(),
    ];

    c.bench_function("scout_scenario", |b| {
        b.iter(|| {
            brute_force_solve(&deck, &visible, &confirmed, &observed, 4, 1, 1, 0, false);
        })
    });
}

fn benchmark_jester(c: &mut Criterion) {
    use Role::*;
    let deck = vec![
        Gemcrafter, Jester, Empress, Hunter, Lover, Wretch, Minion, TwinMinion,
    ];
    let visible = vec![
        Some(Jester),
        None,
        Some(Lover),
        None,
        None,
        Some(Hunter),
        Some(Lover),
        None,
    ];
    let confirmed = vec![None; visible.len()];
    let observed = vec![
        JesterStatement {
            target_indexes: to_bitvec(vec![0, 2, 5]),
            evil_count: 1,
        }
        .into(),
        RoleStatement::NoStatement,
        LoverStatement { evil_count: 1 }.into(),
        RoleStatement::NoStatement,
        RoleStatement::NoStatement,
        HunterStatement { distance: 4 }.into(),
        LoverStatement { evil_count: 0 }.into(),
        RoleStatement::NoStatement,
    ];

    c.bench_function("jester_scenario", |b| {
        b.iter(|| {
            brute_force_solve(&deck, &visible, &confirmed, &observed, 5, 1, 2, 0, false);
        })
    });
}

fn benchmark_twin_and_medium(c: &mut Criterion) {
    use Role::*;
    let deck = vec![
        Judge,
        Lover,
        Gemcrafter,
        Enlightened,
        Medium,
        Wretch,
        Minion,
        TwinMinion,
    ];
    let visible = vec![
        Some(Medium),
        Some(Judge),
        Some(Gemcrafter),
        Some(Lover),
        Some(Gemcrafter),
        None,
        None,
    ];
    let confirmed = vec![None; visible.len()];
    let observed = vec![
        MediumStatement {
            target_index: 2,
            role: Gemcrafter,
        }
        .into(),
        JudgeStatement {
            target_index: 0,
            is_lying: true,
        }
        .into(),
        GemcrafterStatement { target_index: 0 }.into(),
        LoverStatement { evil_count: 1 }.into(),
        GemcrafterStatement { target_index: 3 }.into(),
        RoleStatement::NoStatement,
        RoleStatement::NoStatement,
    ];

    c.bench_function("twin_and_medium_scenario", |b| {
        b.iter(|| {
            brute_force_solve(&deck, &visible, &confirmed, &observed, 4, 1, 2, 0, false);
        })
    });
}

fn benchmark_empress_empress_empress(c: &mut Criterion) {
    let deck = vec![
        Role::Empress,
        Role::Empress,
        Role::Empress,
        Role::Empress,
        Role::Minion,
    ];
    let visible = vec![
        Some(Role::Empress),
        Some(Role::Empress),
        Some(Role::Empress),
        Some(Role::Empress),
        Some(Role::Empress),
    ];
    let confirmed = vec![None; visible.len()];
    let observed = vec![
        EmpressStatement {
            target_indexes: to_bitvec(vec![1, 2, 3]),
        }
        .into(),
        EmpressStatement {
            target_indexes: to_bitvec(vec![0, 3, 4]),
        }
        .into(),
        EmpressStatement {
            target_indexes: to_bitvec(vec![0, 3, 4]),
        }
        .into(),
        EmpressStatement {
            target_indexes: to_bitvec(vec![0, 1, 4]),
        }
        .into(),
        EmpressStatement {
            target_indexes: to_bitvec(vec![0, 1, 2]),
        }
        .into(),
    ];

    c.bench_function("empress_empress_empress_scenario", |b| {
        b.iter(|| {
            brute_force_solve(&deck, &visible, &confirmed, &observed, 4, 0, 1, 0, false);
        })
    });
}

criterion_group!(
    benches,
    benchmark_scout_2,
    benchmark_scout,
    benchmark_jester,
    benchmark_twin_and_medium,
    benchmark_empress_empress_empress
);
criterion_main!(benches);
