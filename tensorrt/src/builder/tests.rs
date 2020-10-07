use super::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

#[test]
fn set_half2_mode_true() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_half2_mode(true);
    assert_eq!(builder.get_half2_mode(), true);
}

#[test]
fn set_half2_mode_false() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_half2_mode(false);
    assert_eq!(builder.get_half2_mode(), false);
}

#[test]
fn set_debug_sync_true() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_debug_sync(true);
    assert_eq!(builder.get_debug_sync(), true);
}

#[test]
fn set_debug_sync_false() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_debug_sync(false);
    assert_eq!(builder.get_debug_sync(), false);
}

#[test]
fn set_min_find_iterations() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_min_find_iterations(10);
    assert_eq!(builder.get_min_find_iterations(), 10);
}

#[test]
fn set_average_find_iterations() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_average_find_iterations(20);
    assert_eq!(builder.get_average_find_iterations(), 20);
}

#[test]
fn platform_has_fast_fp16() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    assert_eq!(builder.platform_has_fast_fp16(), true);
}

#[test]
fn platform_has_fast_int8() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    assert_eq!(builder.platform_has_fast_int8(), true);
}

#[test]
fn set_int8_mode_true() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_int8_mode(true);
    assert_eq!(builder.get_int8_mode(), true);
}

#[test]
fn set_int8_mode_false() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_int8_mode(false);
    assert_eq!(builder.get_int8_mode(), false);
}

#[test]
fn set_fp16_mode_true() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_fp16_mode(true);
    assert_eq!(builder.get_fp16_mode(), true);
}

#[test]
fn set_fp16_mode_false() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_fp16_mode(false);
    assert_eq!(builder.get_fp16_mode(), false);
}

#[test]
fn get_max_dla_batch_size() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    assert_eq!(builder.get_max_dla_batch_size(), 1);
}

#[test]
fn allow_gpu_fallback_true() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.allow_gpu_fallback(true);
}

#[test]
fn get_nb_dla_cores() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    assert_eq!(builder.get_nb_dla_cores(), 0);
}

#[test]
fn set_dla_core() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_dla_core(1);

    assert_eq!(builder.get_dla_core(), 1);
}

#[test]
fn reset_builder() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);
    assert_eq!(builder.get_half2_mode(), false);
    builder.set_half2_mode(true);

    let network = builder.create_network();
    assert_eq!(builder.get_half2_mode(), true);

    builder.reset(network);
    assert_eq!(builder.get_half2_mode(), false);
}

#[test]
fn set_strict_type_constraints_true() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_strict_type_constraints(true);

    assert_eq!(builder.get_strict_type_constraints(), true);
}

#[test]
fn set_strict_type_constraints_false() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_strict_type_constraints(false);

    assert_eq!(builder.get_strict_type_constraints(), false);
}

#[test]
fn set_refittable_true() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_refittable(true);

    assert_eq!(builder.get_refittable(), true);
}

#[test]
fn set_refittable_false() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_refittable(false);

    assert_eq!(builder.get_refittable(), false);
}
