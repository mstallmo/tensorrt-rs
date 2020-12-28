use super::*;
use crate::dims::DimsCHW;
use crate::network::Network;
use crate::uff::{UffFile, UffInputOrder, UffParser};
use lazy_static::lazy_static;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

fn create_network(logger: &Logger) -> (Network, Builder) {
    let builder = Builder::new(&logger);
    let network = builder.create_network_v2(NetworkBuildFlags::DEFAULT);

    let uff_parser = UffParser::new();
    let dim = DimsCHW::new(1, 28, 28);

    uff_parser
        .register_input("in", dim, UffInputOrder::Nchw)
        .unwrap();
    uff_parser.register_output("out").unwrap();
    println!(
        "current dir: {}",
        std::env::current_dir().unwrap().display()
    );
    let uff_file = UffFile::new(Path::new("../assets/lenet5.uff")).unwrap();
    uff_parser.parse(&uff_file, &network).unwrap();

    (network, builder)
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
fn set_device_type_gpu() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let (network, builder) = create_network(&logger);

    let layer = network.get_layer(0);
    builder.set_device_type(&layer, DeviceType::GPU);

    assert_eq!(builder.get_device_type(&layer), DeviceType::GPU);
}

#[test]
fn is_device_type_set_true() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let (network, builder) = create_network(&logger);

    let layer = network.get_layer(0);
    builder.set_device_type(&layer, DeviceType::GPU);

    assert_eq!(builder.is_device_type_set(&layer), true);
}

#[test]
fn is_device_type_set_false() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let (network, builder) = create_network(&logger);

    let layer = network.get_layer(0);

    assert_eq!(builder.is_device_type_set(&layer), false);
}

#[cfg(target_arch = "aarch64")]
#[test]
fn set_device_type_DLA() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let (network, builder) = create_network(&logger);

    builder.set_fp16_mode(true);
    let layer = network.get_layer(0);
    builder.set_device_type(&layer, DeviceType::DLA);

    assert_eq!(builder.get_device_type(&layer), DeviceType::DLA);
}

#[cfg(target_arch = "aarch64")]
#[test]
fn set_default_device_type_GPU() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_fp16_mode(true);
    builder.set_default_device_type(DeviceType::GPU);

    assert_eq!(builder.get_default_device_type(), DeviceType::GPU);
}

#[test]
fn reset_device_type() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let (network, builder) = create_network(&logger);

    let layer = network.get_layer(0);
    builder.set_device_type(&layer, DeviceType::GPU);
    builder.reset_device_type(&layer);

    assert_eq!(builder.get_device_type(&layer), DeviceType::GPU);
}

#[cfg(target_arch = "aarch64")]
#[test]
fn can_run_on_dla() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let (network, builder) = create_network(&logger);

    let layer = network.get_layer(0);
    assert_eq!(builder.can_run_on_dla(&layer), true);
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

    let network = builder.create_network_v2(NetworkBuildFlags::EXPLICIT_BATCH);
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

#[test]
fn set_engine_capability() {
    let logger = match LOGGER.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let builder = Builder::new(&logger);

    builder.set_engine_capability(EngineCapability::Default);

    assert_eq!(builder.get_engine_capability(), EngineCapability::Default);
}
