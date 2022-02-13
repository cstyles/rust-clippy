#![allow(unused_braces, unused_variables, dead_code)]
#![allow(
    clippy::collapsible_else_if,
    clippy::unused_unit,
    clippy::never_loop,
    clippy::let_unit_value
)]
#![warn(clippy::manual_let_else)]

fn g() -> Option<()> {
    None
}

fn main() {}

fn fire() {
    let v = if let Some(v_some) = g() { v_some } else { return };
    let v = if let Some(v_some) = g() {
        v_some
    } else {
        return;
    };

    let v = if let Some(v) = g() {
        // Blocks around the identity should have no impact
        {
            { v }
        }
    } else {
        // Some computation should still make it fire
        g();
        return;
    };

    // continue and break diverge
    loop {
        let v = if let Some(v_some) = g() { v_some } else { continue };
        let v = if let Some(v_some) = g() { v_some } else { break };
    }

    // panic also diverges
    let v = if let Some(v_some) = g() { v_some } else { panic!() };

    // abort also diverges
    let v = if let Some(v_some) = g() {
        v_some
    } else {
        std::process::abort()
    };

    // If whose two branches diverge also diverges
    let v = if let Some(v_some) = g() {
        v_some
    } else {
        if true { return } else { panic!() }
    };

    // Top level else if
    let v = if let Some(v_some) = g() {
        v_some
    } else if true {
        return;
    } else {
        panic!("diverge");
    };

    // All match arms diverge
    let v = if let Some(v_some) = g() {
        v_some
    } else {
        match (g(), g()) {
            (Some(_), None) => return,
            (None, Some(_)) => {
                if true {
                    return;
                } else {
                    panic!();
                }
            },
            _ => return,
        }
    };

    // Tuples supported for the declared variables
    let (v, w) = if let Some(v_some) = g().map(|v| (v, 42)) {
        v_some
    } else {
        return;
    };
}

fn not_fire() {
    let v = if let Some(v_some) = g() {
        // Nothing returned. Should not fire.
    } else {
        return;
    };

    let w = 0;
    let v = if let Some(v_some) = g() {
        // Different variable than v_some. Should not fire.
        w
    } else {
        return;
    };

    let v = if let Some(v_some) = g() {
        // Computation in then clause. Should not fire.
        g();
        v_some
    } else {
        return;
    };

    let v = if let Some(v_some) = g() {
        v_some
    } else {
        if false {
            return;
        }
        // This doesn't diverge. Should not fire.
        ()
    };

    let v = if let Some(v_some) = g() {
        v_some
    } else {
        // There is one match arm that doesn't diverge. Should not fire.
        match (g(), g()) {
            (Some(_), None) => return,
            (None, Some(_)) => return,
            (Some(_), Some(_)) => (),
            _ => return,
        }
    };

    let v = if let Some(v_some) = g() {
        v_some
    } else {
        // loop with a break statement inside does not diverge.
        loop {
            break;
        }
    };


    let v = if let Some(v_some) = g() {
        v_some
    } else {
        enum Uninhabited {}
        fn un() -> Uninhabited {
            panic!()
        }
        // Don't lint if the type is uninhabited but not !
        un()
    };

    fn question_mark() -> Option<()> {
        let v = if let Some(v) = g() {
            v
        } else {
            // Question mark does not diverge
            g()?
        };
        Some(v)
    }
}
