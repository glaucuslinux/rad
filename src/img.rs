// Copyright (c) 2018-2023, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Stdio;

#[cfg(test)]
use super::bootstrap;
use super::clean;
use super::constants;

use tokio::{fs, process::Command};

pub async fn radula_behave_bootstrap_cross_img() -> Result<(), Box<dyn Error>> {
    // A custom rsync function to prevent permission errors (`-S` is not used)
    let radula_behave_rsync = |source: PathBuf, destination: PathBuf| async move {
        Command::new(constants::RADULA_CERAS_RSYNC)
            .args(&[
                "-vaHAXx",
                source.to_str().unwrap_or_default(),
                destination.to_str().unwrap_or_default(),
                "--delete",
            ])
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();
    };

    let img = &String::from(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?)
            .join(constants::RADULA_FILE_GLAUCUS_IMG)
            .to_str()
            .unwrap_or_default(),
    );

    // Create a new IMG image file
    clean::radula_behave_remove_file_force(img).await?;
    Command::new(constants::RADULA_TOOTH_QEMU_IMG)
        .args(&[
            "create",
            "-f",
            "raw",
            img,
            constants::RADULA_FILE_GLAUCUS_IMG_SIZE,
        ])
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?;
    // Write mbr.bin (from SYSLINUX) to the first 440 bytes of the IMG image file
    Command::new(constants::RADULA_TOOTH_DD)
        .args(&[
            &format!(
                "if={}",
                Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
                    .join(constants::RADULA_DIRECTORY_GLAUCUS)
                    .join(constants::RADULA_FILE_SYSLINUX_MBR_BIN)
                    .to_str()
                    .unwrap_or_default()
            ),
            &format!("of={}", img),
            "conv=notrunc",
            "bs=440",
            "count=1",
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?;

    // Partition the IMG image file
    Command::new(constants::RADULA_TOOTH_PARTED)
        .args(&[
            constants::RADULA_TOOTH_PARTED_FLAGS,
            img,
            "mklabel",
            "msdos",
        ])
        .spawn()?
        .wait()
        .await?;
    Command::new(constants::RADULA_TOOTH_PARTED)
        .args(&[
            constants::RADULA_TOOTH_PARTED_FLAGS,
            "-a",
            "none",
            img,
            "mkpart",
            "primary",
            "ext4",
            "0",
            constants::RADULA_FILE_GLAUCUS_IMG_SIZE,
        ])
        .spawn()?
        .wait()
        .await?;
    Command::new(constants::RADULA_TOOTH_PARTED)
        .args(&[
            constants::RADULA_TOOTH_PARTED_FLAGS,
            "-a",
            "none",
            img,
            "set",
            "1",
            "boot",
            "on",
        ])
        .spawn()?
        .wait()
        .await?;

    Command::new(constants::RADULA_TOOTH_MODPROBE)
        .arg("loop")
        .spawn()?
        .wait()
        .await?;

    // Detach all used loop devices
    Command::new(constants::RADULA_TOOTH_LOSETUP)
        .arg("-D")
        .spawn()?
        .wait()
        .await?;

    // Find the first unused loop device
    let device = &String::from_utf8_lossy(
        &Command::new(constants::RADULA_TOOTH_LOSETUP)
            .arg("-f")
            .output()
            .await?
            .stdout,
    )
    .trim()
    .to_owned();

    let partition = &[device, "p1"].concat();

    // Associate the first unused loop device with the IMG image file
    Command::new(constants::RADULA_TOOTH_LOSETUP)
        .args(&[device, img])
        .spawn()?
        .wait()
        .await?;

    // Notify the kernel about the new partition on the IMG image file
    Command::new(constants::RADULA_TOOTH_PARTX)
        .args(&["-a", device])
        .spawn()?
        .wait()
        .await?;
    Command::new(constants::RADULA_TOOTH_MKE2FS)
        .args(&[constants::RADULA_TOOTH_MKE2FS_FLAGS, "ext4", partition])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?;

    let mount = &String::from(
        Path::new(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
            .join(constants::RADULA_PATH_MNT)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .to_str()
            .unwrap_or_default(),
    );

    fs::create_dir_all(mount).await?;

    Command::new(constants::RADULA_TOOTH_MOUNT)
        .args(&[partition, mount])
        .spawn()?
        .wait()
        .await?;

    // Remove `/lost+found` directory
    clean::radula_behave_remove_dir_all_force(
        Path::new(mount)
            .join(constants::RADULA_PATH_LOST_FOUND)
            .to_str()
            .unwrap_or_default(),
    )
    .await?;

    radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
        PathBuf::from(mount),
    )
    .await;

    let path = &String::from(
        Path::new(mount)
            .join(constants::RADULA_PATH_BOOT)
            .join(constants::RADULA_TOOTH_EXTLINUX)
            .to_str()
            .unwrap_or_default(),
    );

    // Install extlinux as the default bootloader
    fs::create_dir_all(path).await?;
    radula_behave_rsync(
        Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .join(constants::RADULA_FILE_SYSLINUX_EXTLINUX_CONF),
        PathBuf::from(path),
    )
    .await;
    Command::new(constants::RADULA_TOOTH_EXTLINUX)
        .args(&[constants::RADULA_TOOTH_EXTLINUX_FLAGS, path])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?;

    // Change ownerships
    Command::new(constants::RADULA_TOOTH_CHOWN)
        .args(&[constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS, "0:0", mount])
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?;
    Command::new(constants::RADULA_TOOTH_CHOWN)
        .args(&[
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
            "20:20",
            Path::new(mount)
                .join(constants::RADULA_PATH_ETC)
                .join(constants::RADULA_PATH_UTMPS)
                .to_str()
                .unwrap_or_default(),
        ])
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?;

    // Clean up
    Command::new(constants::RADULA_TOOTH_UMOUNT)
        .args(&[constants::RADULA_TOOTH_UMOUNT_FLAGS, mount])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?;
    Command::new(constants::RADULA_TOOTH_PARTX)
        .args(&["-d", partition])
        .spawn()?
        .wait()
        .await?;
    Command::new(constants::RADULA_TOOTH_LOSETUP)
        .args(&["-d", device])
        .spawn()?
        .wait()
        .await?;

    // Backup the new IMG image file
    radula_behave_rsync(
        PathBuf::from(img),
        PathBuf::from(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?),
    )
    .await;

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_bootstrap_cross_img() -> Result<(), Box<dyn Error>> {
    bootstrap::radula_behave_bootstrap_environment().await?;

    println!(
        "\nglaucus.img   :: {}",
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?)
            .join(constants::RADULA_FILE_GLAUCUS_IMG)
            .to_str()
            .unwrap_or_default(),
    );
    println!(
        "mbr.bin       :: {}",
        Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .join(constants::RADULA_FILE_SYSLINUX_MBR_BIN)
            .to_str()
            .unwrap_or_default(),
    );
    println!(
        "mount         :: {}",
        Path::new(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
            .join(constants::RADULA_PATH_MNT)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .to_str()
            .unwrap_or_default(),
    );
    println!(
        "boot          :: {}",
        Path::new(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
            .join(constants::RADULA_PATH_MNT)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .join(constants::RADULA_PATH_BOOT)
            .join(constants::RADULA_TOOTH_EXTLINUX)
            .to_str()
            .unwrap_or_default(),
    );
    println!(
        "extlinux.conf :: {}\n",
        Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .join(constants::RADULA_FILE_SYSLINUX_EXTLINUX_CONF)
            .to_str()
            .unwrap_or_default(),
    );

    assert!(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?)
            .join(constants::RADULA_FILE_GLAUCUS_IMG)
            .ends_with("glaucus.img")
    );
    assert!(Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
        .join(constants::RADULA_DIRECTORY_GLAUCUS)
        .join(constants::RADULA_FILE_SYSLINUX_MBR_BIN)
        .ends_with("syslinux/mbr.bin"));

    assert_eq!(
        Path::new(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
            .join(constants::RADULA_PATH_MNT)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .to_str()
            .unwrap_or_default(),
        "/mnt/glaucus"
    );

    assert!(Path::new(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
        .join(constants::RADULA_PATH_MNT)
        .join(constants::RADULA_DIRECTORY_GLAUCUS)
        .join(constants::RADULA_PATH_BOOT)
        .join(constants::RADULA_TOOTH_EXTLINUX)
        .ends_with("extlinux"));

    Ok(())
}
