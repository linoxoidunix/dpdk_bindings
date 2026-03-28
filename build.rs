// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use anyhow::Result;
use bindgen::{Bindings, Builder};
use cc::Build;
use std::{env, path::Path};

#[cfg(target_os = "windows")]
fn os_build() -> Result<()> {
    use std::path::PathBuf;

    let out_dir_s: String = env::var("OUT_DIR")?;
    let out_dir: &Path = Path::new(&out_dir_s);

    let devxlib_path: String = env::var("DEVX_LIB_PATH")?;
    let libdpdk_path: String = env::var("LIBDPDK_PATH")?;

    let include_path: String = format!("{}{}", libdpdk_path, "\\include");
    let library_path: String = format!("{}{}", libdpdk_path, "\\lib");

    let libraries: Vec<&str> = vec![
        "librte_bitratestats",
        "librte_bus_auxiliary",
        "librte_bus_pci",
        "librte_bus_vdev",
        "librte_cfgfile",
        "librte_cmdline",
        "librte_common_iavf",
        "librte_common_idpf",
        "librte_common_mlx5",
        "librte_cryptodev",
        "librte_crypto_mlx5",
        "librte_dmadev",
        "librte_dma_idxd",
        "librte_dma_ioat",
        "librte_dma_skeleton",
        "librte_eal",
        "librte_ethdev",
        "librte_gpudev",
        "librte_gro",
        "librte_gso",
        "librte_hash",
        "librte_kvargs",
        "librte_latencystats",
        "librte_mbuf",
        "librte_mempool",
        "librte_mempool_ring",
        "librte_mempool_stack",
        "librte_meter",
        "librte_metrics",
        "librte_net",
        "librte_net_e1000",
        "librte_net_i40e",
        "librte_net_iavf",
        "librte_net_ice",
        "librte_net_ixgbe",
        "librte_net_mlx5",
        "librte_net_octeon_ep",
        "librte_net_vmxnet3",
        "librte_pci",
        "librte_rcu",
        "librte_ring",
        "librte_security",
        "librte_stack",
        "librte_telemetry",
        "librte_timer",
    ];

    let cflags: &str = "-mavx";

    // Step 1: Now that we've compiled and installed DPDK, point cargo to the libraries.
    println!("cargo:rustc-link-search={}", library_path);
    println!("cargo:rustc-link-search={}", devxlib_path);

    for lib in &libraries {
        println!("cargo:rustc-link-lib=static:-bundle,+whole-archive={}", lib);
    }

    println!("cargo:rustc-link-lib=dylib={}", "mlx5devx");
    println!("cargo:rustc-link-lib=dylib={}", "setupapi");
    println!("cargo:rustc-link-lib=dylib={}", "dbghelp");
    println!("cargo:rustc-link-lib=dylib={}", "mincore");

    // Step 2: Generate bindings for the DPDK headers.
    let bindings: Bindings = Builder::default()
        .clang_arg(&format!("-I{}", include_path))
        .clang_arg("-std=c11")
        .clang_arg("-mrtm")
        .clang_arg("-mcldemote")
        .allowlist_recursively(true)
        .allowlist_function("rte_auxiliary_register")
        .allowlist_function("rte_delay_us_block")
        .allowlist_function("rte_eal_init")
        .allowlist_function("rte_eth_conf")
        .allowlist_function("rte_eth_dev_configure")
        .allowlist_function("rte_eth_dev_count_avail")
        .allowlist_function("rte_eth_dev_flow_ctrl_get")
        .allowlist_function("rte_eth_dev_flow_ctrl_set")
        .allowlist_function("rte_eth_dev_get_mtu")
        .allowlist_function("rte_eth_dev_info_get")
        .allowlist_function("rte_eth_dev_is_valid_port")
        .allowlist_function("rte_eth_dev_set_mtu")
        .allowlist_function("rte_eth_dev_socket_id")
        .allowlist_function("rte_eth_dev_start")
        .allowlist_function("rte_eth_find_next_owned_by")
        .allowlist_function("rte_eth_link_get_nowait")
        .allowlist_function("rte_eth_macaddr_get")
        .allowlist_function("rte_eth_promiscuous_enable")
        .allowlist_function("rte_eth_rx_burst")
        .allowlist_function("rte_eth_rx_queue_setup")
        .allowlist_function("rte_eth_tx_burst")
        .allowlist_function("rte_eth_tx_queue_setup")
        .allowlist_function("rte_mempool_avail_count")
        .allowlist_function("rte_mempool_create_empty")
        .allowlist_function("rte_mempool_free")
        .allowlist_function("rte_mempool_in_use_count")
        .allowlist_function("rte_mempool_mem_iter")
        .allowlist_function("rte_mempool_obj_iter")
        .allowlist_function("rte_mempool_populate_default")
        .allowlist_function("rte_pktmbuf_clone")
        .allowlist_function("rte_pktmbuf_init")
        .allowlist_function("rte_pktmbuf_pool_create")
        .allowlist_function("rte_pktmbuf_pool_init")
        .allowlist_function("rte_pktmbuf_prepend")
        .allowlist_function("rte_socket_id")
        .allowlist_function("rte_strerror")
        .allowlist_type("rte_eth_fc_conf")
        .allowlist_type("rte_eth_rxconf")
        .allowlist_type("rte_eth_txconf")
        .allowlist_type("rte_ether_addr")
        .allowlist_type("rte_mbuf")
        .allowlist_type("rte_mempool")
        .allowlist_type("rte_pktmbuf_pool_private")
        .allowlist_var("RTE_ETH_DEV_NO_OWNER")
        .allowlist_var("RTE_ETH_LINK_FULL_DUPLEX")
        .allowlist_var("RTE_ETH_LINK_UP")
        .allowlist_var("RTE_ETH_MQ_RX_RSS")
        .allowlist_var("RTE_ETH_MQ_TX_NONE")
        .allowlist_var("RTE_ETH_RSS_IP")
        .allowlist_var("RTE_ETH_RX_OFFLOAD_IPV4_CKSUM")
        .allowlist_var("RTE_ETH_RX_OFFLOAD_TCP_CKSUM")
        .allowlist_var("RTE_ETH_RX_OFFLOAD_UDP_CKSUM")
        .allowlist_var("RTE_ETH_TX_OFFLOAD_MULTI_SEGS")
        .allowlist_var("RTE_ETH_TX_OFFLOAD_TCP_CKSUM")
        .allowlist_var("RTE_ETH_TX_OFFLOAD_UDP_CKSUM")
        .allowlist_var("RTE_ETHER_MAX_JUMBO_FRAME_LEN")
        .allowlist_var("RTE_ETHER_MAX_JUMBO_FRAME")
        .allowlist_var("RTE_ETHER_MAX_LEN")
        .allowlist_var("RTE_MAX_ETHPORTS")
        .allowlist_var("RTE_MBUF_DEFAULT_BUF_SIZE")
        .allowlist_var("RTE_PKTMBUF_HEADROOM")
        .clang_arg(cflags)
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_comments(false)
        .generate()?;
    let bindings_out: PathBuf = out_dir.join("bindings.rs");
    bindings.write_to_file(bindings_out)?;

    // Step 3: Compile a stub file so Rust can access `inline` functions in the headers
    // that aren't compiled into the libraries.
    let mut builder: Build = cc::Build::new();
    builder.opt_level(3);
    builder.flag("-std=c11");
    builder.flag("-march=native");
    builder.flag("-mavx");
    builder.flag("-mrtm");
    builder.flag("-mcldemote");
    builder.file("inlined.c");
    builder.include(include_path);
    builder.compile("inlined");

    Ok(())
}

use std::path::PathBuf;

#[cfg(target_os = "linux")]
fn os_build(install_dir: &Path, pkg_config_path: &str) -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    // 1. ПОЛУЧАЕМ ФЛАГИ ЧЕРЕЗ PKG-CONFIG
    // Мы принудительно устанавливаем PKG_CONFIG_PATH для вызова команды
    let mut pkg_cmd = Command::new("pkg-config");
    pkg_cmd.env("PKG_CONFIG_PATH", pkg_config_path);

    // CFLAGS (Инклюды)
    // 1. ПОЛУЧАЕМ CFLAGS (Инклюды)
    let cflags_out = new_pkg_config_cmd(pkg_config_path)
        .args(&["--cflags", "libdpdk"])
        .output()?;
    let mut cflags = String::from_utf8_lossy(&cflags_out.stdout).to_string();

    if cflags.trim().is_empty() {
        let fallback_inc = install_dir.join("include");
        if fallback_inc.exists() {
            cflags = format!("-I{}", fallback_inc.display());
            println!("cargo:warning=[FALLBACK] Using CFLAGS: {}", cflags);
        } else {
            panic!("DPDK include directory not found at {:?}", fallback_inc);
        }
    }

    // LIBS (Библиотеки)
    let libs_out = pkg_cmd.args(&["--libs", "libdpdk"]).output()?;
    let ldflags = String::from_utf8_lossy(&libs_out.stdout);

    let mut header_locations = vec![];
    for flag in cflags.split_whitespace() {
        if flag.starts_with("-I") {
            header_locations.push(flag[2..].to_string());
        }
    }

    let mut library_location = None;
    let mut lib_names = vec![];

    for flag in ldflags.split_whitespace() {
        if flag.starts_with("-L") {
            library_location = Some(flag[2..].to_string());
        } else if flag.starts_with("-l") {
            lib_names.push(flag[2..].to_string());
        }
    }

    // 2. ДОБАВЛЯЕМ СПЕЦИФИЧНЫЕ ЛИБЫ (MLX5, NUMA)
    // Внутри build.rs фичи проверяются через CARGO_FEATURE_<NAME>
    if env::var("CARGO_FEATURE_MLX5").is_ok() {
        lib_names.extend(vec![
            "rte_net_mlx5".to_string(),
            "rte_common_mlx5".to_string(),
            "ibverbs".to_string(),
            "mlx5".to_string(),
        ]);
    }
    lib_names.push("numa".to_string());

    lib_names.sort();
    lib_names.dedup();

    // 3. ЭКСПОРТ ДЛЯ CARGO (Чтобы зависимые проекты видели эти пути)
    let final_lib_dir =
        library_location.unwrap_or_else(|| install_dir.join("lib64").display().to_string());

    // ВОТ ЭТА СТРОКА КРИТИЧЕСКИ ВАЖНА для links = "dpdk"
    // Она создаст переменную DEP_DPDK_ROOT для всех, кто зависит от этого крейта
    println!("cargo:root={}", final_lib_dir);

    println!("cargo:rustc-link-search=native={}", final_lib_dir);

    // Этот rpath сработает для самого крейта биндингов
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", final_lib_dir);

    println!("cargo:warning=---------------------------------------");
    println!("cargo:warning=LNK: Linking DPDK libraries:");
    for lib in &lib_names {
        println!("cargo:warning=  |- lib{}", lib);
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }
    println!("cargo:warning=---------------------------------------");

    // Step 2: Generate bindings for the DPDK headers.
    let mut builder: Builder = Builder::default();
    for header_location in &header_locations {
        builder = builder.clang_arg(&format!("-I{}", header_location));
    }
    let bindings: Bindings = builder
        .allowlist_recursively(true)
        .allowlist_function("rte_auxiliary_register")
        .allowlist_function("rte_delay_us_block")
        .allowlist_function("rte_eal_init")
        .allowlist_function("rte_eth_conf")
        .allowlist_function("rte_eth_dev_configure")
        .allowlist_function("rte_eth_dev_count_avail")
        .allowlist_function("rte_eth_dev_flow_ctrl_get")
        .allowlist_function("rte_eth_dev_flow_ctrl_set")
        .allowlist_function("rte_eth_dev_get_mtu")
        .allowlist_function("rte_eth_dev_info_get")
        .allowlist_function("rte_eth_dev_is_valid_port")
        .allowlist_function("rte_eth_dev_set_mtu")
        .allowlist_function("rte_eth_dev_socket_id")
        .allowlist_function("rte_eth_dev_start")
        .allowlist_function("rte_eth_find_next_owned_by")
        .allowlist_function("rte_eth_link_get_nowait")
        .allowlist_function("rte_eth_macaddr_get")
        .allowlist_function("rte_eth_promiscuous_enable")
        .allowlist_function("rte_eth_rx_burst")
        .allowlist_function("rte_eth_rx_queue_setup")
        .allowlist_function("rte_eth_tx_burst")
        .allowlist_function("rte_eth_tx_queue_setup")
        .allowlist_function("rte_mempool_avail_count")
        .allowlist_function("rte_mempool_create_empty")
        .allowlist_function("rte_mempool_free")
        .allowlist_function("rte_mempool_in_use_count")
        .allowlist_function("rte_mempool_mem_iter")
        .allowlist_function("rte_mempool_obj_iter")
        .allowlist_function("rte_mempool_populate_default")
        .allowlist_function("rte_pktmbuf_clone")
        .allowlist_function("rte_pktmbuf_init")
        .allowlist_function("rte_pktmbuf_pool_create")
        .allowlist_function("rte_pktmbuf_pool_init")
        .allowlist_function("rte_pktmbuf_prepend")
        .allowlist_function("rte_socket_id")
        .allowlist_function("rte_strerror")
        .allowlist_function("rte_eth_tx_offload_tcp_tso_")
        .allowlist_function("rte_eal_remote_launch") // Запуск
        .allowlist_function("rte_eal_wait_lcore") // Ожидание завершения
        .allowlist_function("rte_eal_get_lcore_state") // Проверка состояния
        .allowlist_function("rte_get_next_lcore") // Перебор доступных ядер
        .allowlist_function("rte_get_main_lcore") // ID главного ядра
        .allowlist_function("rte_get_tsc_hz") // Частота (нужна для FastClock)
        .allowlist_function("rte_get_tsc_cycles") // Частота (нужна для FastClock)
        .allowlist_type("rte_eth_fc_conf")
        .allowlist_type("rte_eth_rxconf")
        .allowlist_type("rte_eth_txconf")
        .allowlist_type("rte_ether_addr")
        .allowlist_type("rte_mbuf")
        .allowlist_type("rte_mempool")
        .allowlist_type("rte_pktmbuf_pool_private")
        .allowlist_type("rte_gso_ctx")
        .allowlist_type("rte_lcore_state_t") // WAIT, RUNNING, FINISHED
        .allowlist_type("lcore_function_t") // Сигнатура функции payload
        .allowlist_var("RTE_ETH_DEV_NO_OWNER")
        .allowlist_var("RTE_ETH_LINK_FULL_DUPLEX")
        .allowlist_var("RTE_ETH_LINK_UP")
        .allowlist_var("RTE_ETH_MQ_RX_RSS")
        .allowlist_var("RTE_ETH_MQ_TX_NONE")
        .allowlist_var("RTE_ETH_RSS_IP")
        .allowlist_var("RTE_ETH_RX_OFFLOAD_IPV4_CKSUM")
        .allowlist_var("RTE_ETH_RX_OFFLOAD_TCP_CKSUM")
        .allowlist_var("RTE_ETH_RX_OFFLOAD_UDP_CKSUM")
        .allowlist_var("RTE_ETH_TX_OFFLOAD_MULTI_SEGS")
        .allowlist_var("RTE_ETH_TX_OFFLOAD_TCP_CKSUM")
        .allowlist_var("RTE_ETH_TX_OFFLOAD_UDP_CKSUM")
        .allowlist_var("RTE_ETHER_MAX_JUMBO_FRAME_LEN")
        .allowlist_var("RTE_ETHER_MAX_JUMBO_FRAME")
        .allowlist_var("RTE_ETHER_MAX_LEN")
        .allowlist_var("RTE_MAX_ETHPORTS")
        .allowlist_var("RTE_MBUF_DEFAULT_BUF_SIZE")
        .allowlist_var("RTE_PKTMBUF_HEADROOM")
        .allowlist_var("RTE_ETH_TX_OFFLOAD_TCP_TSO")
        .clang_arg("-mavx")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_comments(false)
        .generate()
        .unwrap_or_else(|e| panic!("Failed to generate bindings: {:?}", e));
    let bindings_out = out_dir.join("bindings.rs");
    bindings
        .write_to_file(bindings_out)
        .expect("Failed to write bindings");

    // Step 3: Compile a stub file so Rust can access `inline` functions in the headers
    // that aren't compiled into the libraries.
    let mut builder: Build = cc::Build::new();
    builder.opt_level(3);
    builder.pic(true);
    builder.flag("-march=native");
    builder.file("inlined.c");
    for header_location in &header_locations {
        builder.include(header_location);
    }
    builder.compile("inlined");
    Ok(())
}

fn get_dpdk_version_from_script(script_path: &Path) -> String {
    let content = std::fs::read_to_string(script_path)
        .expect("Failed to read build script to extract version");

    // Ищем строку DPDK_VER="XX.XX"
    content
        .lines()
        .find(|line| line.contains("DPDK_VER="))
        .and_then(|line| line.split('"').nth(1))
        .expect("Could not find DPDK_VER variable in bash script. Check the quotes!")
        .to_string()
}

// Вспомогательная функция для создания преднастроенной команды pkg-config
fn new_pkg_config_cmd(pkg_config_path: &str) -> Command {
    let mut cmd = Command::new("pkg-config");
    cmd.env("PKG_CONFIG_PATH", pkg_config_path);
    cmd
}

use std::fs;
use std::process::{Command, Stdio};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = Path::new(&manifest_dir);
    let build_script_path = project_root.join("scripts/build-install-dpdk.sh");

    let dpdk_ver = get_dpdk_version_from_script(&build_script_path);
    let home_dir = env::var("HOME").unwrap();
    let install_dir = Path::new(&home_dir).join(format!(".local/share/dpdk-{}", dpdk_ver));
    let pkg_config_path = install_dir.join("lib64/pkgconfig");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=inlined.c");
    println!("cargo:rerun-if-changed=wrapper.h");

    if !install_dir.exists() || !install_dir.join("lib64/pkgconfig/libdpdk.pc").exists() {
        let build_tmp = env::temp_dir().join(format!("dpdk_build_{}", dpdk_ver.replace('.', "_")));
        fs::create_dir_all(&install_dir).unwrap();

        let status = Command::new("bash")
            .arg(&build_script_path)
            .arg(&install_dir)
            .arg(&build_tmp)
            .current_dir(project_root)
            .stderr(Stdio::inherit())
            .status()
            .expect("Failed to start DPDK build script");

        if !status.success() {
            panic!("DPDK build script failed");
        }
    }

    let pkg_path_str = pkg_config_path.to_str().expect("Path is not UTF-8");
    env::set_var("DPDK_INSTALL_DIR", &install_dir);

    if let Err(e) = os_build(&install_dir, pkg_path_str) {
        panic!("Build failed: {:?}", e);
    }
}
